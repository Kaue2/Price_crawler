use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use futures_lite::stream::StreamExt;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mq_user = match env::var("MQ_USER") {
        Ok(value) => value,
        Err(_) => return Err("Erro: falha ao buscar pelo usuário do RabbitMQ")  
    }; 

    let mq_password = match env::var("MQ_PASS") {
        Ok(value) => value,
        Err(_) => return Err("Erro: falha ao buscar pela senha do RabbitMQ")
    };

    let rabbit_addr = format!("amqp://{}:{}@localhost:5672/%2f", mq_user, mq_password);

    let conn = Connection::connect(&rabbit_addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    Ok(())
}
