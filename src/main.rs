use std::{cell::RefCell, env};
// use std::any::Any;
use std::io::{self, Write};

struct Config {
    newline: RefCell<bool>,
    escape_characters: RefCell<bool>
}

struct Input {
    value: RefCell<String>
}

enum Return {
    StringType(String),
    BoolType(bool)
}

impl Input {
    fn new() ->Input {
        Input {
            value: RefCell::new("hello".to_string())
        }
    }

    fn update_value(&self, value: &char) {
        *self.value.borrow_mut() = value.to_string();
    }

    fn unescape_characters(&self) -> Return {
        let borrowed_value = &self.value.borrow();
        let val = match borrowed_value.as_str() {
            "a" => "\x07",
            "b" => "\x08",
            "c" => return Return::BoolType(true),
            "e" => "\x1b",
            "f" => "\x0c",
            "n" => "\n",
            "r" => "\r",
            "t" => "\t",
            "v" => "\x0b",
            _ => &borrowed_value
        };
        Return::StringType(val.to_string())
    }
}

impl Config {
    fn new() -> Config {
        Config {
            newline: RefCell::new(true),
            escape_characters: RefCell::new(false)
        }
    }

    fn check_options(&self, flag: &str, output: &mut impl Write) {
        match flag {
            "-e" => {
                *self.escape_characters.borrow_mut() = true;
            }
            "-n" => {
                *self.newline.borrow_mut() = false;
            }
            "-E" => {
                *self.escape_characters.borrow_mut() = false;
            }
            "-ne" | "-en" => {
                *self.escape_characters.borrow_mut() = true;
            }
            _ => {
                write!(output, "{}", flag).ok();
            }
        }
    }
}

fn main() {
    let config = Config::new();
    let mut input = env::args();
    input.next();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let mut append = String::from(" ");
    let mut standard_input: Vec<String> = vec![];
    let temp_string: String = String::from("abcefnrtv");
    let mut temp_vec: Vec<String> = vec![];
    config.check_options(&input.next().unwrap(), &mut output);

    for i in temp_string.chars() {
        temp_vec.push(i.to_string());
    }

    for _i in 0..input.len() {
        standard_input.push(input.next().unwrap());
    }

    for i in &standard_input {
        let value = i;
        // print!("{}", *config.escape_characters.borrow());
        if *config.escape_characters.borrow() {
            let unescape = process_input(&value, &mut output, &temp_vec);
            if unescape.is_some() {
                *config.newline.borrow_mut() = false;
                break;
            }
        } else {
            if append != " " {
                append = " ".to_string();
            }
            append += &value;
            write!(output, "{}", append).ok();
        }
    }

    if *config.newline.borrow() {
        writeln!(output).ok();
    }
    if let Err(e) = std::io::stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}

fn process_input(input: &str, mut output: impl Write, temp_vec: &Vec<String>) -> Option<bool> {
    let escape: Input = Input::new();
    let backslash: char = '\\';
    let string_backslash: String = "\\".to_string();
    if input.contains(backslash) {
        for (index, value) in input.chars().enumerate() {
            if value == backslash {
                let temp_string = input[index+1..index+2].to_string();
                if !temp_vec.contains(&temp_string) {
                    write!(output, "{}", backslash).ok();
                }
                continue;
            }
            if index == 0 || input[index-1..index] != string_backslash {
                write!(output, "{}", value).ok();
            }
            if value != backslash && input[index-1..index] == string_backslash {
                escape.update_value(&value);
                let a = match escape.unescape_characters() {
                    Return::StringType(val) => Some(val),
                    Return::BoolType(_val) => None
                };
                if a.is_some() {
                    let b = a.unwrap();
                    if index == input.len()-1 {
                        write!(output, "{} ", b).ok();
                    } else {
                        write!(output, "{}", b).ok();
                    }
                } else {
                    return Some(true);
                }
            }
            // else {
            //     if index == input.len()-1 {
            //         write!(output, "{} ", value).ok();
            //     } else {
            //         write!(output, "{}", value).ok();
            //     }
            // }
        }
    } else {
        write!(output, "{} ", input).ok();
    }
    None
}
