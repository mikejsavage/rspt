use std::num::{ Float, sqrt };
use rand::random;

use maths::vec::Vec3;
use maths::angle::{ Angle, Rad };
use maths::sampling::disk;

// sample the unit hemisphere uniformly
pub fn sample() -> Vec3 {
	let ctheta = random::< f64 >();
	let phi = Rad { r : 2.0 * Float::pi() * random::< f64 >() };

	let stheta = sqrt( 1.0 - ctheta * ctheta );
	let ( sphi, cphi ) = phi.sin_cos();

	return Vec3 {
		x : stheta * cphi,
		y : stheta * sphi,
		z : ctheta,
	};
}

// sample the unit hemisphere weighted by cos( theta )
pub fn sample_cos() -> Vec3 {
	let p = disk::sample();

	return Vec3 {
		x : p.x,
		y : p.y,
		z : sqrt( 1.0 - ( p.x * p.x + p.y * p.y ) ),
	};
}
