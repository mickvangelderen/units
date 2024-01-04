use crate::{const_int::ConstSub, ConstAdd, Unit};

use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

#[derive(Debug)]
pub struct Quantity<D0, D1> {
    value: f64,
    _d0: PhantomData<D0>,
    _d1: PhantomData<D1>,
}

impl<D0, D1> Clone for Quantity<D0, D1> {
    fn clone(&self) -> Self { *self }
}

impl<D0, D1> Copy for Quantity<D0, D1> {}

impl<D0, D1> Quantity<D0, D1> {
    pub fn new<U: Unit<Quantity = Self>>(value: f64) -> Self {
        Self::new_in_base(value * U::TO_BASE)
    }

    /// Prefer to use `new` for construction when you can specify an appropriate `U: Unit`.
    pub(crate) fn new_in_base(value: f64) -> Self {
        Self {
            value,
            _d0: PhantomData,
            _d1: PhantomData,
        }
    }

    pub fn get<U: Unit<Quantity = Self>>(&self) -> f64 {
        self.value * U::FROM_BASE
    }
}

impl<D0, D1> std::cmp::PartialEq for Quantity<D0, D1> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<D0, D1> std::cmp::PartialOrd for Quantity<D0, D1> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<D0, D1> Add for Quantity<D0, D1> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new_in_base(self.value + rhs.value)
    }
}

impl<D0, D1> AddAssign for Quantity<D0, D1> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<D0, D1> Sub for Quantity<D0, D1> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Quantity::new_in_base(self.value - rhs.value)
    }
}

impl<D0, D1> SubAssign for Quantity<D0, D1> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<L0, L1, R0, R1> Mul<Quantity<R0, R1>> for Quantity<L0, L1>
where
    L0: ConstAdd<R0>,
    L1: ConstAdd<R1>,
{
    type Output = Quantity<<L0 as ConstAdd<R0>>::Output, <L1 as ConstAdd<R1>>::Output>;

    fn mul(self, rhs: Quantity<R0, R1>) -> Self::Output {
        Quantity::new_in_base(self.value * rhs.value)
    }
}

impl<L0, L1, R0, R1> Div<Quantity<R0, R1>> for Quantity<L0, L1>
where
    L0: ConstSub<R0>,
    L1: ConstSub<R1>,
{
    type Output = Quantity<<L0 as ConstSub<R0>>::Output, <L1 as ConstSub<R1>>::Output>;

    fn div(self, rhs: Quantity<R0, R1>) -> Self::Output {
        Quantity::new_in_base(self.value / rhs.value)
    }
}
