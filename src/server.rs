use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use std::sync::mpsc;

use crate::config::Config;

pub struct MyWebSocket {
    // Your logic here
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Process the incoming message
                // ...
                println!("Received: {}", text);
            }
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
                println!("Connection closed");
            }
            _ => {}
        }
    }
}

async fn index_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}

async fn index_html() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../static/index.html"))
}

pub async fn run(config: Config) -> std::io::Result<()> {
    // Start the server

    let (host, port) = {
        let mut parts = config.listen.split(':');
        match (parts.next(), parts.next()) {
            (Some(host), Some(port)) => (host, port.parse().expect("port integer")),
            _ => ("127.0.0.1", 8080),
        }
    };

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .route("/ws/", web::get().to(index_ws))
            .route("/", web::get().to(index_html))
    })
    .bind((host, port))?
    .run()
    .await
}
