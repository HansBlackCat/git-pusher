
use chrono::prelude::*;

pub fn chronoTime() ->String {
    // Get User's local time
    let local: String = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{:?}", &local);
    local
}