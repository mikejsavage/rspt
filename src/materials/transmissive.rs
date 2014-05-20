use maths::vec::Vec3;
use maths::color::RGB;

pub struct Transmissive {
	transmission : RGB,
	index_out : f64,
	index_in : f64,
}

impl Transmissive {
	pub fn new( transmission : RGB, index_out : f64, index_in : f64 ) -> Transmissive {
		return Transmissive {
			transmission : transmission,
			index_out : index_out,
			index_in : index_in,
		};
	}
}

impl BxDF for Transmissive {
	fn sample_f( &self, normal : Vec3, indident : Vec3 ) -> ( Vec3, RGB ) {
		let cos_incident = normal.dot( incident )
		let entering = cos_incident > 0.0;

		let ( no, ni ) = if entering {
			( index_out, index_in )
		}
		else {
			( index_in, index_out )
		};

		let eta = ni / no;
		let sin2_incident = 1.0 - ( cos_incident * cos_incident );
		let sin2_transmit = eta * eta * sin2_incident;

		// total internal reflection?
		if sin2_transmit > 1.0 {
			return RGB::black();
		}

		// possible float error, pbrt has a max( 0.0, 1 - sin2t ) here
		assert!( 1.0 - sin2_transmit >= 0.0 );
		let cos_transmit = sqrt( 1.0 - sin2_transmit );
		let cos_scale = if entering { 1.0 } else { -1.0 };

		let f = fresnel_dielectric( cos_incident );

		let transmitted = Vec::i(); // TODO

		return ( Vec3::i(), RGB::black() );
	}
}
