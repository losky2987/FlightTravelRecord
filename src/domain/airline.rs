#[derive(Debug, Clone)]
pub struct Airline {
    code: String, // 2-digi code
    identifier: String, // 3-digi code
    call_sign: String,
    name: String,
    country: String,
    alliance: String, // airlines alliance, e.g. Oneworld, Star Alliance, SkyTeam
    frequent_flyer_program: String,
    etkt_prefix: String
}

impl Airline {
    pub fn new(code: String, identifier: String, call_sign: String, name: String, country: String, alliance: String, frequent_flyer_program: String, etkt_prefix: String) -> Self {
        return Airline { code, identifier, call_sign, name, country, alliance, frequent_flyer_program, etkt_prefix };
    }

    pub fn get_code(&self) -> &str {
        return &self.code;
    }

    pub fn get_identifier(&self) -> &str {
        return &self.identifier;
    }

    pub fn get_call_sign(&self) -> &str {
        return &self.call_sign;
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_country(&self) -> &str {
        return &self.country;
    }

    pub fn get_alliance(&self) -> &str {
        return &self.alliance;
    }

    pub fn get_frequent_flyer_program(&self) -> &str {
        return &self.frequent_flyer_program;
    }

    pub fn get_etkt_prefix(&self) -> &str {
        return &self.etkt_prefix;
    }
}

#[cfg(test)]
mod test_airline {
    use super::*;

    #[test]
    fn test_airline_creation() {
        let airline = Airline::new(
            "AA".into(),
            "American Airlines".into(),
            "American".into(),
            "American Airlines".into(),
            "United States".into(),
            "Oneworld".into(),
            "AAdvantage".into(),
            "001".into(),
        );

        assert_eq!(airline.get_code(), "AA");
        assert_eq!(airline.get_identifier(), "American Airlines");
        assert_eq!(airline.get_call_sign(), "American");
        assert_eq!(airline.get_name(), "American Airlines");
        assert_eq!(airline.get_country(), "United States");
        assert_eq!(airline.get_alliance(), "Oneworld");
        assert_eq!(airline.get_frequent_flyer_program(), "AAdvantage");
        assert_eq!(airline.get_etkt_prefix(), "001");
    }
}
