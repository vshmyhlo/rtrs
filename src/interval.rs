pub struct Interval {
    pub min: f32,
    pub max: f32,
}
impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }
}
