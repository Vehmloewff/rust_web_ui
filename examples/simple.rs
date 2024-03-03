use futures_util::{FutureExt, StreamExt};
use log::error;
use warp::{
	filters::ws::{ws, Ws},
	Filter,
};

#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	let routes = warp::path("echo").and(ws()).map(|ws: Ws| {
		ws.on_upgrade(|websocket| {
			let (tx, rx) = websocket.split();

			rx.forward(tx).map(|result| {
				if let Err(e) = result {
					error!("websocket error: {:?}", e);
				}
			})
		})
	});

	warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
