use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use clipse::stack::{Stack, Stackable};


const CLIPBOARD_FILE: &str = "/tmp/clipboard_stack.clipse";

fn read_clipboard() -> io::Result<String> {
    if Path::new(CLIPBOARD_FILE).exists() {
        let mut file = File::open(CLIPBOARD_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        Ok(String::new())
    }
}

fn write_clipboard(data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(CLIPBOARD_FILE)?;

    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);

    if !Path::new(CLIPBOARD_FILE).exists() {
        fs::File::create(CLIPBOARD_FILE)?;
    }

    let mut stack : Stack<String> = Stack::<String>::new();
    match stack.load(CLIPBOARD_FILE) {
        Ok(_) => {},
        // file empty error
        Err(e) => {
            if e.kind() == std::io::ErrorKind::Other {
                stack.dump(CLIPBOARD_FILE)?;
            } else {
                panic!("Error: {}", e);
            }
        }
    }

    match args.next().as_deref() {
        Some("get") => {
            let clipboard_content = read_clipboard()?;
            match stack.pop() {
                Some(content) => {
                    println!("Popped content: {}", content);
                    stack.push(content);
                }
                None => {
                    println!("Stack is empty");
                }
            }
        }
        Some("set") => {
            let new_content = args.collect::<Vec<String>>().join(" ");
            write_clipboard(&new_content)?;
            stack.push(new_content);
            stack.dump(CLIPBOARD_FILE)?;
            println!("Clipboard updated successfully");
        }
        _ => {
            println!("Usage: clipboard_manager [get|set <content>]");
        }
    }

    Ok(())
}

