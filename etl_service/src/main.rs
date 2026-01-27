use chrono::{NaiveDateTime};
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

enum StoreType {
    Kabum,
    Unknown,
}

struct PriceHisotry {
    id: Uuid,
    product_id: Uuid,
    value: Decimal,
    created_at: NaiveDateTime
}

impl fmt::Display for PriceHisotry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Price History: \nID: {} \nProduct ID: {} \nLast checked Price: {} \nChecked At: {}",
        &self.id, &self.product_id, &self.value, &self.created_at
    )}
}

struct Product {
    id: Uuid,
    url: String,
    title: String,
    store: String,
    last_checked_at: NaiveDateTime,
    created_at: NaiveDateTime,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "Product {}: \nID: {} \nUrl: {} \nStore: {} \nLast Checked At: {} \nCreated At: {}", 
        &self.title, &self.id, &self.url, &self.store, &self.last_checked_at, &self.created_at
    )}
}

struct SiteRules {
    title_selector: String,
    price_selector: String,
}

fn get_site_rules(url: &str) -> Result<SiteRules, String> {
    if url.contains("kabum.com.br") {
        Ok(SiteRules { 
            title_selector: "h1".to_string(), 
            price_selector: "h4".to_string(),
        })
    } else {
        Err(format!("Site não suportado: {}", url))
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
    let conn_str = env::var("DATABASE_URL").expect("ERROR: variável POSTGRES_USER não encontrada");
    println!("LOG: Conectando ao Postgres em: {}", conn_str);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str).await?;
    Ok(pool)
}

fn extract_kabum(document: RawPage) -> Result<(Product, PriceHisotry), String> {
    let rules = get_site_rules(&document.url)?;
    let fragment = Html::parse_fragment(&document.html);
    
    let title_sel = Selector::parse(&rules.title_selector)
        .map_err(|_| "ERROR: falha ao buscar pelo seletor do título")?;

    let title = fragment.select(&title_sel)
        .next()
        .map(|el| el.text().collect::<String>())
        .ok_or("ERROR: elemento Título não encontrado")?;

    let price_sel = Selector::parse(&rules.price_selector)
        .map_err(|_| "ERROR: falha ao buscar pelo seletor do preço")?;

    let price = fragment.select(&price_sel)
        .next()
        .map(|el| el.text().collect::<String>())
        .ok_or("ERROR: elemento preço não encontrado")?;

    let price_clean = price
        .replace("R$", "")
        .replace(" ", "")
        .replace(".", "")
        .replace(",", ".")
        .replace("&nbsp;", "");


    let price_clean = price_clean.trim();
    let value = Decimal::from_str(&price_clean).unwrap_or(Decimal::ZERO);

    let product = Product {
        id:Uuid::new_v4(), 
        title: title, 
        store: "kabum".to_string(),
        url: document.url,
        last_checked_at: chrono::Utc::now().naive_utc(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let price_chapter = PriceHisotry {
        id:Uuid::new_v4(),
        product_id:product.id,
        value:value,
        created_at:chrono::Utc::now().naive_utc(),
    };

    return Ok((product, price_chapter));
}

fn detect_store(url: &str) -> StoreType {
    if url.contains("kabum.com.br") {
        return StoreType::Kabum;
    } else {
        return StoreType::Unknown;
    }
}

fn detect_store_string(url: &str) -> String {
    if url.contains("kabum.com.br") {
        return "kabum".to_string();
    } else {
        return "unknown".to_string();
    }
}

async fn save_product(pool: &PgPool, product: &Product) -> Result<Uuid, sqlx::Error> {
    let product_record = sqlx::query!(
        r#"
        INSERT INTO products (url, title, store, last_checked_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (url)
        DO UPDATE SET 
            last_checked_at = EXCLUDED.last_checked_at,
            title = EXCLUDED.title
        RETURNING id
        "#,
        product.url,
        product.title,
        detect_store_string(&product.url),
        product.created_at
    )
    .fetch_one(pool)
    .await?;

    Ok(product_record.id)
}

async fn save_price_history(pool: &PgPool, price_chapter: &PriceHisotry, product_id: Uuid) -> Result<(), sqlx::error::Error>{
    sqlx::query!(
        r#"
        INSERT INTO price_history (product_id, value, created_at)
        VALUES ($1, $2, $3)
        "#,
        product_id,
        price_chapter.value,
        price_chapter.created_at
    )
    .execute(pool)
    .await?;

    println!("Histórico do produto ID: {} criado", price_chapter.product_id);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let channel = connect_rabbit().await?;
    let collection = connect_mongo().await?;
    let pool = connect_postgres().await?;

    // criando uma stream (pipe para comunicação)
    let mut consumer = channel
        .basic_consume(
            "raw_prices",
            "rust_worker",
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
                    println!("LOG: Documento encontrado. URL: {}", document.url);

                    let result = match detect_store(&document.url) {
                        StoreType::Kabum => extract_kabum(document),
                        StoreType::Unknown => Err(format!("Loja não suportada ou desconhecida: {}", document.url))
                    };
                    
                    match result {
                        Ok((product, price_chapter)) => {
                            match save_product(&pool, &product).await {
                                Ok(product_id) => {
                                    match save_price_history(&pool, &price_chapter, product_id).await {
                                        Ok(()) => {
                                            println!("DEBUG: Sucesso ao salvar o preço: {}", price_chapter);
                                        },
                                        Err(e) =>{
                                            eprintln!("ERROR: falha ao salvar o histórico do produto: {}", e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    eprintln!("ERROR: falha ao salvar/atualizar produto: {}", e);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("ERROR: falha ao buscar pela loja desejada: {}", e);
                        }
                    }
                },
                Ok(None) => eprintln!("WARNING: documento não encontrado no mongo, ID: {}", payload.url),
                Err(e) => eprintln!("ERROR: falha de conexão com Mongo: {}", e),
            }

            // confirma processamento
            delivery.ack(BasicAckOptions::default()).await?;
        }
    };

    Ok(())
}