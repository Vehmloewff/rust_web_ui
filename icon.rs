use std::fmt::{self, Display};

use crate::{Ctx, Style, Widget};

pub struct ViewBox {
	min_x: usize,
	min_y: usize,
	width: usize,
	height: usize,
}

impl Display for ViewBox {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {} {} {}", self.min_x, self.min_y, self.width, self.height)
	}
}

pub trait Svg {
	fn view_box(&self) -> ViewBox;
	fn inner(&self) -> &str;
}

pub struct Icon;

pub struct IconProps {
	styles: Vec<Style>,
	kind: Option<Box<dyn Svg>>,
}

impl IconProps {
	pub fn style(&mut self, mut style: Vec<Style>) {
		self.styles.append(&mut style)
	}

	pub fn kind(&mut self, kind: impl Svg + 'static) {
		self.kind = Some(Box::new(kind))
	}
}

impl Default for IconProps {
	fn default() -> IconProps {
		IconProps {
			styles: Vec::new(),
			kind: None,
		}
	}
}

impl Widget<'_> for Icon {
	type Props = IconProps;

	fn provide_tag() -> String {
		"svg".into()
	}

	fn render(mut ctx: Ctx<'_>, props: IconProps) {
		ctx.styles(props.styles);

		if let Some(svg) = props.kind {
			ctx.set_attribute("view-box", &svg.view_box().to_string());
			ctx.set_attribute("__innerHtml", svg.inner())
		}
	}
}
