use std::{fs::File, io::Write};
use std::io::Read;

pub struct Stack<T> {
    data: Vec<T>,
    len: usize,
}

// interface that must be implemented for a type to be used in a stack
// dump() write stack to file
// load() read stack from file
pub trait Stackable {
    fn dump(&self, filename: &str) -> std::io::Result<()>;
    fn load(&mut self, filename: &str) -> std::io::Result<()>;
}

// impl stack interface for type String
impl Stackable for Stack<String> {
    fn dump(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        for item in &self.data {
            file.write_all(item.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }

    fn load(&mut self, filename: &str) -> std::io::Result<()> {
        let mut fobj = match File::open(filename) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        let mut contents = String::new();
        fobj.read_to_string(&mut contents)?;
        // if empty file return Err
        if contents.len() == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Empty file"));
        }
        for line in contents.lines() {
            self.push(line.to_string());
        }

        Ok(())
    }
}

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }
        self.data.last()
    }

    pub fn len(&self) -> usize {
        self.len
    }
    
}

// use RAII to drop temp file
struct TempFile {
    filename: String,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        std::fs::remove_file(&self.filename).unwrap();
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_dump_and_load() {
        let _f = TempFile { filename: String::from("test.txt") };
        let mut stack = Stack::new();
        stack.push("one".to_string());
        stack.push("two".to_string());
        stack.push("three".to_string());
        stack.dump("test.txt").unwrap();

        let mut stack2 = Stack::new();
        stack2.load("test.txt").unwrap();
        assert_eq!(stack2.len(), 3);
        assert_eq!(stack2.pop().unwrap(), "three");
        assert_eq!(stack2.pop().unwrap(), "two");
        assert_eq!(stack2.pop().unwrap(), "one");
    }

}
