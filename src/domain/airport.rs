#[derive(Debug, Clone)]
pub struct Airport {
    iata_code: String, // 3-digi code
    icao_code: String, // 4-digi code
    name: String,
    city: String,
    province: String,
    country: String,
}

impl Airport {
    pub fn new(iata_code: String, icao_code: String, name: String, city: String, province: String, country: String) -> Self {
        return Airport { iata_code, icao_code, name, city, province, country };
    }

    pub fn get_iata_code(&self) -> &str {
        return &self.iata_code;
    }

    pub fn get_icao_code(&self) -> &str {
        return &self.icao_code;
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_city(&self) -> &str {
        return &self.city;
    }

    pub fn get_province(&self) -> &str {
        return &self.province;
    }

    pub fn get_country(&self) -> &str {
        return &self.country;
    }

    pub fn get_code(&self) -> String {
        return format!("{}/{}", self.iata_code, self.icao_code);
    }
}

#[cfg(test)]
mod test_airport {
    use super::*;

    #[test]
    fn test_if_airport_creation_works() {
        let airport = Airport::new(
            "JFK".into(),
            "KJFK".into(),
            "John F. Kennedy International Airport".into(),
            "New York".into(),
            "NY".into(),
            "USA".into(),
        );

        assert_eq!(airport.get_iata_code(), "JFK");
        assert_eq!(airport.get_icao_code(), "KJFK");
        assert_eq!(airport.get_name(), "John F. Kennedy International Airport");
        assert_eq!(airport.get_city(), "New York");
        assert_eq!(airport.get_province(), "NY");
        assert_eq!(airport.get_country(), "USA");
    }

    #[test]
    fn test_get_code() {
        let airport = Airport::new(
            "JFK".into(),
            "KJFK".into(),
            "John F. Kennedy International Airport".into(),
            "New York".into(),
            "NY".into(),
            "USA".into(),
        );

        assert_eq!(airport.get_code(), "JFK/KJFK");
    }
}
