pub struct ClipBoard {
    pub content: Vec<String>,
}

impl ClipBoard {

    pub fn new() -> ClipBoard {
        ClipBoard {
            content: vec![
                "first",
                "second",
                "third",
                "fourth"
            ].iter().map(|s| s.to_string()).collect()
        }
    }

}
