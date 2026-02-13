#[cfg(windows)]
use iced::widget::checkbox;
use iced::widget::{button, column, container, radio, row, text, toggler};
use iced::{Command, Theme, alignment, font};

use crate::options_types::{Display, OperatingSystem, OptionType, RebootAction};
use crate::text::Capitalize;

use super::{Dialog, Outcome};

#[derive(Clone, Copy, Debug)]
pub struct ScriptOptions {
    pub next_boot_operating_system: Option<OperatingSystem>,
    pub next_windows_boot_display: Option<Display>,
    #[cfg(windows)]
    pub switch_display: bool,
    pub reboot_action: Option<RebootAction>,
}

#[cfg(windows)]
const WINDOW_HEIGHT: u32 = 428;
#[cfg(not(windows))]
const WINDOW_HEIGHT: u32 = 406;

pub(crate) fn window_size() -> (u32, u32) {
    (340, WINDOW_HEIGHT)
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Message {
    NextBootOperatingSystem(Option<OperatingSystem>),
    NextWindowsBootDisplay(Option<Display>),
    #[cfg(windows)]
    SwitchDisplay(bool),
    Action(Option<RebootAction>),
    Confirm,
}

pub(crate) fn update(dialog: &mut Dialog, message: Message) -> iced::Command<Message> {
    match message {
        Message::NextBootOperatingSystem(os) => {
            dialog.script_options.next_boot_operating_system = os;
            Command::none()
        }
        Message::NextWindowsBootDisplay(display) => {
            dialog.script_options.next_windows_boot_display = display;
            Command::none()
        }
        #[cfg(windows)]
        Message::SwitchDisplay(switch) => {
            dialog.script_options.switch_display = switch;
            Command::none()
        }
        Message::Action(action) => {
            dialog.script_options.reboot_action = action;
            Command::none()
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
        container($content).padding([0, 0, 0, 8])
    };
}

pub(crate) fn view(dialog: &Dialog) -> iced::Element<'_, super::Message, iced::Renderer<Theme>> {
    let next_boot_os_widgets = {
        let widgets = create_option_group!(
            crate::text::operating_system::ON_NEXT_BOOT_DESCRIPTION.capitalize()
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

    let next_win_boot_display_widgets = {
        let widgets = create_option_group!(
            crate::text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION.capitalize()
        );
        add_to_option_group!(
            widgets,
            option_radios!(
                Display;
                dialog.script_options.next_windows_boot_display,
                |op: Display| op.to_string(),
                crate::text::display::UNDEFINED,
                |display| super::Message::AdvancedDialog(Message::NextWindowsBootDisplay(display)),
            )
        )
    };

    let reboot_action_widgets = {
        let widgets = create_option_group!("Ação");
        #[cfg(windows)]
        let widgets = add_to_option_group!(
            widgets,
            [checkbox(
                "trocar de tela antes",
                dialog.script_options.switch_display,
                |switch| super::Message::AdvancedDialog(Message::SwitchDisplay(switch)),
            )]
        );
        add_to_option_group!(
            widgets,
            option_radios!(
                RebootAction;
                dialog.script_options.reboot_action,
                |op| match op {
                    RebootAction::Reboot => "reiniciar".to_string(),
                    RebootAction::Shutdown => "desligar".to_string(),
                },
                "continuar usando",
                |action| super::Message::AdvancedDialog(Message::Action(action)),
            )
        )
    };

    column![
        next_boot_os_widgets,
        next_win_boot_display_widgets,
        reboot_action_widgets,
        row![
            button("OK")
                .on_press(super::Message::AdvancedDialog(Message::Confirm))
                .padding([4, 30]),
            mode_toggler!(true),
        ]
    ]
    .spacing(16)
    .padding([8, 12])
    .into()
}
