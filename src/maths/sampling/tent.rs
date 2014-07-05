use std::num::Float;
use std::rand::random;

pub fn tent( x : f64 ) -> f64 {
	let twox = 2.0 * x;

	if twox < 1.0 {
		return twox.sqrt() - 1.0;
	}

	return 1.0 - ( 2.0 - twox ).sqrt();
}

pub fn sample() -> f64 {
	return tent( random() );
}
