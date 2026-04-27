use windows::Win32::Devices::Display::DISPLAYCONFIG_PATH_INFO;
use windows::Win32::Graphics::Gdi::{
    DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID, DISPLAYCONFIG_PATH_MODE_IDX_INVALID,
    DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID, DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE,
    DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID,
};

use crate::Result;

// An implementation of [try_find](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_find) to be used
// while the official one is still experimental.
pub(crate) trait TryFind: Iterator {
    fn try_find<E>(
        &mut self,
        f: impl FnMut(&Self::Item) -> Result<bool, E>,
    ) -> Result<Option<Self::Item>, E>;
}

impl<I> TryFind for I
where
    I: Iterator,
{
    fn try_find<E>(
        &mut self,
        mut f: impl FnMut(&Self::Item) -> Result<bool, E>,
    ) -> Result<Option<Self::Item>, E> {
        for item in self {
            if f(&item)? {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}

pub(crate) trait PathInfoExt {
    fn support_virtual_mode(&self) -> bool;

    fn source_mode_idx(&self) -> Option<usize>;
    #[expect(dead_code)]
    fn target_mode_idx(&self) -> Option<usize>;
    fn desktop_mode_idx(&self) -> Option<usize>;

    fn set_source_mode_idx(&mut self, idx: Option<usize>);
    fn set_target_mode_idx(&mut self, idx: Option<usize>);
    fn set_desktop_mode_idx(&mut self, idx: Option<usize>);
}

impl PathInfoExt for DISPLAYCONFIG_PATH_INFO {
    fn support_virtual_mode(&self) -> bool {
        self.flags.contains(DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE)
    }

    fn source_mode_idx(&self) -> Option<usize> {
        unsafe {
            if self.support_virtual_mode() {
                self.sourceInfo
                    .Anonymous
                    .Anonymous
                    ._bitfield
                    .higher_u16()
                    .to_source_mode_idx_option()
            } else {
                self.sourceInfo.Anonymous.modeInfoIdx.to_mode_idx_option()
            }
        }
    }

    fn target_mode_idx(&self) -> Option<usize> {
        unsafe {
            if self.support_virtual_mode() {
                self.targetInfo
                    .Anonymous
                    .Anonymous
                    ._bitfield
                    .higher_u16()
                    .to_target_mode_idx_option()
            } else {
                self.targetInfo.Anonymous.modeInfoIdx.to_mode_idx_option()
            }
        }
    }

    fn desktop_mode_idx(&self) -> Option<usize> {
        unsafe {
            if self.support_virtual_mode() {
                self.targetInfo
                    .Anonymous
                    .Anonymous
                    ._bitfield
                    .lower_u16()
                    .to_desktop_mode_idx_option()
            } else {
                DISPLAYCONFIG_PATH_MODE_IDX_INVALID.to_mode_idx_option()
            }
        }
    }

    fn set_source_mode_idx(&mut self, idx: Option<usize>) {
        if self.support_virtual_mode() {
            unsafe {
                self.sourceInfo
                    .Anonymous
                    .Anonymous
                    ._bitfield
                    .set_higher_u16(u16::from_source_mode_idx_option(idx));
            }
        } else {
            self.sourceInfo.Anonymous.modeInfoIdx = u32::from_mode_idx_option(idx);
        }
    }

    fn set_target_mode_idx(&mut self, idx: Option<usize>) {
        if self.support_virtual_mode() {
            unsafe {
                self.targetInfo
                    .Anonymous
                    .Anonymous
                    ._bitfield
                    .set_higher_u16(u16::from_target_mode_idx_option(idx));
            }
        } else {
            self.targetInfo.Anonymous.modeInfoIdx = u32::from_mode_idx_option(idx);
        }
    }

    fn set_desktop_mode_idx(&mut self, idx: Option<usize>) {
        if self.support_virtual_mode() {
            unsafe {
                self.targetInfo
                    .Anonymous
                    .Anonymous
                    ._bitfield
                    .set_lower_u16(u16::from_desktop_mode_idx_option(idx));
            }
        } else {
            assert!(idx.is_none(), "Desktop mode idx cannot be set");
        }
    }
}

pub(crate) trait U32Ext {
    fn contains(&self, flag: Self) -> bool;

    fn to_mode_idx_option(self) -> Option<usize>;
    fn from_mode_idx_option(idx: Option<usize>) -> Self;

    fn lower_u16(self) -> u16;
    fn higher_u16(self) -> u16;

    fn set_lower_u16(&mut self, value: u16);
    fn set_higher_u16(&mut self, value: u16);
}

impl U32Ext for u32 {
    fn contains(&self, flag: Self) -> bool {
        *self & flag == flag
    }

    fn to_mode_idx_option(self) -> Option<usize> {
        (self != DISPLAYCONFIG_PATH_MODE_IDX_INVALID).then_some(self as usize)
    }

    fn from_mode_idx_option(idx: Option<usize>) -> Self {
        match idx {
            #[expect(clippy::cast_possible_truncation)]
            Some(idx) => idx as u32,
            None => DISPLAYCONFIG_PATH_MODE_IDX_INVALID,
        }
    }

    fn lower_u16(self) -> u16 {
        (self & 0x0000_FFFF) as u16
    }

    fn higher_u16(self) -> u16 {
        (self >> 16) as u16
    }

    fn set_lower_u16(&mut self, value: u16) {
        *self &= 0xFFFF_0000;
        *self |= u32::from(value) & 0x0000_FFFF;
    }

    fn set_higher_u16(&mut self, value: u16) {
        *self &= 0x0000_FFFF;
        *self |= u32::from(value) << 16;
    }
}

trait U16Ext {
    fn to_source_mode_idx_option(self) -> Option<usize>;
    fn to_target_mode_idx_option(self) -> Option<usize>;
    fn to_desktop_mode_idx_option(self) -> Option<usize>;
    fn to_mode_idx_option(self, invalid: u32) -> Option<usize>;

    fn from_source_mode_idx_option(idx: Option<usize>) -> Self;
    fn from_target_mode_idx_option(idx: Option<usize>) -> Self;
    fn from_desktop_mode_idx_option(idx: Option<usize>) -> Self;
    fn from_mode_idx_option(idx: Option<usize>, invalid: u32) -> Self;
}

impl U16Ext for u16 {
    fn to_source_mode_idx_option(self) -> Option<usize> {
        self.to_mode_idx_option(DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID)
    }

    fn to_target_mode_idx_option(self) -> Option<usize> {
        self.to_mode_idx_option(DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID)
    }

    fn to_desktop_mode_idx_option(self) -> Option<usize> {
        self.to_mode_idx_option(DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID)
    }

    fn to_mode_idx_option(self, invalid: u32) -> Option<usize> {
        (u32::from(self) != invalid).then_some(self as usize)
    }

    fn from_source_mode_idx_option(idx: Option<usize>) -> Self {
        Self::from_mode_idx_option(idx, DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID)
    }

    fn from_target_mode_idx_option(idx: Option<usize>) -> Self {
        Self::from_mode_idx_option(idx, DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID)
    }

    fn from_desktop_mode_idx_option(idx: Option<usize>) -> Self {
        Self::from_mode_idx_option(idx, DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID)
    }

    fn from_mode_idx_option(idx: Option<usize>, invalid: u32) -> Self {
        #[expect(clippy::cast_possible_truncation)]
        match idx {
            Some(idx) => idx as u16,
            None => invalid as u16,
        }
    }
}

pub(crate) fn from_windows_string(s: &[u16]) -> Result<String> {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    Ok(String::from_utf16(&s[..len])?)
}
