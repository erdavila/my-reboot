use std::fmt::Debug;
use std::ops::{BitAnd, BitOr, BitOrAssign, Not};

use serde::Serialize;

pub trait WinFlags:
    BitAnd<Output = Self> + BitOr<Output = Self> + BitOrAssign + Not<Output = Self> + Copy + PartialEq
{
    fn test(self, mask: Self) -> bool {
        self & mask == mask
    }

    fn test_any(self, mask: Self) -> bool {
        self & mask != Self::zero()
    }

    fn zero() -> Self;
}
macro_rules! impl_win_flags_for {
    ($type:ty) => {
        impl $crate::win_api::types::flags::WinFlags for $type {
            fn zero() -> Self {
                Self::default()
            }
        }

        impl<T: $crate::win_api::types::flags::Flag<WinFlags = $type>> From<$type>
            for $crate::win_api::types::flags::Flags<T>
        {
            fn from(value: $type) -> Self {
                $crate::win_api::types::flags::Flags(value)
            }
        }

        impl<T: $crate::win_api::types::flags::Flag<WinFlags = $type>>
            From<$crate::win_api::types::flags::Flags<T>> for $type
        {
            fn from(value: $crate::win_api::types::flags::Flags<T>) -> Self {
                value.0
            }
        }
    };
}
pub(crate) use impl_win_flags_for;
impl_win_flags_for!(u32);

pub trait Flag: Debug + Copy + Eq + Serialize + 'static {
    type WinFlags: WinFlags + From<Self> + Into<Self>;

    fn all() -> &'static [(Self, Self::WinFlags)];
    fn unknown_bits(bits: Self::WinFlags) -> Self;
    fn unknown_bits_from(value: Self) -> Option<Self::WinFlags>;

    fn values_from(bits: Self::WinFlags) -> impl Iterator<Item = Self> {
        let all_values_mask = Self::all()
            .iter()
            .fold(Self::WinFlags::zero(), |bits, (_value, mask)| bits | *mask);
        let unknown_bits = bits & !all_values_mask;

        Self::all()
            .iter()
            .map(move |(value, mask)| bits.test(*mask).then_some(*value))
            .chain(std::iter::once(
                (unknown_bits != Self::WinFlags::zero()).then(|| Self::unknown_bits(unknown_bits)),
            ))
            .flatten()
    }
}

