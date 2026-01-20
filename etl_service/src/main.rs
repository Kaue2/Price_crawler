use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use futures_lite::stream::StreamExt;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mq_user = env::var("MQ_USER").expect("Erro: variável MQ_USER não encontrada");

    let mq_password = env::var("MQ_PASS").expect("Erro: variável MQ_PASS não encontrada");

    let rabbit_addr = format!("amqp://{}:{}@localhost:5672/%2f", mq_user, mq_password);
    println!("Conectado em: {}", rabbit_addr);

    let conn = Connection::connect(&rabbit_addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // criando uma stream (pipe para comunicação)
    let mut consumer = channel
        .basic_consume(
            "raw_prices",
            "my_consumer_tag",
            BasicConsumeOptions::default(), 
            FieldTable::default(),
        ).await?;

    while let Some(delivery) = consumer.next().await {
        println!("iniciando processamento");
        if let Ok(delivery) = delivery {
            let payload = std::str::from_utf8(&delivery.data)?;
            println!("LOG: Mensagem recebida: {}", payload);

            //processamento
            delivery.ack(BasicAckOptions::default()).await?;
        }
    };

    Ok(())
}
