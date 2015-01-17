#![ allow( dead_code ) ]

use maths::color::RGB;
use shapes::Shape;
use lights::area::AreaLight;
use materials::Material;

pub struct Entity {
	pub material : Box< Material + Sync + Send >,
	pub shape : Box< Shape + Send + Sync >,
	pub light : Option< AreaLight >,
}

impl Entity {
	pub fn new( material : Box< Material + Sync + Send >, shape : Box< Shape + Sync + Send > ) -> Entity {
		return Entity {
			material : material,
			shape : shape,
			light : None,
		};
	}

	pub fn new_light( material : Box< Material + Sync + Send >, shape : Box< Shape + Sync + Send >, emission : RGB ) -> Entity {
		let light = AreaLight::new( emission, &shape );

		return Entity {
			material : material,
			shape : shape,
			light : Some( light ),
		};
	}
}
