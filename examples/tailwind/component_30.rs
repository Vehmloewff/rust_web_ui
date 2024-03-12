use rust_web_ui::*;

pub struct Example30;

pub struct Example30Props {}

impl Default for Example30Props {
	fn default() -> Example30Props {
		Example30Props { }
	}
}

impl Widget<'_> for Example30 {
	type Props = Example30Props;

	fn render(mut ctx: Ctx<'_>, props: Example30Props) {
		
	}
}
