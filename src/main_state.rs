use orbtk::prelude::*;

pub enum Message {
	Clear,
}

#[derive(Default, AsAny)]
pub struct MainState {
}

impl State for MainState {
    fn update(&mut self, _: &mut Registry, _: &mut Context) {
    }


	fn messages(
			&mut self,
			mut messages: MessageReader,
			_registry: &mut Registry,
			ctx: &mut Context
	) {
		for message in messages.read::<Message>() {
			match message {
				Message::Clear => {
					ctx.child("password").set::<String>("text", String::new());
				}
			}
		}
	}
}
