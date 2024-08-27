use tokio::sync::mpsc;
use tui::{Terminal, widgets::{Widget, Block, Borders}, layout::{Layout, Constraint}, backend::CrosstermBackend};
use crossterm::{event::{self, Event, KeyCode}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use std::io;

async fn draw_ui(tx: mpsc::Sender<String>) -> crossterm::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("UI").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('s') => tx.send("Start something".to_string()).await.unwrap(),
                _ => {}
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    let ui_task = tokio::spawn(async move {
        let _ = draw_ui(tx).await;
    });

    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }

    execute!(
        io::stdout(),
        LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()?;
}
