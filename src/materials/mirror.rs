#![ allow( dead_code ) ]

use maths::color::RGB;
use materials::{ Material, BxDF };
use materials::specular::Specular;

pub struct Mirror {
	color : RGB,
}

impl Mirror {
	pub fn new( color : RGB ) -> Mirror {
		return Mirror {
			color : color,
		};
	}
}

impl Material for Mirror {
	fn get_bxdf( &self, _ : f64, _ : f64 ) -> ~BxDF {
		return ~Specular::new( self.color ) as ~BxDF;
	}
}
