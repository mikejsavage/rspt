#![ allow( dead_code ) ]

use maths::color::RGB;
use shapes::Shape;
use lights::area::AreaLight;
use materials::Material;

pub struct Entity {
	material : ~Material : Share + Send,
	shape : ~Shape : Send + Share,
	light : Option< AreaLight >,
}

impl Entity {
	pub fn new( material : ~Material : Share + Send, shape : ~Shape : Share + Send ) -> Entity {
		return Entity {
			material : material,
			shape : shape,
			light : None,
		};
	}

	pub fn new_light( material : ~Material : Share + Send, shape : ~Shape : Share + Send, emission : RGB ) -> Entity {
		let light = AreaLight::new( emission, shape );

		return Entity {
			material : material,
			shape : shape,
			light : Some( light ),
		};
	}
}
