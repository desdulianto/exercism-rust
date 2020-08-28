// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

macro_rules! orbital_periods (
    { $($t:ty => $x:expr),* } => {
        $(
            impl Planet for $t {
                fn orbital_period() -> f64 {
                    $x
                }
            }
        )*
    };
);

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

    fn orbital_period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (Self::orbital_period() * Self::EARTH_YEAR_SECONDS)
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

orbital_periods!(
    Mercury => 0.2408467,
    Venus   => 0.61519726,
    Earth   => 1.0,
    Mars    => 1.8808158,
    Jupiter => 11.862615,
    Saturn  => 29.447498,
    Uranus  => 84.016846,
    Neptune => 164.79132
);
