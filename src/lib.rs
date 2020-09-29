//! Combine a reader + writer into a duplex of Read + Write.
//!
//! This is useful when you need a reader + writer, but it doesn't come neatly
//! pre-packaged. This allows wiring it together with minimal effort.
//! See also: [`io::empty`], [`io::sink`].
//!
//! [`io::empty`]: https://docs.rs/async-std/1.1.0/async_std/io/fn.empty.html
//! [`io::sink`]: https://docs.rs/async-std/1.1.0/async_std/io/fn.sink.html
//!
//! # Examples
//!
//! Read a line from stdin, and write it to stdout. All from the same `stdio`
//! object:
//!
//! ```no_run
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! use async_std::io::{self, BufReader, prelude::*};
//! use duplexify::Duplex;
//!
//! // Create a reader and writer, and merge them into a single "duplex".
//! let stdin = BufReader::new(io::stdin());
//! let stdout = io::stdout();
//! let mut stdio = Duplex::new(stdin, stdout);
//!
//! // We can now read + write from and to the duplex.
//! let mut line = String::new();
//! stdio.read_line(&mut line).await?;
//! stdio.write_all(&line.as_bytes()).await?;
//! #
//! # Ok(()) }) }
//! ```

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(
    missing_docs,
    missing_doc_code_examples,
    unreachable_pub,
    rust_2018_idioms
)]

use async_std::io::{self, BufRead, Read, Write};
use async_std::task::{Context, Poll};
use std::pin::Pin;

pin_project_lite::pin_project! {
    /// Combine a reader + writer into a duplex of `Read` + `Write`.
    #[derive(Debug)]
    pub struct Duplex<R, W> {
        #[pin]
        reader: R,
        #[pin]
        writer: W,
    }
}

impl<R, W> Duplex<R, W> {
    /// Create a new instance.
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    /// Decomposes a duplex into its components.
    pub fn into_inner(self) -> (R, W) {
        (self.reader, self.writer)
    }
}

impl<R: Read, W> Read for Duplex<R, W> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        this.reader.poll_read(cx, buf)
    }
}

impl<R, W: Write> Write for Duplex<R, W> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        this.writer.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.project();
        this.writer.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.project();
        this.writer.poll_close(cx)
    }
}

impl<R: BufRead, W> BufRead for Duplex<R, W> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let this = self.project();
        this.reader.poll_fill_buf(cx)
    }
    fn consume(self: Pin<&mut Self>, amt: usize) {
        let this = self.project();
        this.reader.consume(amt)
    }
}

impl<R, W> Clone for Duplex<R, W>
where
    R: Clone,
    W: Clone,
{
    fn clone(&self) -> Self {
        Self {
            reader: self.reader.clone(),
            writer: self.writer.clone(),
        }
    }
}
