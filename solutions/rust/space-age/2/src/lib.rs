// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        //todo!("s, measured in seconds: {s}")
        Duration {
            seconds: s,
        }
    }
}

const Earth_Year: f64 = 31_557_600.0;

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        *seconds as f64
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        let duration:f64 = *seconds as f64;
        let mercury_year: f64 = Earth_Year * 0.2408467_f64;
        duration / mercury_year
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        let duration:f64 = *seconds as f64;
        let venus_year: f64 = Earth_Year * 0.61519726_f64;
        duration / venus_year
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        let duration:f64 = *seconds as f64;
        duration / Earth_Year
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        let duration:f64 = *seconds as f64;
        let mars_year: f64 = Earth_Year * 1.8808158_f64;
        duration / mars_year
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        let duration:f64 = *seconds as f64;
        let jupiter_year: f64 = Earth_Year * 11.862615_f64;
        duration / jupiter_year
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        let duration:f64 = *seconds as f64;
        let saturn_year: f64 = Earth_Year * 29.447498_f64;
        duration / saturn_year
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        let duration:f64 = *seconds as f64;
        let uranus_year: f64 = Earth_Year * 84.016846_f64;
        duration / uranus_year
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let Duration { seconds } = d;
        let duration:f64 = *seconds as f64;
        let neptune_year: f64 = Earth_Year * 164.79132_f64;
        duration / neptune_year
    }
}
