use std::fmt::Debug;
use std::iter::Peekable;

use anyhow::Result;
use serde::Serialize;

pub enum OutputFormat {
    Debug,
    JSON,
}

impl OutputFormat {
    pub fn from_args(args: &mut Peekable<impl Iterator<Item = impl AsRef<str>>>) -> Option<Self> {
        args.next_if_map(|arg| match arg.as_ref() {
            "--json" => Ok(Self::JSON),
            "--debug" => Ok(Self::Debug),
            _ => Err(arg),
        })
    }

    pub fn write<T>(&self, value: &T) -> Result<()>
    where
        T: Debug + Serialize,
    {
        match self {
            OutputFormat::Debug => println!("{value:#?}"),
            OutputFormat::JSON => serde_json::to_writer_pretty(std::io::stdout(), value)?,
        }
        Ok(())
    }
}
