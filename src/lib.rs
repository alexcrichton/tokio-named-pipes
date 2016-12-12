#![cfg(windows)]

extern crate tokio_core;
extern crate mio_named_pipes;
extern crate futures;

use std::ffi::OsStr;
use std::fmt;
use std::io::{self, Read, Write};
use std::os::windows::io::*;

use futures::Async;
use tokio_core::io::Io;
use tokio_core::reactor::{PollEvented, Handle};

pub struct NamedPipe {
    io: PollEvented<mio_named_pipes::NamedPipe>,
}

impl NamedPipe {
    pub fn new<P: AsRef<OsStr>>(p: P, handle: &Handle) -> io::Result<NamedPipe> {
        NamedPipe::_new(p.as_ref(), handle)
    }

    fn _new(p: &OsStr, handle: &Handle) -> io::Result<NamedPipe> {
        let inner = try!(mio_named_pipes::NamedPipe::new(p));
        NamedPipe::from_pipe(inner, handle)
    }

    pub fn from_pipe(pipe: mio_named_pipes::NamedPipe,
                     handle: &Handle)
                     -> io::Result<NamedPipe> {
        Ok(NamedPipe {
            io: try!(PollEvented::new(pipe, handle)),
        })
    }

    pub fn connect(&self) -> io::Result<()> {
        self.io.get_ref().connect()
    }

    pub fn disconnect(&self) -> io::Result<()> {
        self.io.get_ref().disconnect()
    }

    pub fn poll_read(&self) -> Async<()> {
        self.io.poll_read()
    }

    pub fn poll_write(&self) -> Async<()> {
        self.io.poll_write()
    }
}

impl Read for NamedPipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.io.read(buf)
    }
}

impl Write for NamedPipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.io.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

impl Io for NamedPipe {
    fn poll_read(&mut self) -> Async<()> {
        <NamedPipe>::poll_read(self)
    }

    fn poll_write(&mut self) -> Async<()> {
        <NamedPipe>::poll_write(self)
    }
}

impl<'a> Read for &'a NamedPipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.io).read(buf)
    }
}

impl<'a> Write for &'a NamedPipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&self.io).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&self.io).flush()
    }
}

impl<'a> Io for &'a NamedPipe {
    fn poll_read(&mut self) -> Async<()> {
        <NamedPipe>::poll_read(self)
    }

    fn poll_write(&mut self) -> Async<()> {
        <NamedPipe>::poll_write(self)
    }
}

impl fmt::Debug for NamedPipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.io.get_ref().fmt(f)
    }
}

impl AsRawHandle for NamedPipe {
    fn as_raw_handle(&self) -> RawHandle {
        self.io.get_ref().as_raw_handle()
    }
}
