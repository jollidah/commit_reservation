use chrono::{prelude::Local, TimeZone, Datelike, DateTime, Duration};
// use std::time;

pub struct TimeHandler{
    startingTime: DateTime<Local>,
    checkingTime: DateTime<Local>,
    inteval: std::time::Duration
}

impl TimeHandler{
    pub fn new() -> Self{
        let now = Local::now();
        let y = now.year();
        let m = now.month();
        let d = now.day();
        let target = Local.with_ymd_and_hms(y, m, d, 18, 02, 0).unwrap();
        let time_sub = target.signed_duration_since(now).to_std().unwrap();
        TimeHandler{
            startingTime: now,
            checkingTime: target,
            inteval: time_sub
        }
    }

    pub fn get_interval(self) -> std::time::Duration{
        self.inteval
    }

    pub fn get_checking_time(self) -> DateTime<Local>{
        self.checkingTime
    }
}