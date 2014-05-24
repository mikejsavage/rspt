#![ allow( dead_code ) ]

use maths::color::RGB;
use maths::vec::Vec3;

pub use materials::matte::Matte;
pub use materials::mirror::Mirror;
pub use materials::checkerboard::Checkerboard;

pub mod matte;
pub mod mirror;
pub mod checkerboard;
pub mod lambertian;
pub mod specular;

pub trait Material {
	fn get_bxdf( &self, u : f64, v : f64 ) -> ~BxDF;
}

pub trait Texture {
	fn eval( &self, u : f64, v : f64 ) -> RGB;
}

// used for BRDF/BTDF
#[ allow( unused_variable ) ]
pub trait BxDF {
	fn sample_f( &self, normal : Vec3, indicent : Vec3 ) -> ( Vec3, RGB, f64 );
}
