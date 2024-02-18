use postgres::{Client, NoTls};
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::{env, error};
use dotenv::dotenv;
use crate::kw_list::input_list::keyword_input;


pub fn connect_local() -> Result<Client, Box<dyn error::Error>> {
    dotenv().ok(); // Load env variables

    // Use environment variables that match your docker-compose.yml configuration
    let db_connection_string = env::var("DB_CONNECTION_DOCKER")
        .expect("DB_CONNECTION_DOCKER must be set in .env file");
    // Format: "host=localhost user=postgres password=whatever dbname=keywords_data"

    // Connect without TLS since it's a local Docker connection
    Ok(Client::connect(&db_connection_string, NoTls)?)
}

pub fn connect() -> Result<Client, Box<dyn error::Error>> {
    dotenv().ok(); // Load env variables

    let db_connection_string = env::var("DB_CONNECTION")
        .expect("DB_CONNECTION_STRING must be set in .env file");

    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());

    Ok(Client::connect(&db_connection_string, connector)?)
}

pub fn create_table(client: &mut Client) {
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS keyword  (
            id              SERIAL PRIMARY KEY,
            canonical       VARCHAR NOT NULL,
            variations      VARCHAR[]
        )
    ").expect("TODO: panic message");
}

pub fn seed_db(client: &mut Client) {
    let keywords = keyword_input();
    for (canonical, variations) in keywords {
        client.execute(
            "INSERT INTO keyword (canonical, variations) VALUES ($1, $2)",
            &[&canonical, &variations]
        ).unwrap();
    }
}

pub fn read_db(client: &mut Client) {
    for row in client.query("SELECT id, canonical, variations FROM keyword", &[]).unwrap() {
        let id: i32 = row.get(0);
        let canonical: &str = row.get(1);
        let variations: Vec<String> = row.get(2);
        println!("Found keyword {}: {} {:?}", id, canonical, variations);
    }
}

