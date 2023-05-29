use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};

use crate::utils::macros::map;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordStation {
    id: Thing,
    // fid: usize,
    name_fi: String,
    // address_fi: String,
    //  city_fi: String,
    // capacity: usize,
    // latitude: f32,
    // longitude: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SingleStationView {
    id: Thing,
    name_fi: String,
    address_fi: String,
    capacity: usize,
    latitude: f32,
    longitude: f32,
    data: StationData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StationData {
    starting: usize,
    ending: usize,
    // mean_start: f32,
    // mean_end: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Station {
    #[serde(rename(deserialize = "FID"))]
    fid: usize,
    #[serde(rename(deserialize = "ID"))]
    id: usize,
    #[serde(rename(deserialize = "Nimi"))]
    name_fi: String,
    #[serde(rename(deserialize = "Namn"))]
    name_swe: String,
    #[serde(rename(deserialize = "Name"))]
    name_eng: String,
    #[serde(rename(deserialize = "Osoite"))]
    address_fi: String,
    #[serde(rename(deserialize = "Adress"))]
    address_swe: String,
    #[serde(rename(deserialize = "Kaupunki"))]
    city_fi: Option<String>,
    #[serde(rename(deserialize = "Stad"))]
    city_swe: Option<String>,
    #[serde(rename(deserialize = "Operaattor"))]
    operator: Option<String>,
    #[serde(rename(deserialize = "Kapasiteet"))]
    capacity: usize,
    #[serde(rename(deserialize = "x"))]
    latitude: f32,
    #[serde(rename(deserialize = "y"))]
    longitude: f32,
}

impl Station {
    pub fn new(
        fid: usize,
        id: usize,
        name_fi: String,
        name_swe: String,
        name_eng: String,
        address_fi: String,
        address_swe: String,
        city_fi: Option<String>,
        city_swe: Option<String>,
        operator: Option<String>,
        capacity: usize,
        latitude: f32,
        longitude: f32,
    ) -> Station {
        Station {
            fid,
            id,
            name_fi,
            name_swe,
            name_eng,
            address_fi,
            address_swe,
            city_fi,
            city_swe,
            operator,
            capacity,
            latitude,
            longitude,
        }
    }

    pub fn validate(&self) -> bool {
        // Checks are a station in hard coded coordinate box
        if (self.longitude > 58.0 && self.longitude < 64.0)
            && (self.latitude > 23.0 && self.latitude < 27.0)
        {
            return true;
        }
        false
    }
}

impl From<Station> for Value {
    fn from(val: Station) -> Self {
        let mut value: BTreeMap<String, Value> = map![
            "fid".into() => val.fid.into(),
            "id".into() => val.id.into(),
            "name_fi".into() => val.name_fi.into(),
            "name_eng".into() => val.name_eng.into(),
            "name_swe".into() => val.name_swe.into(),
            "address_fi".into() => val.address_fi.into(),
            "address_swe".into() => val.address_swe.into(),
            "capacity".into() => val.capacity.into(),
            "latitude".into() => val.latitude.into(),
            "longitude".into() => val.longitude.into(),
        ]
        .into();

        if let Some(cfi) = val.city_fi {
            value.insert("city_fi".into(), cfi.into());
        }
        if let Some(cswe) = val.city_swe {
            value.insert("city_swe".into(), cswe.into());
        }
        if let Some(oper) = val.operator {
            value.insert("operator".into(), oper.into());
        }

        Value::from(value)
    }
}

pub trait Creatable: Into<Value> {}
impl Creatable for Station {}
