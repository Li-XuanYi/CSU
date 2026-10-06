use crate::error::AppError;
use image::{DynamicImage, GenericImageView, ImageFormat, imageops::FilterType};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

/// 生成缩略图
pub fn generate_thumbnail(
    source_path: &Path,
    dest_path: &Path,
    width: u32,
    height: u32,
) -> Result<(), AppError> {
    // 读取原图
    let img = image::open(source_path)
        .map_err(|e| AppError::Internal(format!("Failed to open image: {}", e)))?;

    // 计算缩略图尺寸（保持宽高比）
    let (thumb_width, thumb_height) = calculate_thumbnail_size(&img, width, height);

    // 生成缩略图
    let thumbnail = img.resize(thumb_width, thumb_height, FilterType::Lanczos3);

    // 确定图片格式
    let format = ImageFormat::from_path(source_path)
        .map_err(|e| AppError::Internal(format!("Failed to determine image format: {}", e)))?;

    // 保存缩略图
    thumbnail
        .save_with_format(dest_path, format)
        .map_err(|e| AppError::Internal(format!("Failed to save thumbnail: {}", e)))?;

    Ok(())
}

/// 计算缩略图尺寸（保持宽高比）
fn calculate_thumbnail_size(img: &DynamicImage, max_width: u32, max_height: u32) -> (u32, u32) {
    let (orig_width, orig_height) = img.dimensions();

    let width_ratio = max_width as f32 / orig_width as f32;
    let height_ratio = max_height as f32 / orig_height as f32;

    let ratio = width_ratio.min(height_ratio);

    let new_width = (orig_width as f32 * ratio) as u32;
    let new_height = (orig_height as f32 * ratio) as u32;

    (new_width, new_height)
}

/// 获取图片基本信息
pub fn get_image_info(path: &Path) -> Result<ImageInfo, AppError> {
    let img = image::open(path)
        .map_err(|e| AppError::Internal(format!("Failed to open image: {}", e)))?;

    let (width, height) = img.dimensions();
    let format = ImageFormat::from_path(path)
        .map_err(|e| AppError::Internal(format!("Failed to determine image format: {}", e)))?;

    // 获取文件大小
    let file_size = std::fs::metadata(path)
        .map_err(|e| AppError::Internal(format!("Failed to read file metadata: {}", e)))?
        .len();

    Ok(ImageInfo {
        width,
        height,
        format: format_to_string(format),
        file_size,
    })
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub file_size: u64,
}

fn format_to_string(format: ImageFormat) -> String {
    match format {
        ImageFormat::Png => "png".to_string(),
        ImageFormat::Jpeg => "jpeg".to_string(),
        ImageFormat::Gif => "gif".to_string(),
        ImageFormat::WebP => "webp".to_string(),
        ImageFormat::Bmp => "bmp".to_string(),
        ImageFormat::Tiff => "tiff".to_string(),
        _ => "unknown".to_string(),
    }
}

/// 计算文件的 SHA-256 哈希值
pub fn calculate_file_hash(path: &Path) -> Result<String, AppError> {
    let mut file = std::fs::File::open(path)
        .map_err(|e| AppError::Internal(format!("Failed to open file for hashing: {}", e)))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // 8KB buffer for efficient reading

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| AppError::Internal(format!("Failed to read file: {}", e)))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
