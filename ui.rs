use crate::{style::Style, Element, Node, State, Window};
use std::marker::PhantomData;

pub struct Dynamic;

pub struct Ctx<'a> {
	element: &'a mut Element,
	window: &'a mut Window,
}

impl Ctx<'_> {
	pub fn child<T>(&mut self, key: &str, _widget: T) -> Ui<'_, T> {
		let (node, state) = self.element.child(key);

		Ui::new(node, state, &mut self.window)
	}

	pub fn set_attribute(&mut self, name: &str, value: &str) {
		self.element.set_attribute(name, value)
	}

	pub fn styles(&mut self, styles: &[Style]) {
		for style in styles {
			style.apply_css(&self.window.theme, String::new(), &mut self.element.attributes)
		}
	}
}

#[must_use = "Call `run` or `sun` to build the widget, failure to do so may result in it's accidental deletion"]
pub struct Ui<'a, WidgetSpecifier> {
	node: &'a mut Node,
	state: &'a mut State,
	window: &'a mut Window,
	phantom_widget: PhantomData<WidgetSpecifier>,
}

impl<'a, WidgetSpecifier> Ui<'a, WidgetSpecifier> {
	pub fn new(node: &'a mut Node, state: &'a mut State, window: &'a mut Window) -> Ui<'a, WidgetSpecifier> {
		Ui {
			node,
			state,
			window,
			phantom_widget: PhantomData,
		}
	}
}

impl<'a, Props, WidgetSpecifier> Ui<'a, WidgetSpecifier>
where
	Props: Default,
	WidgetSpecifier: Widget<'a, Props = Props>,
{
	pub fn run(self, mut func: impl FnMut(&mut Props)) {
		let mut props = WidgetSpecifier::Props::default();
		func(&mut props);

		let element = self.node.get_element(|| Element::new(WidgetSpecifier::provide_tag()));

		WidgetSpecifier::render(Ctx { element, window: self.window }, props)
	}
}

impl<'a, Props, State, WidgetSpecifier> Ui<'a, WidgetSpecifier>
where
	Props: Default,
	State: Default + 'static,
	WidgetSpecifier: StatefulWidget<'a, Props = Props, State = State>,
{
	pub fn sun(self, func: impl FnOnce(&mut Props)) {
		let mut props = WidgetSpecifier::Props::default();
		func(&mut props);

		let element = self.node.get_element(|| Element::new(WidgetSpecifier::provide_tag()));
		let state = unsafe { self.state.get::<State>() };

		WidgetSpecifier::render(Ctx { element, window: self.window }, props, state)
	}
}

impl<'a> Ui<'a, Dynamic> {
	pub fn run(self, tag: impl Into<String>, func: impl FnOnce(Ctx<'a>)) {
		let element = self.node.get_element(|| Element::new(tag));

		func(Ctx { element, window: self.window })
	}
}

pub trait Widget<'a> {
	type Props: Default + 'a;

	fn provide_tag() -> String {
		"div".to_string()
	}

	fn render(ctx: Ctx<'_>, props: Self::Props);
}

pub trait StatefulWidget<'a> {
	type Props: Default + 'a;
	type State: Default + 'static;

	fn provide_tag() -> String {
		"div".to_string()
	}

	fn render(ctx: Ctx<'_>, props: Self::Props, state: &mut Self::State);
}
