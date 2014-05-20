use maths::vec::Vec3;
use shapes::Intersection;

pub mod simple;
pub mod union;

static EPSILON : f64 = 0.001;

pub trait World {
	fn intersects( &self, start : Vec3, dir : Vec3, len : f64 ) -> bool {
		match self.intersection( start + dir * EPSILON, dir ) {
			None => false,
			Some( is ) => is.t <= len - EPSILON,
		}
	}

	fn intersection< 'a >( &'a self, start : Vec3, dir : Vec3 ) -> Option< Intersection< 'a > >;
}
