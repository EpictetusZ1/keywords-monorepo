use postgres::{Client, NoTls};
// use openssl::ssl::{SslConnector, SslMethod};
// use postgres_openssl::MakeTlsConnector;
use std::{env, error};
use dotenv::dotenv;
use crate::kw_list::input_list::keyword_input;


pub fn connect_local() -> Result<Client, Box<dyn error::Error>> {
    dotenv().ok();

    let db_connection_string = env::var("DB_CONNECTION_DOCKER")
        .expect("DB_CONNECTION_DOCKER must be set in .env file");

    Ok(Client::connect(&db_connection_string, NoTls)?)
}

// pub fn connect() -> Result<Client, Box<dyn error::Error>> {
//     dotenv().ok(); // Load env variables
//
//     let db_connection_string = env::var("DB_CONNECTION")
//         .expect("DB_CONNECTION_STRING must be set in .env file");
//
//     let builder = SslConnector::builder(SslMethod::tls())?;
//     let connector = MakeTlsConnector::new(builder.build());
//
//     Ok(Client::connect(&db_connection_string, connector)?)
// }

pub fn create_table(client: &mut Client) {
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS keyword  (
            id              SERIAL PRIMARY KEY,
            canonical       VARCHAR NOT NULL,
            variations      VARCHAR[]
        )
    ").expect("TODO: panic message");

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS listing (
            id              SERIAL PRIMARY KEY,
            description     VARCHAR NOT NULL,
            position        VARCHAR NOT NULL,
            requirements    VARCHAR NOT NULL
    )
    ").expect("panic!");

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS listing_keyword (
            listing_id      INTEGER NOT NULL,
            keyword_id      INTEGER NOT NULL,
            PRIMARY KEY(listing_id, keyword_id),
            FOREIGN KEY(listing_id) REFERENCES listing(id) ON DELETE CASCADE,
            FOREIGN KEY(keyword_id) REFERENCES keyword(id) ON DELETE CASCADE
        )
    ").expect("panic!");
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

pub fn add_listing(client: &mut Client, description: &str, position: &str, requirements: &str, skills: &Vec<String>) {
    let listing_id: i32 = client.query_one(
        "INSERT INTO listing (description, position, requirements) VALUES ($1, $2, $3) RETURNING id",
        &[&description, &position, &requirements]
    ).unwrap().get(0);

    for skill in skills {
        let keyword_id: i32 = client.query_one(
            "SELECT id FROM keyword WHERE canonical = $1",
            &[&skill]
        ).unwrap().get(0);

        client.execute(
            "INSERT INTO listing_keyword (listing_id, keyword_id) VALUES ($1, $2)",
            &[&listing_id, &keyword_id]
        ).unwrap();
    }
}

