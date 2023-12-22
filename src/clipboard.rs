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
