use std::num::Float;

use maths::vec::Vec3;
use maths::rotation::Rotation;
use shapes::Shape;

pub struct Plane {
	pub normal : Vec3,
	pub d : f64,
}

impl Shape for Plane {
	// fn intersects( normal : Vec3, d : f64, start : Vec3, end : Vec3 ) -> bool {
	// 	return ( start.dot( normal ) - d ) * ( end.dot( normal ) - d ) <= 0.0;
	// }

	fn intersection( &self, start : Vec3, dir : Vec3 ) -> Option< f64 > {
		let t = ( self.d - start.dot( self.normal ) ) / ( dir.dot( self.normal ) );

		if t < 0.0 {
			return None;
		}

		return Some( t );
	}

	fn normal( &self, _ : Vec3 ) -> Vec3 {
		return self.normal;
	}

	fn uv( &self, point : Vec3 ) -> ( f64, f64 ) {
		let rot = Rotation::between( self.normal, Vec3::k() );
		let point_ = rot.apply( point );

		return ( point_.x, point_.y );
	}

	fn surface_area( &self ) -> f64 {
		return Float::infinity();
	}
}
