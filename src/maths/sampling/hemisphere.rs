use rand;
use std::f64::consts::{ PI, FRAC_1_PI };

use maths::vec::Vec3;
use maths::angle::{ Angle, Rad };
use maths::sampling::disk;

// sample the unit hemisphere uniformly
pub fn sample() -> Vec3 {
	let cos_theta = rand::random();
	let phi = Rad { r : 2.0 * PI * rand::random::< f64 >() };

	let sin_theta = f64::sqrt( 1.0 - cos_theta * cos_theta );
	let ( sin_phi, cos_phi ) = phi.sin_cos();

	return Vec3 {
		x : sin_theta * cos_phi,
		y : sin_theta * sin_phi,
		z : cos_theta,
	};
}

pub fn pdf() -> f64 {
	return 1.0 / ( PI * 2.0 );
}

// sample the unit hemisphere weighted by cos( theta )
pub fn sample_cos() -> Vec3 {
	let p = disk::sample();

	return Vec3 {
		x : p.x,
		y : p.y,
		z : ( 1.0 - ( p.x * p.x + p.y * p.y ) ).sqrt(),
	};
}

pub fn pdf_cos( v : Vec3 ) -> f64 {
	return v.z * FRAC_1_PI;
}
