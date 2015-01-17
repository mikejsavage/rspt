#![ allow( dead_code ) ]

use maths::color::RGB;
use materials::{ Material, BxDF };
use materials::lambertian::Lambertian;

#[ derive( Clone ) ]
pub struct Matte {
	pub color : RGB,
}

impl Matte {
	pub fn new( color : RGB ) -> Matte {
		return Matte {
			color : color,
		};
	}
}

impl Material for Matte {
	fn get_bxdf( &self, _ : f64, _ : f64 ) -> Box< BxDF > {
		return Box::new( Lambertian::new( self.color ) );// as Box< BxDF >;
	}
}
