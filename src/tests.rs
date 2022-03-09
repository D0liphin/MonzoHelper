use crate::types::time::Time;

fn display_result<T: std::fmt::Display, E: std::fmt::Display>(result: Result<T, E>) -> String {
    match result {
        Ok(t) => t.to_string(),
        Err(e) => e.to_string(),
    }
}

pub fn time_parsing() {
    let time_1 = Time::try_parse_str("t+10s");
    let time_2 = Time::try_parse_str("T - 10h");
    let time_3 = Time::try_parse_str("t5");
    let time_4 = Time::try_parse_str("T");
    let time_5 = Time::try_parse_str("t-90d");
    println!("time_1: {}", display_result(time_1));
    println!("time_2: {}", display_result(time_2));
    println!("time_3: {}", display_result(time_3));
    println!("time_4: {}", display_result(time_4));
    println!("time_5: {}", display_result(time_5));
}
