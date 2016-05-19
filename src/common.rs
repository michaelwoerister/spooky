// Copyright (c) 2016 Michael Woerister
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::env;
use std::fs;
use std::io;
use std::path::{PathBuf};

pub const SPOOKY_DIR: &'static str = ".spooky";

pub struct Error(pub String);

pub fn err<T>(message: String) -> Result<T, Error> {
    Err(Error(message))
}

impl ::std::convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error(format!("{}", error))
    }
}

pub fn config_dir() -> Result<PathBuf, Error>
{
    let home_dir = try!{
        env::home_dir()
            .ok_or(Error(format!("Could not find home directory")))
    };
    let mut p = home_dir.to_path_buf();
    p.push(SPOOKY_DIR);
    create_dir_if_necessary(p)
}

pub fn create_dir_if_necessary(path: PathBuf) -> Result<PathBuf, Error> {
    if path.exists() {
        if !path.is_dir() {
            return err(format!("Invalid local-locks directory: {}", path.display()));
        }
    } else {
        try!(fs::create_dir_all(&path));
    }

    Ok(path)
}
