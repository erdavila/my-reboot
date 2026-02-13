use iced::widget::{button, column, horizontal_space, row, toggler};
use iced::{Length, Theme, alignment};

use super::{Dialog, Outcome};

const WINDOW_WIDTH: u16 = 340;
const PADDING: u16 = 12;
const BUTTON_HEIGHT: u16 = 32;

pub(crate) fn window_size(label_count: u32) -> (u32, u32) {
    (
        WINDOW_WIDTH as u32,
        (BUTTON_HEIGHT + 1) as u32 * label_count + 23 + 34,
    )
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Message {
    Confirm(usize),
}

pub(crate) fn update(dialog: &mut Dialog, message: Message) -> iced::Command<Message> {
    match message {
        Message::Confirm(index) => {
            dialog.set_outcome_and_close_window(Some(Outcome::PredefinedScriptIndex(index)))
        }
    }
}

pub(crate) fn view(dialog: &Dialog) -> iced::Element<'_, super::Message, iced::Renderer<Theme>> {
    let buttons = dialog
        .predefined_script_labels
        .iter()
        .enumerate()
        .fold(column![], |column, (index, label)| {
            let button = button(*label)
                .on_press(super::Message::BasicDialog(Message::Confirm(index)))
                .height(BUTTON_HEIGHT)
                .width(WINDOW_WIDTH - 2 * PADDING);
            column.push(button)
        })
        .spacing(1);

    column![
        buttons,
        row![horizontal_space(Length::Fill), mode_toggler!(false),],
    ]
    .spacing(16)
    .padding(PADDING)
    .into()
}
