#![expect(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    non_camel_case_types,
    non_snake_case
)]

pub mod common;
pub mod get_device_info;
pub mod output_format;
pub mod win_api;
