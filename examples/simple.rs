use rust_web_ui::{
	create_driver,
	views::{Button, Text},
	ViewTree,
};
use warp::{filters::path::FullPath, reply::html, Filter};

#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	let favicon_route = warp::get()
		.and(warp::path("favicon.ico"))
		.map(|| warp::reply::with_status("not found", warp::http::StatusCode::NOT_FOUND));

	let html_route = warp::get().and(warp::path::full()).then(|path: FullPath| async move {
		let (handle, driver) = create_driver(Button::new(Text::new("hello, world")), path.as_str());

		tokio::spawn(driver.drive());

		let mut tree = ViewTree::new(path.as_str());
		tree.apply_downstream_messages(
			handle
				.subscribe()
				.await
				.expect("driver was dropped")
				.take_next_downstream_messages()
				.await
				.expect("driver was dropped"),
		)
		.expect("failed to apply downstream messages");

		html(tree.to_string())
	});

	warp::serve(favicon_route.or(html_route)).run(([127, 0, 0, 1], 3030)).await;
}
