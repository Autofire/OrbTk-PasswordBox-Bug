use orbtk::prelude::*;

use crate::{MainState,Message};

widget!(
    MainView<MainState> {
        title: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
			.child(Stack::new()
				.child(PasswordBox::new().id("password").build(ctx))
				.child(Button::new()
					.text("clear")
					.on_click(move |states,_| {
						states.send_message(Message::Clear, id);
						true
					})
					.build(ctx)
				)
				.build(ctx)
			)
    }
}
