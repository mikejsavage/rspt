#![ allow( dead_code ) ]

use std::num::Float;
use std::fmt;

use maths::vec::Vec3;
use maths::angle::Angle;

pub struct Rotation {
	elems : [ [ f64, ..3 ], ..3 ],
}

impl Rotation {
	pub fn det( &self ) -> f64 {
		return
			self.elems[ 0 ][ 0 ] * ( self.elems[ 1 ][ 1 ] * self.elems[ 2 ][ 2 ] - self.elems[ 2 ][ 1 ] * self.elems[ 1 ][ 2 ] )
			-( self.elems[ 0 ][ 1 ] * ( self.elems[ 1 ][ 0 ] * self.elems[ 2 ][ 2 ] - self.elems[ 2 ][ 0 ] * self.elems[ 1 ][ 2 ] ) )
			+ self.elems[ 0 ][ 2 ] * ( self.elems[ 1 ][ 0 ] * self.elems[ 2 ][ 1 ] - self.elems[ 2 ][ 0 ] * self.elems[ 1 ][ 1 ] );
	}

	pub fn inverse( &self ) -> Rotation {
		let d = self.det();
		assert!( d != 0.0 ); // this is reasonable
		let e = &self.elems;

		return Rotation {
			elems : [
				[
					e[ 1 ][ 1 ] * e[ 2 ][ 2 ] - e[ 2 ][ 1 ] * e[ 1 ][ 2 ],
					e[ 1 ][ 2 ] * e[ 2 ][ 0 ] - e[ 2 ][ 2 ] * e[ 1 ][ 0 ],
					e[ 1 ][ 0 ] * e[ 2 ][ 1 ] - e[ 2 ][ 0 ] * e[ 1 ][ 1 ],
				],
				[
					e[ 0 ][ 2 ] * e[ 2 ][ 1 ] - e[ 2 ][ 2 ] * e[ 0 ][ 1 ],
					e[ 0 ][ 0 ] * e[ 2 ][ 2 ] - e[ 2 ][ 0 ] * e[ 0 ][ 2 ],
					e[ 0 ][ 1 ] * e[ 2 ][ 0 ] - e[ 2 ][ 1 ] * e[ 0 ][ 0 ],
				],
				[
					e[ 0 ][ 1 ] * e[ 1 ][ 2 ] - e[ 1 ][ 1 ] * e[ 0 ][ 2 ],
					e[ 0 ][ 2 ] * e[ 1 ][ 0 ] - e[ 1 ][ 2 ] * e[ 0 ][ 0 ],
					e[ 0 ][ 0 ] * e[ 1 ][ 1 ] - e[ 1 ][ 0 ] * e[ 0 ][ 1 ],
				],
			],
		};
	}

	pub fn apply( &self, v : Vec3 ) -> Vec3 {
		return Vec3 {
			x : self.elems[ 0 ][ 0 ] * v.x + self.elems[ 0 ][ 1 ] * v.y + self.elems[ 0 ][ 2 ] * v.z,
			y : self.elems[ 1 ][ 0 ] * v.x + self.elems[ 1 ][ 1 ] * v.y + self.elems[ 1 ][ 2 ] * v.z,
			z : self.elems[ 2 ][ 0 ] * v.x + self.elems[ 2 ][ 1 ] * v.y + self.elems[ 2 ][ 2 ] * v.z,
		};
	}

	pub fn zero() -> Rotation {
		return Rotation {
			elems: [
				[ 0.0, 0.0, 0.0 ],
				[ 0.0, 0.0, 0.0 ],
				[ 0.0, 0.0, 0.0 ],
			],
		};
	}

	pub fn identity() -> Rotation {
		return Rotation {
			elems : [
				[ 1.0, 0.0, 0.0 ],
				[ 0.0, 1.0, 0.0 ],
				[ 0.0, 0.0, 1.0 ],
			],
		};
	}

