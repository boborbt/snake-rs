use std::io::{Read};
use termion::AsyncReader;
use std::thread;
use std::time::Duration;

pub(crate) fn wait_char(reader: &mut AsyncReader) -> u8 {
    loop {
        let mut buf = [0; 1];
        if reader.read(&mut buf).unwrap() == 1 {
            return buf[0];
        }

        thread::sleep(Duration::from_millis(100));
    }
}