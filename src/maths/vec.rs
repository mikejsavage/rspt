use std::num::{ sqrt, rsqrt };
use std::fmt;

#[ deriving( Clone, Eq ) ]
pub struct Vec3 {
	x : f64,
	y : f64,
	z : f64,
}

#[ allow( dead_code ) ]
impl Vec3 {
	#[ inline ]
	pub fn new< S : ToPrimitive, T : ToPrimitive, U : ToPrimitive >( x : S, y : T, z : U ) -> Vec3 {
		return Vec3 {
			x : x.to_f64().unwrap(),
			y : y.to_f64().unwrap(),
			z : z.to_f64().unwrap(),
		};
	}

	#[ inline ]
	pub fn length( &self ) -> f64 {
		return sqrt( self.sqlength() );
	}

	#[ inline ]
	pub fn sqlength( &self ) -> f64 {
		return self.x * self.x + self.y * self.y + self.z * self.z;
	}

	#[ inline ]
	pub fn normalised( &self ) -> Vec3 {
		return self / self.length();
	}

	#[ inline ]
	pub fn fastnormalised( &self ) -> Vec3 {
		return self * rsqrt( self.sqlength() );
	}

	#[ inline ]
	pub fn dir_length( &self ) -> ( Vec3, f64 ) {
		let len = self.length();

		return ( self / len, len );
	}

	#[ inline ]
	pub fn dot( &self, other : Vec3 ) -> f64 {
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}

	#[ inline ]
	pub fn cross( &self, other : Vec3 ) -> Vec3 {
		return Vec3 {
			x : self.y * other.z - self.z * other.y,
			y : -( self.x * other.z - self.z * other.x ),
			z : self.x * other.y - self.y * other.x,
		};
	}

	// Hadamard product
	#[ inline ]
	pub fn hprod( &self, other : &Vec3 ) -> Vec3 {
 		return Vec3 {
 			x : self.x * other.x,
 			y : self.y * other.y,
 			z : self.z * other.z
 		};
	}

	#[ inline ]
	pub fn hdiv( &self, other : &Vec3 ) -> Vec3 {
 		return Vec3 {
 			x : self.x / other.x,
 			y : self.y / other.y,
 			z : self.z / other.z
 		};
	}

	#[ inline ]
	pub fn i() -> Vec3 {
		return Vec3 { x : 1.0, y : 0.0, z : 0.0 };
	}

	#[ inline ]
	pub fn j() -> Vec3 {
		return Vec3 { x : 0.0, y : 1.0, z : 0.0 };
	}

	#[ inline ]
	pub fn k() -> Vec3 {
		return Vec3 { x : 0.0, y : 0.0, z : 1.0 };
	}
}

impl Add< Vec3, Vec3 > for Vec3 {
	#[ inline ]
	fn add( &self, other : &Vec3 ) -> Vec3 {
		return Vec3 {
			x : self.x + other.x,
			y : self.y + other.y,
			z : self.z + other.z,
		};
	}
}

impl Sub< Vec3, Vec3 > for Vec3 {
	#[ inline ]
	fn sub( &self, other : &Vec3 ) -> Vec3 {
		return Vec3 {
			x : self.x - other.x,
			y : self.y - other.y,
			z : self.z - other.z,
		};
	}
}

impl Mul< f64, Vec3 > for Vec3 {
	#[ inline ]
	fn mul( &self, s : &f64 ) -> Vec3 {
		return Vec3 {
			x : self.x * *s,
			y : self.y * *s,
			z : self.z * *s,
		};
	}
}

impl Div< f64, Vec3 > for Vec3 {
	#[ inline ]
	fn div( &self, d : &f64 ) -> Vec3 {
		let s = 1.0 / *d;

		return Vec3 {
			x : self.x * s,
			y : self.y * s,
			z : self.z * s,
		};
	}
}

impl Neg< Vec3 > for Vec3 {
	#[ inline ]
	fn neg( &self ) -> Vec3 {
		return Vec3 {
			x : -self.x,
			y : -self.y,
			z : -self.z,
		};
	}
}

impl fmt::Show for Vec3 {
	fn fmt( &self, f : &mut fmt::Formatter ) -> fmt::Result {
		return write!( f.buf, "[ {}, {}, {} ]", self.x, self.y, self.z );
	}
}
