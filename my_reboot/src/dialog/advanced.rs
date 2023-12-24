use std::ptr::NonNull;

use anyhow::Result;
use iced::widget::{button, column, radio, row, text};
use iced::{
    executor, font, keyboard, subscription, window, Application, Command, Element, Event, Settings,
    Subscription, Theme,
};

use crate::options_types::{Display, OperatingSystem, OptionType};
use crate::text::Capitalize;

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
            size: (340, 302),
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
    // TODO
    // pub switch_display: bool, // On Windows only!
    // pub reboot_action: Option<RebootAction>,
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

macro_rules! bold_text {
    ($text:expr) => {
        text($text).font(font::Font {
            weight: font::Weight::Bold,
            ..Default::default()
        })
    };
}

macro_rules! radio_group {
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
                .size(12)
                .spacing(6)
            })
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
        let next_boot_os_widgets = radio_group!(
            OperatingSystem;
            self.flags().options.next_boot_operating_system,
            |op: OperatingSystem| op.to_string(),
            crate::text::operating_system::UNDEFINED,
            Message::NextBootOperatingSystem,
        )
        .fold(
            column![bold_text!(
                crate::text::operating_system::ON_NEXT_BOOT_DESCRIPTION.capitalize()
            )],
            |column, radio| column.push(radio),
        )
        .spacing(2);

        let next_win_boot_display_widgets = radio_group!(
            Display;
            self.flags().options.next_windows_boot_display,
            |op: Display| op.to_string(),
            crate::text::display::UNDEFINED,
            Message::NextWindowsBootDisplay,
        )
        .fold(
            column![bold_text!(
                crate::text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION.capitalize()
            )],
            |column, radio| column.push(radio),
        )
        .spacing(2);

        column![
            next_boot_os_widgets,
            next_win_boot_display_widgets,
            row![button("OK").on_press(Message::Confirm).padding([4, 30])]
        ]
        .spacing(16)
        .padding([8, 12])
        .into()
    }
}
