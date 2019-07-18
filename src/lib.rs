use std::io::{Result, Write};
use std::iter::repeat;

const BEGIN_LINE: &[u8] = b"| ";
const END_LINE: &[u8] = b"\n";
const FERRIS: &[u8] = br#"
              \
               \
                  _~^~^~_
              \) /  o o  \ (/
                '_   -   _'
                / '-----' \
"#;
const NEWLINE: u8 = '\n' as u8;
const SPACE: u8 = ' ' as u8;
const DASH: u8 = '-' as u8;

const OFFSET: usize = 2;
const WIDTH: usize = 40;

pub fn crabsay<W>(input: &[u8], writer: &mut W) -> Result<()>
where
    W: Write,
{
    let mut write_buffer = Vec::<u8>::new();

    let bar_buffer: Vec<u8> = repeat(DASH).take(WIDTH + OFFSET).collect();

    write_buffer.extend_from_slice(&bar_buffer);
    write_buffer.push(NEWLINE);
    for i in input.split(|x| *x == '\n' as u8) {
        for j in i.chunks(WIDTH) {
            write_buffer.extend_from_slice(BEGIN_LINE);
            write_buffer.extend_from_slice(j);
            for _ in 0..(WIDTH - j.len()) {
                write_buffer.push(SPACE);
            }
            write_buffer.extend_from_slice(END_LINE);
        }
    }
    write_buffer.extend_from_slice(&bar_buffer);
    write_buffer.extend_from_slice(FERRIS);

    writer.write_all(&write_buffer)?;

    Ok(())
}
