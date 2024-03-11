use crate::{Ctx, Widget};

pub struct Button;

pub struct ButtonProps<'a> {
	label: &'a str,
}

impl<'a> ButtonProps<'a> {
	pub fn label(&mut self, text: &'a str) {
		self.label = text
	}
}

impl<'a> Default for ButtonProps<'a> {
	fn default() -> Self {
		Self { label: "click me" }
	}
}

impl<'a> Widget<'a> for Button {
	type Props = ButtonProps<'a>;

	fn provide_tag() -> String {
		"button".into()
	}

	fn render(mut ctx: Ctx<'_>, props: Self::Props) {
		ctx.set_attribute("__textContent", props.label)
	}
}
