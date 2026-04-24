use display_profile_experiment::win_api::types;

macro_rules! eq_fields {
    (
        $left:ident , $right:ident ;
        $(
            $field:ident
        ),*
        $(,)?
    ) => {
        [
            $(
                $left.$field == $right.$field,
            )*
        ].into_iter().all(std::convert::identity)
    };
}

pub fn eq_ignoring_mode_idxs<T: EqIgnoringModeIdxs>(left: &T, right: &T) -> bool {
    T::eq_ignoring_mode_idxs(left, right)
}

pub trait EqIgnoringModeIdxs {
    fn eq_ignoring_mode_idxs(left: &Self, right: &Self) -> bool;
}

impl EqIgnoringModeIdxs for types::DISPLAYCONFIG_PATH_INFO {
    fn eq_ignoring_mode_idxs(left: &Self, right: &Self) -> bool {
        eq_fields!(left, right; flags)
            && eq_ignoring_mode_idxs(&left.sourceInfo, &right.sourceInfo)
            && eq_ignoring_mode_idxs(&left.targetInfo, &right.targetInfo)
    }
}

impl EqIgnoringModeIdxs for types::DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn eq_ignoring_mode_idxs(left: &Self, right: &Self) -> bool {
        eq_fields!(left, right; device_id, statusFlags)
            && eq_ignoring_mode_idxs(&left.Anonymous, &right.Anonymous)
    }
}

impl EqIgnoringModeIdxs for types::DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn eq_ignoring_mode_idxs(left: &Self, right: &Self) -> bool {
        use types::DISPLAYCONFIG_PATH_SOURCE_INFO_0::{Anonymous, modeInfoIdx};
        match (left, right) {
            // Ignoring!
            (modeInfoIdx(_), modeInfoIdx(_)) => true,
            (Anonymous(left_anonymous), Anonymous(right_anonymous)) => {
                eq_ignoring_mode_idxs(left_anonymous, right_anonymous)
            }
            _ => false,
        }
    }
}

impl EqIgnoringModeIdxs for types::DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn eq_ignoring_mode_idxs(left: &Self, right: &Self) -> bool {
        // Ignoring sourceModeInfoIdx
        eq_fields!(left, right; cloneGroupId)
    }
}

impl EqIgnoringModeIdxs for types::DISPLAYCONFIG_PATH_TARGET_INFO {
    fn eq_ignoring_mode_idxs(left: &Self, right: &Self) -> bool {
        eq_fields!(left, right;
            device_id,
            outputTechnology,
            rotation,
            scaling,
            refreshRate,
            scanLineOrdering,
            targetAvailable,
            statusFlags
        ) && eq_ignoring_mode_idxs(&left.Anonymous, &right.Anonymous)
    }
}

impl EqIgnoringModeIdxs for types::DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn eq_ignoring_mode_idxs(left: &Self, right: &Self) -> bool {
        use types::DISPLAYCONFIG_PATH_TARGET_INFO_0::{Anonymous, modeInfoIdx};
        match (left, right) {
            // Ignoring!
            (modeInfoIdx(_), modeInfoIdx(_)) => true,
            (Anonymous(left_anonymous), Anonymous(right_anonymous)) => {
                eq_ignoring_mode_idxs(left_anonymous, right_anonymous)
            }
            _ => false,
        }
    }
}

impl EqIgnoringModeIdxs for types::DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn eq_ignoring_mode_idxs(_left: &Self, _right: &Self) -> bool {
        // Ignoring desktopModeInfoIdx and targetModeInfoIdx
        true
    }
}
