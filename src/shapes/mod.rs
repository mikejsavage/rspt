use maths::vec::Vec3;
use entity::Entity;

pub use shapes::sphere::Sphere;
pub use shapes::plane::Plane;

mod sphere;
mod plane;

pub struct Intersection< 'a > {
	other : &'a Entity,
	pos : Vec3,
	t : f64,
}

pub trait Shape {
	fn intersection( &self, start : Vec3, dir : Vec3 ) -> Option< f64 >;
	fn normal( &self, point : Vec3 ) -> Vec3;
	fn uv( &self, point : Vec3 ) -> ( f64, f64 );
	fn surface_area( &self ) -> f64;
}
