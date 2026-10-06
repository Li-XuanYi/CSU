use crate::error::AppError;
use chrono::NaiveDateTime;
use exif::{In, Reader, Tag};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ExifData {
    /// 拍摄时间
    pub date_time_original: Option<NaiveDateTime>,
    /// 相机制造商
    pub make: Option<String>,
    /// 相机型号
    pub model: Option<String>,
    /// 镜头型号
    pub lens_model: Option<String>,
    /// 焦距
    pub focal_length: Option<String>,
    /// 光圈
    pub f_number: Option<String>,
    /// ISO
    pub iso_speed: Option<u32>,
    /// 曝光时间
    pub exposure_time: Option<String>,
    /// GPS 纬度
    pub gps_latitude: Option<f64>,
    /// GPS 经度
    pub gps_longitude: Option<f64>,
    /// GPS 海拔
    pub gps_altitude: Option<f64>,
}

impl Default for ExifData {
    fn default() -> Self {
        Self {
            date_time_original: None,
            make: None,
            model: None,
            lens_model: None,
            focal_length: None,
            f_number: None,
            iso_speed: None,
            exposure_time: None,
            gps_latitude: None,
            gps_longitude: None,
            gps_altitude: None,
        }
    }
}

/// 从图片文件中提取 EXIF 数据
pub fn extract_exif(path: &Path) -> Result<ExifData, AppError> {
    let file = std::fs::File::open(path)
        .map_err(|e| AppError::Internal(format!("Failed to open file for EXIF reading: {}", e)))?;

    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = Reader::new();

    let exif = match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => exif,
        Err(_) => {
            // 没有 EXIF 数据或读取失败，返回空数据
            return Ok(ExifData::default());
        }
    };

    let mut data = ExifData::default();

    // 拍摄时间
    if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        data.date_time_original = parse_exif_datetime(&field.display_value().to_string());
    }

    // 相机制造商
    if let Some(field) = exif.get_field(Tag::Make, In::PRIMARY) {
        data.make = Some(field.display_value().to_string());
    }

    // 相机型号
    if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
        data.model = Some(field.display_value().to_string());
    }

    // 镜头型号
    if let Some(field) = exif.get_field(Tag::LensModel, In::PRIMARY) {
        data.lens_model = Some(field.display_value().to_string());
    }

    // 焦距
    if let Some(field) = exif.get_field(Tag::FocalLength, In::PRIMARY) {
        data.focal_length = Some(field.display_value().to_string());
    }

    // 光圈
    if let Some(field) = exif.get_field(Tag::FNumber, In::PRIMARY) {
        data.f_number = Some(field.display_value().to_string());
    }

    // ISO
    if let Some(field) = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY) {
        data.iso_speed = field.display_value().to_string().parse().ok();
    }

    // 曝光时间
    if let Some(field) = exif.get_field(Tag::ExposureTime, In::PRIMARY) {
        data.exposure_time = Some(field.display_value().to_string());
    }

    // GPS 坐标
    data.gps_latitude = extract_gps_coordinate(&exif, Tag::GPSLatitude, Tag::GPSLatitudeRef);
    data.gps_longitude = extract_gps_coordinate(&exif, Tag::GPSLongitude, Tag::GPSLongitudeRef);

    // GPS 海拔
    if let Some(field) = exif.get_field(Tag::GPSAltitude, In::PRIMARY) {
        if let Some(value) = parse_rational(&field.display_value().to_string()) {
            data.gps_altitude = Some(value);
        }
    }

    Ok(data)
}

/// 解析 EXIF 日期时间格式
fn parse_exif_datetime(datetime_str: &str) -> Option<NaiveDateTime> {
    // EXIF 日期格式通常是 "YYYY:MM:DD HH:MM:SS"
    let cleaned = datetime_str.replace(':', "-").replacen('-', ":", 2);
    NaiveDateTime::parse_from_str(&cleaned, "%Y-%m-%d %H:%M:%S").ok()
}

/// 提取 GPS 坐标
fn extract_gps_coordinate(exif: &exif::Exif, coord_tag: Tag, ref_tag: Tag) -> Option<f64> {
    let coord_field = exif.get_field(coord_tag, In::PRIMARY)?;
    let ref_field = exif.get_field(ref_tag, In::PRIMARY)?;

    let coord_str = coord_field.display_value().to_string();
    let ref_str = ref_field.display_value().to_string();

    let value = parse_gps_coordinate(&coord_str)?;

    // 根据方向调整符号
    let multiplier = if ref_str == "S" || ref_str == "W" {
        -1.0
    } else {
        1.0
    };

    Some(value * multiplier)
}

/// 解析 GPS 坐标字符串
fn parse_gps_coordinate(coord_str: &str) -> Option<f64> {
    // GPS 坐标格式通常是 "deg, min, sec"
    let parts: Vec<&str> = coord_str.split(',').collect();
    if parts.len() != 3 {
        return None;
    }

    let degrees = parse_rational(parts[0].trim())?;
    let minutes = parse_rational(parts[1].trim())?;
    let seconds = parse_rational(parts[2].trim())?;

    Some(degrees + minutes / 60.0 + seconds / 3600.0)
}

/// 解析分数字符串（如 "1/100"）
fn parse_rational(s: &str) -> Option<f64> {
    if let Some(slash_pos) = s.find('/') {
        let numerator: f64 = s[..slash_pos].trim().parse().ok()?;
        let denominator: f64 = s[slash_pos + 1..].trim().parse().ok()?;
        if denominator != 0.0 {
            return Some(numerator / denominator);
        }
    } else {
        return s.trim().parse().ok();
    }
    None
}
