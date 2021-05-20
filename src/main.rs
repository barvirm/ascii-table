use ansi_term::Style;
use term_size::dimensions;
use clap::{App, Arg, crate_authors, crate_description, crate_version, value_t};

const CHARS: [&str; 128] = [
        "NUL '\\0' (null character)",
        "SOH (start of heading)",
        "STX (start of text)",
        "ETX (end of text)",
        "EOT (end of transmission)",
        "ENQ (enquiry)",
        "ACK (acknowledge)",
        "BEL '\\a' (bell)",
        "BS  '\\b' (backspace)",
        "HT  '\\t' (horizontal tab)",
        "LF  '\\n' (new line)",
        "VT  '\\v' (vertical tab)",
        "FF  '\\f' (form feed)",
        "CR  '\\r' (carriage ret)",
        "SO  (shift out)",
        "SI  (shift in)",
        "DLE (data link escape)",
        "DC1 (device control 1)",
        "DC2 (device control 2)",
        "DC3 (device control 3)",
        "DC4 (device control 4)",
        "NAK (negative ack.)",
        "SYN (synchronous idle)",
        "ETB (end of trans. blk)",
        "CAN (cancel)",
        "EM  (end of medium)",
        "SUB (substitute)",
        "ESC (escape)",
        "FS  (file separator)",
        "GS  (group separator)",
        "RS  (record separator)",
        "US  (unit separator)",
        "SPACE",
        "!",
        "\"",
        "#",
        "$",
        "%",
        "&",
        "'",
        "(",
        ")",
        "*",
        "+",
        ",",
        "-",
        ".",
        "/",
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        ":",
        ";",
        "<",
        "=",
        ">",
        "?",
        "@",
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
        "L",
        "M",
        "N",
        "O",
        "P",
        "Q",
        "R",
        "S",
        "T",
        "U",
        "V",
        "W",
        "X",
        "Y",
        "Z",
        "[",
        "\\  '\\\\'",
        "]",
        "^",
        "_",
        "`",
        "a",
        "b",
        "c",
        "d",
        "e",
        "f",
        "g",
        "h",
        "i",
        "j",
        "k",
        "l",
        "m",
        "n",
        "o",
        "p",
        "q",
        "r",
        "s",
        "t",
        "u",
        "v",
        "w",
        "x",
        "y",
        "z",
        "{",
        "|",
        "}",
        "~",
        "DEL",
    ];

struct Args {
    style: bool,
    colums: usize,
}

fn parse_args() -> Args {
    let matches = App::new("ASCII TABLE")
    .version(crate_version!())
    .author(crate_authors!("\n"))
    .about(crate_description!())
    .args(&[
        Arg::with_name("limit")
            .short("l")
            .long("limit")
            .value_name("COLUMS")
            .takes_value(true)
            .help("Upper limit for colums"),
        Arg::with_name("no-style")
            .short("n")
            .long("no-style")
            .takes_value(false)
            .help("Without ANSI tags")
    ])
    .get_matches();

    let (width, _) = dimensions().unwrap_or((50, 10));
    let columns = value_t!(matches.value_of("limit"), usize).unwrap_or(width / 50);
    Args {
        style: !matches.is_present("no-style"),
        colums: columns
    }
}

fn table_title(styled: bool, colums: usize) -> String {
    let title = format!("{:6} {:6} {:6} {:28}", "Oct", "Dec", "Hex", "Char").repeat(colums);
    if styled {
        return Style::new().underline().paint(&title).to_string();
    }
    title
}

fn ceil_usize_div(num: usize, den: usize) -> usize {
    (num + den - 1) / den
}

fn print_table(styled_header: bool, colums: usize) {
    let char_per_column = ceil_usize_div(CHARS.len(), colums);

    println!("{}", table_title(styled_header, colums));
    for char in 0..char_per_column {
        let mut row = String::with_capacity(colums * 50);
        for column in 0..colums {
            let i = char + (char_per_column * column);
            if i < CHARS.len() {
                let oct = format!("{:03o}", i);
                let dec = format!("{}", i);
                let hex = format!("{:02x}", i);
                row.push_str(&format!("{:6} {:6} {:6} {:28}", oct, dec, hex, CHARS[i]));
            }
        }
        println!("{}", row);
    }
}

fn main() {
    let args = parse_args();
    print_table(args.style, args.colums)
}
