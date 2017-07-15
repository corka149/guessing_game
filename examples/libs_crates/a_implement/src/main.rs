extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.",
                self.high_temp,
                self.low_temp,
                self.chance_of_precipitation)
    }
}

fn main() {
    let a_forecast = WeatherForecast {
        high_temp: 40.1,
        low_temp: -11.2,
        chance_of_precipitation: 90.2,
    };

    println!("The forecast {}", a_forecast.summary());
}
