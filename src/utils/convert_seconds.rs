pub fn convert_seconds(vec_seconds: Vec<f64>) -> Vec<String> {
    vec_seconds.iter()
        .map(|&s| {
            let total_minutes = (s / 60.0).round() as i64;
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;

            if hours == 0 {
                format!("{}m", minutes)
            } else if minutes == 0 {
                format!("{}h", hours)
            } else {
                format!("{}h{}m", hours, minutes)
            }
        })
        .collect()
}


pub fn convert_seconds_for_prg(duration: f64, current_time: f64) -> String {
            let time_left_s = duration - current_time;
            let total_minutes = (time_left_s / 60.0).round() as i64;
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;

            if current_time == 0.0 {
                format!("")
            }
            else if hours == 0 {
                format!("{}m left,", minutes)
            } else if minutes == 0 {
                format!("{}h left,", hours)
            } else {
                format!("{}h{}m left,", hours, minutes)
            }
        }

