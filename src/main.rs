use regex::Regex;
use std::io;
use std::str::FromStr;

fn main() {
    println!("What time do you want to know the angle for?");
    // get input for the time
    let mut time_value = String::new();
    let _ = io::stdin().read_line(&mut time_value);
    // print the angle

    let re = Regex::new(r"^(\d{1,2})\s*(\d{1,2})\s*(\d{1,2})").unwrap();
    if let Some(caps) = re.captures(&time_value) {
        let hours = u16::from_str(&caps.get(1).unwrap().as_str()).unwrap();
        let minutes = u16::from_str(&caps.get(2).unwrap().as_str()).unwrap();
        let seconds = u16::from_str(&caps.get(3).unwrap().as_str()).unwrap();
        let m_angle = minute_hand_angle(minutes, seconds);
        let h_angle = hour_hand_angle(hours, minutes, seconds);
        println!("The minute hand angle is {}", m_angle);
        println!("The hour hand angle is {}", h_angle);
        let angle_between = (m_angle - h_angle).abs() % 360.0;
        if angle_between > 180.0 {
            println!(
                "The angle between the hands is {}",
                180.0 - (angle_between % 180.0)
            );
        } else {
            println!("The angle between the hands is {}", angle_between);
        }
    } else {
        println!("Invalid input.")
    }
}

fn minute_hand_angle(minutes: u16, seconds: u16) -> f32 {
    let total_seconds = minutes * 60 + seconds;
    return total_seconds as f32 * 0.1;
}

fn hour_hand_angle(hours: u16, minutes: u16, seconds: u16) -> f32 {
    let total_seconds = hours * 3600 + minutes * 60 + seconds;
    let percentage: f32 = total_seconds as f32 / 43200f32;

    return percentage * 360f32;
}
