use std::time::Instant;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use backend::utils::csv_reader::read_files;
use backend::{
    api::station_api::{get_all_stations, get_station_by_id},
    prelude::*,
};
use log::info;
use surrealdb::{engine::local::RocksDb, Surreal};

#[tokio::main]
async fn main() -> Result<(), Error> {
    delete_temp_db_file()?;
    let start = Instant::now();

    // reads datafiles and "bulk inserts" to database
    // If temp.db exists uses that one, otherwise creates a new
    let db = Surreal::new::<RocksDb>("temp.db").await?;
    db.use_ns("citybike").use_db("citybike").await?;

    // if !temp_db_files_exists() {
    //     println!("Database didn't exists");
    read_files(&db).await;
    // }

    let data = Data::new(db);

    let end = Instant::now();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("Setting up duration: {:?}", (end - start));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(get_all_stations)
            .service(get_station_by_id)
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

use std::fs;

fn delete_temp_db_file() -> std::io::Result<()> {
    if let Err(e) = fs::remove_dir_all("temp.db") {
        if e.kind() == std::io::ErrorKind::NotFound {
            println!("temp.db file does not exist.");
        } else {
            return Err(e);
        }
    } else {
        println!("temp.db file has been deleted.");
    }

    Ok(())
}
