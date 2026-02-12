use chrono_tz::Tz;

#[derive(Clone)]
pub struct City {
    pub name: String,
    pub timezone: Tz,
}

impl City {
    pub fn new(name: String, timezone: Tz) -> Self {
        Self { name, timezone }
    }
}

pub fn get_country_flag(city_name: &str) -> &str {
    crate::flags::get_country_flag(city_name)
}

pub fn get_default_cities() -> Vec<City> {
    vec![
        City::new("Tokyo".to_string(), chrono_tz::Asia::Tokyo),
        City::new("Seoul".to_string(), chrono_tz::Asia::Seoul),
        City::new("Singapore".to_string(), chrono_tz::Asia::Singapore),
        City::new("Dubai".to_string(), chrono_tz::Asia::Dubai),
        City::new("Moscow".to_string(), chrono_tz::Europe::Moscow),
        City::new("London".to_string(), chrono_tz::Europe::London),
        City::new("Paris".to_string(), chrono_tz::Europe::Paris),
        City::new("New York".to_string(), chrono_tz::America::New_York),
        City::new("Los Angeles".to_string(), chrono_tz::America::Los_Angeles),
        City::new("Chicago".to_string(), chrono_tz::America::Chicago),
        City::new("Toronto".to_string(), chrono_tz::America::Toronto),
        City::new("Sydney".to_string(), chrono_tz::Australia::Sydney),
        City::new("Auckland".to_string(), chrono_tz::Pacific::Auckland),
    ]
}
