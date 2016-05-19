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

use common::{self, Error, err};
use identifiers;
use std::fs;
use std::path::{Path, PathBuf};

pub struct LocalLock {
    lock_path: PathBuf,
    display_name: PathBuf,
}

fn local_locks_dir() -> Result<PathBuf, Error>
{
    let config_dir = try!(common::config_dir());
    let mut p = config_dir;
    p.push("local-locks");
    common::create_dir_if_necessary(p)
}

impl LocalLock {

    #[must_use]
    pub fn acquire(source_directory: &Path) -> Result<LocalLock, Error>
    {
        let canonical = try!(source_directory.canonicalize());
        print!("Attempting to acquire local lock for '{}' ... ", canonical.display());

        let id = try!(identifiers::src_directory_id(source_directory));

        let local_locks_dir = try!(local_locks_dir());
        let local_locks_dir = try!(local_locks_dir.canonicalize());
        let lock_path = local_locks_dir.join(Path::new(&id[..]));

        match fs::create_dir(&lock_path) {
            Ok(_) => {
                println!("OK");
                Ok(LocalLock {
                    lock_path: lock_path,
                    display_name: canonical,
                })
            }
            Err(_) => {
                println!("FAILED");
                err(format!("Could not acquire local lock '{}'!", id))
            }
        }
    }
}

impl Drop for LocalLock {
    fn drop(&mut self) {
        print!("Attempting to release local lock for '{}' ... ", self.display_name.display());

        match fs::remove_dir(&self.lock_path) {
            Ok(_) => println!("OK"),
            Err(_) => println!("FAILED"),
        };
    }
}
