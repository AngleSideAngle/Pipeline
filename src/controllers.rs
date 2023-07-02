use super::stream::Stream;

pub struct PIDConstants {
    k_p: f64,
    k_i: f64,
    k_d: f64,
}

impl PIDConstants {
    pub fn pid(&self, error: Stream) -> Stream {
        let p = self.k_p * error.clone();
        let i = self.k_i * error.clone().integrate(None);
        let d = self.k_d * error.clone().differentiate(None);
        p + i + d
    }
}
