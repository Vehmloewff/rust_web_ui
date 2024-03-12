use rust_web_ui::{Button, Ctx, Label, StatefulWidget, Theme, View};
use warp::{filters::path::FullPath, reply::html, Filter};

#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	let favicon_route = warp::get()
		.and(warp::path("favicon.ico"))
		.map(|| warp::reply::with_status("not found", warp::http::StatusCode::NOT_FOUND));

	let html_route = warp::get().and(warp::path::full()).then(|path: FullPath| async move {
		let mut view = View::new("Rust Web Ui".into(), path.as_str().into(), Theme::default());

		view.define_root("main", Counter).sun(|_| {});

		html(view.to_string())
	});

	warp::serve(favicon_route.or(html_route)).run(([127, 0, 0, 1], 3030)).await;
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
