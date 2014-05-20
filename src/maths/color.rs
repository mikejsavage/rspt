use std::f64::pow;

#[ deriving( Clone, Eq, Show ) ]
pub struct RGB( f64, f64, f64 );

#[ allow( dead_code ) ]
impl RGB {
	#[ inline ]
	fn clamp( x : f64 ) -> f64 {
		return x.max( 0.0 ).min( 1.0 );
	}

	#[ inline ]
	pub fn gamma( &self, gam : f64 ) -> ( f64, f64, f64 ) {
		match self {
			&RGB( r, g, b ) => {
				let r_ = pow( RGB::clamp( r ), 1.0 / gam );
				let g_ = pow( RGB::clamp( g ), 1.0 / gam );
				let b_ = pow( RGB::clamp( b ), 1.0 / gam );

				return ( r_, g_, b_ );
			}
		}
	}

	#[ inline ]
	pub fn invert( &self ) -> RGB {
		match self {
			&RGB( r, g, b ) => RGB( 1.0 - r, 1.0 - g, 1.0 - b )
		}
	}

	#[ inline ]
	pub fn scale( &self, RGB( r_, g_, b_ ) : RGB ) -> RGB {
		match self {
			&RGB( r, g, b ) => RGB( r * r_, g * g_, b * b_ )
		}
	}

	#[ inline ]
	pub fn black() -> RGB {
		return RGB( 0.0, 0.0, 0.0 );
	}
}

impl Add< RGB, RGB > for RGB {
	#[ inline ]
	fn add( &self, &RGB( r_, g_, b_ ) : &RGB ) -> RGB {
		match self {
			&RGB( r, g, b ) => RGB( r + r_, g + g_, b + b_ )
		}
	}
}

impl Mul< f64, RGB > for RGB {
	#[ inline ]
	fn mul( &self, s : &f64 ) -> RGB {
		match self {
			&RGB( r, g, b ) => RGB( r * *s, g * *s, b * *s )
		}
	}
}

impl Div< f64, RGB > for RGB {
	#[ inline ]
	fn div( &self, d : &f64 ) -> RGB {
		let s = 1.0 / *d;

		match self {
			&RGB( r, g, b ) => RGB( r * s, g * s, b * s )
		}
	}
}