extern crate htmlbuilder;
use htmlbuilder::{
    attr::Attr,
    tag::Tag,
    html::Element as HtmlElement,
    document::Document,
};
extern crate clap;
use clap::Clap;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;
#[derive(PartialEq,Debug,Clone)]
enum ParsePos {
    TagName,
    Id,
    Class,
    AttrKey,
    AttrValOpen,
    AttrVal,
    AttrValClose,
    PostAttr,
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
    /// Prints debug information
    #[clap(short, long)]
    debug: bool,
}
fn get_time_diff_string(start_time: SystemTime) -> String {
    let end_time = SystemTime::now();
    let diff = match end_time.duration_since(start_time) {
        Ok(diff) => diff,
        Err(_) => {
            println!("Failed to retrieve system time difference");
            std::process::exit(0);
        },
    };
    let mut millis = diff.as_millis();
    let secs = millis / 1000;
    millis = millis - (secs * 1000);
    return format!("{}s {}ms", secs, millis);
}
fn get_stack_string(parse_map: Vec<(ParsePos, char)>, limit: usize) -> String {
    let mut output = String::new();
    let min;
    if limit == 0 {
        min = 0;
    } else if parse_map.len() >= limit {
        min = parse_map.len() - limit;
    } else {
        min = 0;
    }
    for i in min..parse_map.len() {
        let (key, val) = match parse_map.clone().into_iter().nth(i) {
            Some(parse) => (parse.0, parse.1),
            None => {
                println!("Failed to get elements from parse map");
                std::process::exit(0);
            },
        };
        output.push_str(format!("{:?}: {}\n", key, val).as_str());
    }
    return output;
}
fn throw_parser_error<'a>(start_time: SystemTime, parse_map: Vec<(ParsePos, char)>, line: usize, msg: &'a str) {
    let diff = get_time_diff_string(start_time);
    println!("Parser failed in {} on line {}\n", diff, line);
    println!("Stack: \n{}\n", get_stack_string(parse_map, 5));
    println!("Message: {}\n", msg);
    std::process::exit(1);
}
fn get_parser_success_string(start_time: SystemTime, parse_map: Vec<(ParsePos, char)>, debug: bool) {
    let diff = get_time_diff_string(start_time);
    println!("Parser succeeded in {}\n", diff);
    if debug {
        println!("Stack:\n{}\n", get_stack_string(parse_map, 0));
    }
    //println!("Elements:\n");
    //for element in elements {
    //    println!("{}", element.to_string());
    //}
}
struct NestInfo {
    level: usize,
    line: usize,
    element: HtmlElement,
}
fn recurse_nest(mut index: usize, elements: &mut Vec<NestInfo>, prev: &mut NestInfo) -> HtmlElement {
    let mut curr = match elements.get(index.clone()) {
        Some(item) => item,
        None => panic!("Nest error. Index {} is out of bounds", index),
    };
    if curr.level > prev.level {
        index = index + 1;
        recurse_nest(index, elements, );
    }
    return HtmlElement::new(false, "something");
}
fn nest_elements(elements: &mut Vec<NestInfo>) -> Vec<HtmlElement> {
    let nest = Vec::new();
    let mut index: usize = 0;
    let mut prev = match elements.get(index) {
        Some(item) => item,
        None => panic!("Nest error. Index 0 is out of bounds"),
    };
    index = index + 1;
    while index < elements.len() {
        recurse_nest(index, elements, &mut prev);
        index = index + 1;
    }
    return nest;
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
    let _out_file = opts.out_file.unwrap();
    let debug = opts.debug;
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
    let start_time = SystemTime::now();
    let mut parse_map: Vec<(ParsePos, char)> = Vec::new();
    let mut line_num: usize = 1;
    let mut doc = Document::new_html5();
    let mut elements: Vec<NestInfo> = Vec::new();
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
        let mut is_only_text: bool = false;
        for c in line.chars() {
            parse_map.push((parse_pos.clone(), c));
            match c {
                '\u{0023}' => { // #
                    match parse_pos {
                        ParsePos::TagName => {
                            if name.is_empty() {
                                name.push_str("div");
                            }
                            match Tag::from_tag_name(&name) {
                                Ok(_) => {
                                    parse_pos = ParsePos::Id;
                                    continue;
                                },
                                Err(_) => {
                                    text.push_str(&name);
                                    text.push(c);
                                    parse_pos = ParsePos::Text;
                                    is_only_text = true;
                                    continue;
                                },
                            }
                        },
                        ParsePos::Id => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '#' cannot follow a '#' in the header line of an element");
                            return;
                        },
                        ParsePos::Class => {
                            if curr_class.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, format!("Parse error. Tag '{}' cannot have a blank class", name).as_str());
                                return;
                            } else {
                                classes.push(curr_class);
                                curr_class = String::new();
                                continue;
                            }
                        },
                        ParsePos::AttrKey => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '#' cannot be located within the key of an element attribute");
                            return;
                        },
                        ParsePos::AttrValOpen => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '#' cannot be located after the '=' after the attribute key");
                            return;
                        },
                        ParsePos::AttrVal => {
                            curr_val.push(c);
                            continue;
                        },
                        ParsePos::AttrValClose => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '#' cannot be located within the attribute enclosure");
                            return;
                        },
                        ParsePos::PostAttr => {
                            if !id.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, "Parse error. An element cannot have two ids");
                                return;
                            }
                            parse_pos = ParsePos::Id;
                            continue;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
                '\u{002e}' => { // .
                    match parse_pos {
                        ParsePos::TagName => {
                            if name.is_empty() {
                                name.push_str("div");
                            }
                            match Tag::from_tag_name(&name) {
                                Ok(_) => {
                                    parse_pos = ParsePos::Class;
                                    continue;
                                },
                                Err(_) => {
                                    text.push_str(&name);
                                    is_only_text = true;
                                    text.push(c);
                                    parse_pos = ParsePos::Text;
                                    continue;
                                },
                            }
                        },
                        ParsePos::Id => {
                            if id.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, "Parse error. A '.' cannot be located within the name of an element id");
                                return;
                            }
                            parse_pos = ParsePos::Class;
                            continue;
                        },
                        ParsePos::Class => {
                            if curr_class.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, "Parse error. A '.' cannot be located within the name of an element class");
                                return;
                            }
                            classes.push(curr_class);
                            curr_class = String::new();
                            continue;
                        },
                        ParsePos::AttrKey => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '.' cannot be located within the attribute enclosure");
                            return;
                        },
                        ParsePos::AttrValOpen => {
                            throw_parser_error(start_time, parse_map, line_num, format!("Parse error. The next character may only be \". '{}' not allowed", c.to_string()).as_str());
                            return;
                        },
                        ParsePos::AttrVal => {
                            curr_val.push(c);
                            continue;
                        },
                        ParsePos::AttrValClose => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. The only valid characters after the closing of an attribute value are ',' and ')'");
                            return;
                        },
                        ParsePos::PostAttr => {
                            parse_pos = ParsePos::Class;
                            continue;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
                '\u{0028}' => { // (
                    match parse_pos {
                        ParsePos::TagName => {
                            if name.is_empty() {
                                name.push_str("div");
                            }
                            match Tag::from_tag_name(&name) {
                                Ok(_) => {
                                    parse_pos = ParsePos::AttrKey;
                                    continue;
                                },
                                Err(_) => {
                                    text.push_str(&name);
                                    is_only_text = true;
                                    text.push(c);
                                    parse_pos = ParsePos::Text;
                                    continue;
                                },
                            }
                        },
                        ParsePos::Id => {
                            if id.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, "Parse error. '(' is an invalid character for an element id");
                                return;
                            }
                            parse_pos = ParsePos::AttrKey;
                            continue;
                        },
                        ParsePos::Class => {
                            if curr_class.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, "Parse error. '(' is an invalid character for an element class");
                                return;
                            }
                            parse_pos = ParsePos::AttrKey;
                            classes.push(curr_class);
                            curr_class = String::new();
                            continue;
                        },
                        ParsePos::AttrKey => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. '(' is an invalid character for an element attribute key");
                            return;
                        },
                        ParsePos::AttrValOpen => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. '(' is an invalid character within the attribute enclosure");
                            return;
                        },
                        ParsePos::AttrVal => {
                            curr_val.push(c);
                            continue;
                        },
                        ParsePos::AttrValClose => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. ',' and ')' are the only valid characters following an element attribute value closure");
                            return;
                        },
                        ParsePos::PostAttr => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. Concatenate the attribute enclusures, only one is allowed");
                            return;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
                '\u{003d}' => { // =
                    match parse_pos {
                        ParsePos::TagName => {
                            parse_pos = ParsePos::Text;
                            continue;
                        },
                        ParsePos::Id => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '=' cannot be found in an element's id");
                            return;
                        },
                        ParsePos::Class => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '=' cannot be found in an element's class");
                            return;
                        },
                        ParsePos::AttrKey => {
                            parse_pos = ParsePos::AttrValOpen;
                            continue;
                        },
                        ParsePos::AttrValOpen => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. Only a '\"' or '\'' can follow the '=' signifying the start of an element's attribute's value");
                            return;
                        },
                        ParsePos::AttrVal => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '=' cannot be found in an element's value");
                            return;
                        },
                        ParsePos::AttrValClose => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. Only a ',' or a ')' can follow the closure of an element's attribute's value");
                            return;
                        },
                        ParsePos::PostAttr => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. '=' is not allowed following an attribute enclosure");
                            return;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
                '\u{0027}'|'\u{0022}' => { // ' or "
                    match parse_pos {
                        ParsePos::TagName => {
                            parse_pos = ParsePos::Text;
                            continue;
                        },
                        ParsePos::Id => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '\'' or '\"' cannot be found in an element's id");
                            return;
                        },
                        ParsePos::Class => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '\'' or '\"' cannot be found in an element's class");
                            return;
                        },
                        ParsePos::AttrKey => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '\'' or '\"' cannot be found in an element's attribute's key");
                            return;
                        },
                        ParsePos::AttrValOpen => {
                            parse_pos = ParsePos::AttrVal;
                            continue;
                        },
                        ParsePos::AttrVal => {
                            attributes.push((curr_key, curr_val));
                            curr_key = String::new();
                            curr_val = String::new();
                            parse_pos = ParsePos::AttrValClose;
                            continue;
                        },
                        ParsePos::AttrValClose => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. Only the ')' or ',' character may be found following an element's attribute's value");
                            return;
                        },
                        ParsePos::PostAttr => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A '\"' or '\"' may not directly follow an attribute enclosure");
                            return;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
                '\u{002c}' => { // ,
                    match parse_pos {
                        ParsePos::TagName => {
                            parse_pos = ParsePos::Text;
                            continue;
                        },
                        ParsePos::Id => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ',' cannot be found in an element's id");
                            return;
                        },
                        ParsePos::Class => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ',' cannot be found in an element's class attribute");
                            return;
                        },
                        ParsePos::AttrKey => {
                            if curr_key.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, "Parse error. An element's attribute's key cannot be blank");
                                return;
                            }
                            attributes.push((curr_key.clone(), curr_key));
                            curr_key = String::new();
                            continue;
                        },
                        ParsePos::AttrValOpen => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. Only a '\"' or a '\'' may follow the '=' character in an element's attribute enclosure");
                            return;
                        },
                        ParsePos::AttrVal => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ',' character cannot be found in the value of an element's attribute");
                            return;
                        },
                        ParsePos::AttrValClose => {
                            parse_pos = ParsePos::AttrKey;
                            continue;
                        },
                        ParsePos::PostAttr => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ',' character cannot directly follow an attribute enclosure");
                            return;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
                '\u{0029}' => { // )
                    match parse_pos {
                        ParsePos::TagName => {
                            parse_pos = ParsePos::Text;
                            continue;
                        },
                        ParsePos::Id => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ')' cannot be found in an element's id");
                            return;
                        },
                        ParsePos::Class => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ')' cannot be found in an element's class attribute");
                            return;
                        },
                        ParsePos::AttrKey => {
                            curr_val = curr_key.clone();
                            attributes.push((curr_key, curr_val));
                            curr_key = String::new();
                            curr_val = String::new();
                            parse_pos = ParsePos::PostAttr;
                            continue;
                        },
                        ParsePos::AttrValOpen => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. Only a '\"' or a '\'' may follow the '=' character in an element's attribute enclosure");
                            return;
                        },
                        ParsePos::AttrVal => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ')' character cannot be found in the value of an element's attribute");
                            return;
                        },
                        ParsePos::AttrValClose => {
                            parse_pos = ParsePos::PostAttr;
                            continue;
                        },
                        ParsePos::PostAttr => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ')' character may not directly follow an attribute enclosure");
                            return;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
                '\u{0020}' => { // {space}
                    match parse_pos {
                        ParsePos::TagName => {
                            if name.is_empty() {
                                name.push_str("div");
                            }
                            match Tag::from_tag_name(&name) {
                                Ok(_) => {
                                    parse_pos = ParsePos::Text;
                                    continue;
                                },
                                Err(_) => {
                                    text.push_str(&name);
                                    is_only_text = true;
                                    text.push(c);
                                    parse_pos = ParsePos::Text;
                                    continue;
                                },
                            }
                        },
                        ParsePos::Id => {
                            if id.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, "Parse error. A ' ' cannot be found in an element's id");
                                return;
                            }
                            parse_pos = ParsePos::Text;
                            continue;
                        },
                        ParsePos::Class => {
                            if curr_class.is_empty() {
                                throw_parser_error(start_time, parse_map, line_num, "Parse error. A ' ' cannot be found in an element's class");
                                return;
                            }
                            classes.push(curr_class);
                            curr_class = String::new();
                            parse_pos = ParsePos::Text;
                            continue;
                        },
                        ParsePos::AttrKey => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ' ' cannot be found in an element's attribute key");
                            return;
                        },
                        ParsePos::AttrValOpen => {
                            throw_parser_error(start_time, parse_map, line_num, "Parse error. A ' ' cannot be found in an element's attribute enclosure");
                            return;
                        },
                        ParsePos::AttrVal => {
                            curr_val.push(c);
                            continue;
                        },
                        ParsePos::AttrValClose => {
                            parse_pos = ParsePos::Text;
                            continue;
                        },
                        ParsePos::PostAttr => {
                            parse_pos = ParsePos::Text;
                            continue;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
                _ => {
                    match parse_pos {
                        ParsePos::TagName => {
                            name.push(c);
                            continue;
                        },
                        ParsePos::Id => {
                            id.push(c);
                            continue;
                        },
                        ParsePos::Class => {
                            curr_class.push(c);
                            continue;
                        },
                        ParsePos::AttrKey => {
                            curr_key.push(c);
                            continue;
                        },
                        ParsePos::AttrValOpen => {
                            parse_pos = ParsePos::AttrVal;
                            continue;
                        },
                        ParsePos::AttrVal => {
                            curr_val.push(c);
                            continue;
                        },
                        ParsePos::AttrValClose => {
                            parse_pos = ParsePos::AttrValOpen;
                            continue;
                        },
                        ParsePos::PostAttr => {
                            throw_parser_error(start_time, parse_map, line_num, format!("Parse error. '{}' can not directly follow an attribute enclosure", c).as_str());
                            return;
                        },
                        ParsePos::Text => {
                            text.push(c);
                            continue;
                        },
                    }
                },
            }
        }
        match parse_pos {
            ParsePos::Class => {
                if curr_class.is_empty() {
                    throw_parser_error(start_time, parse_map, line_num, "Parse error. Element cannot have an empty class name");
                    return;
                } else {
                    classes.push(curr_class);
                }
            },
            ParsePos::AttrKey => {
                throw_parser_error(start_time, parse_map, line_num, "Parse error. Invalid line ending");
                return;
            },
            ParsePos::AttrValOpen => {
                throw_parser_error(start_time, parse_map, line_num, "Parse error. Invalid line ending");
                return;
            },
            ParsePos::AttrVal => {
                throw_parser_error(start_time, parse_map, line_num, "Parse error. Invalid line ending");
                return;
            },
            _ => {},
        }
        if name.eq(&"doctype") {
            // TODO: Add support for other doctypes
            continue;
        }
        let mut elem;
        if is_only_text && !text.is_empty() {
            elem = HtmlElement::new_text(&text);
        } else {
            let tag_name = match Tag::from_tag_name(&name) {
                Ok(tag_name) => tag_name,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                },
            };
            elem = doc.create_element(tag_name);
            if !id.is_empty() {
                elem.id(&id);
            }
            for class in classes {
                elem.class(&class);
            }
            for attr in attributes {
                elem.attr(Attr::from_name(attr.0.as_str(), attr.1.as_str()));
            }
        }
        if !text.is_empty() {
            elem.inner_text(text.as_str());
        }
        elements.push(NestInfo { level: dent, line: line_num, element: elem, });
        line_num = line_num + 1;
    }
    let proper_nesting = nest_elements(elements);
    get_parser_success_string(start_time, parse_map, debug);
    return;
}

