use std::num::{ Float, sqrt };

use maths::vec::Vec3;
use shapes::Shape;

pub struct Sphere {
	centre : Vec3,
	radius : f64,
}

impl Shape for Sphere {
	// fn intersects( pos : Vec3, r : f64, start : Vec3, end : Vec3 ) -> bool {
	// 	let dir = end - start;
	// 	let len = dir.length();
        //
	// 	let ois = intersection( pos, r, start, dir / len );
        //
	// 	match ois {
	// 		None => return false,
	// 		Some( t ) => return t >= 0.0 && t <= len,
	// 	}
	// }

	fn intersection( &self, start : Vec3, dir : Vec3 ) -> Option< f64 > {
		let m = self.centre - start;

		let b = m.dot( dir );
		let c = m.sqlength() - self.radius * self.radius;

		if b < 0.0 && c > 0.0 {
			return None;
		}

		let disc = b * b - c;

		if disc < 0.0 {
			return None;
		}

		let sqrt_d = sqrt( disc );

		let tl = b - sqrt_d;
		let tf = b + sqrt_d;

		if tl < 0.0 {
			if tf < 0.0 {
				return None;
			}

			return Some( tf );
		}

		return Some( tl );
	}

	fn normal( &self, point : Vec3 ) -> Vec3 {
		return ( point - self.centre ) / self.radius;
	}

	// TODO
	fn uv( &self, point : Vec3 ) -> ( f64, f64 ) {
		return ( ( self.centre * self.radius + point ).x, 1.0 );
	}

	fn surface_area( &self ) -> f64 {
		return 4.0 * Float::pi() * self.radius * self.radius;
	}
}
