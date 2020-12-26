// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use profile::{self, OperationSupport, OperationSupportLevel, Profile};
use sandbox::{ChildSandboxMethods, Command, SandboxMethods};
use std::io;

pub mod process;

impl OperationSupport for profile::Operation {
    fn support(&self) -> OperationSupportLevel {
        OperationSupportLevel::AlwaysAllowed
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Operation;

pub struct Sandbox {
    profile: Profile,
}

impl Sandbox {
    pub fn new(profile: Profile) -> Sandbox {
        Sandbox { profile }
    }
}

impl SandboxMethods for Sandbox {
    fn profile(&self) -> &Profile {
        &self.profile
    }

    fn start(&self, _: &mut Command) -> io::Result<process::Process> {
        unimplemented!()
    }
}

pub struct ChildSandbox {
}

impl ChildSandbox {
    pub fn new(_: Profile) -> ChildSandbox {
        ChildSandbox {
        }
    }
}

impl ChildSandboxMethods for ChildSandbox {
    fn activate(&self) -> Result<(),()> {
        Err(())
    }
}
