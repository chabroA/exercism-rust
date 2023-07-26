use duplicate::duplicate_item;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

#[duplicate_item(
    planet          orbital_period;
    [ Mercury ]     [ 0.240_846_7 ];
    [ Venus ]       [ 0.615_197_26 ];
    [ Earth ]       [ 1.0 ];
    [ Mars ]        [ 1.880_815_8 ];
    [ Jupiter ]     [ 11.862_615 ];
    [ Saturn ]      [ 29.447_498 ];
    [ Uranus ]      [ 84.016_846 ];
    [ Neptune ]     [ 164.791_32 ];
  )]
impl Planet for planet {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / orbital_period / 31_557_600.0
    }
}
