// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Child process management on POSIX systems.

use sandbox::Command;
use std::io;

pub fn exec(_: &Command) -> io::Error {
    unimplemented!()
}

pub fn spawn(_: &Command) -> io::Result<Process> {
    unimplemented!()
}

#[allow(missing_copy_implementations)]
pub struct Process {
}

impl Process {
    pub fn wait(&self) -> io::Result<ExitStatus> {
        unimplemented!()
    }
}

pub enum ExitStatus {
    Code(i32),
    Signal(i32),
}

impl ExitStatus {
    #[inline]
    pub fn success(&self) -> bool {
        match *self {
            ExitStatus::Code(0) => true,
            _ => false,
        }
    }
}
