mod models;
use models::Location;

fn main() {
    location();
}

fn location() {
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
}