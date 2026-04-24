use std::fmt::Debug;
use std::hash::Hash;

use display_profile_lib::{Dimensions, PixelFormat, Position, Rational, Rotation, Scaling};
use serde::{Deserialize, Serialize};
use windows::Wdk::Graphics::Direct3D as WinDirect3D;
use windows::Win32::Devices::Display as WinDisplay;
use windows::Win32::Foundation as WinFoundation;
use windows::Win32::Graphics::Gdi as WinGdi;

use crate::win_api::types::device_id::DeviceId;
use crate::win_api::types::flags::Flags;
use crate::win_api::types::validated::Validated;

pub mod device_id;
pub mod flags;
pub mod hex_formatted_bits;
mod validated;

macro_rules! define_enum {
    (
        $win_path:ident :: $name:ident {
            $(
                $variant:ident => $short_variant:ident ,
            )*
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum $name {
            $(
                $short_variant,
            )*
        }

        impl From<$win_path::$name> for $name {
            fn from(value: $win_path::$name) -> Self {
                match value {
                    $(
                        $win_path::$variant => $name::$short_variant,
                    )*
                    _ => unreachable!(),
                }
            }
        }

        impl From<$name> for $win_path::$name {
            fn from(value: $name) -> Self {
                match value {
                    $(
                        $name::$short_variant => $win_path::$variant,
                    )*
                }
            }
        }
    };
}

macro_rules! define_profile_enum {
    (
        $win_path:ident :: $name:ident => $profile_type:ident {
            $(
                $variant:ident => $short_variant:ident => $profile_variant:ident ,
            )*
        }
    ) => {
        define_enum!(
            $win_path :: $name {
                $(
                    $variant => $short_variant ,
                )*
            }
        );

        impl From<$profile_type> for $name {
            fn from(value: $profile_type) -> Self {
                match value {
                    $(
                        $profile_type::$profile_variant => $name::$short_variant,
                    )*
                }
            }
        }
        impl From<$name> for $profile_type {
            fn from(value: $name) -> Self {
                match value {
                    $(
                        $name::$short_variant => $profile_type::$profile_variant,
                    )*
                }
            }
        }
    };
}

macro_rules! define_flag_type {
    ($( $token:tt )*) => {
        $crate::win_api::types::flags::define_flag_type!($( $token )*);
    };
}
pub(crate) use define_flag_type;

macro_rules! define_trivial_struct {
    (
        $win_path:ident :: $name:ident {
            $(
                $field:ident : $type:ty
            ),*
            $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name {
            $(
                pub $field: $type,
            )*
        }

        impl From<$win_path::$name> for $name {
            fn from(value: $win_path::$name) -> Self {
                $name {
                    $(
                        $field: value.$field.into(),
                    )*
                }
            }
        }

        impl From<$name> for $win_path::$name {
            fn from(value: $name) -> Self {
                $win_path::$name {
                    $(
                        $field: value.$field.into(),
                    )*
                }
            }
        }
    };
}

macro_rules! define_trivial_profile_struct {
    (
        $win_path:ident :: $name:ident => $profile_type:ident {
            $(
                $field:ident => $profile_field:ident : $type:ty
            ),*
            $(,)?
        }
    ) => {
        define_trivial_struct!(
            $win_path::$name {
                $(
                    $field: $type,
                )*
            }
        );

        impl From<$profile_type> for $name {
            fn from(value: $profile_type) -> Self {
                $name {
                    $(
                        $field: value.$profile_field.into(),
                    )*
                }
            }
        }

        impl From<$name> for $profile_type {
            fn from(value: $name) -> Self {
                $profile_type {
                    $(
                        $profile_field: value.$field,
                    )*
                }
            }
        }
    };
}

define_enum!(
    WinDirect3D::D3DKMDT_VIDEO_SIGNAL_STANDARD {
        D3DKMDT_VSS_UNINITIALIZED => UNINITIALIZED,
        D3DKMDT_VSS_VESA_DMT => VESA_DMT,
        D3DKMDT_VSS_VESA_GTF => VESA_GTF,
        D3DKMDT_VSS_VESA_CVT => VESA_CVT,
        D3DKMDT_VSS_IBM => IBM,
        D3DKMDT_VSS_APPLE => APPLE,
        D3DKMDT_VSS_NTSC_M => NTSC_M,
        D3DKMDT_VSS_NTSC_J => NTSC_J,
        D3DKMDT_VSS_NTSC_443 => NTSC_443,
        D3DKMDT_VSS_PAL_B => PAL_B,
        D3DKMDT_VSS_PAL_B1 => PAL_B1,
        D3DKMDT_VSS_PAL_G => PAL_G,
        D3DKMDT_VSS_PAL_H => PAL_H,
        D3DKMDT_VSS_PAL_I => PAL_I,
        D3DKMDT_VSS_PAL_D => PAL_D,
        D3DKMDT_VSS_PAL_N => PAL_N,
        D3DKMDT_VSS_PAL_NC => PAL_NC,
        D3DKMDT_VSS_SECAM_B => SECAM_B,
        D3DKMDT_VSS_SECAM_D => SECAM_D,
        D3DKMDT_VSS_SECAM_G => SECAM_G,
        D3DKMDT_VSS_SECAM_H => SECAM_H,
        D3DKMDT_VSS_SECAM_K => SECAM_K,
        D3DKMDT_VSS_SECAM_K1 => SECAM_K1,
        D3DKMDT_VSS_SECAM_L => SECAM_L,
        D3DKMDT_VSS_SECAM_L1 => SECAM_L1,
        D3DKMDT_VSS_EIA_861 => EIA_861,
        D3DKMDT_VSS_EIA_861A => EIA_861A,
        D3DKMDT_VSS_EIA_861B => EIA_861B,
        D3DKMDT_VSS_PAL_K => PAL_K,
        D3DKMDT_VSS_PAL_K1 => PAL_K1,
        D3DKMDT_VSS_PAL_L => PAL_L,
        D3DKMDT_VSS_PAL_M => PAL_M,
        D3DKMDT_VSS_OTHER => OTHER,
    }
);
impl From<u32> for D3DKMDT_VIDEO_SIGNAL_STANDARD {
    fn from(value: u32) -> Self {
        WinDirect3D::D3DKMDT_VIDEO_SIGNAL_STANDARD(value.cast_signed()).into()
    }
}
impl From<D3DKMDT_VIDEO_SIGNAL_STANDARD> for u32 {
    fn from(value: D3DKMDT_VIDEO_SIGNAL_STANDARD) -> Self {
        WinDirect3D::D3DKMDT_VIDEO_SIGNAL_STANDARD::from(value)
            .0
            .cast_unsigned()
    }
}

define_trivial_profile_struct!(
    WinDisplay::DISPLAYCONFIG_2DREGION => Dimensions {
        cx => width: u32,
        cy => height: u32,
    }
);

define_trivial_struct!(WinDisplay::DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    PathSourceSize: POINTL,
    DesktopImageRegion: RECTL,
    DesktopImageClip: RECTL,
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DISPLAYCONFIG_MODE_INFO {
    // pub infoType: DISPLAYCONFIG_MODE_INFO_TYPE, // Encoded in the Anonymous enum
    pub device_id: DeviceId,
    pub Anonymous: DISPLAYCONFIG_MODE_INFO_0,
}
impl From<WinDisplay::DISPLAYCONFIG_MODE_INFO> for DISPLAYCONFIG_MODE_INFO {
    fn from(value: WinDisplay::DISPLAYCONFIG_MODE_INFO) -> Self {
        DISPLAYCONFIG_MODE_INFO {
            device_id: DeviceId::from((value.adapterId.into(), value.id)),
            Anonymous: From::from((value.Anonymous, value.infoType.into())),
        }
    }
}
impl From<DISPLAYCONFIG_MODE_INFO> for WinDisplay::DISPLAYCONFIG_MODE_INFO {
    fn from(value: DISPLAYCONFIG_MODE_INFO) -> Self {
        let infoType = match value.Anonymous {
            DISPLAYCONFIG_MODE_INFO_0::targetMode(_) => DISPLAYCONFIG_MODE_INFO_TYPE::TARGET,
            DISPLAYCONFIG_MODE_INFO_0::sourceMode(_) => DISPLAYCONFIG_MODE_INFO_TYPE::SOURCE,
            DISPLAYCONFIG_MODE_INFO_0::desktopImageInfo(_) => {
                DISPLAYCONFIG_MODE_INFO_TYPE::DESKTOP_IMAGE
            }
        };

        WinDisplay::DISPLAYCONFIG_MODE_INFO {
            infoType: infoType.into(),
            id: value.device_id.id,
            adapterId: value.device_id.adapterId.into(),
            Anonymous: value.Anonymous.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DISPLAYCONFIG_MODE_INFO_0 {
    targetMode(DISPLAYCONFIG_TARGET_MODE),
    sourceMode(DISPLAYCONFIG_SOURCE_MODE),
    desktopImageInfo(DISPLAYCONFIG_DESKTOP_IMAGE_INFO),
}
impl
    From<(
        WinDisplay::DISPLAYCONFIG_MODE_INFO_0,
        DISPLAYCONFIG_MODE_INFO_TYPE,
    )> for DISPLAYCONFIG_MODE_INFO_0
{
    fn from(
        (value, info_type): (
            WinDisplay::DISPLAYCONFIG_MODE_INFO_0,
            DISPLAYCONFIG_MODE_INFO_TYPE,
        ),
    ) -> Self {
        match info_type {
            DISPLAYCONFIG_MODE_INFO_TYPE::DESKTOP_IMAGE => {
                DISPLAYCONFIG_MODE_INFO_0::desktopImageInfo(unsafe {
                    value.desktopImageInfo.into()
                })
            }
            DISPLAYCONFIG_MODE_INFO_TYPE::SOURCE => {
                DISPLAYCONFIG_MODE_INFO_0::sourceMode(unsafe { value.sourceMode.into() })
            }
            DISPLAYCONFIG_MODE_INFO_TYPE::TARGET => {
                DISPLAYCONFIG_MODE_INFO_0::targetMode(unsafe { value.targetMode.into() })
            }
        }
    }
}
impl From<DISPLAYCONFIG_MODE_INFO_0> for WinDisplay::DISPLAYCONFIG_MODE_INFO_0 {
    fn from(value: DISPLAYCONFIG_MODE_INFO_0) -> Self {
        match value {
            DISPLAYCONFIG_MODE_INFO_0::targetMode(displayconfig_target_mode) => {
                WinDisplay::DISPLAYCONFIG_MODE_INFO_0 {
                    targetMode: displayconfig_target_mode.into(),
                }
            }
            DISPLAYCONFIG_MODE_INFO_0::sourceMode(displayconfig_source_mode) => {
                WinDisplay::DISPLAYCONFIG_MODE_INFO_0 {
                    sourceMode: displayconfig_source_mode.into(),
                }
            }
            DISPLAYCONFIG_MODE_INFO_0::desktopImageInfo(displayconfig_desktop_image_info) => {
                WinDisplay::DISPLAYCONFIG_MODE_INFO_0 {
                    desktopImageInfo: displayconfig_desktop_image_info.into(),
                }
            }
        }
    }
}

define_enum!(
    WinDisplay::DISPLAYCONFIG_MODE_INFO_TYPE {
        DISPLAYCONFIG_MODE_INFO_TYPE_DESKTOP_IMAGE => DESKTOP_IMAGE,
        DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE => SOURCE,
        DISPLAYCONFIG_MODE_INFO_TYPE_TARGET => TARGET,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DISPLAYCONFIG_PATH_INFO {
    pub sourceInfo: DISPLAYCONFIG_PATH_SOURCE_INFO,
    pub targetInfo: DISPLAYCONFIG_PATH_TARGET_INFO,
    pub flags: Flags<DISPLAYCONFIG_PATH_INFO_flag>,
}
impl DISPLAYCONFIG_PATH_INFO {
    #[must_use]
    pub fn source_mode_idx(&self) -> Option<usize> {
        match self.sourceInfo.Anonymous {
            DISPLAYCONFIG_PATH_SOURCE_INFO_0::modeInfoIdx(mode_info_idx) => mode_info_idx.into(),
            DISPLAYCONFIG_PATH_SOURCE_INFO_0::Anonymous(anonymous) => {
                anonymous.sourceModeInfoIdx.into()
            }
        }
    }

    pub fn set_source_mode_idx(&mut self, idx: Option<usize>) {
        match &mut self.sourceInfo.Anonymous {
            DISPLAYCONFIG_PATH_SOURCE_INFO_0::modeInfoIdx(mode_info_idx) => {
                *mode_info_idx = idx.into();
            }
            DISPLAYCONFIG_PATH_SOURCE_INFO_0::Anonymous(anonymous) => {
                anonymous.sourceModeInfoIdx = idx.into();
            }
        }
    }

    #[must_use]
    pub fn target_mode_idx(&self) -> Option<usize> {
        match self.targetInfo.Anonymous {
            DISPLAYCONFIG_PATH_TARGET_INFO_0::modeInfoIdx(mode_info_idx) => mode_info_idx.into(),
            DISPLAYCONFIG_PATH_TARGET_INFO_0::Anonymous(anonymous) => {
                anonymous.targetModeInfoIdx.into()
            }
        }
    }
    pub fn set_target_mode_idx(&mut self, idx: Option<usize>) {
        match &mut self.targetInfo.Anonymous {
            DISPLAYCONFIG_PATH_TARGET_INFO_0::modeInfoIdx(mode_info_idx) => {
                *mode_info_idx = idx.into();
            }
            DISPLAYCONFIG_PATH_TARGET_INFO_0::Anonymous(anonymous) => {
                anonymous.targetModeInfoIdx = idx.into();
            }
        }
    }

    #[must_use]
    pub fn desktop_mode_idx(&self) -> Option<usize> {
        match self.targetInfo.Anonymous {
            DISPLAYCONFIG_PATH_TARGET_INFO_0::modeInfoIdx(_) => None,
            DISPLAYCONFIG_PATH_TARGET_INFO_0::Anonymous(anonymous) => {
                anonymous.desktopModeInfoIdx.into()
            }
        }
    }

    pub fn set_desktop_mode_idx(&mut self, idx: Option<usize>) {
        match &mut self.targetInfo.Anonymous {
            DISPLAYCONFIG_PATH_TARGET_INFO_0::modeInfoIdx(_) => {
                panic!("Desktop mode idx can't be set");
            }
            DISPLAYCONFIG_PATH_TARGET_INFO_0::Anonymous(anonymous) => {
                anonymous.desktopModeInfoIdx = idx.into();
            }
        }
    }
}
impl From<WinDisplay::DISPLAYCONFIG_PATH_INFO> for DISPLAYCONFIG_PATH_INFO {
    fn from(value: WinDisplay::DISPLAYCONFIG_PATH_INFO) -> Self {
        let flags: Flags<_> = value.flags.into();
        let support_virtual_mode =
            flags.contains(DISPLAYCONFIG_PATH_INFO_flag::SUPPORT_VIRTUAL_MODE);
        DISPLAYCONFIG_PATH_INFO {
            sourceInfo: From::from((value.sourceInfo, support_virtual_mode)),
            targetInfo: From::from((value.targetInfo, support_virtual_mode)),
            flags,
        }
    }
}
impl From<DISPLAYCONFIG_PATH_INFO> for WinDisplay::DISPLAYCONFIG_PATH_INFO {
    fn from(value: DISPLAYCONFIG_PATH_INFO) -> Self {
        WinDisplay::DISPLAYCONFIG_PATH_INFO {
            sourceInfo: value.sourceInfo.into(),
            targetInfo: value.targetInfo.into(),
            flags: value.flags.into(),
        }
    }
}

define_flag_type!(
    DISPLAYCONFIG_PATH_INFO_flag : u32 {
        WinGdi;
        DISPLAYCONFIG_PATH_ACTIVE => ACTIVE,
        DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE => SUPPORT_VIRTUAL_MODE,
        // BOOST_REFRESH_RATE,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO {
    pub device_id: DeviceId,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0,
    pub statusFlags: Flags<DISPLAYCONFIG_PATH_SOURCE_INFO_statusFlag>,
}
impl From<(WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO, bool)> for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn from(
        (value, support_virtual_mode): (WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO, bool),
    ) -> Self {
        DISPLAYCONFIG_PATH_SOURCE_INFO {
            device_id: DeviceId::from((value.adapterId.into(), value.id)),
            Anonymous: From::from((value.Anonymous, support_virtual_mode)),
            statusFlags: value.statusFlags.into(),
        }
    }
}
impl From<DISPLAYCONFIG_PATH_SOURCE_INFO> for WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn from(value: DISPLAYCONFIG_PATH_SOURCE_INFO) -> Self {
        WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO {
            adapterId: value.device_id.adapterId.into(),
            id: value.device_id.id,
            Anonymous: value.Anonymous.into(),
            statusFlags: value.statusFlags.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    modeInfoIdx(Validated<u32, { WinGdi::DISPLAYCONFIG_PATH_MODE_IDX_INVALID }>),
    Anonymous(DISPLAYCONFIG_PATH_SOURCE_INFO_0_0),
}
impl From<(WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0, bool)>
    for DISPLAYCONFIG_PATH_SOURCE_INFO_0
{
    fn from(
        (value, support_virtual_mode): (WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0, bool),
    ) -> Self {
        if support_virtual_mode {
            DISPLAYCONFIG_PATH_SOURCE_INFO_0::Anonymous(unsafe { value.Anonymous.into() })
        } else {
            DISPLAYCONFIG_PATH_SOURCE_INFO_0::modeInfoIdx(unsafe { value.modeInfoIdx.into() })
        }
    }
}
impl From<DISPLAYCONFIG_PATH_SOURCE_INFO_0> for WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn from(value: DISPLAYCONFIG_PATH_SOURCE_INFO_0) -> Self {
        match value {
            DISPLAYCONFIG_PATH_SOURCE_INFO_0::modeInfoIdx(modeInfoIdx) => {
                WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
                    modeInfoIdx: modeInfoIdx.to_value(),
                }
            }
            DISPLAYCONFIG_PATH_SOURCE_INFO_0::Anonymous(Anonymous) => {
                WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
                    Anonymous: Anonymous.into(),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    pub cloneGroupId: Validated<u16, { WinGdi::DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID }>,
    pub sourceModeInfoIdx: Validated<u16, { WinGdi::DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID }>,
}
impl From<WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0_0> for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn from(value: WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0_0) -> Self {
        #[expect(clippy::cast_possible_truncation)]
        let cloneGroupId = value._bitfield as u16;
        let sourceModeInfoIdx = (value._bitfield >> 16) as u16;

        DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
            cloneGroupId: cloneGroupId.into(),
            sourceModeInfoIdx: sourceModeInfoIdx.into(),
        }
    }
}
impl From<DISPLAYCONFIG_PATH_SOURCE_INFO_0_0> for WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn from(value: DISPLAYCONFIG_PATH_SOURCE_INFO_0_0) -> Self {
        let sourceModeInfoIdx = value.sourceModeInfoIdx.to_value();
        let cloneGroupId = value.cloneGroupId.to_value();

        WinDisplay::DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
            _bitfield: (u32::from(sourceModeInfoIdx) << 16) | u32::from(cloneGroupId),
        }
    }
}

define_flag_type!(
    DISPLAYCONFIG_PATH_SOURCE_INFO_statusFlag : u32 {
        WinGdi;
        DISPLAYCONFIG_SOURCE_IN_USE => IN_USE,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO {
    pub device_id: DeviceId,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0,
    pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    pub rotation: DISPLAYCONFIG_ROTATION,
    pub scaling: DISPLAYCONFIG_SCALING,
    pub refreshRate: DISPLAYCONFIG_RATIONAL,
    pub scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
    pub targetAvailable: bool,
    pub statusFlags: Flags<DISPLAYCONFIG_PATH_TARGET_INFO_statusFlag>,
}
impl From<(WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO, bool)> for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn from(
        (value, support_virtual_mode): (WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO, bool),
    ) -> Self {
        DISPLAYCONFIG_PATH_TARGET_INFO {
            device_id: DeviceId::from((value.adapterId.into(), value.id)),
            Anonymous: From::from((value.Anonymous, support_virtual_mode)),
            outputTechnology: value.outputTechnology.into(),
            rotation: value.rotation.into(),
            scaling: value.scaling.into(),
            refreshRate: value.refreshRate.into(),
            scanLineOrdering: value.scanLineOrdering.into(),
            targetAvailable: value.targetAvailable.into(),
            statusFlags: value.statusFlags.into(),
        }
    }
}
impl From<DISPLAYCONFIG_PATH_TARGET_INFO> for WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO {
    fn from(value: DISPLAYCONFIG_PATH_TARGET_INFO) -> Self {
        WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO {
            adapterId: value.device_id.adapterId.into(),
            id: value.device_id.id,
            Anonymous: value.Anonymous.into(),
            outputTechnology: value.outputTechnology.into(),
            rotation: value.rotation.into(),
            scaling: value.scaling.into(),
            refreshRate: value.refreshRate.into(),
            scanLineOrdering: value.scanLineOrdering.into(),
            targetAvailable: value.targetAvailable.into(),
            statusFlags: value.statusFlags.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    modeInfoIdx(Validated<u32, { WinGdi::DISPLAYCONFIG_PATH_MODE_IDX_INVALID }>),
    Anonymous(DISPLAYCONFIG_PATH_TARGET_INFO_0_0),
}
impl From<(WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0, bool)>
    for DISPLAYCONFIG_PATH_TARGET_INFO_0
{
    fn from(
        (value, support_virtual_mode): (WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0, bool),
    ) -> Self {
        if support_virtual_mode {
            DISPLAYCONFIG_PATH_TARGET_INFO_0::Anonymous(unsafe { value.Anonymous.into() })
        } else {
            DISPLAYCONFIG_PATH_TARGET_INFO_0::modeInfoIdx(unsafe { value.modeInfoIdx.into() })
        }
    }
}
impl From<DISPLAYCONFIG_PATH_TARGET_INFO_0> for WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn from(value: DISPLAYCONFIG_PATH_TARGET_INFO_0) -> Self {
        match value {
            DISPLAYCONFIG_PATH_TARGET_INFO_0::modeInfoIdx(modeInfoIdx) => {
                WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0 {
                    modeInfoIdx: modeInfoIdx.to_value(),
                }
            }
            DISPLAYCONFIG_PATH_TARGET_INFO_0::Anonymous(displayconfig_path_target_info_0_0) => {
                WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0 {
                    Anonymous: displayconfig_path_target_info_0_0.into(),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    pub desktopModeInfoIdx:
        Validated<u16, { WinGdi::DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID }>,
    pub targetModeInfoIdx: Validated<u16, { WinGdi::DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID }>,
}
impl From<WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0_0> for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn from(value: WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0_0) -> Self {
        #[expect(clippy::cast_possible_truncation)]
        let desktopModeInfoIdx = value._bitfield as u16;
        let targetModeInfoIdx = (value._bitfield >> 16) as u16;

        DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
            desktopModeInfoIdx: desktopModeInfoIdx.into(),
            targetModeInfoIdx: targetModeInfoIdx.into(),
        }
    }
}
impl From<DISPLAYCONFIG_PATH_TARGET_INFO_0_0> for WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn from(value: DISPLAYCONFIG_PATH_TARGET_INFO_0_0) -> Self {
        let targetModeInfoIdx = value.targetModeInfoIdx.to_value();
        let desktopModeInfoIdx = value.desktopModeInfoIdx.to_value();

        WinDisplay::DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
            _bitfield: (u32::from(targetModeInfoIdx) << 16) | u32::from(desktopModeInfoIdx),
        }
    }
}

define_flag_type!(
    DISPLAYCONFIG_PATH_TARGET_INFO_statusFlag : u32 {
        WinGdi;
        DISPLAYCONFIG_TARGET_IN_USE => IN_USE,
        DISPLAYCONFIG_TARGET_FORCIBLE => FORCIBLE,
        DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT => FORCED_AVAILABILITY_BOOT,
        DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH => FORCED_AVAILABILITY_PATH,
        DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM => FORCED_AVAILABILITY_SYSTEM,
        DISPLAYCONFIG_TARGET_IS_HMD => IS_HMD,
    }
);

define_profile_enum!(
    WinDisplay::DISPLAYCONFIG_PIXELFORMAT => PixelFormat {
        DISPLAYCONFIG_PIXELFORMAT_16BPP => _16BPP => BitsPerPixel16,
        DISPLAYCONFIG_PIXELFORMAT_24BPP => _24BPP => BitsPerPixel24,
        DISPLAYCONFIG_PIXELFORMAT_32BPP => _32BPP => BitsPerPixel32,
        DISPLAYCONFIG_PIXELFORMAT_8BPP => _8BPP => BitsPerPixel8,
        DISPLAYCONFIG_PIXELFORMAT_NONGDI => NONGDI => NONGDI,
    }
);

define_trivial_profile_struct!(
    WinDisplay::DISPLAYCONFIG_RATIONAL => Rational {
        Numerator => numerator: u32,
        Denominator => denominator: u32,
    }
);

define_profile_enum!(
    WinDisplay::DISPLAYCONFIG_ROTATION => Rotation {
        DISPLAYCONFIG_ROTATION_IDENTITY => IDENTITY => IDENTITY,
        DISPLAYCONFIG_ROTATION_ROTATE180 => ROTATE180 => ROTATE180,
        DISPLAYCONFIG_ROTATION_ROTATE270 => ROTATE270 => ROTATE270,
        DISPLAYCONFIG_ROTATION_ROTATE90 => ROTATE90 => ROTATE90,
    }
);

define_profile_enum!(
    WinDisplay::DISPLAYCONFIG_SCALING => Scaling {
        DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX => ASPECTRATIOCENTEREDMAX => ASPECTRATIOCENTEREDMAX,
        DISPLAYCONFIG_SCALING_CENTERED => CENTERED => CENTERED,
        DISPLAYCONFIG_SCALING_CUSTOM => CUSTOM => CUSTOM,
        DISPLAYCONFIG_SCALING_IDENTITY => IDENTITY => IDENTITY,
        DISPLAYCONFIG_SCALING_PREFERRED => PREFERRED => PREFERRED,
        DISPLAYCONFIG_SCALING_STRETCHED => STRETCHED => STRETCHED,
    }
);

define_enum!(
    WinDisplay::DISPLAYCONFIG_SCANLINE_ORDERING {
        // DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED => INTERLACED, // Same as DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST
        DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST => INTERLACED_LOWERFIELDFIRST,
        DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST => INTERLACED_UPPERFIELDFIRST,
        DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE => PROGRESSIVE,
        DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED => UNSPECIFIED,
    }
);

define_trivial_struct!(WinDisplay::DISPLAYCONFIG_SOURCE_MODE {
    width: u32,
    height: u32,
    pixelFormat: DISPLAYCONFIG_PIXELFORMAT,
    position: POINTL,
});

define_trivial_struct!(WinDisplay::DISPLAYCONFIG_TARGET_MODE {
    targetVideoSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO,
});

define_enum!(
    WinDisplay::DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO => COMPONENT_VIDEO,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO => COMPOSITE_VIDEO,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED => DISPLAYPORT_EMBEDDED,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL => DISPLAYPORT_EXTERNAL,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_USB_TUNNEL => DISPLAYPORT_USB_TUNNEL,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI => DVI,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN => D_JPN,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15 => HD15,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI => HDMI,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_VIRTUAL => INDIRECT_VIRTUAL,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_WIRED => INDIRECT_WIRED,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL => INTERNAL,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS => LVDS,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST => MIRACAST,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER => OTHER,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI => SDI,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE => SDTVDONGLE,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO => SVIDEO,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED => UDI_EMBEDDED,
        DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL => UDI_EXTERNAL,
    }
);

define_trivial_struct!(WinDisplay::DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    pixelRate: u64,
    hSyncFreq: DISPLAYCONFIG_RATIONAL,
    vSyncFreq: DISPLAYCONFIG_RATIONAL,
    activeSize: DISPLAYCONFIG_2DREGION,
    totalSize: DISPLAYCONFIG_2DREGION,
    Anonymous: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0,
    scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    pub videoStandard: D3DKMDT_VIDEO_SIGNAL_STANDARD,
}
impl From<WinDisplay::DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0> for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    fn from(value: WinDisplay::DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0) -> Self {
        DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
            videoStandard: unsafe { value.videoStandard.into() },
        }
    }
}
impl From<DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0> for WinDisplay::DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    fn from(value: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0) -> Self {
        WinDisplay::DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
            videoStandard: value.videoStandard.into(),
        }
    }
}

define_trivial_struct!(WinFoundation::LUID {
    LowPart: u32,
    HighPart: i32,
});
impl PartialOrd for LUID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for LUID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.HighPart
            .cmp(&other.HighPart)
            .then(self.LowPart.cmp(&other.LowPart))
    }
}

define_trivial_profile_struct!(
    WinFoundation::POINTL => Position {
        x => x: i32,
        y => y: i32
    }
);

define_trivial_struct!(WinFoundation::RECTL {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
});
