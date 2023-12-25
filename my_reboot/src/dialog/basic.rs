use std::ptr::NonNull;

use anyhow::Result;
use iced::widget::{button, column};
use iced::{
    executor, keyboard, subscription, window, Application, Command, Event, Settings, Subscription,
    Theme,
};

const WINDOW_WIDTH: u16 = 340;
const PADDING: u16 = 12;
const BUTTON_HEIGHT: u16 = 32;

pub fn show(labels: Vec<&'static str>) -> Result<Option<usize>> {
    /*
       Unfortunatelly there is no way to "return" an outcome from an iced window, so we have to pass
       a pointer for the outcome as a flag, and use it after the window is closed.
    */

    let label_count = labels.len() as u32;

    let mut user_choice = None;
    let flags = Flags {
        user_choice: NonNull::from(&mut user_choice),
        labels,
    };
    let settings = Settings::with_flags(flags);
    let settings = Settings {
        window: window::Settings {
            size: (
                WINDOW_WIDTH as u32,
                (BUTTON_HEIGHT + 1) as u32 * label_count + 23,
            ),
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

    BasicDialog::run(settings)?;

    Ok(user_choice)
}

type UserChoice = Option<usize>;

struct Flags {
    user_choice: NonNull<UserChoice>,
    labels: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Confirm(usize),
    Dismiss,
}

struct BasicDialog {
    flags: Flags,
}

impl BasicDialog {
    fn user_choice_mut(&mut self) -> &mut UserChoice {
        unsafe { self.flags.user_choice.as_mut() }
    }
}

impl Application for BasicDialog {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { flags }, Command::none())
    }

    fn title(&self) -> String {
        String::from("My Reboot")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events_with(|event, _status| {
            if let Event::Keyboard(keyboard::Event::KeyPressed {
                key_code: keyboard::KeyCode::Escape,
                ..
            }) = event
            {
                return Some(Message::Dismiss);
            }

            None
        })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Confirm(index) => {
                *self.user_choice_mut() = Some(index);
                window::close()
            }
            Message::Dismiss => {
                *self.user_choice_mut() = None;
                window::close()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let column =
            self.flags
                .labels
                .iter()
                .enumerate()
                .fold(column![], |column, (index, label)| {
                    let button = button(*label)
                        .on_press(Message::Confirm(index))
                        .height(BUTTON_HEIGHT)
                        .width(WINDOW_WIDTH - 2 * PADDING);
                    column.push(button)
                });

        column.spacing(1).padding(PADDING).into()
    }
}
