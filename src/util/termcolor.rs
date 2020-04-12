use std::io::{stderr, stdout, Result, Write};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

pub type Buffer = Vec<u8>;

pub struct BufferWriter {
    use_stderr: bool,
}

impl BufferWriter {
    pub fn buffer(&self) -> Buffer {
        vec![]
    }

    pub fn stderr(_: ColorChoice) -> Self {
        Self { use_stderr: true }
    }

    pub fn stdout(_: ColorChoice) -> Self {
        Self { use_stderr: false }
    }

    pub fn print(&self, buf: &Buffer) -> Result<()> {
        if self.use_stderr {
            stderr().lock().write(buf)?;
        } else {
            stdout().lock().write(buf)?;
        }

        Ok(())
    }
}
