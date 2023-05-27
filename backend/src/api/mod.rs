use serde::Deserialize;

pub mod journey_api;
pub mod station_api;


#[derive(Debug, Deserialize)]
pub struct PageRequest {
    page: Option<usize>,
    order: Option<String>,
    sort: Option<bool>
}

impl PageRequest {
    pub fn get_page(&self) -> usize {
        match self.page {
            Some(x) => return x,
            None => return 0
        }
    }
    pub fn get_order(&self) -> &str {
        match &self.order {
            Some(x) => return x,
            None => return ""
        }
    }
    pub fn get_sort(&self) -> bool {
        self.sort.is_some()
    }
}
