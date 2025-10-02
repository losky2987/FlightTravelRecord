use crate::domain::{aircraft::Aircraft, airline::Airline, airport::Airport, time_point::TimePoint};

pub struct Flight {
    flight_number: Vec<FlightNumber>, // 0: operator flight number, other: codeshare flight numbers
    city: Vec<Airport>, // 0: takeoff, last: landing, other: layovers
    time: Vec<FlightTime>, // 0: takeoff, last: landing, other: layovers
    distance: i32, // unit: km
    aircraft: Aircraft,
    airline: Airline
}


pub struct FlightTime {
    pub plan: TimePoint,
    pub actual: TimePoint
}

pub struct FlightNumber {
    airline_code: String,
    flight_code: String
}