	pub fn x( angle : &Angle ) -> Rotation {
		let ( s, c ) = angle.sin_cos();

		return Rotation {
			elems : [
				[ 1.0, 0.0, 0.0 ],
				[ 0.0,   c,  -s ],
				[ 0.0,   s,   c ],
			],
		};
	}

	pub fn y( angle : &Angle ) -> Rotation {
		let ( s, c ) = angle.sin_cos();

		return Rotation {
			elems : [
				[   c, 0.0,   s ],
				[ 0.0, 1.0, 0.0 ],
				[  -s, 0.0,   c ],
			],
		};
	}

	pub fn z( angle : &Angle ) -> Rotation {
		let ( s, c ) = angle.sin_cos();

		return Rotation {
			elems : [
				[   c,  -s, 0.0 ],
				[  -s,   c, 0.0 ],
				[ 0.0, 0.0, 1.0 ],
			],
		};
	}

	pub fn about( axis : Vec3, angle : &Angle ) -> Rotation {
		let ( s, c ) = angle.sin_cos();

		return Rotation::about_with_sin_cos( axis, s, c );
	}

	pub fn about_with_sin_cos( axis : Vec3, s : f64, c : f64 ) -> Rotation {
		return Rotation {
			elems : [
				[
					axis.x * axis.x + ( 1.0 - axis.x * axis.x ) * c,
					axis.x * axis.y * ( 1.0 - c ) - axis.z * s,
					axis.x * axis.z * ( 1.0 - c ) + axis.y * s,
				],
				[
					axis.y * axis.x * ( 1.0 - c ) + axis.z * s,
					axis.y * axis.y + ( 1.0 - axis.y * axis.y ) * c,
					axis.y * axis.z * ( 1.0 - c ) - axis.x * s,
				],
				[
					axis.z * axis.x * ( 1.0 - c ) - axis.y * s,
					axis.z * axis.y * ( 1.0 - c ) + axis.x * s,
					axis.z * axis.z + ( 1.0 - axis.z * axis.z ) * c,
				],
			],
		};
	}

	pub fn between( from : Vec3, to : Vec3 ) -> Rotation {
		if from == to {
			return Rotation::identity();
		}
		else if from == -to {
			return -Rotation::identity();
		}

		let axis = from.cross( to ).normalised();
		let cos_theta = from.dot( to );

		return Rotation::about_with_sin_cos( axis, ( 1.0 - cos_theta * cos_theta ).sqrt(), cos_theta );
	}
}

impl Mul< Rotation, Rotation > for Rotation {
	fn mul( &self, other : &Rotation ) -> Rotation {
		let mut ret = Rotation::zero();

		for x in range( 0u, 3 ) {
			for y in range( 0u, 3 ) {
				ret.elems[ y ][ x ] =
					self.elems[ y ][ 0 ] * other.elems[ 0 ][ x ] +
					self.elems[ y ][ 1 ] * other.elems[ 1 ][ x ] +
					self.elems[ y ][ 2 ] * other.elems[ 2 ][ x ];
			}
		}

		return ret;
	}
}

impl Neg< Rotation > for Rotation {
	#[ inline ]
	fn neg( &self ) -> Rotation {
		let mut ret = Rotation::zero();

		for x in range( 0u, 3 ) {
			for y in range( 0u, 3 ) {
				ret.elems[ y ][ x ] = -self.elems[ y ][ x ];
			}
		}

		return ret;
	}
}

impl fmt::Show for Rotation {
	fn fmt( &self, f : &mut fmt::Formatter ) -> fmt::Result {
		return write!( f,
			"[\n\t[ {}, {}, {} ]\n\t[ {}, {}, {} ]\n\t[ {}, {}, {} ]\n]",
			self.elems[ 0 ][ 0 ], self.elems[ 0 ][ 1 ], self.elems[ 0 ][ 2 ],
			self.elems[ 1 ][ 0 ], self.elems[ 1 ][ 1 ], self.elems[ 1 ][ 2 ],
			self.elems[ 2 ][ 0 ], self.elems[ 2 ][ 1 ], self.elems[ 2 ][ 2 ]
		);
	}
}
