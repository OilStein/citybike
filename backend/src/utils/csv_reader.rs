use crate::{
    models::{journey::Journey, station::Station},
    prelude::{Error, W},
};
use anyhow::Result;
use chrono::DateTime;
use serde::Deserialize;
use std::{env::current_dir, path::PathBuf};
use surrealdb::{
    engine::local::Db,
    sql::{Object, Value},
    Surreal,
};

async fn read_stations(db: &Surreal<Db>, file_name: &str) -> Result<usize, Error> {
    let path: PathBuf = [
        current_dir()?,
        "data".into(),
        format!("{}", file_name).into(),
    ]
    .iter()
    .collect();

    /*
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;

        let mut stations: Vec<Station> = vec![];

        for result in rdr.deserialize() {
            let record: Station = match result {
                Err(_err) => continue,
                Ok(x) => x,
            };

            if record.validate() {
                stations.push(record);
            }
        }
    */
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
        let data: Object = W(station.into()).try_into()?;
        db.create("station").content(Value::from(data)).await?;
    }

    db.query(r#"COMMIT TRANSACTION;"#);

    Ok(*count)
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

fn read_journeys(file_name: &str) -> Result<usize> {
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

    /*
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;

        let mut journeys: Vec<Journey> = vec![];

        for result in rdr.deserialize() {
            let csv_journey: CsvJourney = match result {
                Err(_err) => continue,
                Ok(x) => x,
            };
            // Date parse checking
            let departure = match parse_date(&csv_journey.departure) {
                Ok(dep) => dep,
                Err(_e) => continue,
            };
            let arrival = match parse_date(&csv_journey.arrival) {
                Ok(arr) => arr,
                Err(_e) => continue,
            };
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
                journeys.push(journey);
            }
        }
    */
    Ok(journeys.len())
}

pub async fn read_files(db: &Surreal<Db>) {
    println!(
        "stations: {}",
        read_stations(db, "stations.csv").await.unwrap()
    );
    // println!("journeys: {}", read_journeys("2021-05.csv").unwrap());
    // println!("journeys: {}", read_journeys("2021-06.csv").unwrap());
    // println!("journeys: {}", read_journeys("2021-07.csv").unwrap());
    println!("journeys: {}", read_journeys("journeys.csv").unwrap());
}
