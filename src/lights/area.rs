#![ allow( dead_code ) ]

use std::f64::consts::PI;

use maths::color::RGB;
use maths::vec::Vec3;
use shapes::Shape;
use lights::Light;

#[ derive( Clone, Copy ) ]
pub struct AreaLight {
	color : RGB,
	area : f64,
}

impl AreaLight {
	pub fn new( color : RGB, shape : &Box< Shape + Sync + Send > ) -> AreaLight {
		return AreaLight {
			color : color,
			area : shape.surface_area(),
		};
	}
}

impl Light for AreaLight {
	fn power( &self ) -> RGB {
		return self.color * ( self.area * PI );
	}

	fn emittance( &self, normal : Vec3, dir : Vec3 ) -> RGB {
		if normal.dot( dir ) > 0.0 {
			return self.color;
		}

		return RGB::black();
	}
}
