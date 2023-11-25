use std::io::Write;
use log::debug;
use mongodb::bson::Document;
use mongodb::Client;
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct MyGreeter {
    db: Client,
}

trait WithDatabase {
    fn with_database(db: Client) -> Self;
}

impl WithDatabase for MyGreeter {
    fn with_database(db: Client) -> Self {
        MyGreeter { db }
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let collection = self
            .db
            .database("test_db")
            .collection::<Document>("test_collection");
        debug!("Before insert");
        if let Ok(on_result) = collection.insert_one(Document::default(), None).await {
            debug!("The insert done with id: {}", on_result.inserted_id);
        }
        debug!("After insert");

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // MongoDB settings
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;

    // Tonic settings
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::with_database(client.clone());

    // Logging settings
    env_logger::builder()
        .format(|buf, record| {
            let mut level_style = buf.default_level_style(record.level());
            level_style.set_bold(true);
            writeln!(
                buf,
                "{} : {:5} | [{}] - {}",
                buf.timestamp(),
                level_style.value(record.level()),
                record.target(),
                record.args()
            )
        })
        .init();

    debug!("Init is done!");

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
