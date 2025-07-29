use caboose::LimitValues;
use derive_more::{Add, Display, Sub};
use ordered_float::{FloatCore, OrderedFloat, PrimitiveFloat};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Add, Sub, Copy, Clone, Debug, Display)]
pub struct LimitedValue<T>(pub OrderedFloat<T>)
where
    T: FloatCore + Copy + Debug,
    OrderedFloat<T>: Eq + PartialEq + Ord + PartialOrd
;

impl<T> LimitedValue<T> where T: FloatCore + Copy + Debug
{
    pub fn new(v: T) -> LimitedValue<T> {
        LimitedValue(OrderedFloat(v))
    }
}

impl<T: FloatCore + Copy + Debug> PartialEq<Self> for LimitedValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: FloatCore + Copy + Debug> PartialOrd for LimitedValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: FloatCore + Copy + Debug> Eq for LimitedValue<T> {}

impl<T: FloatCore + Copy + Debug> Ord for LimitedValue<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: PrimitiveFloat> LimitValues for LimitedValue<T>
where
    T: FloatCore + Debug + Copy + Clone + From<f64>
{
    fn min_value() -> Self {
        <T as From<f64>>::from(0f64).into()
    }

    fn max_value() -> Self {
        <T as From<f64>>::from(100000f64).into()
    }
}

impl<T: FloatCore + Debug> From<T> for LimitedValue<T> {
    fn from(value: T) -> Self {
        LimitedValue(OrderedFloat(value))
    }
}

impl<T: Default + FloatCore + Debug> Default for LimitedValue<T> {
    fn default() -> Self {
        LimitedValue(Default::default())
    }
}

impl<T: Default + FloatCore + Debug + Hash> Hash for LimitedValue<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}