pub trait Unit {
    type Quantity;

    const TO_BASE: f64;

    const FROM_BASE: f64 = 1.0 / Self::TO_BASE;
}

#[macro_export]
macro_rules! unit {
    ($T:ident (to_base = $to_base:expr) for $Q:ty) => {
        pub struct $T;

        impl $crate::Unit for $T {
            type Quantity = $Q;

            const TO_BASE: f64 = $to_base;
        }
    };
}
