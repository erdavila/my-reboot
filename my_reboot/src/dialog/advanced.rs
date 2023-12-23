use std::ptr::NonNull;

use anyhow::Result;
use iced::widget::{button, column, radio, row, text};
use iced::{
    executor, font, keyboard, subscription, window, Application, Command, Element, Event, Settings,
    Subscription, Theme,
};

use crate::options_types::OperatingSystem;
use crate::text::operating_system::ON_NEXT_BOOT_DESCRIPTION;
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
            size: (330, 190),
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
    // TODO
    // pub next_windows_boot_display: Option<Display>,
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
        let next_boot_os_widgets = [
            Some(OperatingSystem::Windows),
            Some(OperatingSystem::Linux),
            None,
        ]
        .into_iter()
        .fold(
            column![
                text(ON_NEXT_BOOT_DESCRIPTION.capitalize()).font(font::Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
            ],
            |column, option| {
                column.push(
                    radio(
                        match option {
                            Some(os) => os.to_string(),
                            None => crate::text::operating_system::UNDEFINED.to_string(),
                        },
                        option,
                        (self.flags().options.next_boot_operating_system == option)
                            .then_some(option),
                        Message::NextBootOperatingSystem,
                    )
                    .size(12)
                    .spacing(6),
                )
            },
        )
        .spacing(2);

        column![
            next_boot_os_widgets,
            row![button("OK").on_press(Message::Confirm).padding([4, 30])]
        ]
        .spacing(30)
        .padding([8, 12])
        .into()
    }
}
