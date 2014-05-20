use std::num::sqrt;
use rand::random;

pub fn tent( x : f64 ) -> f64 {
	let twox = 2.0 * x;

	if twox < 1.0 {
		return sqrt( twox ) - 1.0;
	}

	return 1.0 - sqrt( 2.0 - twox );
}

pub fn sample() -> f64 {
	return tent( random() );
}
