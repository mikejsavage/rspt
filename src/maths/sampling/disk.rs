use std::num::{ Float, sqrt };
use rand::random;

use maths::vec::Vec3;
use maths::angle::{ Angle, Rad };

pub fn sample() -> Vec3 {
	let theta = Rad { r : 2.0 * random::< f64 >() * Float::pi() };
	let r = sqrt( random::< f64 >() );

	let ( sin_theta, cos_theta ) = theta.sin_cos();

	return Vec3 {
		x : r * cos_theta,
		y : r * sin_theta,
		z : 0.0,
	};
}
