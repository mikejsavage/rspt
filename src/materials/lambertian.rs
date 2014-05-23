#![ allow( dead_code ) ]

use std::num::Float;

use maths::color::RGB;
use maths::vec::Vec3;
use maths::rotation::Rotation;
use maths::sampling::hemisphere;
use materials::BxDF;

pub struct Lambertian {
	reflectance : RGB,
}

impl Lambertian {
	pub fn new( reflectance : RGB ) -> Lambertian {
		return Lambertian {
			reflectance : reflectance,
		};
	}
}

impl BxDF for Lambertian {
	fn sample_f( &self, normal : Vec3, _ : Vec3 ) -> ( Vec3, RGB, f64 ) {
		let rot = Rotation::between( Vec3::k(), normal );
		let sample = hemisphere::sample_cos();
		let out = rot.apply( sample );
		let pdf : f64 = sample.z * Float::frac_1_pi();

		return ( out, self.reflectance, pdf );

	}
}
