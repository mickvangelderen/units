//! Implements types that can be used to count.

/// The number zero.
#[derive(Debug)]
pub struct Z0;

/// Represents the numeric value of T minus one.
#[derive(Debug)]
pub struct N<T>(T);

/// Represents the numeric value of T plus one.
#[derive(Debug)]
pub struct P<T>(T);

/// Teaches the compiler how to add Z0, N<T> and P<T>.
pub trait ConstAdd<R> {
    type Output;
}

// Impl Z0.

impl ConstAdd<Z0> for Z0 {
    type Output = Z0;
}

impl<R> ConstAdd<P<R>> for Z0 {
    type Output = P<R>;
}

impl<R> ConstAdd<N<R>> for Z0 {
    type Output = N<R>;
}

// Impl N<T>.

impl<L> ConstAdd<Z0> for N<L> {
    type Output = Self;
}

impl<L, R> ConstAdd<P<R>> for N<L>
where
    L: ConstAdd<R>,
{
    type Output = <L as ConstAdd<R>>::Output;
}

impl<L, R> ConstAdd<N<R>> for N<L>
where
    L: ConstAdd<R>,
{
    type Output = N<N<<L as ConstAdd<R>>::Output>>;
}

// Impl P<T>.

impl<L> ConstAdd<Z0> for P<L> {
    type Output = Self;
}

impl<L, R> ConstAdd<P<R>> for P<L>
where
    L: ConstAdd<R>,
{
    type Output = P<P<<L as ConstAdd<R>>::Output>>;
}

impl<L, R> ConstAdd<N<R>> for P<L>
where
    L: ConstAdd<R>,
{
    type Output = <L as ConstAdd<R>>::Output;
}

pub trait ConstSub<R> {
    type Output;
}

// Impl Z0.

impl ConstSub<Z0> for Z0 {
    type Output = Z0;
}

impl<R> ConstSub<P<R>> for Z0
where Z0: ConstSub<R> {
    type Output = N<<Z0 as ConstSub<R>>::Output>;
}

impl<R> ConstSub<N<R>> for Z0
where Z0: ConstSub<R> {
    type Output = P<<Z0 as ConstSub<R>>::Output>;
}

// Impl N<T>.

impl<L> ConstSub<Z0> for N<L> {
    type Output = Self;
}

impl<L, R> ConstSub<P<R>> for N<L>
where
    L: ConstSub<R>,
{
    type Output = N<N<<L as ConstSub<R>>::Output>>;
}

impl<L, R> ConstSub<N<R>> for N<L>
where
    L: ConstSub<R>,
{
    type Output = <L as ConstSub<R>>::Output;
}

// Impl P<T>.

impl<L> ConstSub<Z0> for P<L> {
    type Output = Self;
}

impl<L, R> ConstSub<P<R>> for P<L>
where
    L: ConstSub<R>,
{
    type Output = <L as ConstSub<R>>::Output;
}

impl<L, R> ConstSub<N<R>> for P<L>
where
    L: ConstSub<R>,
{
    type Output = N<N<<L as ConstSub<R>>::Output>>;
}



pub type N1 = N<Z0>;
// pub type N2 = N<N1>;
// pub type N3 = N<N2>;
// pub type N4 = N<N3>;

pub type P1 = P<Z0>;
// pub type P2 = P<P1>;
// pub type P3 = P<P2>;
// pub type P4 = P<P3>;
