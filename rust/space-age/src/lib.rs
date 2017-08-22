pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planetary_space {
    { $($planet:ident => $convert:expr),* } => { $(
       pub struct $planet {}

       impl Planet for $planet {
           fn years_during(d: &Duration) -> f64 {
               (d.0 as f64) / ($convert)
           }
       }
    )* }
}

planetary_space!{
    Mercury =>     7_600_543.8199_f64,
    Venus   =>    19_414_149.0522_f64,
    Earth   =>    31_557_600.0_f64,
    Mars    =>    59_354_032.6901_f64,
    Jupiter =>   374_355_659.124_f64,
    Saturn  =>   929_292_362.8848_f64,
    Uranus  => 2_651_370_019.3296_f64,
    Neptune => 5_200_418_560.032_f64
}
