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

// pub async fn new(port: u16) -> SocketServer {
//     let addr = SocketAddr::from(([127, 0, 0, 1], port));
//     let listener = TcpListener::bind(addr).await.unwrap();

//     let mut http = http1::Builder::new();
//     http.keep_alive(true);

//     let (sender, receiver) = channel(100);

//     tokio::spawn(async move {
//         loop {
//             let (stream, _) = listener.accept().await.unwrap();
//             let connection_sender = sender.clone();

//             let connection = http
//                 .serve_connection(
//                     TokioIo::new(stream),
//                     service_fn(move |mut request| {
//                         let single_sender = connection_sender.clone();

//                         async move {
//                             let uri = request.uri();
//                             let path = uri.path().to_owned();
//                             let mut query_params = HashMap::new();

//                             for (key, value) in
//                                 form_urlencoded::parse(uri.query().unwrap_or("").as_bytes())
//                             {
//                                 query_params.insert(key.to_string(), value.to_string());
//                             }

//                             let (response, hyper_socket) = upgrade(&mut request, None).unwrap();

//                             tokio::spawn(async move {
//                                 let socket = hyper_socket.await.unwrap();

//                                 match single_sender
//                                     .clone()
//                                     .send(Connection {
//                                         connection_details: Some(ConnectionDetails {
//                                             path,
//                                             query_params,
//                                         }),
//                                         socket,
//                                         client_pin: None,
//                                         last_value: None,
//                                     })
//                                     .await
//                                 {
//                                     Ok(_) => (),
//                                     Err(mut error) => {
//                                         error.0.close("Failed to queue connection").await;
//                                     }
//                                 };
//                             });

//                             Ok::<_, Infallible>(response)
//                         }
//                     }),
//                 )
//                 .with_upgrades();

//             tokio::spawn(async move {
//                 connection.await.unwrap();
//             });
//         }
//     });
// }