macro_rules! define_flag_type {
    (
        $name:ident : $win_flags:ty {
            $win_path:ident;
            $(
                $win_variant:ident => $variant:ident ,
            )*
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
        pub enum $name {
            $(
                $variant,
            )*
            Unknown($crate::win_api::types::lower_hex::LowerHexFormattedBits<$win_flags>),
        }

        impl $crate::win_api::types::flags::Flag for $name {
            type WinFlags = $win_flags;

            fn all() -> &'static [(Self, Self::WinFlags)] {
                &[
                    $(
                        ($name::$variant, $win_path::$win_variant),
                    )*
                ]
            }

            fn unknown_bits(bits: Self::WinFlags) -> Self {
                Self::Unknown($crate::win_api::types::lower_hex::LowerHexFormattedBits(bits))
            }

            fn unknown_bits_from(value: Self) -> Option<Self::WinFlags> {
                if let Self::Unknown(bits) = value {
                    Some(bits.0)
                } else {
                    None
                }
            }
        }

        impl From<$win_flags> for $name {
            fn from(value: $win_flags) -> Self {
                match value {
                    $(
                        $win_path::$win_variant => $name::$variant,
                    )*
                    _ => $name::Unknown(value.into()),
                }
            }
        }

        impl From<$name> for $win_flags {
            fn from(value: $name) -> Self {
                match value {
                    $(
                        $name::$variant => $win_path::$win_variant,
                    )*
                    $name::Unknown(bits) => bits.0,
                }
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Flags<Self>;

            fn bitor(self, rhs: Self) -> Self::Output {
                let mut output = self.into();
                output |= rhs;
                output
            }
        }
    };
}
pub(crate) use define_flag_type;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Flags<T: Flag>(pub T::WinFlags);
impl<T: Flag> Flags<T> {
    pub fn contains(self, flag: T) -> bool {
        for (value, mask) in T::all() {
            if flag == *value && self.0.test(*mask) {
                return true;
            }
        }

        if let Some(unknown_bits) = T::unknown_bits_from(flag) {
            return self.0.test_any(unknown_bits);
        }

        false
    }
}
impl<T: Flag> From<T> for Flags<T> {
    fn from(value: T) -> Self {
        let value = value.into();
        Flags(value)
    }
}
impl<T: Flag> BitOrAssign for Flags<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl<T: Flag> BitOrAssign<T> for Flags<T> {
    fn bitor_assign(&mut self, rhs: T) {
        self.0 |= rhs.into();
    }
}
impl<T: Flag> Debug for Flags<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(T::values_from(self.0)).finish()
    }
}
impl<T: Flag> Serialize for Flags<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(T::values_from(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod u32 {
        use super::*;

        const MASK1: u32 = 0x0001;
        const MASK2: u32 = 0x0002;
        const MASK4: u32 = 0x0004;

        define_flag_type!(
            U32Flag : u32 {
                self;
                MASK1 => VALUE1,
                MASK2 => VALUE2,
                MASK4 => VALUE4,
            }
        );

        #[test]
        fn contains() {
            let bits = MASK1 | MASK4;

            let flags: Flags<U32Flag> = bits.into();
            assert!(flags.contains(U32Flag::VALUE1));
            assert!(!flags.contains(U32Flag::VALUE2));
            assert!(flags.contains(U32Flag::VALUE4));

            let bits2: u32 = flags.into();
            assert_eq!(bits2, bits);
        }

        #[test]
        fn unknown() {
            const UNKNOWN_BITS: u32 = 0b0101_0000;
            let bits = MASK2 | UNKNOWN_BITS;

            let flags: Flags<U32Flag> = bits.into();
            assert!(!flags.contains(U32Flag::VALUE1));
            assert!(flags.contains(U32Flag::VALUE2));
            assert!(!flags.contains(U32Flag::VALUE4));
            assert!(flags.contains(U32Flag::Unknown(UNKNOWN_BITS.into())));

            let bits2: u32 = flags.into();
            assert_eq!(bits2, bits);
        }

        #[test]
        fn debug() {
            let flags: Flags<U32Flag> = (MASK1 | MASK4).into();

            let formatted = format!("{flags:?}");

            assert_eq!(formatted, "[VALUE1, VALUE4]");
        }

        #[test]
        fn serialize_as_json() {
            let flags: Flags<U32Flag> = (MASK1 | MASK4).into();

            let serialized = serde_json::to_string(&flags).unwrap();

            assert_eq!(serialized, r#"["VALUE1","VALUE4"]"#);
        }
    }
    mod typed {
        use super::*;
        use crate::win_api::types::lower_hex::impl_to_lower_hex_for_newtype;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct TYPED_FLAGS(u32);
        impl BitAnd for TYPED_FLAGS {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }
        impl BitOr for TYPED_FLAGS {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
        impl BitOrAssign for TYPED_FLAGS {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0.bitor_assign(rhs.0);
            }
        }
        impl Not for TYPED_FLAGS {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(self.0.not())
            }
        }
        const FLAG1: TYPED_FLAGS = TYPED_FLAGS(0x0001);
        const FLAG2: TYPED_FLAGS = TYPED_FLAGS(0x0002);
        const FLAG4: TYPED_FLAGS = TYPED_FLAGS(0x0004);

        impl_win_flags_for!(TYPED_FLAGS);

        define_flag_type!(
            TypedFlag : TYPED_FLAGS {
                self;
                FLAG1 => VALUE1,
                FLAG2 => VALUE2,
                FLAG4 => VALUE4,
            }
        );
        impl_to_lower_hex_for_newtype!(TYPED_FLAGS);

        #[test]
        fn contains() {
            let bits = FLAG1 | FLAG4;

            let flags: Flags<TypedFlag> = bits.into();
            assert!(flags.contains(TypedFlag::VALUE1));
            assert!(!flags.contains(TypedFlag::VALUE2));
            assert!(flags.contains(TypedFlag::VALUE4));

            let bits2: TYPED_FLAGS = flags.into();
            assert_eq!(bits2, bits);
        }

        #[test]
        fn unknown() {
            const UNKNOWN_BITS: TYPED_FLAGS = TYPED_FLAGS(0b0101_0000);
            let bits = FLAG2 | UNKNOWN_BITS;

            let flags: Flags<TypedFlag> = bits.into();
            assert!(!flags.contains(TypedFlag::VALUE1));
            assert!(flags.contains(TypedFlag::VALUE2));
            assert!(!flags.contains(TypedFlag::VALUE4));
            assert!(flags.contains(TypedFlag::Unknown(UNKNOWN_BITS.into())));

            let bits2: TYPED_FLAGS = flags.into();
            assert_eq!(bits2, bits);
        }

        #[test]
        fn debug() {
            let flags: Flags<TypedFlag> = (FLAG1 | FLAG4).into();

            let formatted = format!("{flags:?}");

            assert_eq!(formatted, "[VALUE1, VALUE4]");
        }

        #[test]
        fn serialize_as_json() {
            let flags: Flags<TypedFlag> = (FLAG1 | FLAG4).into();

            let serialized = serde_json::to_string(&flags).unwrap();

            assert_eq!(serialized, r#"["VALUE1","VALUE4"]"#);
        }
    }
}
