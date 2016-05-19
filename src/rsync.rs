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

use ssh;
use common::{self, Error, err};
use std::path::Path;
use std::process::Command;
use std::ffi::{OsStr, OsString};

pub fn run(src_directory: &Path, src_host_id: &str, dst_host_name: &OsStr) -> Result<(), Error>
{
    try!(ssh::mk_remote_cache_dir(src_host_id, dst_host_name));

    let mut command = Command::new("rsync");
    command.arg("-azv")
           .arg("-e ssh")
           .arg("--delete")
           .arg(src_directory)
           .arg(dst_spec(dst_host_name, src_host_id));

    if let Some(exclude_from_arg) = exclude_from_arg(src_directory) {
        command.arg(exclude_from_arg);
    }

    println!("Executing {:?}", command);

    let status = try!(command.status());
    if status.success() {
        Ok(())
    } else {
        err(format!("rsync failed with {}", status))
    }
}

fn exclude_from_arg(src_directory: &Path) -> Option<OsString>
{
    let exclusion_file = src_directory.join(".spooky-rsync-filter");

    if !exclusion_file.exists() {
        return None;
    }

    let mut arg = OsString::new();
    arg.push("--exclude-from='");
    arg.push(exclusion_file);
    arg.push("'");
    Some(arg)
}

fn dst_spec(dst_host_name: &OsStr, src_host_id: &str) -> OsString
{
    let mut arg = OsString::new();
    arg.push(dst_host_name);
    arg.push(":~/");
    arg.push(common::SPOOKY_DIR);
    arg.push("/");
    arg.push(src_host_id);
    arg
}
