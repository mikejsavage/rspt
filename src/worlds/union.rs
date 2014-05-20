use maths::vec::Vec3;
use shapes::Intersection;
use worlds::World;

#[ allow( dead_code ) ]
pub struct UnionWorld {
	w1 : ~World : Send,
	w2 : ~World : Send,
}

#[ allow( dead_code ) ]
impl UnionWorld {
	pub fn new( w1 : ~World : Send, w2 : ~World : Send ) -> UnionWorld {
		return UnionWorld { w1 : w1, w2 : w2 };
	}
}

impl World for UnionWorld {
	// fn intersects( &self, start : Vec3, dir : Vec3 ) -> bool {
	// 	return self.w1.intersects( start, dir ) || self.w2.intersects( start, dir );
	// }

	fn intersection< 'a >( &'a self, start : Vec3, dir : Vec3 ) -> Option< Intersection< 'a > > {
		let oi1 = self.w1.intersection( start, dir );
		let oi2 = self.w2.intersection( start, dir );

		match ( oi1, oi2 ) {
			( Some( i1 ), Some( i2 ) ) => {
				if i1.t < i2.t {
					return Some( i1 );
				}

				return Some( i2 );
			}

			( x, None ) => x,
			( None, x ) => x
		}
	}
}
