#![ feature( struct_variant ) ]

extern crate native;
extern crate sync;
extern crate rand;

extern crate gl;
extern crate glfw;

use std::io::timer::sleep;
use sync::Arc;

use maths::color::RGB;
use maths::vec::Vec3;
use maths::angle::{ Angle, Deg };
use maths::lerp::lerp;
use maths::sampling::tent;
use lights::Light;
use worlds::World;
use worlds::simple::SimpleWorld;

use glfw::Context;

mod maths;
mod shapes;
mod worlds;
mod lights;
mod materials;
mod entity;

static HEIGHT : uint = 768;
static WIDTH : uint = 1024;
static ASPECT_RATIO : f64 = WIDTH as f64 / HEIGHT as f64;
// static THREADS : uint = 253;
static THREADS : uint = 4;

static mut pixels : [ ( u32, RGB ), ..WIDTH * HEIGHT ] = [ ( 0, RGB( 0.0, 0.0, 0.0 ) ), ..WIDTH * HEIGHT ];

fn pixelhash( thread : uint ) -> ~[ uint ] {
	let mut ps = ~[ ];
	let mut p = thread;

	while p < WIDTH * HEIGHT {
		ps.push( p );
		p += THREADS;
	}

	return ps;
}

fn coord() -> ( Vec3, Vec3, Vec3 ) {
	let uppy = Vec3::new( 0, 0, 1 );
	let forward = Vec3::new( 1, 0, 0 );

	let left = uppy.cross( forward ).normalised();
	let up = forward.cross( left );

	return ( forward, up, left );
}

fn pixelIndicesToPoints(
    pixels : &~[ uint ],
    tl : Vec3, // top left of view frustrum
    tr : Vec3, // top right
    br : Vec3 // bottom right
) -> ~[ Vec3 ] {
	return pixels.iter().map( | pixel | {
		let x = pixel % WIDTH;
		let y = pixel / WIDTH;

		let tx = x as f64 / WIDTH as f64;
		let ty = y as f64 / HEIGHT as f64;

		let point = lerp( &tl, &tr, tx ) + ( br - tr ) * ty;

		return point;
	} ).collect();
}

// TODO: handle hitting the back of a shape
fn irradiance( world : &World, start : Vec3, dir : Vec3, depth : uint ) -> RGB {
	if depth > 5 {
		return RGB::black();
	}

	let ois = world.intersection( start, dir );

	return ois.map_or( RGB::black(), | is | {
		let normal = is.other.shape.normal( is.pos );
		let ( u, v ) = is.other.shape.uv( is.pos );
		let bxdf = is.other.material.get_bxdf( u, v );

		let ( outgoing, reflectance, pdf ) = bxdf.sample_f( normal, -dir );

		let emittance = is.other.light.map_or( RGB::black(), | light | {
			return ( &light as &Light ).emittance( normal, -dir );
		} );

		return emittance + irradiance( world, is.pos, outgoing, depth + 1 ).scale( reflectance ) * normal.dot( outgoing );
	} );
}

fn sampler( indices : &[ uint ], eye : Vec3, centres : &[ Vec3 ], world : &World, up : Vec3, left : Vec3 ) {
	let mut samples = 0;

	loop {
		samples += 1;

		for i in range( 0, indices.len() ) {
			let dx = left * tent::sample();
			let dy = up * tent::sample();
			let ray = ( centres[ i ] + dx + dy - eye ).normalised();
			let color = irradiance( world, eye, ray, 0 );

			unsafe {
				let p = indices[ i ];
				pixels[ p ] = ( samples, pixels[ p ].val1() + color );
			}
		}

		if samples % 10 == 0 {
			println!( "{}", samples );
		}
	}
}

fn open_window() -> ( glfw::Glfw, glfw::Window, Receiver< glfw::Error > ) {
	let ( glfw, errors ) = glfw::init().unwrap();
	glfw::fail_on_error( &errors );

	glfw.window_hint( glfw::Resizable( false ) );

	let ( window, _ ) = glfw.create_window( WIDTH as u32, HEIGHT as u32, "pt", glfw::Windowed ).unwrap();
	window.make_current();

	gl::load_with( | s | glfw.get_proc_address( s ) );

	gl::Ortho( 0.0, WIDTH as f64, HEIGHT as f64, 0.0, -1.0, 1.0 );

	return ( glfw, window, errors );
}

fn main() {
	let world = SimpleWorld::cornell();
	let world_arc = Arc::new( world );

	let eye = Vec3::new( 0, 0, 0 );
	let ( forward, up, left ) = coord();

	let fov = Deg::new( 90 );
	let focal_length = 0.1;
	let focal_height = focal_length * ( fov / 2.0 ).tan();

	let top_left = eye + forward * focal_length + up * focal_height + left * focal_height * ASPECT_RATIO;
	let top_right = eye + forward * focal_length + up * focal_height - left * focal_height * ASPECT_RATIO;
	let bottom_right = eye + forward * focal_length - up * focal_height - left * focal_height * ASPECT_RATIO;

	let up_pixel = up * focal_height / HEIGHT as f64;
	let left_pixel = left * focal_height * ASPECT_RATIO / WIDTH as f64;

	let hashes = std::slice::from_fn( THREADS, pixelhash );

	let centres : ~[ ~[ Vec3 ] ] = hashes.iter().map( | ps | {
		return pixelIndicesToPoints( ps, top_left, top_right, bottom_right );
	} ).collect();

	let hashes_arc = Arc::new( hashes );
	let centres_arc = Arc::new( centres );

	for n in range( 0, THREADS ) {
		let hashes_arc = hashes_arc.clone();
		let centres_arc = centres_arc.clone();
		let world_arc = world_arc.clone();

		spawn( proc() {
			let hashes = hashes_arc.deref();
			let centres = centres_arc.deref();
			let world = world_arc.deref();

			sampler( hashes[ n ], eye, centres[ n ], world, up_pixel, left_pixel );
		} );
	}

	let ( glfw, window, errors ) = open_window();

	while !window.should_close() {
		glfw.poll_events();
		glfw::fail_on_error( &errors );

		gl::Begin( gl::POINTS );

		unsafe {
			for i in range( 0, pixels.len() ) {
				let x = i % WIDTH;
				let y = i / WIDTH;

				let ( samples, color ) = pixels[ i ];

				let ( r, g, b ) = ( color / samples as f64 ).gamma( 2.2 );

				gl::Color3d( r, g, b );
				gl::Vertex2i( x as i32, y as i32 );
			}
		}

		gl::End();

		window.swap_buffers();
		sleep( 250 );
	}
}

#[ start ]
fn start( argc : int, argv : **u8 ) -> int {
	return native::start( argc, argv, main );
}
