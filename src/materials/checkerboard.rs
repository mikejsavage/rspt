#![ allow( dead_code ) ]

use maths::color::RGB;
use materials::{ Material, BxDF };
use materials::lambertian::Lambertian;

pub struct Checkerboard {
	mat1 : ~Material : Send + Share,
	mat2 : ~Material : Send + Share,
}

impl Checkerboard {
	pub fn new( mat1 : ~Material : Send + Share, mat2 : ~Material : Send + Share ) -> Checkerboard {
		return Checkerboard {
			mat1 : mat1,
			mat2 : mat2,
		};
	}
}

impl Material for Checkerboard {
	fn get_bxdf( &self, u : f64, v : f64 ) -> ~BxDF {
		let u10 = ( u * 10.0 ).floor() as i64;
		let v10 = ( v * 10.0 ).floor() as i64;

		if ( u10 + v10 ) % 2 == 0 {
			return self.mat1.get_bxdf( u, v );
		}
		else {
			return self.mat2.get_bxdf( u, v );
		}
	}
}
