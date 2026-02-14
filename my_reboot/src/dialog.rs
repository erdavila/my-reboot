use std::ptr::NonNull;

use anyhow::Result;
use iced::{Event, Task, Theme, event, keyboard, window};

pub use self::advanced::ScriptOptions;

macro_rules! mode_toggler {
    ($is_checked:expr) => {
        iced::widget::toggler($is_checked)
            .label("Modo avançado")
            .on_toggle(|_| $crate::dialog::Message::SwitchMode)
            .text_size(12)
            .text_alignment(iced::alignment::Horizontal::Right)
            .width(140)
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

    let label_count = predefined_script_labels.len();

    let mut outcome: Option<Outcome> = None;
    let flags = Flags {
        initial_mode,
        predefined_script_labels,
        initial_script_options,
        outcome: NonNull::from(&mut outcome),
    };

    let dialog = Dialog {
        mode: flags.initial_mode,
        predefined_script_labels: flags.predefined_script_labels,
        script_options: flags.initial_script_options,
        outcome: flags.outcome,
    };

    let window_settings = window::Settings {
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
    };

    iced::application("My Reboot", Dialog::update, Dialog::view)
        .window(window_settings)
        .subscription(Dialog::subscription)
        .run_with(move || (dialog, Task::none()))?;

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
    fn set_outcome_and_close_window<M: Send + 'static>(
        &mut self,
        outcome: Option<Outcome>,
    ) -> Task<M> {
        *unsafe { self.outcome.as_mut() } = outcome;
        window::get_latest().and_then(window::close)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
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
                        basic::window_size(self.predefined_script_labels.len()),
                    ),
                };

                self.mode = new_mode;

                window::get_latest().and_then(move |id| window::resize(id, new_size))
                // TODO: reposition window (iced currently does not support it)
            }
            Message::Dismiss => self.set_outcome_and_close_window(None),
        }
    }

    fn view(&self) -> iced::Element<'_, Message, Theme, iced::Renderer> {
        match self.mode {
            Mode::Basic => basic::view(self),
            Mode::Advanced => advanced::view(self),
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with(|event, _status, _window| {
            if let Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) = event
            {
                return Some(Message::Dismiss);
            }
            None
        })
    }
}
