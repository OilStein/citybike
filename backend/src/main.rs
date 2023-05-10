use std::time::Instant;

use backend::utils::csv_reader::read_files;
use surrealdb::{engine::local::RocksDb, Surreal};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let start = Instant::now();

    let db = Surreal::new::<RocksDb>("temp.db").await?;
    db.use_ns("test").use_db("test").await?;

    // reads stations
    read_files();

    let end = Instant::now();

    println!("Duration: {:?}", (end - start));

    Ok(())
}
