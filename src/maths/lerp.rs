use std::ops::{ Add, Mul };

pub fn lerp< T : Add< Output = T > + Mul< f64, Output = T > + Copy >( u : T, v : T, t : f64 ) -> T {
	return u * ( 1.0 - t ) + v * t;
}
