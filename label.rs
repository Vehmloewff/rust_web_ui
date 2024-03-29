use crate::{Ctx, Widget};

pub struct Label;

pub struct LabelProps<'a> {
	text: &'a str,
}

impl Default for LabelProps<'_> {
	fn default() -> Self {
		LabelProps { text: "" }
	}
}

impl<'a> LabelProps<'a> {
	pub fn text(&mut self, text: &'a str) {
		self.text = text;
	}
}

impl<'a> Widget<'a> for Label {
	type Props = LabelProps<'a>;

	fn provide_tag() -> String {
		"span".into()
	}

	fn render(mut ctx: Ctx<'_>, props: LabelProps) {
		ctx.set_attribute("__textContent", props.text)
	}
}
