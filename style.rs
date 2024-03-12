use std::collections::HashMap;

type Attributes = HashMap<String, String>;

pub struct ColorDefinition {
	red: u8,
	green: u8,
	blue: u8,
}

impl ColorDefinition {
	pub fn new(red: u8, green: u8, blue: u8) -> ColorDefinition {
		ColorDefinition { red, green, blue }
	}

	pub fn get_css(&self, opacity: usize) -> String {
		format!("rgba({}, {}, {}, {})", self.red, self.green, self.blue, opacity)
	}
}

pub struct Theme {
	pub primary: ColorDefinition,
	pub light: ColorDefinition,
	pub dark: ColorDefinition,
	pub success: ColorDefinition,
	pub danger: ColorDefinition,
	pub warn: ColorDefinition,
	pub notice: ColorDefinition,
}

impl Default for Theme {
	fn default() -> Theme {
		Theme {
			primary: ColorDefinition::new(28, 78, 216),
			light: ColorDefinition::new(255, 255, 255),
			dark: ColorDefinition::new(16, 24, 39),
			success: ColorDefinition::new(4, 120, 87),
			danger: ColorDefinition::new(185, 28, 27),
			warn: ColorDefinition::new(194, 65, 11),
			notice: ColorDefinition::new(28, 78, 216),
		}
	}
}

pub enum Color {
	Primary(usize),
	Bg(usize),
	Fg(usize),
	Success(usize),
	Danger(usize),
	Warn(usize),
	Notice(usize),
}

impl Color {
	pub fn apply_css(&self, theme: &Theme, key: String, attributes: &mut Attributes) {
		match self {
			Color::Primary(opacity) => {
				attributes.insert(key, theme.primary.get_css(opacity.clone()));
			}
			Color::Bg(opacity) => {
				attributes.insert(format!("media_dark:{}", &key), theme.dark.get_css(opacity.clone()));
				attributes.insert(key, theme.light.get_css(opacity.clone()));
			}
			Color::Fg(opacity) => {
				attributes.insert(format!("media_dark:{}", &key), theme.light.get_css(opacity.clone()));
				attributes.insert(key, theme.dark.get_css(opacity.clone()));
			}
			Color::Success(opacity) => {
				attributes.insert(key, theme.success.get_css(opacity.clone()));
			}
			Color::Danger(opacity) => {
				attributes.insert(key, theme.danger.get_css(opacity.clone()));
			}
			Color::Warn(opacity) => {
				attributes.insert(key, theme.warn.get_css(opacity.clone()));
			}
			Color::Notice(opacity) => {
				attributes.insert(key, theme.notice.get_css(opacity.clone()));
			}
		}
	}
}

pub enum Size {
	Exact(usize),
	Full,
}

impl Size {
	pub fn get_css(&self) -> String {
		match self {
			Size::Exact(exact) => format!("{exact}px"),
			Self::Full => "100%".to_string(),
		}
	}
}

pub enum Style {
	Width(Size),
	Height(Size),
	TextColor(Color),
	TextSize(Size),
	Bg(Color),
	InlineFlex,
	Flex,
}

impl Style {
	pub fn apply_css(&self, theme: &Theme, key_base: String, attributes: &mut Attributes) {
		match self {
			Style::Width(size) => {
				attributes.insert(key_base + "width", size.get_css());
			}
			Style::Height(size) => {
				attributes.insert(key_base + "height", size.get_css());
			}
			Style::Bg(color) => color.apply_css(theme, key_base + "background", attributes),
			Style::TextColor(color) => color.apply_css(theme, key_base + "color", attributes),
			Style::TextSize(size) => {
				attributes.insert(key_base + "font-size", size.get_css());
			}
			Style::InlineFlex => {
				attributes.insert(key_base + "display", "inline-flex".into());
			}
			Style::Flex => {
				attributes.insert(key_base + "display", "flex".into());
			}
		};
	}
}
