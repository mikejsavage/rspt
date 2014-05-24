use std::num::{ Float, sqrt };

use maths::vec::Vec3;

pub fn intersects( pos : Vec3, r : f64, start : Vec3, end : Vec3 ) -> bool {
	let dir = end - start;
	let len = dir.length();

	let ois = intersection( pos, r, start, dir / len );

	match ois {
		None => return false,
		Some( t ) => return t >= 0.0 && t <= len,
	}
}

pub fn intersection( pos : Vec3, r : f64, start : Vec3, dir : Vec3 ) -> Option< f64 > {
	let m = pos - start;

	let b = m.dot( dir );
	let c = m.sqlength() - r * r;

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

// TODO
pub fn uv( pos : Vec3, r : f64, point : Vec3 ) -> ( f64, f64 ) {
	return ( ( pos * r + point ).x, 1.0 );
}

pub fn surface_area( r : f64 ) -> f64 {
	return 4.0 * Float::pi() * r * r;
}
