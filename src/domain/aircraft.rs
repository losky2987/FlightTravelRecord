#[derive(Debug, Clone)]
pub struct Aircraft {
    reg_id: String,
    model: String,
    sub_model: String,
    manufacturer: String,
    operator: String
}

impl Aircraft {
    pub fn new(reg_id: String, model: String, sub_model: String, manufacturer: String, operator: String) -> Self {
        return Aircraft { reg_id, model, sub_model, manufacturer, operator };
    }

    pub fn get_reg_id(&self) -> &String {
        return &self.reg_id;
    }

    pub fn get_model(&self) -> &String {
        return &self.model;
    }

    pub fn get_sub_model(&self) -> &String {
        return &self.sub_model;
    }

    pub fn get_manufacturer(&self) -> &String {
        return &self.manufacturer;
    }

    pub fn get_operator(&self) -> &String {
        return &self.operator;
    }

    pub fn get_full_model(&self) -> String {
        return format!("{}-{}", self.model, self.sub_model);
    }
}

#[cfg(test)]
mod aircraft_tests {
    use super::*;

    #[test]
    fn test_if_aircraft_new_success() {
        let reg_id = String::from("B2624");
        let model = String::from("A330");
        let sub_model = String::from("900ER");
        let manufacturer = String::from("Airbus Tianjin");
        let operator = String::from("Hainan Airlines");

        let aircraft = Aircraft::new(reg_id, model, sub_model, manufacturer, operator);

        assert_eq!(aircraft.get_reg_id(), "B2624");
        assert_eq!(aircraft.get_model(), "A330");
        assert_eq!(aircraft.get_sub_model(), "900ER");
        assert_eq!(aircraft.get_manufacturer(), "Airbus Tianjin");
        assert_eq!(aircraft.get_operator(), "Hainan Airlines");
        assert_eq!(aircraft.get_full_model(), "A330-900ER");
    }

    #[test]
    fn test_get_full_model() {
        let reg_id = String::from("B2624");
        let model = String::from("A330");
        let sub_model = String::from("900ER");
        let manufacturer = String::from("Airbus Tianjin");
        let operator = String::from("Hainan Airlines");

        let aircraft = Aircraft::new(reg_id, model, sub_model, manufacturer, operator);

        assert_eq!(aircraft.get_full_model(), "A330-900ER");
    }
}

