
use clap::Parser;
use std::{
    error::Error,
    io,
    time::{Duration, Instant}, vec,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};


use clipse::{encryption::encrypt, clipboard::ClipBoard, app::{App, run_app}};

#[derive(Parser)]
struct Args {

    #[clap(short, long)]
    add: String,

    #[clap(short, long)]
    new: String
}

fn main() {

    // let args = Args::parse();
    // if no args are given show all clipboard content

    let clipboard = ClipBoard::new();

    // input message
    let message = "Hello, world!";
    let result = encrypt(message);
    println!("{}", result);

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(250);
    let content = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];
    let app = App::new(content);
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode().unwrap();

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();

    terminal.show_cursor().unwrap();

    match res {
        Ok(v) => println!("Selected value: {}", v),
        Err(e) => eprintln!("Application error: {}", e),
    }

}
