extern crate crabsay;

use crabsay::*;
use std::env;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, BufWriter};

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = stdin();
    let stdout = stdout();

    if args.len() > 1 {
        let message = args[1..].join(" ");
        let mut writer = BufWriter::new(stdout.lock());
        crabsay(message.as_bytes(), &mut writer);
    } else {
        let mut buff = BufReader::new(stdin.lock());
        let message = buff.fill_buf().unwrap();
        let mut writer = BufWriter::new(stdout.lock());
        crabsay(&message, &mut writer);
    }
}
