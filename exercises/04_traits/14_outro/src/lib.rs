// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use std::ops::Add;
#[derive(Debug, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}
impl From<u16> for SaturatingU16 {
    fn from(n: u16) -> SaturatingU16 {
        SaturatingU16 { value: n }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(n: &u16) -> SaturatingU16 {
        SaturatingU16 { value: *n }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(n: u8) -> SaturatingU16 {
        SaturatingU16 { value: n as u16 }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(n: &u8) -> SaturatingU16 {
        SaturatingU16 { value: *n as u16 }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<u16> for SaturatingU16 {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(other))
    }
}
impl PartialEq<SaturatingU16> for SaturatingU16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        self.value.eq(&other.value)
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = u16;
    fn add(self, rhs: u16) -> Self::Output {
        self.value.saturating_add(rhs)
    }
}
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}
