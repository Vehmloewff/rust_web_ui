use rust_web_ui::{Button, Ctx, Label, StatefulWidget, View};

fn main() {
	let mut view = View::new("Rust Web Ui".into(), "/hello".into());

	view.define_root("main", Counter).sun(|_| {});

	println!("{view}");
}

struct Counter;

struct CounterState {
	count: usize,
}

impl Default for CounterState {
	fn default() -> Self {
		CounterState { count: 0 }
	}
}

impl StatefulWidget<'_> for Counter {
	type Props = ();
	type State = CounterState;

	fn render(mut ctx: Ctx<'_>, _: Self::Props, state: &mut Self::State) {
		let CounterState { count } = state;

		ctx.child("decrement_button", Button).run(|props| {
			props.label("Decrement");
		});

		let label = format!("current count is {count}");
		ctx.child("label", Label).run(|props| {
			props.text(&label);
		});

		ctx.child("increment_button", Button).run(|props| {
			props.label("Increment");
		});

		*count += 1;
	}
}
