use std::num::Float;

use maths::vec::Vec3;
use maths::rotation::Rotation;

pub fn intersects( normal : Vec3, d : f64, start : Vec3, end : Vec3 ) -> bool {
	return ( start.dot( normal ) - d ) * ( end.dot( normal ) - d ) <= 0.0;
}

pub fn intersection( normal : Vec3, d : f64, start : Vec3, dir : Vec3 ) -> Option< f64 > {
	let t = ( d - start.dot( normal ) ) / ( dir.dot( normal ) );

	if t < 0.0 {
		return None;
	}

	return Some( t );
}

pub fn uv( normal : Vec3, d : f64, point : Vec3 ) -> ( f64, f64 ) {
	let rot = Rotation::between( normal, Vec3::k() );
	let point_ = rot.apply( point );

	return ( point_.x, point_.y );
}

pub fn surface_area() -> f64 {
	return Float::infinity();
}
