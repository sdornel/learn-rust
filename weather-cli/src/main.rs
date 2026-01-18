mod models;
use models::{
    Location,
    CoordinatesResponse,
    Coordinates,
    WeatherResponse,
    WeatherData,
};
use urlencoding::encode;

fn main() {
    let city: String = location();
    let coordinates: Coordinates = location_cordinates(&city);
    println!("Fetching weather for {}, lat: {:?}, lon: {:?}", city, coordinates.latitude, coordinates.longitude);
    let current_weather = current_weather(coordinates.latitude, coordinates.longitude);
    println!("");
    // for some reason the API doesn't always get current time?
    println!("Fetching weather for {} at time {} (local time)", city, current_weather.time);
    println!("Current weather in {}: {} {}",
        city,
        current_weather.temperature,
        current_weather.unit
    );
}

fn location() -> String {
    let response = reqwest::blocking::get("https://ipinfo.io/json").unwrap();

    let location: Location = response.json().unwrap();
    println!("IP: {}", location.ip);
    println!("City: {}", location.city);
    println!("Region: {}", location.region);
    println!("Country: {}", location.country);
    println!("Coordinates: {}", location.loc);
    println!("Organization: {}", location.org);
    println!("Postal Code: {}", location.postal);
    println!("Timezone: {}", location.timezone);
    println!("Readme: {}", location.readme);
    println!("---------------------------");
    location.city
}

fn location_cordinates(city: &str) -> Coordinates {
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json", encode(city));
    let response: CoordinatesResponse = reqwest::blocking::get(&url)
        .unwrap()
        .json()
        .unwrap();

    Coordinates {
        latitude: response.results[0].latitude,
        longitude: response.results[0].longitude,
    }
}

fn current_weather(latitude: f64, longitude: f64) -> WeatherData {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true", latitude, longitude);
    let response: WeatherResponse = reqwest::blocking::get(&url)
        .unwrap()
        .json()
        .unwrap();

    WeatherData {
        time: response.current_weather.time,
        unit: response.current_weather_units.temperature,
        temperature: response.current_weather.temperature,
        timezone: response.timezone,
    }
}