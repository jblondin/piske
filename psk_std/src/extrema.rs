use std::ops::Sub;

use image::ImageData;

pub trait MaxValue {
    fn max_value() -> Self;
}
pub trait MinValue {
    fn min_value() -> Self;
}

macro_rules! impl_extrema {
    ($($impl_ty:ty),*) => {
        $(
        impl MinValue for $impl_ty {
            fn min_value() -> Self {
                Self::min_value()
            }
        }
        impl MaxValue for $impl_ty {
            fn max_value() -> Self {
                Self::max_value()
            }
        }
        )*
    }
}
macro_rules! impl_module_extrema {
    ($($impl_ty:tt),*) => {
        $(
        impl MinValue for $impl_ty {
            fn min_value() -> Self {
                ::std::$impl_ty::MIN
            }
        }
        impl MaxValue for $impl_ty {
            fn max_value() -> Self {
                ::std::$impl_ty::MAX
            }
        }
        )*
    }
}

impl_extrema!(isize, i8, i16, i32, i64, usize, u8, u16, u32, u64);
impl_module_extrema!(f32, f64);

pub trait Extrema {
    type Value: MaxValue + MinValue;

    fn extrema(&self) -> ExtremeValues<Self::Value>;
}
pub struct ExtremeValues<T: MaxValue + MinValue> {
    pub max: T,
    pub min: T
}
impl<T: MaxValue + MinValue> Default for ExtremeValues<T> {
    fn default() -> ExtremeValues<T> {
        ExtremeValues {
            max: T::min_value(),
            min: T::max_value(),
        }
    }
}
impl<T: Copy + MaxValue + MinValue + Sub<Output=T>> ExtremeValues<T> {
    pub fn range(&self) -> T {
        self.max - self.min
    }
}

impl<T: Copy + PartialOrd + MinValue + MaxValue> Extrema for ImageData<T> {
    type Value = T;

    fn extrema(&self) -> ExtremeValues<T> {
        self.values.iter().fold(ExtremeValues::default(), |mut acc, &ref value| {
            if value < &acc.min {
                acc.min = *value;
            }
            if value > &acc.max {
                acc.max = *value;
            }
            acc
        })
    }
}
