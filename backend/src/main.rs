use std::time::Instant;

use backend::utils::csv_reader::read_files;
use surrealdb::{engine::local::RocksDb, Surreal};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let start = Instant::now();

    // If temp.db exists uses that one, otherwise creates a new
    let db = Surreal::new::<RocksDb>("temp.db").await?;
    db.use_ns("citybike").use_db("citybike").await?;

    // reads stations
    read_files(&db).await;

    let end = Instant::now();

    println!("Duration: {:?}", (end - start));

    Ok(())
}
