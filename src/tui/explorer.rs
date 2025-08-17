use anyhow::Result;
use std::{env, fs, path::PathBuf};

use crate::utils::is_image_file;

/// File explorer for navigating directories and selecting images.
pub struct FileExplorer {
    current_dir: PathBuf,
    entries: Vec<PathBuf>,
    selected_index: usize,
    selected_images: Vec<PathBuf>,
}

impl FileExplorer {
    /// Creates a new file explorer starting from the current working directory.
    pub fn new() -> Result<Self> {
        let current_dir = env::current_dir()?;
        let mut explorer = FileExplorer {
            current_dir: current_dir.clone(),
            entries: Vec::new(),
            selected_index: 0,
            selected_images: Vec::new(),
        };
        explorer.refresh_entries()?;
        Ok(explorer)
    }

    /// Refreshes the list of entries in the current directory.
    /// 
    /// Includes parent directory (if exists), subdirectories, and image files.
    /// Entries are sorted with directories first, then files alphabetically.
    fn refresh_entries(&mut self) -> Result<()> {
        let mut entries = Vec::new();
        
        if let Some(parent) = self.current_dir.parent() {
            entries.push(parent.to_path_buf());
        }
        
        for entry in fs::read_dir(&self.current_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() || is_image_file(&path) {
                entries.push(path);
            }
        }
        
        entries.sort_by(|a, b| {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });
        
        self.entries = entries;
        self.selected_index = 0;
        Ok(())
    }

    /// Moves the selection cursor up one entry.
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Moves the selection cursor down one entry.
    pub fn move_down(&mut self) {
        if self.selected_index < self.entries.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Performs action on the currently selected entry.
    /// 
    /// * For directories: enters the directory
    /// * For images: toggles selection (select if unselected, deselect if selected)
    /// * Maximum of 4 images can be selected
    pub fn enter_selected(&mut self) -> Result<()> {
        if let Some(path) = self.entries.get(self.selected_index) {
            if path.is_dir() {
                self.current_dir = path.clone();
                self.refresh_entries()?;
            } else if is_image_file(path) {
                if let Some(pos) = self.selected_images.iter().position(|p| p == path) {
                    self.selected_images.remove(pos);
                } else if self.selected_images.len() < 4 {
                    self.selected_images.push(path.clone());
                }
            }
        }
        Ok(())
    }

    /// Navigates to the parent directory.
    pub fn go_up(&mut self) -> Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.refresh_entries()?;
        }
        Ok(())
    }

    /// Returns the current directory path.
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    /// Returns all entries in the current directory.
    pub fn entries(&self) -> &[PathBuf] {
        &self.entries
    }

    /// Returns the index of the currently selected entry.
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Returns the list of selected image paths.
    pub fn selected_images(&self) -> &[PathBuf] {
        &self.selected_images
    }
}