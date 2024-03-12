use std::fmt;

use crate::style::Theme;

const GLOBAL_CSS: &str = include_str!("./reset.css");

pub enum WindowUpdate {
	SetTitle { new_title: String },
}

pub enum UpstreamWindowUpdate {
	PathChanged { new_path: String },
}

pub struct Window {
	title: String,
	path: String,
	path_did_change: bool,
	updates: Vec<WindowUpdate>,
	pub theme: Theme,
}

impl Window {
	pub fn new(title: impl Into<String>, path: impl Into<String>, theme: Theme) -> Window {
		Window {
			title: title.into(),
			path: path.into(),
			path_did_change: true,
			updates: Vec::new(),
			theme,
		}
	}

	pub fn get_path_change(&self) -> Option<&str> {
		if self.path_did_change {
			Some(&self.path)
		} else {
			None
		}
	}

	pub fn set_title(&mut self, title: String) {
		self.title = title.clone();
		self.updates.push(WindowUpdate::SetTitle { new_title: title });
	}

	pub fn take_updates(&mut self) -> Vec<WindowUpdate> {
		self.updates.drain(..).collect()
	}

	pub fn apply_update(&mut self, update: UpstreamWindowUpdate) {
		self.path_did_change = true;

		match update {
			UpstreamWindowUpdate::PathChanged { new_path } => self.path = new_path,
		}
	}

	pub fn write_html_head(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<title>{}</title>", self.title)?;
		write!(f, "<style>{GLOBAL_CSS}</style>")?;

		Ok(())
	}
}
