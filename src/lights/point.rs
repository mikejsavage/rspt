#![ allow( dead_code ) ]

use std::num::Float;

use maths::color::RGB;
use maths::vec::Vec3;
use shapes::Shape;
use lights::Light;

pub struct PointLight {
	position : Vec3,
	color : RGB,
}

impl PointLight {
	pub fn new( position : Vec3, color : RGB ) -> PointLight {
		return PointLight {
			position : position,
			color : color,
		};
	}
}

impl Light for PointLight {
	fn power( &self ) -> RGB {
		return self.color * 4.0 * Float::pi();
	}

	fn emittance( &self, normal : Vec3, dir : Vec3 ) -> RGB {
		if normal.dot( dir ) > 0.0 {
			return self.color;
		}

		return RGB::black();
	}

	fn is_delta( &self ) -> bool {
		return true;
	}
}
