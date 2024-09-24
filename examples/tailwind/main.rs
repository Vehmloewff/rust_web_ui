use rust_web_ui::{Button, Ctx, Label, StatefulWidget, Theme, View};
use warp::{filters::path::FullPath, reply::html, Filter};

mod component_0;
mod component_1;
mod component_2;
mod component_3;
mod component_4;
mod component_5;
mod component_6;
mod component_7;
mod component_8;
mod component_9;
mod component_10;
mod component_11;
mod component_12;
mod component_13;
mod component_14;
mod component_15;
mod component_16;
mod component_17;
mod component_18;
mod component_19;
mod component_20;
mod component_21;
mod component_22;
mod component_23;
mod component_24;
mod component_25;
mod component_26;
mod component_27;
mod component_28;
mod component_29;
mod component_30;
mod component_31;
mod component_32;
mod component_33;
mod component_34;
mod component_35;
mod component_36;
mod component_37;
mod component_38;
mod component_39;
mod component_40;
mod component_41;
mod component_42;

#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	let favicon_route = warp::get()
		.and(warp::path("favicon.ico"))
		.map(|| warp::reply::with_status("not found", warp::http::StatusCode::NOT_FOUND));

	let html_route = warp::get().and(warp::path::full()).then(|path: FullPath| async move {
		let mut view = View::new("Rust Web Ui", path.as_str(), Theme::default());

		view.define_root("main", component_1::Example1).run(|_| {});

		html(view.to_string())
	});

	warp::serve(favicon_route.or(html_route)).run(([127, 0, 0, 1], 3030)).await;
}
