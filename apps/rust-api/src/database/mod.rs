use postgres::Client;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::{env, error};
use dotenv::dotenv;


pub fn connect() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok(); // Load env variables

    let db_connection_string = env::var("DB_CONNECTION")
        .expect("DB_CONNECTION_STRING must be set in .env file");

    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());
    let mut client = Client::connect(&db_connection_string, connector)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS keyword  (
            id              SERIAL PRIMARY KEY,
            canonical       VARCHAR NOT NULL,
            variations      VARCHAR[]
        )
    ")?;

    Ok(())
}
