#![ allow( dead_code ) ]

use maths::color::RGB;
use shapes::Shape;
use lights::area::AreaLight;
use materials::Material;

pub struct Entity {
	pub material : Box< Material + Share + Send >,
	pub shape : Box< Shape + Send + Share >,
	pub light : Option< AreaLight >,
}

impl Entity {
	pub fn new( material : Box< Material + Share + Send >, shape : Box< Shape + Share + Send > ) -> Entity {
		return Entity {
			material : material,
			shape : shape,
			light : None,
		};
	}

	pub fn new_light( material : Box< Material + Share + Send >, shape : Box< Shape + Share + Send >, emission : RGB ) -> Entity {
		let light = AreaLight::new( emission, shape );

		return Entity {
			material : material,
			shape : shape,
			light : Some( light ),
		};
	}
}
