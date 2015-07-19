#![ allow( dead_code ) ]

use std::f64;
use materials::{ Material, BxDF };

pub struct Checkerboard {
	mat1 : Box< Material + Send + Sync >,
	mat2 : Box< Material + Send + Sync >,
}

impl Checkerboard {
	pub fn new( mat1 : Box< Material + Send + Sync >, mat2 : Box< Material + Send + Sync > ) -> Checkerboard {
		return Checkerboard {
			mat1 : mat1,
			mat2 : mat2,
		};
	}
}

impl Material for Checkerboard {
	fn get_bxdf( &self, u : f64, v : f64 ) -> Box< BxDF > {
		let u10 = f64::floor( u * 10.0 ) as i64;
		let v10 = f64::floor( v * 10.0 ) as i64;
		let uc = u * 10.0 - u10 as f64;
		let vc = v * 10.0 - v10 as f64;

		if ( u10 + v10 ) % 2 == 0 {
			return self.mat1.get_bxdf( uc, vc );
		}
		else {
			return self.mat2.get_bxdf( uc, vc );
		}
	}
}
