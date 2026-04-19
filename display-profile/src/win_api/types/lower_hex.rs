use std::fmt::{Debug, LowerHex};
use std::hash::Hash;

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LowerHexFormattedBits<T>(pub T);
impl<T> From<T> for LowerHexFormattedBits<T> {
    fn from(value: T) -> Self {
        LowerHexFormattedBits(value)
    }
}
impl<T: ToLowerHex> Debug for LowerHexFormattedBits<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0.to_lower_hex())
    }
}
impl<T: ToLowerHex> Serialize for LowerHexFormattedBits<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:#?}"))
    }
}
impl<T: ToLowerHex> Hash for LowerHexFormattedBits<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_lower_hex().hash(state);
    }
}

pub trait ToLowerHex: Copy {
    fn to_lower_hex(self) -> impl LowerHex + Hash;
}
impl ToLowerHex for u32 {
    fn to_lower_hex(self) -> impl LowerHex + Hash {
        self
    }
}
macro_rules! impl_to_lower_hex_for_newtype {
    ($name:ty) => {
        impl $crate::win_api::types::lower_hex::ToLowerHex for $name {
            fn to_lower_hex(self) -> impl std::fmt::LowerHex + std::hash::Hash {
                self.0
            }
        }
    };
}
pub(crate) use impl_to_lower_hex_for_newtype;
