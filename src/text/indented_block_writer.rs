use std::fmt::{self, Arguments, Display};
use std::io;

pub(crate) struct IndentedBlockWriter<T> {
    inner: T,
    indent: usize,
}
impl<T: WriteFmt> IndentedBlockWriter<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self { inner, indent: 0 }
    }

    pub(crate) fn write_block(
        &mut self,
        header: impl Display,
        block: impl FnOnce(&mut Self) -> Result<(), T::Error>,
    ) -> Result<(), T::Error> {
        self.write_indent()?;
        writeln!(self.inner, "{header}")?;

        self.indent += 1;
        block(self)?;
        self.indent -= 1;

        Ok(())
    }

    pub(crate) fn write(&mut self, line: impl Display) -> Result<(), T::Error> {
        self.write_block(line, |_| Ok(()))
    }

    fn write_indent(&mut self) -> Result<(), T::Error> {
        for _ in 0..self.indent {
            write!(self.inner, "  ")?;
        }
        Ok(())
    }
}
impl<T: io::Write> From<T> for IndentedBlockWriter<IoWrite<T>> {
    fn from(value: T) -> Self {
        IndentedBlockWriter::new(IoWrite(value))
    }
}
impl<T: fmt::Write> From<T> for IndentedBlockWriter<FmtWrite<T>> {
    fn from(value: T) -> Self {
        IndentedBlockWriter::new(FmtWrite(value))
    }
}

pub(crate) trait WriteFmt {
    type Error;

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Self::Error>;
}

pub(crate) struct IoWrite<T>(T);
impl<T: io::Write> WriteFmt for IoWrite<T> {
    type Error = io::Error;

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Self::Error> {
        self.0.write_fmt(args)
    }
}

pub(crate) struct FmtWrite<T>(T);
impl<T: fmt::Write> WriteFmt for FmtWrite<T> {
    type Error = fmt::Error;

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Self::Error> {
        self.0.write_fmt(args)
    }
}
