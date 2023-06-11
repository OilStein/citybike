use std::time::Instant;

use actix_cors::Cors;
use actix_web::http;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use backend::api::journey_api::get_journeys_page;
use backend::api::station_api::get_stations_by_page;
use backend::utils::csv_reader::read_files;
use backend::{
    api::journey_api::{get_all_journeys, get_journey_by_id},
    api::station_api::get_station_by_id,
    prelude::*,
};
use log::info;
use surrealdb::{engine::local::RocksDb, Surreal};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // ! true: resets database
    // ! false: uses already initialized database, make sure you have initialized database once
    let init = true;

    if init {
        delete_temp_db_file()?;
    }

    // Times initialzation
    let start = Instant::now();

    // reads datafiles and "bulk inserts" to database
    // If temp.db exists uses that one, otherwise creates a new
    let db = Surreal::new::<RocksDb>("temp.db").await?;
    db.use_ns("citybike").use_db("citybike").await?;

    if init {
        read_files(&db).await?;
    }

    let data = Data::new(db);

    let end = Instant::now();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("Setting up duration: {:?}", (end - start));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .service(get_station_by_id)
            .service(get_all_journeys)
            .service(get_journey_by_id)
            .service(get_stations_by_page)
            .service(get_journeys_page)
    })
    .bind("localhost:8080")?
    .run()
    .await?;

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_rt::test]
    async fn test_get_station_by_id() {
        let db = Surreal::new::<RocksDb>("temp.db").await.unwrap();
        let data = Data::new(db);
        let mut app =
            test::init_service(App::new().app_data(data).service(get_station_by_id)).await;
        let req = test::TestRequest::get().uri("/stations/1").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert!(body.contains("station_id"));
    }
}
