use std::path::Path;

/// Determines if a path points to a supported image file based on its extension.
/// 
/// # Supported formats
/// 
/// * JPEG (.jpg, .jpeg)
/// * PNG (.png)
/// * GIF (.gif)
/// * BMP (.bmp)
/// * WebP (.webp)
/// * TIFF (.tiff, .tif)
/// 
/// # Arguments
/// 
/// * `path` - The file path to check
/// 
/// # Returns
/// 
/// `true` if the file extension indicates an image file, `false` otherwise
pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif")
    } else {
        false
    }
}