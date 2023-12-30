use std::ptr::NonNull;

use anyhow::Result;
use iced::{
    executor, keyboard, subscription, window, Application, Command, Event, Settings, Size, Theme,
};

pub use self::advanced::ScriptOptions;

macro_rules! mode_toggler {
    ($is_checked:expr) => {
        toggler(Some(String::from("Modo avançado")), $is_checked, |_| {
            $crate::dialog::Message::SwitchMode
        })
        .text_size(12)
        .text_alignment(alignment::Horizontal::Right)
        .spacing(2)
    };
}

mod advanced;
mod basic;

#[derive(Clone, Copy)]
pub enum Mode {
    Basic,
    Advanced,
}

#[derive(Debug)]
pub enum Outcome {
    PredefinedScriptIndex(usize),
    ScriptOptions(ScriptOptions),
}

pub fn show(
    initial_mode: Mode,
    predefined_script_labels: Vec<&'static str>,
    initial_script_options: ScriptOptions,
) -> Result<Option<Outcome>> {
    /*
       Unfortunatelly there is no way to "return" an outcome from an iced window, so we have to pass
       a pointer for the outcome as a flag, and use it after the window is closed.
    */

    let label_count = predefined_script_labels.len() as u32;

    let mut outcome: Option<Outcome> = None;
    let flags = Flags {
        initial_mode,
        predefined_script_labels,
        initial_script_options,
        outcome: NonNull::from(&mut outcome),
    };
    let settings = Settings::with_flags(flags);
    let settings = Settings {
        window: window::Settings {
            size: match initial_mode {
                Mode::Basic => basic::window_size(label_count),
                Mode::Advanced => advanced::window_size(),
            },
            position: window::Position::Centered,
            resizable: false,
            icon: Some(window::icon::from_file_data(
                include_bytes!("../../assets/icon-256x256.png"),
                None,
            )?),
            ..Default::default()
        },
        ..settings
    };

    Dialog::run(settings)?;

    Ok(outcome)
}

struct Flags {
    initial_mode: Mode,
    predefined_script_labels: Vec<&'static str>,
    initial_script_options: ScriptOptions,
    outcome: NonNull<Option<Outcome>>,
}

#[derive(Clone, Copy, Debug)]
enum Message {
    BasicDialog(basic::Message),
    AdvancedDialog(advanced::Message),
    SwitchMode,
    Dismiss,
}

struct Dialog {
    mode: Mode,
    predefined_script_labels: Vec<&'static str>,
    script_options: ScriptOptions,
    outcome: NonNull<Option<Outcome>>,
}

impl Dialog {
    fn set_outcome_and_close_window<M>(&mut self, outcome: Option<Outcome>) -> Command<M> {
        *unsafe { self.outcome.as_mut() } = outcome;
        window::close()
    }
}

impl Application for Dialog {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let this = Self {
            mode: flags.initial_mode,
            predefined_script_labels: flags.predefined_script_labels,
            script_options: flags.initial_script_options,
            outcome: flags.outcome,
        };
        (this, Command::none())
    }

    fn title(&self) -> String {
        String::from("My Reboot")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
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
            Message::BasicDialog(message) => basic::update(self, message).map(Message::BasicDialog),
            Message::AdvancedDialog(message) => {
                advanced::update(self, message).map(Message::AdvancedDialog)
            }
            Message::SwitchMode => {
                let (new_mode, new_size) = match self.mode {
                    Mode::Basic => (Mode::Advanced, advanced::window_size()),
                    Mode::Advanced => (
                        Mode::Basic,
                        basic::window_size(self.predefined_script_labels.len() as u32),
                    ),
                };

                self.mode = new_mode;

                Command::batch([
                    window::resize(Size {
                        width: new_size.0,
                        height: new_size.1,
                    }),
                    // TODO: reposition window (iced currently does not support it)
                ])
            }
            Message::Dismiss => self.set_outcome_and_close_window(None),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self.mode {
            Mode::Basic => basic::view(self),
            Mode::Advanced => advanced::view(self),
        }
    }
}
