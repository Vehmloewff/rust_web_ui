use rust_web_ui::{Button, Ctx, Label, StatefulWidget, Theme, View};
use warp::{filters::path::FullPath, reply::html, Filter};

mods!();

#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	let favicon_route = warp::get()
		.and(warp::path("favicon.ico"))
		.map(|| warp::reply::with_status("not found", warp::http::StatusCode::NOT_FOUND));

	let html_route = warp::get().and(warp::path::full()).then(|path: FullPath| async move {
		let mut view = View::new("Rust Web Ui", path.as_str(), Theme::default());

		view.define_root("main", widget1!()).run(|_| {});

		html(view.to_string())
	});

	warp::serve(favicon_route.or(html_route)).run(([127, 0, 0, 1], 3030)).await;
}
