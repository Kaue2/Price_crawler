use chrono::NaiveDateTime;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use futures_lite::stream::StreamExt;
use rust_decimal::Decimal;
use core::fmt;
use std::{env, str::FromStr};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions};
use scraper::{Selector, Html};
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::{Uuid};

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

struct Item {
    id: Uuid,
    title: String,
    value: Decimal,
    url: String,
    created_at: NaiveDateTime,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "Id: {}\ntitle: {}\nValue: {} \nUrl: {} \nCriado em: {}", 
        &self.id, &self.title, &self.value, &self.url, &self.created_at)
    }
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

fn extract_data(document: RawPage, url: &String) -> Result<Item, String>{
     println!("LOG: Documento encontrado. URL: {}", document.url);
    let fragment = Html::parse_fragment(&document.html);
    
    let title_sel = Selector::parse("div.product_main > h1")
        .map_err(|_| "ERROR: falha ao buscar pelo seletor do título")?;

    let title = fragment.select(&title_sel)
        .next()
        .map(|el| el.text().collect::<String>())
        .ok_or("ERROR: elemento Título não encontrado")?;

    let price_sel = Selector::parse("p.price_color")
        .map_err(|_| "ERROR: falha ao buscar pelo seletor do preço")?;

    let price = fragment.select(&price_sel)
        .next()
        .map(|el| el.text().collect::<String>())
        .ok_or("ERROR: elemento preço não encontrado")?;

    let price = price.replace("£", "");
    let price = Decimal::from_str(&price).unwrap_or(Decimal::ZERO);

    let item = Item{
        id:Uuid::new_v4(), 
        title: title, 
        value: price, 
        url: url.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    return Ok(item);
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

            match collection.find_one(filter, None).await {
                Ok(Some(document)) => {
                   let item: Item = extract_data(document, &payload.url)?;
                   println!("DEBUG: item extraido: \n{}", item);
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
