use std::ptr::NonNull;

use anyhow::Result;
#[cfg(windows)]
use iced::widget::checkbox;
use iced::widget::{button, column, container, radio, row, text};
use iced::{
    executor, font, keyboard, subscription, window, Application, Command, Element, Event, Settings,
    Subscription, Theme,
};

use crate::options_types::{Display, OperatingSystem, OptionType, RebootAction};
use crate::text::Capitalize;

#[cfg(windows)]
const WINDOW_HEIGHT: u32 = 428;
#[cfg(not(windows))]
const WINDOW_HEIGHT: u32 = 406;

pub fn show(options: Options) -> Result<Option<Options>> {
    /*
       Unfortunatelly there is no way to "return" an outcome from an iced window, so we have to pass
       a pointer for the outcome as a flag, and use it after the window is closed.
    */

    let mut flags = Flags {
        options,
        confirmed: false,
    };
    let settings = Settings::with_flags(NonNull::from(&mut flags));
    let settings = Settings {
        window: window::Settings {
            size: (340, WINDOW_HEIGHT),
            position: window::Position::Centered,
            resizable: false,
            icon: Some(window::icon::from_file_data(
                include_bytes!("../../../256x256.png"),
                None,
            )?),
            ..Default::default()
        },
        ..settings
    };

    AdvancedDialog::run(settings)?;

    let outcome = flags.confirmed.then_some(flags.options);
    Ok(outcome)
}

#[derive(Default)]
pub struct Options {
    pub next_boot_operating_system: Option<OperatingSystem>,
    pub next_windows_boot_display: Option<Display>,
    #[cfg(windows)]
    pub switch_display: bool,
    pub reboot_action: Option<RebootAction>,
}

#[derive(Default)]
struct Flags {
    options: Options,
    confirmed: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NextBootOperatingSystem(Option<OperatingSystem>),
    NextWindowsBootDisplay(Option<Display>),
    #[cfg(windows)]
    SwitchDisplay(bool),
    Action(Option<RebootAction>),
    Confirm,
    Dismiss,
}

struct AdvancedDialog {
    flags: NonNull<Flags>,
}

impl AdvancedDialog {
    fn flags(&self) -> &Flags {
        unsafe { self.flags.as_ref() }
    }

    fn flags_mut(&mut self) -> &mut Flags {
        unsafe { self.flags.as_mut() }
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

impl Application for AdvancedDialog {
    type Executor = executor::Default;
    type Flags = NonNull<Flags>;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { flags }, Command::none())
    }

    fn title(&self) -> String {
        String::from("My Reboot - modo avançado")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events_with(|event, _status| {
            if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
                match key_code {
                    keyboard::KeyCode::Enter => return Some(Message::Confirm),
                    keyboard::KeyCode::Escape => return Some(Message::Dismiss),
                    _ => (),
                }
            }

            None
        })
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::NextBootOperatingSystem(os) => {
                self.flags_mut().options.next_boot_operating_system = os;
                Command::none()
            }
            Message::NextWindowsBootDisplay(display) => {
                self.flags_mut().options.next_windows_boot_display = display;
                Command::none()
            }
            #[cfg(windows)]
            Message::SwitchDisplay(switch) => {
                self.flags_mut().options.switch_display = switch;
                Command::none()
            }
            Message::Action(action) => {
                self.flags_mut().options.reboot_action = action;
                Command::none()
            }
            Message::Confirm => {
                self.flags_mut().confirmed = true;
                window::close()
            }
            Message::Dismiss => {
                self.flags_mut().confirmed = false;
                window::close()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let next_boot_os_widgets = {
            let widgets = create_option_group!(
                crate::text::operating_system::ON_NEXT_BOOT_DESCRIPTION.capitalize()
            );
            add_to_option_group!(
                widgets,
                option_radios!(
                    OperatingSystem;
                    self.flags().options.next_boot_operating_system,
                    |op: OperatingSystem| op.to_string(),
                    crate::text::operating_system::UNDEFINED,
                    Message::NextBootOperatingSystem,
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
                    self.flags().options.next_windows_boot_display,
                    |op: Display| op.to_string(),
                    crate::text::display::UNDEFINED,
                    Message::NextWindowsBootDisplay,
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
                    self.flags().options.switch_display,
                    Message::SwitchDisplay,
                )]
            );
            add_to_option_group!(
                widgets,
                option_radios!(
                    RebootAction;
                    self.flags().options.reboot_action,
                    |op| match op {
                        RebootAction::Reboot => "reiniciar".to_string(),
                        RebootAction::Shutdown => "desligar".to_string(),
                    },
                    "continuar usando",
                    Message::Action,
                )
            )
        };

        column![
            next_boot_os_widgets,
            next_win_boot_display_widgets,
            reboot_action_widgets,
            row![button("OK").on_press(Message::Confirm).padding([4, 30])]
        ]
        .spacing(16)
        .padding([8, 12])
        .into()
    }
}
