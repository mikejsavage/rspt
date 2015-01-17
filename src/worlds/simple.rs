#![ allow( dead_code ) ]

use std::f64::INFINITY;

use maths::color::RGB;
use maths::vec::Vec3;
use maths::sampling::cdf::CDF;
use lights::Light;
use shapes::{ Plane, Sphere };
use entity::Entity;
use worlds::{ World, Intersection };
use materials::{ Matte, Mirror, Checkerboard };

pub struct SimpleWorld {
	entities : Vec< Entity >,
	// lights_cdf : CDF< Box< Light > >,
}

impl SimpleWorld {
	pub fn new( entities : Vec< Entity >, lights : Vec< Box< Light > > ) -> SimpleWorld {
		// let mut lights_powers : Vec< ( &Light, f64 ) > = vec!();
                //
		// for ent in entities.iter() {
		// 	for l in ent.light.iter() {
		// 		lights_powers.push( ( l as &Light, l.power() ) );
		// 	};
		// }
                //
		// for light in lights.iter() {
		// 	lights_powers.push( ( light, light.power() ) );
		// }

		return SimpleWorld {
			entities : entities,
			// lights_cdf : CDF::new( lights_powers ),
		};
	}

	pub fn cornell() -> SimpleWorld {
		let white = Matte::new( RGB( 0.75, 0.75, 0.75 ) );
		let red = Matte::new( RGB( 0.75, 0.25, 0.25 ) );
		let blue = Matte::new( RGB( 0.25, 0.25, 0.75 ) );
		let spec_green = Mirror::new( RGB( 0.25, 0.75, 0.25 ) );
		let spec_white = Mirror::new( RGB( 1.0, 1.0, 1.0 ) );

		let check1 = Matte::new( RGB( 0.75, 0.75, 0.75 ) );
		let check2 = Mirror::new( RGB( 0.6, 0.4, 0.2 ) );
		let check = Checkerboard::new( Box::new( check1 ), Box::new( check2 ) );

		return SimpleWorld::new( vec!(
			// right
			Entity::new( Box::new( red ), Box::new( Plane { normal : Vec3::new( 0.0, 1.0, 0.0 ), d : -6.0 } ) ),
			// left
			Entity::new( Box::new( blue ), Box::new( Plane { normal : Vec3::new( 0.0, -1.0, 0.0 ), d : -6.0 } ) ),
			// floor
			Entity::new( Box::new( check ), Box::new( Plane { normal : Vec3::new( 0.0, 0.0, 1.0 ), d : -3.0 } ) ),
			// ceiling
			Entity::new( Box::new( white.clone() ), Box::new( Plane { normal : Vec3::new( 0.0, 0.0, -1.0 ), d : -8.0 } ) ),
			// back
			Entity::new( Box::new( white.clone() ), Box::new( Plane { normal : Vec3::new( -1.0, 0.0, 0.0 ), d : -10.0 } ) ),

			// spheres
			Entity::new( Box::new( spec_green ), Box::new( Sphere { centre : Vec3::new( 8.0, 3.0, -1.0 ), radius : 2.0 } ) ),
			Entity::new( Box::new( spec_white ), Box::new( Sphere { centre : Vec3::new( 6.0, -2.0, -1.0 ), radius : 2.0 } ) ),

			// light
			Entity::new_light( Box::new( white ), Box::new( Sphere { centre : Vec3::new( 7.0, 0.0, 3.0 ), radius : 1.0 } ), RGB( 1.0, 1.0, 1.0 ) ),
		), vec!() );
	}
}

static EPSILON : f64 = 0.001;

impl World for SimpleWorld {
	// fn intersects( &self, start : Vec3, dir : Vec3 ) -> bool {
	// 	for shape in self.entities.iter() {
	// 		if shape.intersects( start, dir ) {
	// 			return true;
	// 		}
	// 	}
        //
	// 	return false;
	// }

	fn intersection< 'a >( &'a self, start : Vec3, dir : Vec3 ) -> Option< Intersection< 'a > > {
		let mut min_t = INFINITY;
		let mut nearest = None;

		for entity in self.entities.iter() {
			let ot = entity.shape.intersection( start, dir );

			match ot {
				Some( t ) => {
					if t < min_t && t > EPSILON {
						min_t = t;
						nearest = Some( entity );
					}
				},
				None => { }
			}
		}

		return nearest.map( | entity | Intersection {
			other : entity,
			pos : start + dir * min_t,
			t : min_t,
		} );
	}
}
