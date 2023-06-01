use crate::{
    models::{journey::Journey, station::Station},
    prelude::Error,
};
use anyhow::Result;
use chrono::DateTime;
use serde::Deserialize;
use std::{env::current_dir, path::PathBuf};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};

/// Reads a file that contains station data. Sends that data to database.
async fn read_stations(file_name: &str) -> Result<Vec<Station>, Error> {
    // For OS reasons
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

    Ok(stations)
}

#[derive(Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

/// TODO: Consider to move this fn to different file
async fn send_stations_to_db(db: &Surreal<Db>, stations: Vec<Station>) -> Result<(), Error> {
    db.query(r#"BEGIN TRANSACTION;"#).await?;
    for station in stations {
        let _: Record = db.create("station").content(station).await?;
    }
    db.query(r#"COMMIT TRANSACTION;"#);
    Ok(())
}

/// Helper struct when deserializing csv file.
#[derive(Deserialize, Debug)]
struct CsvJourney {
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

/// Parses string to chrono::DateTime
fn parse_date(s: &str) -> Result<DateTime<chrono::FixedOffset>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(format!("{}Z", s).as_str())
}

/// Reads a file that contains journey data. After reading rows to vector of journeys, vector is
/// sent to database.
async fn read_journeys(file_name: &str) -> Result<Vec<Journey>, Error> {
    // for OS reasons
    let path: PathBuf = [
        current_dir()?,
        "data".into(),
        format!("{}", file_name).into(),
    ]
    .iter()
    .collect();

    // TODO Consider to send data on 100k row vectors to database rather than in a whole vector.
    // Might use lesser amount RAM when reading 1M rows and then sending whole vector.
    let journeys: Vec<Journey> = csv::ReaderBuilder::new()
        .from_path(path)
        .expect("panic error")
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

    Ok(journeys)
}

/// Sends journey data to the database in a transaction block.
// TODO: Consider to move to different file.
async fn send_journeys_to_db(db: &Surreal<Db>, journeys: Vec<Journey>) -> Result<(), Error> {
    db.query(r#"BEGIN TRANSACTION"#).await?;

    for journey in journeys {
        let _: Record = db.create("journey").content(journey).await?;
    }

    db.query(r#"COMMIT TRANSACTION"#).await?;
    Ok(())
}

/// Initiliazes database with stations and journey that are read from csv files.
pub async fn read_files(db: &Surreal<Db>) -> Result<(), Error> {
    send_stations_to_db(db, read_stations("stations.csv").await?).await?;

    // with all of these imported, duration: 48min
    // specs: wsl2 ubuntu restricted to 2 cores and 4GB RAM
    // TODO: journey page query failed
    send_journeys_to_db(db, read_journeys("journeys.csv").await?).await?;

    // Optional dataset - Make sure that these are in the data folder
    
    // send_journeys_to_db(db, read_journeys("2021-07.csv").await?).await?;
    // send_journeys_to_db(db, read_journeys("2021-06.csv").await?).await?;
    // send_journeys_to_db(db, read_journeys("2021-05.csv").await?).await?;
    
    Ok(())
}
