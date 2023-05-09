use scylla::{FromUserType, IntoUserType};
use serde::Deserialize;

#[derive(Deserialize, IntoUserType, FromUserType)]
pub struct Station {
    #[serde(rename(deserialize = "FID"))]
    fid: i16,
    #[serde(rename(deserialize = "ID"))]
    id: i16,
    #[serde(rename(deserialize = "Nimi"))]
    name_fi: String,
    #[serde(rename(deserialize = "Namn"))]
    name_eng: String,
    #[serde(rename(deserialize = "Name"))]
    name_swe: String,
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
    capacity: i16,
    #[serde(rename(deserialize = "x"))]
    latitude: f32,
    #[serde(rename(deserialize = "y"))]
    longitude: f32,
}

impl Station {
    pub fn new(
        fid: i16,
        id: i16,
        name_fi: String,
        name_swe: String,
        name_eng: String,
        address_fi: String,
        address_swe: String,
        city_fi: Option<String>,
        city_swe: Option<String>,
        operator: Option<String>,
        capacity: i16,
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
