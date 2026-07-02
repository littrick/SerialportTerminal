use std::{marker::PhantomData, pin::Pin, sync::Arc};
use tokio::io::{AsyncRead, AsyncWrite, Stdin, Stdout, stdin, stdout};

pub struct Terminal {
    _marker: PhantomData<()>,
}

impl Terminal {
    pub fn new() -> Self {
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");

        Terminal {
            _marker: PhantomData,
        }
    }

    pub fn split(self) -> (impl AsyncRead, impl AsyncWrite) {
        let terminal = Arc::new(self);
        let reader = TerminalReader {
            stdin: stdin(),
            _terminal: terminal.clone(),
        };

        let writer = TerminalWriter {
            stdout: stdout(),
            _terminal: terminal,
        };
        (reader, writer)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}

struct TerminalReader {
    stdin: Stdin,
    _terminal: Arc<Terminal>,
}

impl AsyncRead for TerminalReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        AsyncRead::poll_read(Pin::new(&mut self.get_mut().stdin), cx, buf)
    }
}

struct TerminalWriter {
    stdout: Stdout,
    _terminal: Arc<Terminal>,
}

impl AsyncWrite for TerminalWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        AsyncWrite::poll_write(Pin::new(&mut self.get_mut().stdout), cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        AsyncWrite::poll_flush(Pin::new(&mut self.get_mut().stdout), cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut self.get_mut().stdout), cx)
    }
}
