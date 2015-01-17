use std::num::Float;
use std::f64::consts::{ PI };
use std::rand::random;

use maths::vec::Vec3;
use maths::angle::{ Angle, Rad };

pub fn sample() -> Vec3 {
	let theta = Rad { r : 2.0 * random::< f64 >() * PI };
	let r = Float::sqrt( random::< f64 >() );

	let ( sin_theta, cos_theta ) = theta.sin_cos();

	return Vec3 {
		x : r * cos_theta,
		y : r * sin_theta,
		z : 0.0,
	};
}
