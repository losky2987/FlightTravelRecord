use std::collections::HashMap;

use crate::domain::flight::Flight;

pub struct Ticket {
    etkt: String,
    pnr: String,
    flights: Vec<Flight>,
    price: HashMap<String, f64>, // string is currency code, f64 is amount
    ota: String,// online travel agency
    checked_in_luggage: HashMap<String, Vec<u32>>, // 0: 1st flgiht...; 0: amount limit, 1: max. allowed limit, 2: actual limit 1,...
    cabin_level: Vec<HashMap<String, String>>, // e.g. "Economy","W", 0: 1st flight,...
    identification: String, // e.g. passport number
    seat_number: Vec<String>, // 0: 1st flight...
}
