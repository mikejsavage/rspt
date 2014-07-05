use std::num::Float;

#[ deriving( Clone, Show ) ]
pub struct RGB( pub f64, pub f64, pub f64 );

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
				let r_ = RGB::clamp( r ).powf( 1.0 / gam );
				let g_ = RGB::clamp( g ).powf( 1.0 / gam );
				let b_ = RGB::clamp( b ).powf( 1.0 / gam );

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
	pub fn scale( &self, s : f64 ) -> RGB {
		match self {
			&RGB( r, g, b ) => RGB( r * s, g * s, b * s )
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

impl Mul< RGB, RGB > for RGB {
	#[ inline ]
	fn mul( &self, &RGB( r_, g_, b_ ) : &RGB ) -> RGB {
		match self {
			&RGB( r, g, b ) => RGB( r * r_, g * g_, b * b_ )
		}
	}
}

impl Div< f64, RGB > for RGB {
	#[ inline ]
	fn div( &self, d : &f64 ) -> RGB {
		let s = 1.0 / *d;

		return self.scale( s );
	}
}
