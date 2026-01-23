use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use futures_lite::stream::StreamExt;
use std::env;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use mongodb::{Client, bson::document, options::ClientOptions};
use scraper::{Selector, Html};
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Debug, Deserialize)]
struct RabbitMessage {
    id: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawPage {
    #[serde(rename = "_id")] // mudando do padrão mongo para rust
    id: String,
    url: String,
    html: String,
}

async fn connect_rabbit() -> Result<lapin::Channel, lapin::Error> {
    let mq_user = env::var("MQ_USER").expect("Erro: variável MQ_USER não encontrada");
    let mq_password = env::var("MQ_PASS").expect("Erro: variável MQ_PASS não encontrada");
    let rabbit_addr = format!("amqp://{}:{}@localhost:5672/%2f", mq_user, mq_password);
    println!("LOG: Conectado ao Rabbit em: {}", rabbit_addr);

    let conn = Connection::connect(&rabbit_addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    Ok(channel)
}

async fn connect_mongo() -> Result<mongodb::Collection<RawPage>, mongodb::error::Error> {
    let mongo_user = env::var("MONGO_USER").expect("ERROR: variável MONGO_USER não encontrada");
    let mongo_pass = env::var("MONGO_PASSWORD").expect("ERROR: variável MONGO_PASS não encontrada");
    let mongo_uri = format!("mongodb://{}:{}@localhost:27017", mongo_user, mongo_pass);
    println!("LOG: Conectando ao MongoDb em: {}", mongo_uri);

    let clinet_options = ClientOptions::parse(&mongo_uri).await?;
    let mongo_client = Client::with_options(clinet_options)?;
    let db = mongo_client.database("crawler_db");
    let collection = db.collection::<RawPage>("raw_pages");

    Ok(collection)
}

async fn connect_postgres() -> Result<PgPool, sqlx::Error> {
    let postgres_user = env::var("POSTGRES_USER").expect("ERROR: variável POSTGRES_USER não encontrada");
    let postgres_pass = env::var("POSTGRES_PASSWORD").expect("ERROR: variável POSTGRES_PASSWORD não encontrada");
    let conn_str = format!("postgres://{}:{}@localhost:5432/price_crawler_db", postgres_user, postgres_pass);
    println!("LOG: Conectando ao Postgres em: {}", conn_str);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str).await?;


    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let channel = connect_rabbit().await?;
    let collection = connect_mongo().await?;
    let _pool = connect_postgres().await?;

    // criando uma stream (pipe para comunicação)
    let mut consumer = channel
        .basic_consume(
            "raw_prices",
            "my_consumer_tag",
            BasicConsumeOptions::default(), 
            FieldTable::default(),
        ).await?;

    // loop para pegar mensagens
    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let payload: RabbitMessage = match serde_json::from_slice(&delivery.data){
                Ok(p) => p,
                Err(e) => {
                    eprintln!("ERROR: falha ao ler JSON do rabbit: {}", e);
                    delivery.ack(BasicAckOptions::default()).await?;
                    continue;
                }
            };
            println!("LOG: prcessando mensagem ID {}", payload.id);

            // query em mongo
            let filter = mongodb::bson::doc! {"_id": &payload.id};

            match collection.find_one(filter).await {
                Ok(Some(document)) => {
                    println!("LOG: Documento encontrado. URL: {}", document.url);
                    let fragment = Html::parse_fragment(&document.html);
                    let selector = Selector::parse("p.price_color").unwrap();
                    let price_element = fragment.select(&selector).next().unwrap();
                    let price = price_element.inner_html();
                    let price = price.replace("£", "");
                    let price: f64 = price.parse().expect("ERROR: falha ao converter preço do item para número.");
                    println!("DEBUG: preço encontrado: {}", price);
                },
                Ok(None) => eprintln!("WARNING: documento não encontrado no mongo, ID: {}", payload.id),
                Err(e) => eprintln!("ERROR: falha de conexão com Mongo: {}", e),
            }

            //processamento
            delivery.ack(BasicAckOptions::default()).await?;
        }
    };

    Ok(())
}
