#![ allow( dead_code ) ]

use std::num::Float;

use maths::color::RGB;
use maths::vec::Vec3;
use shapes::{ Intersection, Plane, Sphere };
use entity::Entity;
use worlds::World;
use materials::{ Matte, Mirror };

pub struct SimpleWorld {
	priv entities : ~[ Entity ],
}

impl SimpleWorld {
	pub fn new( entities : ~[ Entity ] ) -> SimpleWorld {
		return SimpleWorld { entities : entities };
	}

	pub fn cornell() -> SimpleWorld {
		let white = Matte::new( RGB( 0.75, 0.75, 0.75 ) );
		let red = Matte::new( RGB( 0.75, 0.25, 0.25 ) );
		let blue = Matte::new( RGB( 0.25, 0.25, 0.75 ) );
		let spec_green = Mirror::new( RGB( 0.25, 0.75, 0.25 ) );
		let spec_white = Mirror::new( RGB( 1.0, 1.0, 1.0 ) );

		return SimpleWorld::new( ~[
			Entity::new( ~red, ~Plane { normal : Vec3::new( 0, 1, 0 ), d : -6.0 } ),
			Entity::new( ~blue, ~Plane { normal : Vec3::new( 0, -1, 0 ), d : -6.0 } ),
			Entity::new( ~white, ~Plane { normal : Vec3::new( 0, 0, 1 ), d : -3.0 } ),
			// Entity::new( ~white, ~Plane { normal : Vec3::new( 0, 0, -1 ), d : -8.0 } ),
			Entity::new_light( ~white, ~Plane { normal : Vec3::new( 0, 0, -1 ), d : -8.0 }, RGB( 2.0, 2.0, 2.0 ) ),
			Entity::new( ~white, ~Plane { normal : Vec3::new( -1, 0, 0 ), d : -10.0 } ),
			Entity::new( ~spec_green, ~Sphere { pos : Vec3::new( 8, 3, -1 ), r : 2.0 } ),
			Entity::new( ~spec_white, ~Sphere { pos : Vec3::new( 6, -2, -1 ), r : 2.0 } ),
			// Entity::new_light( ~white, ~Sphere { pos : Vec3::new( 7, 0, 3 ), r : 1.0 }, RGB( 12.0, 12.0, 12.0 ) ),
		] );
	}
}

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
					if t < minT {
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
