use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherSearch {
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WeatherCondition {
    pub text: String,
    pub icon: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CurrentWeather {
    pub temp_c: f64,
    pub feelslike_c: f64,
    pub temp_f: f64,
    pub feelslike_f: f64,
    pub humidity: f64,
    pub is_day: i32,
    pub condition: WeatherCondition,
    pub last_updated_epoch: i64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Weather {
    pub location: Location,
    pub current: CurrentWeather,
}
