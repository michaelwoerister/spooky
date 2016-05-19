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
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

pub fn src_directory_id(source_directory: &Path) -> Result<String, Error>
{
    use std::hash::{Hash, SipHasher, Hasher};

    let canonical = try!(source_directory.canonicalize());

    let string = canonical.as_os_str().to_os_string();
    let mut hash_state = SipHasher::new();
    string.hash(&mut hash_state);
    let hash = hash_state.finish();
    Ok(format!("{}", hash))
}

pub fn src_host_id() -> Result<String, Error> {
    let config_dir = try!(common::config_dir());
    let host_id_file_path = config_dir.join("hostid");

    let host_id = if host_id_file_path.exists() {
        if !host_id_file_path.is_file() {
            return err(format!("Invalid host-id file: {}", host_id_file_path.display()));
        }

        let mut file_contents = String::new();
        let mut file = try!(File::open(host_id_file_path));
        let _ = try!(file.read_to_string(&mut file_contents));

        if file_contents.len() != src_host_id_length() {
            return err(format!("Invalid host-id file contents: '{}'", file_contents));
        }

        file_contents
    } else {
        let mut file = try!(File::create(host_id_file_path));
        let host_id = generate_new_id();
        try!(file.write_all(host_id.as_bytes()));
        host_id
    };

    return Ok(host_id);

    fn src_host_id_length() -> usize {
        generate_new_id().len()
    }

    fn generate_new_id() -> String {
        format!("{}", Uuid::new_v4().simple())
    }
}
