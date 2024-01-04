mod const_int;
use const_int::*;

mod quantity;
pub use quantity::*;

mod unit;
pub use unit::*;

pub type Length = Quantity<P1, Z0>;
pub type Time = Quantity<Z0, P1>;
pub type Velocity = Quantity<P1, N1>;
pub type Ratio = Quantity<Z0, Z0>;
pub type Angle = Ratio; // NOTE: I guess it depends on the context of the application whether this is desirable or a potential disaster.

unit!(Second (to_base = 1.0) for Time);
unit!(Hour (to_base = 3600.0) for Time);
unit!(Meter (to_base = 1.0) for Length);
unit!(Kilometer (to_base = 1000.0) for Length);
unit!(MeterPerSecond (to_base = 1.0) for Velocity);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x: Length = Length::new::<Kilometer>(45.0);
        let t: Time = Time::new::<Hour>(0.5);
        let v: Velocity = Velocity::new::<MeterPerSecond>(25.0);

        assert_eq!(x + x, Length::new::<Meter>(90000.0));
        assert_eq!(t - Time::new::<Second>(3600.0), Time::new::<Hour>(-0.5));
        assert_eq!(v, x / t);
        assert_eq!(x, v * t);
        assert_eq!(t, x / v);
    }
}
