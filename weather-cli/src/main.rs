mod models;
mod helper;
use models::{
    Location,
    CoordinatesResponse,
    Coordinates,
    WeatherResponse,
    WeatherData,
};
use urlencoding::encode;
use helper::{read_input, has_no_color_flag, colorize};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";

fn main() {
    let no_color = has_no_color_flag();
    let (display_name, coordinates) = loop {
        let (display_name, query_name) = choose_location(no_color);
        match location_cordinates(&query_name) {
            Ok(Some(coords)) => break (display_name, coords),
            Ok(None) => {
                println!(
                    "{}No results found for that location. Try again (format: \"City Country\").{}",
                    colorize(RED, no_color),
                    colorize(RESET, no_color)
                );
            }
            Err(reason) => {
                println!(
                    "{}Geocoding error: {}. Try again.{}",
                    colorize(RED, no_color),
                    reason,
                    colorize(RESET, no_color)
                );
            }
        }
    };
    println!(
        "{}{}Fetching weather for {}{}{}, lat: {:?}, lon: {:?}{}",
        colorize(CYAN, no_color),
        colorize(BOLD, no_color),
        colorize(GREEN, no_color),
        display_name,
        colorize(CYAN, no_color),
        coordinates.latitude,
        coordinates.longitude,
        colorize(RESET, no_color)
    );
    let current_weather = current_weather(coordinates.latitude, coordinates.longitude);
    println!();
    // for some reason the API doesn't always get current time?
    println!(
        "{}Fetching weather for {} at time {} (local time){}",
        colorize(YELLOW, no_color),
        display_name,
        current_weather.time,
        colorize(RESET, no_color)
    );
    println!(
        "{}Current weather in {}: {} {}{}",
        colorize(GREEN, no_color),
        display_name,
        current_weather.temperature,
        current_weather.unit,
        colorize(RESET, no_color)
    );
}

fn choose_location(no_color: bool) -> (String, String) {
    println!(
        "{}{}Choose location: 1) current location  2) enter city/country{}",
        colorize(BOLD, no_color),
        colorize(CYAN, no_color),
        colorize(RESET, no_color)
    );
    println!(
        "{}1 = current location, 2 = specified location{}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color)
    );

    loop {
        let choice = read_input("Enter 1 or 2: ");
        match choice.trim() {
            "1" => {
                let location = current_location(no_color);
                return (location.city.clone(), location.city);
            }
            "2" => {
                println!(
                    "{}Manual entry format: \"City Country\" (example: \"Paris France\"){}",
                    colorize(CYAN, no_color),
                    colorize(RESET, no_color)
                );
                let input = read_input("City Country: ").trim().to_string();
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() < 2 {
                    println!(
                        "{}Please enter both city and country separated by a space. Try again.{}",
                        colorize(RED, no_color),
                        colorize(RESET, no_color)
                    );
                    continue;
                }
                let city = parts[..parts.len() - 1].join(" ");
                let country = parts[parts.len() - 1];
                let display = format!("{}, {}", city, country);
                return (display.clone(), display);
            }
            _ => {
                println!(
                    "{}Invalid choice. Enter 1 or 2.{}",
                    colorize(RED, no_color),
                    colorize(RESET, no_color)
                );
            }
        }
    }
}

fn current_location(no_color: bool) -> Location {
    let response = reqwest::blocking::get("https://ipinfo.io/json").unwrap();

    let location: Location = response.json().unwrap();
    println!(
        "{}{}Detected location:{}{}",
        colorize(BOLD, no_color),
        colorize(CYAN, no_color),
        colorize(RESET, no_color),
        colorize(RESET, no_color)
    );
    println!(
        "{}IP:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.ip
    );
    println!(
        "{}City:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.city
    );
    println!(
        "{}Region:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.region
    );
    println!(
        "{}Country:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.country
    );
    println!(
        "{}Coordinates:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.loc
    );
    println!(
        "{}Organization:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.org
    );
    println!(
        "{}Postal Code:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.postal
    );
    println!(
        "{}Timezone:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.timezone
    );
    println!(
        "{}Readme:{} {}",
        colorize(YELLOW, no_color),
        colorize(RESET, no_color),
        location.readme
    );
    println!(
        "{}---------------------------{}",
        colorize(CYAN, no_color),
        colorize(RESET, no_color)
    );
    location
}

fn location_cordinates(city: &str) -> Result<Option<Coordinates>, String> {
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json", encode(city));
    let response: CoordinatesResponse = reqwest::blocking::get(&url)
        .unwrap()
        .json()
        .unwrap();

    if response.error.unwrap_or(false) {
        return Err(response.reason.unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(response.results.first().map(|result| Coordinates {
        latitude: result.latitude,
        longitude: result.longitude,
    }))
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
    }
}
