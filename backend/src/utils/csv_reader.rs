use crate::{
    models::{journey::Journey, station::Station},
    prelude::Error,
};
use anyhow::Result;
use chrono::DateTime;
use serde::Deserialize;
use std::{env::current_dir, path::PathBuf};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};

async fn read_stations(db: &Surreal<Db>, file_name: &str) -> Result<usize, Error> {
    let path: PathBuf = [
        current_dir()?,
        "data".into(),
        format!("{}", file_name).into(),
    ]
    .iter()
    .collect();

    let stations: Vec<Station> = csv::ReaderBuilder::new()
        .from_path(path)
        .expect("Bad error hendling")
        .deserialize::<Station>()
        .filter_map(Result::ok)
        .filter(Station::validate)
        .collect();

    let count = &stations.len();

    // Transaction
    db.query(r#"BEGIN TRANSACTION;"#).await?;

    for station in stations {
        create_station(db, station).await?;
    }

    db.query(r#"COMMIT TRANSACTION;"#);

    Ok(*count)
}

#[derive(Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

async fn create_station(db: &Surreal<Db>, station: Station) -> Result<(), Error> {
    // let data: Object = W(station.into()).try_into()?;
    let _: Record = db.create("station").content(station).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct CsvJourney {
    #[serde(rename(deserialize = "Departure"))]
    departure: String,
    #[serde(rename(deserialize = "Return"))]
    arrival: String,
    #[serde(rename(deserialize = "Departure station id"))]
    dep_station_id: usize,
    #[serde(rename(deserialize = "Departure station name"))]
    dep_station_name: String,
    #[serde(rename(deserialize = "Return station id"))]
    tar_station_id: usize,
    #[serde(rename(deserialize = "Return station name"))]
    tar_station_name: String,
    #[serde(rename(deserialize = "Covered distance (m)"))]
    distance: usize,
    #[serde(rename(deserialize = "Duration (sec.)"))]
    duration: usize,
}

fn parse_date(s: &str) -> Result<DateTime<chrono::FixedOffset>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(format!("{}Z", s).as_str())
}

async fn read_journeys(db: &Surreal<Db>, file_name: &str) -> Result<usize> {
    let path: PathBuf = [
        current_dir()?,
        "data".into(),
        format!("{}", file_name).into(),
    ]
    .iter()
    .collect();

    let journeys: Vec<Journey> = csv::ReaderBuilder::new()
        .from_path(path)?
        .deserialize::<CsvJourney>()
        .filter_map(Result::ok)
        .filter_map(|csv_journey| {
            parse_date(&csv_journey.departure)
                .ok()
                .and_then(|departure| {
                    parse_date(&csv_journey.arrival)
                        .ok()
                        .map(|arrival| (csv_journey, departure, arrival))
                })
        })
        .filter_map(|(csv_journey, departure, arrival)| {
            let journey = Journey::new(
                departure.into(),
                arrival.into(),
                csv_journey.dep_station_id,
                csv_journey.dep_station_name,
                csv_journey.tar_station_id,
                csv_journey.tar_station_name,
                csv_journey.distance,
                csv_journey.duration,
            );
            if journey.validate() {
                Some(journey)
            } else {
                None
            }
        })
        .collect();

    let count = &journeys.len();

    create_journey(db, journeys).await?;

    Ok(*count)
}

async fn create_journey(db: &Surreal<Db>, journeys: Vec<Journey>) -> Result<(), Error> {
    db.query(r#"BEGIN TRANSACTION"#).await?;

    for journey in journeys {
        let _: Record = db.create("journey").content(journey).await?;
    }

    db.query(r#"COMMIT TRANSACTION"#).await?;
    Ok(())
}

pub async fn read_files(db: &Surreal<Db>) {
    println!(
        "stations: {}",
        read_stations(db, "stations.csv").await.unwrap()
    );
    // println!("journeys: {}", read_journeys("2021-05.csv").unwrap());
    // println!("journeys: {}", read_journeys("2021-06.csv").unwrap());
    // println!("journeys: {}", read_journeys("2021-07.csv").unwrap());
    println!(
        "journeys: {}",
        read_journeys(db, "journeys.csv").await.unwrap()
    );
}
