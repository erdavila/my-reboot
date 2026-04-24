#![expect(clippy::missing_errors_doc)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use windows::Win32::Devices::Display::DISPLAYCONFIG_RATIONAL;
use windows::Win32::Foundation::POINTL;

mod error;
mod get_profile;
mod set_profile;
mod win_api;

pub use error::Error;
pub use get_profile::get_profile;
pub use set_profile::set_profile;

type Result<T, E = Error> = std::result::Result<T, E>;

macro_rules! define_windows_mapped_enum {
    (
        $name:ident => $win_type:ident in $($win_path:ident)::+ {
            $(
                $( #[ $attribute:meta ] )?
                $variant:ident => $win_val:ident ,
            )*
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum $name {
            $(
                $( #[ $attribute ] )?
                $variant,
            )*
        }

        impl From<$($win_path::)+ $win_type> for $name {
            fn from(value: $($win_path::)+ $win_type) -> Self {
                use $($win_path)::+ as WinPath;
                match value {
                    $( WinPath::$win_val => $name::$variant, )*
                    _ => unreachable!(),
                }
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Monitor {
    pub friendly_device_name: String,
    pub source_device_name: String,
    pub device_path: String,
    pub dimensions: Dimensions,
    pub pixel_format: PixelFormat,
    pub position: Position,
    pub rotation: Rotation,
    pub scaling: Scaling,
    pub refresh_rate: Rational,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

define_windows_mapped_enum!(
    PixelFormat => DISPLAYCONFIG_PIXELFORMAT in windows::Win32::Devices::Display {
        #[cfg_attr(feature = "serde", serde(rename = "8BPP"))]
        BitsPerPixel8 => DISPLAYCONFIG_PIXELFORMAT_8BPP,
        #[cfg_attr(feature = "serde", serde(rename = "16BPP"))]
        BitsPerPixel16 => DISPLAYCONFIG_PIXELFORMAT_16BPP,
        #[cfg_attr(feature = "serde", serde(rename = "24BPP"))]
        BitsPerPixel24 => DISPLAYCONFIG_PIXELFORMAT_24BPP,
        #[cfg_attr(feature = "serde", serde(rename = "32BPP"))]
        BitsPerPixel32 => DISPLAYCONFIG_PIXELFORMAT_32BPP,
        NONGDI => DISPLAYCONFIG_PIXELFORMAT_NONGDI,
    }
);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl From<POINTL> for Position {
    fn from(value: POINTL) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

define_windows_mapped_enum!(
    Rotation => DISPLAYCONFIG_ROTATION in windows::Win32::Devices::Display {
        IDENTITY => DISPLAYCONFIG_ROTATION_IDENTITY,
        ROTATE90 => DISPLAYCONFIG_ROTATION_ROTATE90,
        ROTATE180 => DISPLAYCONFIG_ROTATION_ROTATE180,
        ROTATE270 => DISPLAYCONFIG_ROTATION_ROTATE270,
    }
);

define_windows_mapped_enum!(
    Scaling => DISPLAYCONFIG_SCALING in windows::Win32::Devices::Display {
        IDENTITY => DISPLAYCONFIG_SCALING_IDENTITY,
        CENTERED => DISPLAYCONFIG_SCALING_CENTERED,
        STRETCHED => DISPLAYCONFIG_SCALING_STRETCHED,
        ASPECTRATIOCENTEREDMAX => DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX,
        CUSTOM => DISPLAYCONFIG_SCALING_CUSTOM,
        PREFERRED => DISPLAYCONFIG_SCALING_PREFERRED,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rational {
    pub numerator: u32,
    pub denominator: u32,
}
impl From<DISPLAYCONFIG_RATIONAL> for Rational {
    fn from(value: DISPLAYCONFIG_RATIONAL) -> Self {
        Self {
            numerator: value.Numerator,
            denominator: value.Denominator,
        }
    }
}

pub type Profile = Vec<Monitor>;

fn is_flag_set(flags: u32, flag: u32) -> bool {
    flags & flag == flag
}

fn windows_string_to_string(s: &[u16]) -> Result<String> {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    Ok(String::from_utf16(&s[..len])?)
}
