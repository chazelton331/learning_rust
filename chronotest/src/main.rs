extern crate chrono;

use chrono::*;

fn add_time(datetime: DateTime<UTC>) -> DateTime<UTC> {
    datetime + Duration::seconds(60*3)
}

fn main() {
    let datetime = UTC.ymd(2014, 11, 05).and_hms(8,9,0);

    println!("{}", datetime.year());
    println!("{}", datetime.month());
    println!("{}", datetime.day());
    println!("{}", datetime.hour());
    println!("{}", datetime.minute());

    println!("{}", add_time(datetime).minute());
}
