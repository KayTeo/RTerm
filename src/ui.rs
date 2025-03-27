use ratatui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Span, Spans},
  widgets::{Block, Borders, List, ListItem, Paragraph},
  Terminal,
};
use std::io;
use crate::app::App;

pub struct Tui {
  terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
  pub fn new() -> io::Result<Self> {
      let backend = CrosstermBackend::new(io::stdout());
      let terminal = Terminal::new(backend)?;
      Ok(Self { terminal })
  }

  pub fn draw(&mut self, app: &App) -> io::Result<()> {
      self.terminal.draw(|f| {
          let chunks = Layout::default()
              .direction(Direction::Vertical)
              .margin(1)
              .constraints([
                  Constraint::Length(3),
                  Constraint::Min(0),
                  Constraint::Length(3),
              ])
              .split(f.size());

          // Title
          let title = Paragraph::new("Google Calendar TUI")
              .style(Style::default().fg(Color::Cyan))
              .block(Block::default().borders(Borders::ALL));
          f.render_widget(title, chunks[0]);

          // Events list
          let events: Vec<ListItem> = app.events
              .iter()
              .enumerate()
              .map(|(i, event)| {
                  let style = if i == app.selected_index {
                      Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                  } else {
                      Style::default()
                  };
                  
                  ListItem::new(event.format_for_display())
                      .style(style)
              })
              .collect();

          let events_list = List::new(events)
              .block(Block::default().title("Events").borders(Borders::ALL))
              .highlight_style(Style::default().add_modifier(Modifier::BOLD));

          f.render_widget(events_list, chunks[1]);

          // Help text
          let help = Paragraph::new("↑/↓: Navigate | q: Quit")
              .style(Style::default().fg(Color::Gray))
              .block(Block::default().borders(Borders::ALL));
          f.render_widget(help, chunks[2]);
      })?;
      Ok(())
  }

  pub fn cleanup(&mut self) -> io::Result<()> {
      // Terminal cleanup code
      Ok(())
  }
}