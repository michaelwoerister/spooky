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
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ffi::{OsStr, OsString};

pub fn mk_remote_cache_dir(src_host_id: &str,
                           dst_host_name: &OsStr)
                           -> Result<(), Error>
{
    let mut command = Command::new("ssh");
    command.arg(dst_host_name);
    command.arg(mk_remote_dir(src_host_id));

    println!("Executing {:?}", command);

    let status = try!(command.status());
    if status.success() {
        Ok(())
    } else {
        err(format!("mk_remote_cache_dir failed with {}", status))
    }
}

fn mk_remote_dir(src_host_id: &str) -> OsString {
    let mut arg = OsString::new();
    arg.push("mkdir -p ~/");
    arg.push(common::SPOOKY_DIR);
    arg.push("/");
    arg.push(src_host_id);
    arg
}

pub fn exec_remote_commands(src_directory: &Path,
                            src_host_id: &str,
                            dst_host_name: &OsStr,
                            remote_command: &OsStr)
                            -> Result<(), Error>
{
    let mut command = Command::new("ssh");
    command.arg("-tt");
    command.arg(dst_host_name);

    let mut actual_remote_command = OsString::new();
    actual_remote_command.push("cd ");
    actual_remote_command.push(remote_working_dir(src_directory, src_host_id));
    actual_remote_command.push(" && ");
    actual_remote_command.push(remote_command);

    command.arg(actual_remote_command);

    println!("Executing {:?}", command);

    let status = try!(command.status());
    if status.success() {
        Ok(())
    } else {
        err(format!("exec_remote_commands failed with {}", status))
    }
}

fn remote_working_dir(src_directory: &Path, src_host_id: &str) -> PathBuf {
    let mut dir = PathBuf::from("~");

    dir.push(common::SPOOKY_DIR);
    dir.push(src_host_id);
    dir.push(&src_directory.strip_prefix(src_directory.parent().unwrap()).unwrap());

    dir
}
