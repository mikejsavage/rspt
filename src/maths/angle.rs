use std::num::{ ToPrimitive, Zero, zero, Float, sin, cos, tan, asin, acos, atan, atan2 };
use std::fmt;

#[ deriving( Clone, Eq, Ord ) ]
pub struct Deg { d : f64 }

#[ deriving( Clone, Eq, Ord ) ]
pub struct Rad { r : f64 }

pub trait Angle : Clone + Eq + Ord
	+ Equiv< Self >
	+ Zero
	+ Add< Self, Self > + Sub< Self, Self > + Neg< Self >
	+ Mul< f64, Self > + Div< f64, Self >
	+ fmt::Show
{
	fn deg( &self ) -> Deg;
	fn rad( &self ) -> Rad;

	fn turn() -> Self;

	#[ inline ]
	fn normalised( &self ) -> Self {
		if *self > Angle::turn() {
			return *self - Angle::turn();
		}

		if *self < zero() {
			return *self + Angle::turn();
		}

		return self.clone();
	}

	fn sin( &self ) -> f64;
	fn cos( &self ) -> f64;
	fn tan( &self ) -> f64;
	fn asin( n : f64 ) -> Self;
	fn acos( n : f64 ) -> Self;
	fn atan( n : f64 ) -> Self;
	fn atan2( y : f64, x : f64 ) -> Self;
	fn sin_cos( &self ) -> ( f64, f64 );
}

#[ allow( dead_code ) ]
impl Deg {
	#[ inline ]
	pub fn new< T : ToPrimitive >( x : T ) -> Deg {
		return Deg { d : x.to_f64().unwrap() };
	}
}

impl Angle for Deg {
	#[ inline ]
	fn deg( &self ) -> Deg {
		return *self;
	}

	#[ inline ]
	fn rad( &self ) -> Rad {
		return Rad { r : self.d * Float::pi() / 180.0 };
	}

	#[ inline ]
	fn turn() -> Deg {
		return Deg { d : 360.0 };
	}

	#[ inline ]
	fn sin( &self ) -> f64 {
		return sin( self.rad().r );
	}

	#[ inline ]
	fn cos( &self ) -> f64 {
		return cos( self.rad().r );
	}

	#[ inline ]
	fn tan( &self ) -> f64 {
		return tan( self.rad().r );
	}

	#[ inline ]
	fn asin( x : f64 ) -> Deg {
		return Rad { r : asin( x ) }.deg();
	}

	#[ inline ]
	fn acos( x : f64 ) -> Deg {
		return Rad { r : acos( x ) }.deg();
	}

	#[ inline ]
	fn atan( x : f64 ) -> Deg {
		return Rad { r : atan( x ) }.deg();
	}

	#[ inline ]
	fn atan2( y : f64, x : f64 ) -> Deg {
		return Rad { r : atan2( y, x ) }.deg();
	}

	#[ inline ]
	fn sin_cos( &self ) -> ( f64, f64 ) {
		return ( sin( self.rad().r ), cos( self.rad().r ) );
	}
}

impl Equiv< Deg > for Deg {
	#[ inline ]
	fn equiv( &self, other : &Deg ) -> bool {
		return self.normalised() == other.normalised();
	}
}

impl Zero for Deg {
	#[ inline ]
	fn zero() -> Deg {
		return Deg { d : 0.0 };
	}

	#[ inline ]
	fn is_zero( &self ) -> bool {
		return *self == zero();
	}
}

impl Add< Deg, Deg > for Deg {
	#[ inline ]
	fn add( &self, other : &Deg ) -> Deg {
		return Deg { d : self.d + other.d };
	}
}

impl Sub< Deg, Deg > for Deg {
	#[ inline ]
	fn sub( &self, other : &Deg ) -> Deg {
		return Deg { d : self.d - other.d };
	}
}

impl Mul< f64, Deg > for Deg {
	#[ inline ]
	fn mul( &self, s : &f64 ) -> Deg {
		return Deg { d : self.d * *s };
	}
}

impl Div< f64, Deg > for Deg {
	#[ inline ]
	fn div( &self, s : &f64 ) -> Deg {
		return Deg { d : self.d / *s };
	}
}

impl Neg< Deg > for Deg {
	#[ inline ]
	fn neg( &self ) -> Deg {
		return Deg { d : -self.d };
	}
}

impl fmt::Show for Deg {
	#[ inline ]
	fn fmt( &self, f : &mut fmt::Formatter ) -> fmt::Result {
		return write!( f.buf, "{} deg", self.d );
	}
}

#[ allow( dead_code ) ]
impl Rad {
	// having a ToPrimitive constructor makes no sense for radians
	#[ inline ]
	pub fn new( x : f64 ) -> Rad {
		return Rad { r : x };
	}
}

impl Angle for Rad {
	#[ inline ]
	fn deg( &self ) -> Deg {
		return Deg { d : self.r * 180.0 / Float::pi() };
	}

	#[ inline ]
	fn rad( &self ) -> Rad {
		return *self;
	}

	#[ inline ]
	fn turn() -> Rad {
		return Rad { r : Float::two_pi() };
	}

	#[ inline ]
	fn sin( &self ) -> f64 {
		return sin( self.r );
	}

	#[ inline ]
	fn cos( &self ) -> f64 {
		return cos( self.r );
	}

	#[ inline ]
	fn tan( &self ) -> f64 {
		return tan( self.r );
	}

	#[ inline ]
	fn asin( x : f64 ) -> Rad {
		return Rad { r : asin( x ) };
	}

	#[ inline ]
	fn acos( x : f64 ) -> Rad {
		return Rad { r : acos( x ) };
	}

	#[ inline ]
	fn atan( x : f64 ) -> Rad {
		return Rad { r : atan( x ) };
	}

	#[ inline ]
	fn atan2( y : f64, x : f64 ) -> Rad {
		return Rad { r : atan2( y, x ) };
	}

	#[ inline ]
	fn sin_cos( &self ) -> ( f64, f64 ) {
		return ( sin( self.r ), cos( self.r ) );
	}
}

impl Equiv< Rad > for Rad {
	#[ inline ]
	fn equiv( &self, other : &Rad ) -> bool {
		return self.normalised() == other.normalised();
	}
}

impl Zero for Rad {
	#[ inline ]
	fn zero() -> Rad {
		return Rad { r : 0.0 };
	}

	#[ inline ]
	fn is_zero( &self ) -> bool {
		return *self == zero();
	}
}

impl Add< Rad, Rad > for Rad {
	#[ inline ]
	fn add( &self, other : &Rad ) -> Rad {
		return Rad { r : self.r + other.r };
	}
}

impl Sub< Rad, Rad > for Rad {
	#[ inline ]
	fn sub( &self, other : &Rad ) -> Rad {
		return Rad { r : self.r - other.r };
	}
}

impl Mul< f64, Rad > for Rad {
	#[ inline ]
	fn mul( &self, s : &f64 ) -> Rad {
		return Rad { r : self.r * *s };
	}
}

impl Div< f64, Rad > for Rad {
	#[ inline ]
	fn div( &self, s : &f64 ) -> Rad {
		return Rad { r : self.r / *s };
	}
}

impl Neg< Rad > for Rad {
	#[ inline ]
	fn neg( &self ) -> Rad {
		return Rad { r : -self.r };
	}
}

impl fmt::Show for Rad {
	fn fmt( &self, f : &mut fmt::Formatter ) -> fmt::Result {
		return write!( f.buf, "{}π rad", self.r / Float::pi() );
	}
}
