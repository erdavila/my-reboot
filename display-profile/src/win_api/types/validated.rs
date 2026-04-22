use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Validated<T, const INV: u32> {
    Valid(T),
    Invalid,
}
impl<T: U32Convertible, const INV: u32> Validated<T, INV> {
    pub fn to_value(self) -> T {
        match self {
            Validated::Valid(value) => value,
            Validated::Invalid => T::from_u32(INV),
        }
    }
}
impl<T: U32Convertible, const INV: u32> From<T> for Validated<T, INV> {
    fn from(value: T) -> Self {
        if value.to_u32() == INV {
            Validated::Invalid
        } else {
            Validated::Valid(value)
        }
    }
}
// impl<T, const INV: u32> From<Validated<T, INV>> for T
// where T: U32Convertible
// {
//     fn from(value: Validated<T, INV>) -> Self {
//         match value {
//             Validated::Valid(value) => value,
//             Validated::Invalid => T::from_u32(INV),
//         }
//     }
// }
impl<T: U32Convertible, const INV: u32> From<Option<usize>> for Validated<T, INV> {
    fn from(value: Option<usize>) -> Self {
        match value {
            #[allow(clippy::cast_possible_truncation)]
            Some(value) => Validated::Valid(T::from_u32(value as u32)),
            None => Validated::Invalid,
        }
    }
}
impl<T, const INV: u32> From<Validated<T, INV>> for Option<usize>
where
    T: U32Convertible,
{
    fn from(value: Validated<T, INV>) -> Self {
        match value {
            Validated::Valid(value) => Some(value.to_u32() as usize),
            Validated::Invalid => None,
        }
    }
}

pub trait U32Convertible: Copy {
    fn from_u32(value: u32) -> Self;
    fn to_u32(self) -> u32;
}
impl U32Convertible for u32 {
    fn from_u32(value: u32) -> Self {
        value
    }

    fn to_u32(self) -> u32 {
        self
    }
}
impl U32Convertible for u16 {
    #[expect(clippy::cast_possible_truncation)]
    fn from_u32(value: u32) -> Self {
        value as u16
    }

    fn to_u32(self) -> u32 {
        self.into()
    }
}
