use anyhow::{Context, Result};
use clap::Parser;
use image_collage::{
    collage::{create_collage, load_and_process_image},
    tui::{self, TuiConfig},
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "image-collage",
    version,
    author,
    about = "Create beautiful 2x2 image collages with ease",
    long_about = r#"
image-collage - Create beautiful 2x2 image collages with ease

This tool allows you to create a 2x2 grid collage from 4 images, with
customizable borders and sizing. Images are automatically center-cropped
to squares and resized for perfect alignment.

MODES:
  CLI Mode:  Provide 4 image paths as arguments
  TUI Mode:  Run without arguments for interactive file browser

VISUAL LAYOUT:
  The collage arranges your images in a 2x2 grid:

    ┌─────────┬─────────┐
    │ Image 1 │ Image 2 │
    ├─────────┼─────────┤
    │ Image 3 │ Image 4 │
    └─────────┴─────────┘

  With borders (--border 10):

    ┌─────────────────────────┐
    │ ┌───────┐   ┌───────┐   │
    │ │  Img1 │   │  Img2 │   │
    │ └───────┘   └───────┘   │
    │                         │
    │ ┌───────┐   ┌───────┐   │
    │ │  Img3 │   │  Img4 │   │
    │ └───────┘   └───────┘   │
    └─────────────────────────┘

EXAMPLES:
  # CLI mode with 4 images
  image-collage photo1.jpg photo2.png photo3.gif photo4.bmp

  # CLI mode with custom output and border
  image-collage img1.jpg img2.jpg img3.jpg img4.jpg -o my_collage.jpg -b 5

  # CLI mode with 800x800 images and thick border
  image-collage *.jpg --size 800 --border 20 --output large_collage.jpg

  # Interactive TUI mode (launches file browser)
  image-collage

  # TUI mode with custom settings
  image-collage --output ~/Desktop/collage.jpg --size 600 --quality 100

TUI CONTROLS:
  j/↓     Move down in file list
  k/↑     Move up in file list
  l/Enter Select/deselect image or enter directory
  h/←     Go to parent directory
  c       Create collage (when 4 images selected)
  q       Quit without creating

SUPPORTED FORMATS:
  Input:  JPEG, PNG, GIF, BMP, WebP, TIFF
  Output: Format determined by file extension (default: JPEG)

NOTES:
  • Images are automatically center-cropped to squares
  • Border color is always white (#FFFFFF)
  • Quality setting only affects JPEG output
  • In TUI mode, selected images are marked with ✓
"#
)]
struct Args {
    #[arg(help = "First image path (leave empty for interactive mode)")]
    image1: Option<PathBuf>,
    
    #[arg(help = "Second image path")]
    image2: Option<PathBuf>,
    
    #[arg(help = "Third image path")]
    image3: Option<PathBuf>,
    
    #[arg(help = "Fourth image path")]
    image4: Option<PathBuf>,
    
    #[arg(
        short,
        long,
        default_value = "collage.jpg",
        help = "Output file path",
        long_help = "Output file path. Format is determined by extension (e.g., .jpg, .png)"
    )]
    output: PathBuf,
    
    #[arg(
        short,
        long,
        default_value_t = 2,
        help = "Border width in pixels",
        long_help = "Width of the white border between images in pixels (0 for no border)"
    )]
    border: u32,
    
    #[arg(
        short,
        long,
        default_value_t = 400,
        help = "Size of each image in pixels",
        long_help = "Size (width and height) of each image in the grid. Final collage will be (size*2 + border) pixels"
    )]
    size: u32,
    
    #[arg(
        short,
        long,
        default_value_t = 95,
        value_parser = clap::value_parser!(u8).range(1..=100),
        help = "JPEG quality (1-100)",
        long_help = "JPEG compression quality from 1 (worst) to 100 (best). Only affects JPEG output"
    )]
    quality: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.image1.is_none() {
        let config = TuiConfig {
            output: args.output,
            border: args.border,
            size: args.size,
            quality: args.quality,
        };
        tui::run(config)?;
    } else if let (Some(img1), Some(img2), Some(img3), Some(img4)) = 
        (args.image1.as_ref(), args.image2.as_ref(), args.image3.as_ref(), args.image4.as_ref()) {
        
        println!("Loading images...");
        let images = vec![
            load_and_process_image(img1, args.size)?,
            load_and_process_image(img2, args.size)?,
            load_and_process_image(img3, args.size)?,
            load_and_process_image(img4, args.size)?,
        ];
        
        println!("Creating collage...");
        let collage = create_collage(images, args.size, args.border);
        
        println!("Saving to {}...", args.output.display());
        collage
            .save(&args.output)
            .with_context(|| format!("Failed to save output to {}", args.output.display()))?;
        
        println!("✓ Collage created successfully!");
    } else {
        anyhow::bail!("Please provide either no images (for interactive mode) or all 4 images");
    }
    
    Ok(())
}