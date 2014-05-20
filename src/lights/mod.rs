#![ allow( dead_code ) ]

use maths::color::RGB;
use maths::vec::Vec3;

pub mod area;

pub trait Light {
	fn power( &self ) -> RGB;
	fn emittance( &self, normal : Vec3, dir : Vec3 ) -> RGB;

	fn is_delta( &self ) -> bool {
		return false;
	}
}
