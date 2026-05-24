use iced::Length::Fill;
#[cfg(windows)]
use iced::widget::checkbox;
use iced::widget::{button, column, container, radio, row, space, text};
use iced::{Padding, Size, Task, Theme, font};

use super::{Dialog, Outcome};
use crate::options_types::{OperatingSystem, OptionType, ProfileId, RebootAction};
use crate::text::Capitalized;

#[derive(Clone, Copy, Debug)]
pub struct ScriptOptions {
    pub next_boot_operating_system: Option<OperatingSystem>,
    pub(crate) next_windows_boot_profile: Option<ProfileId>,
    #[cfg(windows)]
    pub(crate) switch_profile: bool,
    pub reboot_action: Option<RebootAction>,
}

#[cfg(windows)]
const WINDOW_HEIGHT: f32 = 428.0;
#[cfg(not(windows))]
const WINDOW_HEIGHT: f32 = 406.0;

pub(crate) fn window_size() -> Size {
    Size {
        width: 350.0,
        height: WINDOW_HEIGHT,
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Message {
    NextBootOperatingSystem(Option<OperatingSystem>),
    NextWindowsBootProfile(Option<ProfileId>),
    #[cfg(windows)]
    SwitchProfile(bool),
    Action(Option<RebootAction>),
    Confirm,
}

pub(crate) fn update(dialog: &mut Dialog, message: Message) -> Task<Message> {
    match message {
        Message::NextBootOperatingSystem(os) => {
            dialog.script_options.next_boot_operating_system = os;
            Task::none()
        }
        Message::NextWindowsBootProfile(profile_id) => {
            dialog.script_options.next_windows_boot_profile = profile_id;
            Task::none()
        }
        #[cfg(windows)]
        Message::SwitchProfile(switch) => {
            dialog.script_options.switch_profile = switch;
            Task::none()
        }
        Message::Action(action) => {
            dialog.script_options.reboot_action = action;
            Task::none()
        }
        Message::Confirm => {
            dialog.set_outcome_and_close_window(Some(Outcome::ScriptOptions(dialog.script_options)))
        }
    }
}

macro_rules! create_option_group {
    ($title:expr) => {
        column![text($title).font(font::Font {
            weight: font::Weight::Bold,
            ..Default::default()
        })]
        .spacing(2)
    };
}

macro_rules! add_to_option_group {
    ($option_group:expr, $widgets:expr) => {
        $widgets.into_iter().fold($option_group, |og, widget| {
            og.push(indented!(widget.size(12).spacing(6)))
        })
    };
}

macro_rules! option_radios {
    ($option_type:ident; $current_value:expr, $label:expr, $none_label:expr, $message:expr $(,)?) => {
        $option_type::values()
            .into_iter()
            .map(Some)
            .chain(std::iter::once(None))
            .map(|option| {
                let label = $label;
                radio(
                    match option {
                        Some(op) => label(op),
                        None => $none_label.to_string(),
                    },
                    option,
                    ($current_value == option).then_some(option),
                    $message,
                )
            })
    };
}

macro_rules! indented {
    ($content:expr) => {
        container($content).padding(Padding::default().left(8))
    };
}

pub(crate) fn view(dialog: &Dialog) -> iced::Element<'_, super::Message, Theme, iced::Renderer> {
    let next_boot_os_widgets = {
        let widgets = create_option_group!(
            Capitalized(crate::text::operating_system::ON_NEXT_BOOT_DESCRIPTION).to_string()
        );
        add_to_option_group!(
            widgets,
            option_radios!(
                OperatingSystem;
                dialog.script_options.next_boot_operating_system,
                |op: OperatingSystem| op.to_string(),
                crate::text::operating_system::UNDEFINED,
                |os| super::Message::AdvancedDialog(Message::NextBootOperatingSystem(os)),
            )
        )
    };

    let next_win_boot_profile_widgets = {
        let widgets = create_option_group!(
            Capitalized(crate::text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION).to_string()
        );
        add_to_option_group!(
            widgets,
            option_radios!(
                ProfileId;
                dialog.script_options.next_windows_boot_profile,
                |op: ProfileId| {
                    match op {
                        ProfileId::A => dialog.profile_labels[0].clone(),
                        ProfileId::B => dialog.profile_labels[1].clone(),
                    }
                },
                crate::text::profile::UNDEFINED,
                |profile_id| super::Message::AdvancedDialog(Message::NextWindowsBootProfile(profile_id)),
            )
        )
    };

    let reboot_action_widgets = {
        let widgets = create_option_group!("Ação");
        #[cfg(windows)]
        let widgets = add_to_option_group!(
            widgets,
            [checkbox(dialog.script_options.switch_profile)
                .label("trocar de perfil antes")
                .on_toggle(
                    |switch| super::Message::AdvancedDialog(Message::SwitchProfile(switch))
                )]
        );
        add_to_option_group!(
            widgets,
            option_radios!(
                RebootAction;
                dialog.script_options.reboot_action,
                |op: RebootAction| op.to_string(),
                "continuar usando",
                |action| super::Message::AdvancedDialog(Message::Action(action)),
            )
        )
    };

    column![
        next_boot_os_widgets,
        next_win_boot_profile_widgets,
        reboot_action_widgets,
        row![
            button("OK")
                .on_press(super::Message::AdvancedDialog(Message::Confirm))
                .padding([4, 30]),
            space().width(Fill),
            mode_toggler!(true),
        ]
    ]
    .spacing(16)
    .padding([8, 12])
    .into()
}
