use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::explorer::FileExplorer;

/// Renders the TUI interface for the file explorer.
/// 
/// Layout consists of three sections:
/// 1. Title bar showing selection count
/// 2. File browser with current directory contents
/// 3. Help bar with keyboard shortcuts
pub fn draw(f: &mut Frame, explorer: &FileExplorer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(4),
        ].as_ref())
        .split(f.area());

    draw_title(f, chunks[0], explorer.selected_images().len());
    draw_file_list(f, chunks[1], explorer);
    draw_help(f, chunks[2], explorer);
}

/// Draws the title bar with selection count.
fn draw_title(f: &mut Frame, area: ratatui::layout::Rect, selected_count: usize) {
    let title = format!(" Image Collage - Select 4 Images ({}/4) ", selected_count);
    let title_block = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(title_block, area);
}

/// Draws the file browser list.
fn draw_file_list(f: &mut Frame, area: ratatui::layout::Rect, explorer: &FileExplorer) {
    let items: Vec<ListItem> = explorer.entries()
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let is_selected = explorer.selected_images().contains(path);
            let display = if path.is_dir() {
                format!("📁 {}/", path.file_name().unwrap_or_default().to_string_lossy())
            } else {
                let mark = if is_selected { "✓ " } else { "  " };
                format!("{}{}", mark, path.file_name().unwrap_or_default().to_string_lossy())
            };
            
            let style = if i == explorer.selected_index() {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            
            ListItem::new(display).style(style)
        })
        .collect();

    let files_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", explorer.current_dir().display())));
    f.render_widget(files_list, area);
}

/// Draws the help bar with keyboard shortcuts and selected files.
fn draw_help(f: &mut Frame, area: ratatui::layout::Rect, explorer: &FileExplorer) {
    let help_text = vec![
        Line::from(vec![
            Span::styled("j/k", Style::default().fg(Color::Yellow)),
            Span::raw(": navigate  "),
            Span::styled("l/Enter", Style::default().fg(Color::Yellow)),
            Span::raw(": select/enter  "),
            Span::styled("h", Style::default().fg(Color::Yellow)),
            Span::raw(": up dir  "),
            Span::styled("c", Style::default().fg(Color::Yellow)),
            Span::raw(": create  "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(": quit"),
        ]),
        Line::from(vec![
            Span::raw("Selected: "),
            Span::styled(
                explorer.selected_images().iter()
                    .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                Style::default().fg(Color::Green)
            ),
        ]),
    ];
    
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, area);
}