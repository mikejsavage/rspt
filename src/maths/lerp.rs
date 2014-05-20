pub fn lerp< T : Add< T, T > + Mul< f64, T > >( u : &T, v : &T, t : f64 ) -> T {
	return *u * ( 1.0 - t ) + *v * t;
}
