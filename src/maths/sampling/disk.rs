use rand;
use std::f64;
use std::f64::consts::{ PI };

use maths::vec::Vec3;
use maths::angle::{ Angle, Rad };

pub fn sample() -> Vec3 {
	let theta = Rad { r : 2.0 * rand::random::< f64 >() * PI };
	let r = f64::sqrt( rand::random() );

	let ( sin_theta, cos_theta ) = theta.sin_cos();

	return Vec3 {
		x : r * cos_theta,
		y : r * sin_theta,
		z : 0.0,
	};
}
