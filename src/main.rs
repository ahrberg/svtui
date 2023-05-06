use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

mod svt;

enum Event<I> {
    Input(I),
    Tick,
}

/// App holds the state of the application
#[derive(Default)]
struct App {
    /// Current value of the input box
    input: String,
    /// Body of text
    body: String,
    /// Body scoll position
    scroll: u16,
}

fn handle_page_respose(result: &Result<svt::SvtResponse, reqwest::Error>, app: &mut App) {
    match result {
        Ok(result) => {
            for p in &result.data.sub_pages {
                let page = p.alt_text.to_owned();
                app.body.push_str(&page);
            }
        }
        Err(error) => app.body = format!("Could not get page {}. Error: {:?}", app.input, error),
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::default();
    let client = svt::SvtClient::new(String::from("https://www.svt.se/text-tv/api"));

    let start_page = String::from("100");
    let res = client.get_page(&start_page).await;
    handle_page_respose(&res, &mut app);

    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let input = Paragraph::new(app.input.as_ref())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL).title("Sök"));
            rect.render_widget(input, chunks[0]);

            let body = Paragraph::new(app.body.as_ref())
                .style(Style::default())
                .scroll((app.scroll, 0))
                .block(Block::default().borders(Borders::ALL));
            rect.render_widget(body, chunks[1]);

            let copyright = Paragraph::new("svtui - SVT Text TV")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(copyright, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::PageUp => {
                    if app.scroll != 0 {
                        app.scroll -= 1;
                    }
                }
                KeyCode::PageDown => app.scroll += 1,
                KeyCode::Enter => {
                    app.body.clear();
                    app.scroll = 0;

                    let res = client.get_page(&app.input).await;
                    handle_page_respose(&res, &mut app);

                    app.input.clear();
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
