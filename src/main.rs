use clap::Parser;
use confy::{load, store};
use std::{
    io,
    time::Duration,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;


use clipse::{clipboard::ClipBoard, app::{App, run_app, ClipboardState}, config::ClipConfig};

#[derive(Parser)]
struct Args {

    #[clap(short, long)]
    add: Option<String>,

}

fn main() {

    let args = Args::parse();
    // if no args are given show all clipboard content

    // load existing clipboard content
    let cfg: ClipConfig = load("clipse", None).expect("something went wrong with the config file!");
    let mut clipboard: ClipBoard = cfg.clipboard;

    if args.add.is_some() {
        clipboard.add(args.add.unwrap());
        let cfg: ClipConfig = ClipConfig { clipboard: clipboard.clone() };
        store("clipse", None, cfg).unwrap();
    }

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(250);
    let app = App::new(clipboard.content.iter().map(|x| x.as_str()).collect());
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    terminal.show_cursor().unwrap();

    match res {
        Ok(v) => {
            match v {
                ClipboardState::Delete(li) => {
                    let cfg: ClipConfig = ClipConfig { clipboard: ClipBoard { content: li } };
                    store("clipse", None, cfg).expect("something went wrong with the config file!");
                },
                ClipboardState::Select(i) => {
                    println!("{}", i);
                },
            }
        },
        Err(e) => eprintln!("Application error: {}", e),
    }

}
