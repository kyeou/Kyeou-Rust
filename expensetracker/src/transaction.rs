extern crate serde;

use serde::{Serialize, Deserialize};
use std::fmt;




#[derive(Serialize, Deserialize, Debug)]
pub struct Date {pub _month: String, pub _day: i32, pub _year: i32}
impl Date {
    fn new(_month: String, _day: i32, _year: i32) -> Date {
        Date { _month, _day, _year }
    }
}
 impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self._month, self._day, self._year)
    }
 }

 #[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TRANSACTION {pub _date: Date, pub _amount: f32, pub _desc: String}
impl TRANSACTION {
    pub fn new() -> TRANSACTION {
      TRANSACTION { _date: Date::new("Jan".to_string(), 1, 2000), _amount: 12.34, _desc: "This is a test".to_string() }
    }
}

impl fmt::Display for TRANSACTION {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Summary {} \n Date {:?} \n Amount {}", self._desc, self._date, self._amount)
    }
 }

