// TODO: conductive fresnel reflection

use maths::color::RGB;
use maths::vec::Vec3;
use materials::BxDF;

pub struct Specular {
	reflectance : RGB,
}

impl Specular {
	pub fn new( reflectance : RGB ) -> Specular {
		return Specular {
			reflectance : reflectance,
		};
	}
}

impl BxDF for Specular {
	fn sample_f( &self, normal : Vec3, incident : Vec3 ) -> ( Vec3, RGB, f64 ) {
		let reflected = -incident + normal * 2.0 * normal.dot( incident );

		return ( reflected, self.reflectance / normal.dot( incident ), 1.0 );
	}
}
