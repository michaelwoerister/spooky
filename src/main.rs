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

use std::path::Path;

use lock::LocalLock;
use common::{Error, err};

extern crate uuid;
extern crate clap;
extern crate crc;

mod common;
mod identifiers;
mod lock;
mod rsync;
mod ssh;

fn run<F: FnOnce() -> Result<(), Error>>(f: F) {
    if let Err(Error(message)) = f() {
        println!("Error: {}", message);
    }
}

fn clap_app<'a, 'b>() -> clap::App<'a, 'b> {
    use clap::{Arg, App};

    App::new("spooky")
        .version("0.1")
        .author("Michael Woerister (michaelwoerister@posteo.net)")
        .about("👻 spooky action at a distance 👻")
        .arg(Arg::with_name("source")
            .short("s")
            .long("source")
            .value_name("DIR")
            .help("... help ...")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("target")
            .short("d")
            .long("target")
            .value_name("HOST")
            .help("... help ...")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .value_name("HOST")
            .help("... help ...")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("remote command")
            .short("c")
            .long("command")
            .value_name("REMOTE COMMAND")
            .multiple(false)
            .takes_value(true)
            .required(true))
}

fn main()
{
    run(|| {
        let arg_matches = clap_app().get_matches();

        let verbose = arg_matches.is_present("verbose");

        let src_directory = arg_matches.value_of_os("source").unwrap();

        if verbose {
            println!("Source directory is: {}", src_directory.to_string_lossy());
        }

        let dst_host_name = arg_matches.value_of_os("target").unwrap();

        if verbose {
            println!("Destination host name is: {}", dst_host_name.to_string_lossy());
        }

        let remote_command = arg_matches.value_of_os("remote command")
                                         .unwrap();
        if verbose {
            println!("Remote command is: {}", remote_command.to_string_lossy());
        }
        let src_host_id = try!(identifiers::src_host_id());
        let src_host_id = &src_host_id[..];

        {
            let src_directory = Path::new(src_directory);

            let _local_lock = try!(LocalLock::acquire(src_directory));

            try!(rsync::run(&Path::new(src_directory),
                            src_host_id,
                            dst_host_name));

            try!(ssh::exec_remote_command(src_directory,
                                          src_host_id,
                                          dst_host_name,
                                          remote_command));
        }
        Ok(())
    })
}
