use std::error::Error;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    execute,
};
use std::io;
use std::time::Duration;
use crate::calendar::CalendarClient;
use crate::ui::Tui;

pub struct App {
    events: Vec<calendar::Event>,
    selected_index: usize,
    calendar_client: CalendarClient,
}

impl App {
    pub fn new(calendar_client: CalendarClient) -> Self {
        Self {
            events: Vec::new(),
            selected_index: 0,
            calendar_client,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // Setup terminal
        let mut tui = Tui::new()?;
        
        // Fetch initial events
        self.events = self.calendar_client.fetch_events().await?;
        
        loop {
            // Render UI
            tui.draw(self)?;
            
            // Handle input
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => self.previous(),
                        KeyCode::Down => self.next(),
                        _ => {}
                    }
                }
            }
        }
        
        // Cleanup
        tui.cleanup()?;
        Ok(())
    }

    pub fn next(&mut self) {
        if !self.events.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.events.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.events.is_empty() {
            self.selected_index = self.selected_index.checked_sub(1).unwrap_or(self.events.len() - 1);
        }
    }
}
