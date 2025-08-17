use anyhow::{Context, Result};
use image::{DynamicImage, ImageBuffer, Rgb};
use std::path::Path;

/// Loads an image from the given path and processes it to a square of the target size.
/// 
/// The image is center-cropped if not already square, then resized to the exact
/// target dimensions using Lanczos3 filtering for high quality.
/// 
/// # Arguments
/// 
/// * `path` - Path to the image file
/// * `target_size` - The desired width and height in pixels
/// 
/// # Returns
/// 
/// A processed `DynamicImage` ready for collage creation
pub fn load_and_process_image<P: AsRef<Path>>(path: P, target_size: u32) -> Result<DynamicImage> {
    let path = path.as_ref();
    let img = image::open(path)
        .with_context(|| format!("Failed to open image: {}", path.display()))?;
    
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    
    let cropped = if width != height {
        let min_dim = width.min(height);
        let x_offset = (width - min_dim) / 2;
        let y_offset = (height - min_dim) / 2;
        
        let cropped = DynamicImage::ImageRgb8(img).crop(x_offset, y_offset, min_dim, min_dim);
        cropped
    } else {
        DynamicImage::ImageRgb8(img)
    };
    
    let resized = cropped.resize_exact(
        target_size,
        target_size,
        image::imageops::FilterType::Lanczos3,
    );
    
    Ok(resized)
}

/// Creates a 2x2 collage from four images with customizable borders.
/// 
/// Images are arranged in a grid:
/// ```text
/// [img1] | [img2]
/// -------+-------
/// [img3] | [img4]
/// ```
/// 
/// # Arguments
/// 
/// * `images` - Vector of exactly 4 processed images
/// * `size` - The size of each image in the grid
/// * `border_width` - Width of the white border between images in pixels
/// 
/// # Returns
/// 
/// A `DynamicImage` containing the final collage
pub fn create_collage(images: Vec<DynamicImage>, size: u32, border_width: u32) -> DynamicImage {
    let collage_size = size * 2 + border_width;
    let white = Rgb([255u8, 255u8, 255u8]);
    
    let mut collage = ImageBuffer::from_pixel(collage_size, collage_size, white);
    
    let positions = [
        (0, 0),
        (size + border_width, 0),
        (0, size + border_width),
        (size + border_width, size + border_width),
    ];
    
    for (img, &(x, y)) in images.iter().zip(positions.iter()) {
        let rgb_img = img.to_rgb8();
        for (px, py, pixel) in rgb_img.enumerate_pixels() {
            if px + x < collage_size && py + y < collage_size {
                collage.put_pixel(px + x, py + y, *pixel);
            }
        }
    }
    
    DynamicImage::ImageRgb8(collage)
}