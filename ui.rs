use crate::{Ctx, Element, Node, State};
use std::marker::PhantomData;

pub struct Ui<'a, WidgetSpecifier> {
	node: &'a mut Node,
	state: &'a mut State,
	context: &'a mut Ctx,
	phantom_widget: PhantomData<WidgetSpecifier>,
}

impl<'a, WidgetSpecifier> Ui<'a, WidgetSpecifier> {
	pub fn new(node: &'a mut Node, state: &'a mut State, context: &'a mut Ctx) -> Ui<'a, WidgetSpecifier> {
		Ui {
			node,
			state,
			context,
			phantom_widget: PhantomData,
		}
	}
}

impl<'a, Props, WidgetSpecifier> Ui<'a, WidgetSpecifier>
where
	Props: Default,
	WidgetSpecifier: Widget<'a, Props = Props>,
{
	pub fn conf(self, func: impl FnOnce(&mut Props)) {
		let mut props = WidgetSpecifier::Props::default();
		func(&mut props);

		WidgetSpecifier::render(self.node.get_element(|| Element::new(WidgetSpecifier::provide_tag())), self.context, props)
	}
}

impl<'a, Props, State, WidgetSpecifier> Ui<'a, WidgetSpecifier>
where
	Props: Default,
	State: Default + 'static,
	WidgetSpecifier: StatefulWidget<'a, Props = Props, State = State>,
{
	pub fn stateful_conf(self, func: impl FnOnce(&mut Props)) {
		let mut props = WidgetSpecifier::Props::default();
		func(&mut props);

		let el = self.node.get_element(|| Element::new(WidgetSpecifier::provide_tag()));
		let state = unsafe { self.state.get::<State>() };

		WidgetSpecifier::render(el, self.context, props, state)
	}
}

pub trait Widget<'a> {
	type Props: Default + 'a;

	fn provide_tag() -> String {
		"div".to_string()
	}

	fn render(el: &mut Element, ctx: &mut Ctx, props: Self::Props);
}

pub trait StatefulWidget<'a> {
	type Props: Default + 'a;
	type State: Default + 'static;

	fn provide_tag() -> String {
		"div".to_string()
	}

	fn render(el: &mut Element, ctx: &mut Ctx, props: Self::Props, state: &mut Self::State);
}
