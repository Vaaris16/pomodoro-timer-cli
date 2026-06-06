pub fn get_time(total_time: usize) -> String {
    let secs = total_time % 60;
    let min = total_time / 60;

    format!("{:02} : {:02}", min, secs)
}
