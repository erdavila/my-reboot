use iced::widget::{button, column, row, space};
use iced::{Fill, Size, Task, Theme};

use super::{Dialog, Outcome};

const WINDOW_WIDTH: f32 = 350.0;
const PADDING: f32 = 12.0;
const BUTTON_HEIGHT: f32 = 32.0;
#[cfg(windows)]
const ADDITIONAL_WINDOW_HEIGHT: f32 = 60.0;
#[cfg(not(windows))]
const ADDITIONAL_WINDOW_HEIGHT: f32 = 94.0;

pub(crate) fn window_size(label_count: usize) -> Size {
    Size {
        width: WINDOW_WIDTH,
        #[expect(clippy::cast_precision_loss)]
        height: BUTTON_HEIGHT + 1.0 * label_count as f32 + ADDITIONAL_WINDOW_HEIGHT,
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Message {
    Confirm(usize),
}

pub(crate) fn update(dialog: &mut Dialog, message: Message) -> Task<Message> {
    match message {
        Message::Confirm(index) => {
            dialog.set_outcome_and_close_window(Some(Outcome::PredefinedScriptIndex(index)))
        }
    }
}

pub(crate) fn view(dialog: &Dialog) -> iced::Element<'_, super::Message, Theme, iced::Renderer> {
    let buttons = dialog
        .predefined_script_labels
        .iter()
        .enumerate()
        .fold(column![], |column, (index, label)| {
            let button = button(*label)
                .on_press(super::Message::BasicDialog(Message::Confirm(index)))
                .height(BUTTON_HEIGHT)
                .width(WINDOW_WIDTH - 2.0 * PADDING);
            column.push(button)
        })
        .spacing(1);

    column![buttons, row![space().width(Fill), mode_toggler!(false),],]
        .spacing(16)
        .padding(PADDING)
        .into()
}
