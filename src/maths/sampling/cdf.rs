use std::rand::random;

pub struct CDF< T > {
	elems : Vec< T >,
	weights : Vec< f64 >,
	total_weight : f64,
}

impl< T > CDF< T > {
	pub fn new( elems : Vec< T >, weights : Vec< f64 > ) -> CDF< T > {
		let mut total = 0.0;

		for w in weights.iter() {
			total = total + *w;
		}

		return CDF {
			elems : elems,
			weights : weights,
			total_weight : total,
		};
	}

	pub fn sample< 'a >( &'a self ) -> &'a T {
		return self.sample_with( random::< f64 >() );
	}

	pub fn sample_with< 'a >( &'a self, u : f64 ) -> &'a T {
		let mut subtotal = 0.0;

		for ( x, weight ) in self.elems.iter().zip( self.weights.iter() ) {
			if subtotal + *weight >= self.total_weight * u {
				return x;
			}

			subtotal = subtotal + *weight;
		}

		panic!( "u must lie in [0..1]" );
	}
}
