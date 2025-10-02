use crate::domain::time_point::TimePoint;

#[derive(Debug, Clone)]
pub struct Aircraft {
    operator: String,
    nationality_code: String,
    reg_id: String,
    manufacturer: String,
    model: String,
    sub_model: String,
    in_service_date: TimePoint,
    cabin_config: String // FirstClass: F, BusinessClass: C, EconomyClass: Y, e.g.: F6C12Y263
}

impl Aircraft {
    pub fn new(operator: String, nationality_code: String, reg_id: String, manufacturer: String, model: String, sub_model: String, in_service_date: TimePoint, cabin_config: String) -> Self {
        return Aircraft { operator: operator, nationality_code: nationality_code, reg_id: reg_id, manufacturer: manufacturer, model: model, sub_model: sub_model, in_service_date: in_service_date, cabin_config: cabin_config };
    }

    pub fn get_reg_code(&self) -> String {
        return format!("{}{}", self.nationality_code, self.reg_id);
    }

    pub fn get_model(&self) -> String {
        return format!("{}-{}", self.model, self.sub_model);
    }

    pub fn get_manufacturer(&self) -> String {
        return self.manufacturer.to_string();
    }

    pub fn get_operator(&self) -> String {
        return self.operator.to_string();
    }

    pub fn get_age(&self) -> f64 {
        let mut now = TimePoint::new(0, 0, 0, 0, 0, 0, 0);
        now.get_now();
        let mut month_diff = now.month - self.in_service_date.month;
        let mut year_diff = now.year - self.in_service_date.year;
        if month_diff < 0 {
            year_diff -= 1;
            month_diff += 12;
        }
        return year_diff as f64 + (month_diff as f64 / 12.0 * 10.0).round() / 10.0;
    }

    pub fn to_string(&self) -> String {
        return format!("{} {} {}: Registed with {}, serveced {} {}, cabin configured as {}.",
         self.operator, self.manufacturer, self.get_model(), self.get_reg_code(), self.get_age(), if self.get_age() <= 1.0 {"year"} else {"years"}, self.cabin_config);
    }
}

#[cfg(test)]
mod aircraft_tests {
    use super::*;

    fn get_test_use() -> Aircraft {
        return Aircraft::new(
            "Air China".to_string(),
            "B".to_string(),
            "2627".to_string(),
            "Boeing".to_string(),
            "747".to_string(),
            "400".to_string(),
            TimePoint::new(2010, 9, 30, 0, 0, 0, 8),
            "F6C12Y128".to_string()
        );
    }

    #[test]
    fn test_get_reg_code() {
        assert_eq!(get_test_use().get_reg_code(), "B2627");
    }

    #[test]
    fn test_get_model() {
        assert_eq!(get_test_use().get_model(), "747-400");
    }

    #[test]
    fn test_get_manufacturer() {
        assert_eq!(get_test_use().get_manufacturer(), "Boeing");
    }

    #[test]
    fn test_get_operator() {
        assert_eq!(get_test_use().get_operator(), "Air China");
    }

    #[test]
    fn test_get_age() {
        assert_eq!(get_test_use().get_age(), 15.1);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(get_test_use().to_string(), "Air China Boeing 747-400: Registed with B2627, serveced 15.1 years, cabin configured as F6C12Y128.");
    }
}

