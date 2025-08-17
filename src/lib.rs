/// Image collage creation library.
/// 
/// This library provides functionality to create 2x2 image collages
/// with customizable borders and sizing options.

pub mod collage;
pub mod utils;
pub mod tui;

pub use collage::{create_collage, load_and_process_image};
pub use utils::is_image_file;