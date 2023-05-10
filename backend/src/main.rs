use std::time::Instant;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use backend::utils::csv_reader::read_files;
use backend::{api::station_api::get_stations, prelude::*};
use surrealdb::{engine::local::RocksDb, Surreal};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start = Instant::now();

    // reads datafiles and "bulk inserts" to database
    // If temp.db exists uses that one, otherwise creates a new
    let db = Surreal::new::<RocksDb>("temp.db").await?;
    db.use_ns("citybike").use_db("citybike").await?;

    if !temp_db_files_exists() {
        println!("Database didn't exists");
        read_files(&db).await;
    }

    let data = Data::new(db);

    let end = Instant::now();
    println!("Setting up duration: {:?}", (end - start));
    println!("Starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(get_stations)
    })
    .bind("localhost:8080")?
    .run()
    .await?;

    Ok(())
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

use std::path::Path;

fn temp_db_files_exists() -> bool {
    let path = Path::new("temp.db/CURRENT");
    path.is_file()
}
