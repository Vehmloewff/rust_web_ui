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

pub enum Screen {
	Small,
	Medium,
	Large,
	ExtraLarge(usize),
}

impl Screen {
	pub fn get_size(&self) -> usize {
		match self {
			Screen::Small => 640,
			Screen::Medium => 768,
			Screen::Large => 1024,
			Screen::ExtraLarge(times) => (240 * times) + 1024,
		}
	}
}

pub enum Style {
	Width(Size),
	Height(Size),
	Size(Size),

	Padding(Size),
	PaddingLeft(Size),
	PaddingRight(Size),
	PaddingTop(Size),
	PaddingBottom(Size),
	PaddingX(Size),
	PaddingY(Size),

	Margin(Size),
	MarginLeft(Size),
	MarginRight(Size),
	MarginTop(Size),
	MarginBottom(Size),
	MarginX(Size),
	MarginY(Size),

	SpaceX(Size),
	SpaceY(Size),

	Flex,
	InlineFlex,
	JustifyBetween,
	JustifyCenter,
	ItemsCenter,

	InlineBlock,
	Block,

	Rounded(Size),

	Color(Color),
	TextColor(Color),

	TextSize(Size),
	FontSemibold,
	FontBold,
	FontLight,

	OnHover(&'static [Style]),
	OnActive(&'static [Style]),
	OnFocus(&'static [Style]),
	OnScreen(Screen, &'static [Style]),
	Noop(&'static str),
	NoopGroup(&'static str, &'static [Style]),
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
			Style::Size(size) => {
				attributes.insert(key_base.clone() + "width", size.get_css());
				attributes.insert(key_base + "height", size.get_css());
			}

			Style::Padding(size) => {
				attributes.insert(key_base + "padding", size.get_css());
			}
			Style::PaddingLeft(size) => {
				attributes.insert(key_base + "padding-left", size.get_css());
			}
			Style::PaddingRight(size) => {
				attributes.insert(key_base + "padding-right", size.get_css());
			}
			Style::PaddingTop(size) => {
				attributes.insert(key_base + "padding-top", size.get_css());
			}
			Style::PaddingBottom(size) => {
				attributes.insert(key_base + "padding-bottom", size.get_css());
			}
			Style::PaddingX(size) => {
				attributes.insert(key_base.clone() + "padding-right", size.get_css());
				attributes.insert(key_base + "padding-left", size.get_css());
			}
			Style::PaddingY(size) => {
				attributes.insert(key_base.clone() + "padding-top", size.get_css());
				attributes.insert(key_base + "padding-bottom", size.get_css());
			}

			Style::Margin(size) => {
				attributes.insert(key_base + "margin", size.get_css());
			}
			Style::MarginLeft(size) => {
				attributes.insert(key_base + "margin-left", size.get_css());
			}
			Style::MarginRight(size) => {
				attributes.insert(key_base + "margin-right", size.get_css());
			}
			Style::MarginTop(size) => {
				attributes.insert(key_base + "margin-top", size.get_css());
			}
			Style::MarginBottom(size) => {
				attributes.insert(key_base + "margin-bottom", size.get_css());
			}
			Style::MarginX(size) => {
				attributes.insert(key_base.clone() + "margin-right", size.get_css());
				attributes.insert(key_base + "margin-left", size.get_css());
			}
			Style::MarginY(size) => {
				attributes.insert(key_base.clone() + "margin-top", size.get_css());
				attributes.insert(key_base + "margin-bottom", size.get_css());
			}

			Style::SpaceX(size) => {
				attributes.insert(key_base.clone() + "*:margin-left", size.get_css());
				attributes.insert(key_base + "*:margin-right", size.get_css());
			}
			Style::SpaceY(size) => {
				attributes.insert(key_base.clone() + "*:margin-top", size.get_css());
				attributes.insert(key_base + "*:margin-bottom", size.get_css());
			}

			Style::Flex => {
				attributes.insert(key_base + "display", "flex".into());
			}
			Style::InlineFlex => {
				attributes.insert(key_base + "display", "inline-flex".into());
			}
			Style::JustifyBetween => {
				attributes.insert(key_base + "justify-content", "between".into());
			}
			Style::JustifyCenter => {
				attributes.insert(key_base + "justify-content", "center".into());
			}
			Style::ItemsCenter => {
				attributes.insert(key_base + "align-items", "center".into());
			}

			Style::InlineBlock => {
				attributes.insert(key_base + "display", "inline-block".into());
			}
			Style::Block => {
				attributes.insert(key_base + "display", "block".into());
			}

			Style::Rounded(size) => {
				attributes.insert(key_base + "border-radius", size.get_css());
			}

			Style::Color(color) => color.apply_css(theme, key_base + "background-color", attributes),
			Style::TextColor(color) => color.apply_css(theme, key_base + "color", attributes),

			Style::TextSize(size) => {
				attributes.insert(key_base + "font-size", size.get_css());
			}
			Style::FontSemibold => {
				attributes.insert(key_base + "font-weight", "semibold".into());
			}
			Style::FontBold => {
				attributes.insert(key_base + "font-weight", "bold".into());
			}
			Style::FontLight => {
				attributes.insert(key_base + "font-weight", "light".into());
			}

			Style::OnHover(styles) => {
				for style in *styles {
					style.apply_css(theme, key_base.clone() + "hover:", attributes)
				}
			}
			Style::OnActive(styles) => {
				for style in *styles {
					style.apply_css(theme, key_base.clone() + "active:", attributes)
				}
			}
			Style::OnFocus(styles) => {
				for style in *styles {
					style.apply_css(theme, key_base.clone() + "focus:", attributes)
				}
			}
			Style::OnScreen(screen, styles) => {
				for style in *styles {
					style.apply_css(theme, format!("screen-{}:", screen.get_size()) + &key_base, attributes)
				}
			}
			Style::Noop(_) => {}
			Style::NoopGroup(_, _) => {}
		};
	}
}
