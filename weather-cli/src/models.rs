use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
    pub readme: String,
}

#[derive(Deserialize, Debug)]
pub struct CoordinatesResponse {
    pub results: Vec<Coordinates>,
}

#[derive(Deserialize, Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub timezone: String,
    pub current_weather_units: CurrentWeatherUnits,
    pub current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
pub struct CurrentWeatherUnits {
    pub temperature: String,
}

#[derive(Deserialize, Debug)]
pub struct CurrentWeather {
    pub time: String,
    pub temperature: f64,
    pub windspeed: f64,
    pub winddirection: f64,
    pub is_day: u8,
}

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub time: String,
    pub temperature: f64,
    pub unit: String,
    pub timezone: String,
    // pub windspeed: f64,
    // pub winddirection: f64,
    // pub is_day: u8,
}