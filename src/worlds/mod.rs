use maths::vec::Vec3;
use entity::Entity;

pub mod simple;
pub mod union;

pub struct Intersection< 'a > {
	pub other : &'a Entity,
	pub pos : Vec3,
	pub t : f64,
}

pub trait World {
	fn intersects( &self, start : Vec3, dir : Vec3, len : f64 ) -> bool {
		match self.intersection( start, dir ) {
			None => false,
			Some( is ) => is.t <= len,
		}
	}

	fn intersection< 'a >( &'a self, start : Vec3, dir : Vec3 ) -> Option< Intersection< 'a > >;
}
