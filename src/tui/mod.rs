/// Terminal User Interface module for interactive image selection.

pub mod explorer;
pub mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::collage::{create_collage, load_and_process_image};
use explorer::FileExplorer;

/// Configuration for the TUI session.
pub struct TuiConfig {
    pub output: std::path::PathBuf,
    pub border: u32,
    pub size: u32,
    pub quality: u8,
}

/// Runs the interactive Terminal User Interface for image selection.
/// 
/// Provides a file browser with vim-style navigation keys to select
/// exactly 4 images for collage creation.
/// 
/// # Key bindings
/// 
/// * `j`/`k` or arrow keys - Navigate up/down
/// * `l`/`Enter` - Select/deselect image or enter directory
/// * `h` or left arrow - Go up to parent directory
/// * `c` - Create collage (when 4 images selected)
/// * `q` - Quit without creating
/// 
/// # Arguments
/// 
/// * `config` - Configuration for output path, border width, size, and quality
/// 
/// # Returns
/// 
/// Ok(()) on successful completion or user quit, Error on failure
pub fn run(config: TuiConfig) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut explorer = FileExplorer::new()?;
    
    loop {
        terminal.draw(|f| ui::draw(f, &explorer))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') | KeyCode::Down => explorer.move_down(),
                KeyCode::Char('k') | KeyCode::Up => explorer.move_up(),
                KeyCode::Char('l') | KeyCode::Enter => explorer.enter_selected()?,
                KeyCode::Char('h') | KeyCode::Left => explorer.go_up()?,
                KeyCode::Char('c') => {
                    if explorer.selected_images().len() == 4 {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        
                        println!("Creating collage...");
                        let images = explorer.selected_images().iter()
                            .map(|p| load_and_process_image(p, config.size))
                            .collect::<Result<Vec<_>>>()?;
                        
                        let collage = create_collage(images, config.size, config.border);
                        collage.save(&config.output)
                            .map_err(|e| anyhow::anyhow!("Failed to save output: {}", e))?;
                        
                        println!("✓ Collage created successfully at {}!", config.output.display());
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}