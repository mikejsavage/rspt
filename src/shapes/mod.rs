use maths::vec::Vec3;
use entity::Entity;

#[ allow( dead_code ) ]
mod sphere;
#[ allow( dead_code ) ]
mod plane;

pub struct Intersection< 'a > {
	other : &'a Entity,
	pos : Vec3,
	t : f64,
}

#[ deriving( Clone, Eq, Show ) ]
pub enum Shape {
	Plane { normal : Vec3, d : f64 },
	Sphere { pos : Vec3, r : f64 },
}

#[ allow( dead_code ) ]
impl Shape {
	pub fn intersects( &self, start : Vec3, end : Vec3 ) -> bool {
		match *self {
			Plane { normal, d } => plane::intersects( normal, d, start, end ),
			Sphere { pos, r } => sphere::intersects( pos, r, start, end ),
		}
	}

	pub fn intersection( &self, start : Vec3, dir : Vec3 ) -> Option< f64 > {
		let ot = match *self {
			Plane { normal, d } => plane::intersection( normal, d, start, dir ),
			Sphere { pos, r } => sphere::intersection( pos, r, start, dir ),
		};

		return match ot {
			None => None,
			Some( t ) => {
				if t < 0.001 {
					return None;
				}

				return ot;
			}
		};
	}

 	pub fn normal( &self, point : Vec3 ) -> Vec3 {
		match *self {
			Plane { normal, .. } => normal,
			Sphere { pos, r } => ( point - pos ) / r,
		}
	}

	pub fn uv( &self, point : Vec3 ) -> ( f64, f64 ) {
		match *self {
			Plane { normal, d } => plane::uv( normal, d, point ),
			Sphere { pos, r } => sphere::uv( pos, r, point ),
		}
	}

	pub fn surface_area( &self ) -> f64 {
		match *self {
			Plane { .. } => plane::surface_area(),
			Sphere { r, .. } => sphere::surface_area( r ),
		}
	}
}
