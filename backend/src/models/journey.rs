use chrono::prelude::*;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
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
