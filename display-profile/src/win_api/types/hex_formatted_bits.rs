use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use anyhow::Result;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HexFormattedBits<T>(pub T);
impl<T> From<T> for HexFormattedBits<T> {
    fn from(value: T) -> Self {
        HexFormattedBits(value)
    }
}
impl<T: U32Adapter> Debug for HexFormattedBits<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#X}", self.0.inner())
    }
}
impl<T: U32Adapter> Serialize for HexFormattedBits<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:#?}"))
    }
}
impl<'de, T: U32Adapter> Deserialize<'de> for HexFormattedBits<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Vis<T>(PhantomData<T>);

        impl<T: U32Adapter> Visitor<'_> for Vis<T> {
            type Value = HexFormattedBits<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("An hex-formatted number as string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let s = s.strip_prefix("0x").unwrap_or(s);
                let value = u32::from_str_radix(s, 16)
                    .map_err(|err| E::custom(format!("Could not parse {s:?}: {err}")))?;

                Ok(HexFormattedBits(T::new(value)))
            }
        }

        deserializer.deserialize_str(Vis(PhantomData))
    }
}
impl<T: U32Adapter> Hash for HexFormattedBits<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.inner().hash(state);
    }
}

pub trait U32Adapter: Copy {
    fn new(inner: u32) -> Self;
    fn inner(self) -> u32;
}
impl U32Adapter for u32 {
    fn new(inner: u32) -> Self {
        inner
    }

    fn inner(self) -> u32 {
        self
    }
}
macro_rules! impl_hex_formatter_for_newtype {
    ($name:ty, $inner:ty) => {
        impl $crate::win_api::types::hex_formatted_bits::U32Adapter for $name {
            fn new(inner: u32) -> Self {
                Self(inner)
            }

            fn inner(self) -> u32 {
                self.0
            }
        }
    };
}
pub(crate) use impl_hex_formatter_for_newtype;
