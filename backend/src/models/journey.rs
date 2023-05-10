use std::collections::BTreeMap;

use chrono::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use surrealdb::sql::Datetime;
use surrealdb::sql::Value;

use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Journey {
    departure: DateTime<Utc>,
    arrival: DateTime<Utc>,
    dep_station_id: usize,
    dep_station_name: String,
    tar_station_id: usize,
    tar_station_name: String,
    distance: usize,
    duration: usize,
}

impl Journey {
    pub fn new(
        departure: DateTime<Utc>,
        arrival: DateTime<Utc>,
        dep_station_id: usize,
        dep_station_name: String,
        tar_station_id: usize,
        tar_station_name: String,
        distance: usize,
        duration: usize,
    ) -> Self {
        Journey {
            departure,
            arrival,
            dep_station_id,
            dep_station_name,
            tar_station_id,
            tar_station_name,
            distance,
            duration,
        }
    }
    fn is_departure_first(&self) -> bool {
        self.departure.le(&self.arrival)
    }

    pub fn validate(&self) -> bool {
        if self.duration <= 10 {
            return false;
        }
        if self.distance < 10 {
            return false;
        }
        if !self.is_departure_first() {
            return false;
        }
        true
    }
}

impl From<Journey> for Value {
    fn from(val: Journey) -> Self {
        let value: BTreeMap<String, Value> = map![
            "departure".into() => val.departure.into(),
            "arrival".into() => val.arrival.into(),
            "dep_station_id".into() => val.dep_station_id.into(),
            "dep_station_name".into() => val.dep_station_name.into(),
            "tar_station_id".into() => val.tar_station_id.into(),
            "tar_station_name".into() => val.tar_station_name.into(),
            "distance".into() => val.distance.into(),
            "duration".into() => val.duration.into(),
        ]
        .into();

        Value::from(value)
    }
}

pub trait Creatable: Into<Value> {}
impl Creatable for Journey {}
