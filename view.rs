use crate::{style::Theme, Element, Ui, Window};
use std::fmt::{self, Display};

pub struct View {
	root: Element,
	window: Window,
}

impl View {
	pub fn new(title: String, path: String, theme: Theme) -> View {
		View {
			root: Element::new("body"),
			window: Window::new(title, path, theme),
		}
	}

	pub fn define_root<T>(&mut self, key: &str, widget: T) -> Ui<'_, T> {
		self.root.child(key, &mut self.window, widget)
	}
}

impl Display for View {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<!DOCTYPE html><html lang=\"en\">")?;

		self.window.write_html_head(f)?;
		self.root.get_repr().write_html(f, None)?;

		write!(f, "</html>")?;

		Ok(())
	}
}
