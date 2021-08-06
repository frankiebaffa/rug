extern crate htmlbuilder;
extern crate clap;
use clap::Clap;
use std::fs::File;
use std::io::Read;
#[derive(PartialEq)]
enum ParsePos {
    TagName,
    Id,
    Class,
    AttrKey,
    AttrValOpen,
    AttrVal,
    AttrValClose,
    Text,
}
#[derive(Clap)]
#[clap(version = "0.1", author = "Frankie Baffa <frankiebaffa@gmail.com>")]
struct Opts {
    /// Sets an input file
    #[clap(short, long)]
    in_file: Option<String>,
    /// Sets an output file
    #[clap(short, long)]
    out_file: Option<String>,
}
fn main() {
    let opts: Opts = Clap::parse();
    if opts.in_file.is_none() {
        println!("Must include input file");
        return;
    } else if opts.out_file.is_none() {
        println!("Must include output file");
        return;
    }
    let in_file = opts.in_file.unwrap();
    let out_file = opts.out_file.unwrap();
    let mut file = match File::open(in_file) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open input file. File may not exist");
            return;
        },
    };
    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to read input file.");
            return;
        },
    }
    for mut line in file_string.lines() {
        if line.len() == 0 {
            continue;
        }
        let mut dent = 0;
        while line[0..1].eq("\t") {
            dent = dent + 1;
            line = &line[1..];
        }
        let mut name: String = String::new();
        let mut id: String = String::new();
        let mut curr_class: String = String::new();
        let mut classes: Vec<String> = Vec::new();
        let mut curr_key: String = String::new();
        let mut curr_val: String = String::new();
        let mut attributes: Vec<(String, String)> = Vec::new();
        let mut text: String = String::new();
        let mut parse_pos: ParsePos = ParsePos::TagName;
        for c in line.chars() {
            match c {
                '#' => {
                    if !id.is_empty() {
                        println!("Parse error. Tag '{}' already has an id", name);
                        return;
                    }
                    match parse_pos {
                        ParsePos::TagName => {
                            if name.is_empty() {
                                name.push_str("div");
                            }
                            parse_pos = ParsePos::Id;
                        },
                        ParsePos::Id => {
                            println!("Parse error. A '#' cannot follow a '#' in the header line of an element");
                            return;
                        },
                        ParsePos::Class => {
                            if curr_class.is_empty() {
                                println!("Parse error. Tag '{}' cannot have a blank class", name);
                                return;
                            } else {
                                classes.push(curr_class);
                                curr_class = String::new();
                            }
                        },
                        ParsePos::AttrKey => {
                            println!("Parse error. A '#' cannot be located within the key of an element attribute");
                            return;
                        },
                        ParsePos::AttrValOpen => {
                            println!("Parse error. A '#' cannot be located after the '=' after the attribute key");
                            return;
                        },
                        ParsePos::AttrVal => {
                            curr_val.push_str(&c.to_string());
                        },
                        ParsePos::AttrValClose => {
                            println!("Parse error. A '#' cannot be located within the attribute enclosure");
                            return;
                        },
                        ParsePos::Text => {
                            text.push_str(&c.to_string());
                        },
                    }
                },
                '.' => {
                    match parse_pos {
                        ParsePos::TagName => {
                            if name.is_empty() {
                                name.push_str("div");
                            }
                            parse_pos = ParsePos::Class;
                        },
                        ParsePos::Id => {
                            if id.is_empty() {
                                println!("Parse error. A '.' cannot be located within the name of an element id");
                                return;
                            }
                            parse_pos = ParsePos::Class;
                        },
                        ParsePos::Class => {
                            if curr_class.is_empty() {
                                println!("Parse error. A '.' cannot be located within the name of an element class");
                                return;
                            }
                            classes.push(curr_class);
                            curr_class = String::new();
                        },
                        ParsePos::AttrKey => {
                            println!("Parse error. A '.' cannot be located within the attribute enclosure");
                            return;
                        },
                        ParsePos::AttrValOpen => {
                            println!("Parse error. The next character may only be \". '{}' not allowed", c.to_string());
                            return;
                        },
                        ParsePos::AttrVal => {
                            curr_val.push_str(&c.to_string());
                        },
                        ParsePos::AttrValClose => {
                            println!("Parse error. The only valid characters after the closing of an attribute value are ',' and ')'");
                            return;
                        },
                        ParsePos::Text => {
                            text.push_str(&c.to_string());
                        },
                    }
                },
                '(' => {
                    match parse_pos {
                        ParsePos::TagName => {
                            if !name.is_empty() {
                                name.push_str("div");
                            }
                            parse_pos = ParsePos::AttrKey;
                        },
                        ParsePos::Id => {
                            if id.is_empty() {
                                println!("Parse error. '(' is an invalid character for an element id");
                                return;
                            }
                            parse_pos = ParsePos::AttrKey;
                        },
                        ParsePos::Class => {
                            if curr_class.is_empty() {
                                println!("Parse error. '(' is an invalid character for an element class");
                                return;
                            }
                            parse_pos = ParsePos::AttrKey;
                            classes.push(curr_class);
                            curr_class = String::new();
                        },
                        ParsePos::AttrKey => {
                            println!("Parse error. '(' is an invalid character for an element attribute key");
                            return;
                        },
                        ParsePos::AttrValOpen => {
                            println!("Parse error. '(' is an invalid character within the attribute enclosure");
                            return;
                        },
                        ParsePos::AttrVal => {
                            curr_val.push_str(&c.to_string());
                        },
                        ParsePos::AttrValClose => {
                            println!("Parse error. ',' and ')' are the only valid characters following an element attribute value closure");
                            return;
                        },
                        ParsePos::Text => {
                            text.push_str(&c.to_string());
                        },
                    }
                },
                ')' => {
                    match parse_pos {
                        ParsePos::TagName => {

                        },
                        ParsePos::Id => {

                        },
                        ParsePos::Class => {

                        },
                        ParsePos::AttrKey => {

                        },
                        ParsePos::AttrValOpen => {

                        },
                        ParsePos::AttrVal => {

                        },
                        ParsePos::AttrValClose => {

                        },
                        ParsePos::Text => {

                        },
                    }
                },
                ' ' => {
                    match parse_pos {
                        ParsePos::TagName => {

                        },
                        ParsePos::Id => {

                        },
                        ParsePos::Class => {

                        },
                        ParsePos::AttrKey => {

                        },
                        ParsePos::AttrValOpen => {

                        },
                        ParsePos::AttrVal => {

                        },
                        ParsePos::AttrValClose => {

                        },
                        ParsePos::Text => {

                        },
                    }
                },
                _ => {
                    match parse_pos {
                        ParsePos::TagName => {
                            name.push_str(&c.to_string());
                        },
                        ParsePos::Id => {
                            id.push_str(&c.to_string());
                        },
                        ParsePos::Class => {
                            curr_class.push_str(&c.to_string());
                        },
                        ParsePos::AttrKey => {
                            if c.eq(&'=') {
                                parse_pos = ParsePos::AttrValOpen;
                            } else {
                                curr_key.push_str(&c.to_string());
                            }
                        },
                        ParsePos::AttrValOpen => {
                            if !c.eq(&'"') {
                                println!("Parse error: A '\"' character must follow '=' after an attribute key within the attribute enclosure");
                                return;
                            }
                            parse_pos = ParsePos::AttrKey;
                        },
                        ParsePos::AttrVal => {
                            if c.eq(&'"') {
                                parse_pos = ParsePos::AttrValClose;
                                attributes.push((curr_key, curr_val));
                                curr_key = String::new();
                                curr_val = String::new();
                            } else {
                                curr_val.push_str(&c.to_string());
                            }
                        },
                        ParsePos::AttrValClose => {
                            if !c.eq(&',') {
                                println!("Parse error. Only a ',' or a ')' may follow the closing of an attribute value");
                                return;
                            }
                            parse_pos = ParsePos::AttrValOpen;
                        },
                        ParsePos::Text => {
                            text.push_str(&c.to_string());
                        },
                    }
                },
            }
        }
    }
}

