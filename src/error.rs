// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// This is the standard error type used throughout this crate
/// Implementation based on suggestions in http://blog.burntsushi.net/rust-error-handling

use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum VrError {
    Io(String),
    AlreadyExists,
    BadEncoding(&'static str),
    Eof,
    Timeout
}

impl Error for VrError {
    fn description(&self) -> &str {
        match *self {
            VrError::Io(ref string) => string,
            VrError::AlreadyExists => "resource already exists",
            VrError::BadEncoding(_)  => "could not read encoded data",
            VrError::Eof => "end of file",
            VrError::Timeout => "timeout"
        }
    }
}

impl fmt::Display for VrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VrError::Io(ref string) => write!(f, "IO error: {}", string),
            VrError::AlreadyExists => write!(f, "Error: resource already exists"),
            VrError::BadEncoding(encoding_type) => write!(f, "Could not read data encoded with {}",
                                                          encoding_type),
            VrError::Eof => write!(f, "Error: End Of File"),
            VrError::Timeout => write!(f, "Error: Timeout")
        }
    }
}

/// Need to implement from so we can use try!
impl From<io::Error> for VrError {
    fn from(err: io::Error) -> VrError {
        let s = format!("{}", err);
        VrError::Io(s)
    }
}
