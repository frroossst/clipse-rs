use std::{process::Stdio, io::Write};

use serde::{Deserialize, Serialize};


#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClipBoard {
    pub content: Vec<String>,
}

impl ClipBoard {

    pub fn new() -> ClipBoard {
        ClipBoard {
            content: Vec::new(),
        }
    }

    pub fn add(&mut self, content: String) {
        self.content.push(content);
    }

    pub fn remove(&mut self, item: &str) {
        let index = self.content.iter().position(|x| x == item);
        if let Some(i) = index {
            self.content.remove(i);
        }
    }

}

pub fn copy_to_system_clipboard(content: &str) {
    // run xclip -sel c `i` to copy to clipboard
    let mut command = std::process::Command::new("xclip")
        .arg("-sel")
        .arg("c")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    // write to stdin of xclip
    let mut child_stdin = command.stdin.take().unwrap();
    child_stdin.write_all(content.as_bytes()).expect("failed to write to stdin of xclip");
}
