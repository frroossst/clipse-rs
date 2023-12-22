use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
};
use ratatui::{prelude::*, widgets::*};

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is
/// a wrapper around `ListState`. Keeping track of the items state let us render the associated
/// widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    pub fn new(li: Vec<&'a str>) -> App<'a> {
        // construct tuple of (&'a str, usize) from li arg 
        // li arg is of type Vec<&str>
        let list_items = li.into_iter().enumerate().map(|(i, s)| (s, i)).collect::<Vec<_>>();
        App {
            items: StatefulList::with_items(
                list_items,
            ),
        }
    }
}

pub enum ClipboardState {
    Delete(Vec<String>),
    Select(String),
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<ClipboardState, io::Error> {
    let mut last_tick = Instant::now();
    let mut last_key: Option<KeyCode> = None;
    let mut did_delete = false;
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if app.items.items.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "No items to select"));
        }

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            if did_delete {
                                return Ok(ClipboardState::Delete(app.items.items.iter().map(|x| x.0.to_string() ).collect()));
                            } else {
                                return Err(io::Error::new(io::ErrorKind::Other, "User quit"));
                            }
                        },
                        KeyCode::Left | KeyCode::Char('h') => app.items.unselect(),
                        KeyCode::Down | KeyCode::Char('j') => app.items.next(),
                        KeyCode::Up | KeyCode::Char('k') => app.items.previous(),
                        KeyCode::Char('d') => {
                            if last_key.is_some_and(|k| k == KeyCode::Char('d')) {
                                let index = app.items.state.selected().unwrap();
                                app.items.items.remove(index);
                                app.items.unselect();
                                app.items.select(index.saturating_sub(1));
                                did_delete = true;
                            }
                        },
                        KeyCode::Enter => {
                            let index = app.items.state.selected().unwrap();
                            let item = app.items.items[index].0;
                            return Ok(ClipboardState::Select(item.to_string()));
                        }
                        _ => {},
                    }
                    last_key = Some(key.code);
                } 
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)])
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.0)];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

}