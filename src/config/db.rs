use std::env;

use mongodb::{Client, Database, options::ClientOptions};

pub async fn connect_db() -> mongodb::error::Result<Database> {
    let client_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let options = ClientOptions::parse(client_uri).await?;

    let client = Client::with_options(options)?;

    Ok(client.database("blogSpot"))
}
