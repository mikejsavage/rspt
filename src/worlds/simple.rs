#![ allow( dead_code ) ]

use std::num::Float;

use maths::color::RGB;
use maths::vec::Vec3;
use shapes::{ Plane, Sphere };
use entity::Entity;
use worlds::{ World, Intersection };
use materials::{ Matte, Mirror, Checkerboard };

pub struct SimpleWorld {
	entities : Vec< Entity >,
}

impl SimpleWorld {
	pub fn new( entities : Vec< Entity > ) -> SimpleWorld {
		return SimpleWorld { entities : entities };
	}

	pub fn cornell() -> SimpleWorld {
		let white = Matte::new( RGB( 0.75, 0.75, 0.75 ) );
		let red = Matte::new( RGB( 0.75, 0.25, 0.25 ) );
		let blue = Matte::new( RGB( 0.25, 0.25, 0.75 ) );
		let spec_green = Mirror::new( RGB( 0.25, 0.75, 0.25 ) );
		let spec_white = Mirror::new( RGB( 1.0, 1.0, 1.0 ) );

		let check1 = Matte::new( RGB( 0.75, 0.75, 0.75 ) );
		let check2 = Mirror::new( RGB( 0.6, 0.4, 0.2 ) );
		let check = Checkerboard::new( box check1, box check2 );

		return SimpleWorld::new( vec!(
			// right
			Entity::new( box red, box Plane { normal : Vec3::new( 0.0, 1.0, 0.0 ), d : -6.0 } ),
			// left
			Entity::new( box blue, box Plane { normal : Vec3::new( 0.0, -1.0, 0.0 ), d : -6.0 } ),
			// floor
			Entity::new( box check, box Plane { normal : Vec3::new( 0.0, 0.0, 1.0 ), d : -3.0 } ),
			// ceiling
			Entity::new( box white, box Plane { normal : Vec3::new( 0.0, 0.0, -1.0 ), d : -8.0 } ),
			// back
			Entity::new( box white, box Plane { normal : Vec3::new( -1.0, 0.0, 0.0 ), d : -10.0 } ),

			// spheres
			Entity::new( box spec_green, box Sphere { centre : Vec3::new( 8.0, 3.0, -1.0 ), radius : 2.0 } ),
			Entity::new( box spec_white, box Sphere { centre : Vec3::new( 6.0, -2.0, -1.0 ), radius : 2.0 } ),

			// light
			Entity::new_light( box white, box Sphere { centre : Vec3::new( 7.0, 0.0, 3.0 ), radius : 1.0 }, RGB( 1.0, 1.0, 1.0 ) ),
		) );
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
		let mut minT = Float::infinity();
		let mut minE = None;

		for entity in self.entities.iter() {
			let ot = entity.shape.intersection( start, dir );

			match ot {
				Some( t ) => {
					if t < minT && t > EPSILON {
						minT = t;
						minE = Some( entity );
					}
				},
				None => { }
			}
		}

		return minE.map( | entity | Intersection {
			other : entity,
			pos : start + dir * minT,
			t : minT,
		} );
	}
}
