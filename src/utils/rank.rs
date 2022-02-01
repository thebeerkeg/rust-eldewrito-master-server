pub fn calc_rank(experience: u32, max_rank: u8) -> u8 {
    (((experience as f64).sqrt() / 4.0).floor() as u32).clamp(0, max_rank as u32) as u8
}
