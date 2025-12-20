use std::str::FromStr;

use crate::filter::*;
use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgb};
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum FilterOperation {
    Palette,
    Pixelate(u32),
    FloydSteinberg,
    Reverse,
}

impl std::str::FromStr for FilterOperation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "pal" {
            Ok(FilterOperation::Palette)
        } else if s == "floyd" {
            Ok(FilterOperation::FloydSteinberg)
        } else if s == "rev" {
            Ok(FilterOperation::Reverse)
        } else if let Some(size_str) = s.strip_prefix("pix=") {
            let size = size_str.parse::<u32>()
                .map_err(|_| format!("Invalid pixel size {}", size_str))?;
            if size == 0 {
                return Err("Pixel size must be greater than 0".to_string());
            }
            Ok(FilterOperation::Pixelate(size))
        } else if s == "pix" {
            Ok(FilterOperation::Pixelate(8))
        } else {
            Err(format!("Unknown filter operation: {}", s))
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "image-filter")]
#[command(about = "Apply various filters to images", long_about = None)]
struct Args {
    #[arg(short = 'f', long = "filter", required = true, value_delimiter = ',')]
    filters: Vec<String>,

    #[arg(value_name = "INPUT", required = true)]
    input_path: String,

    #[arg(value_name = "OUTPUT", required = true)]
    output_path: String,
}

pub fn apply() {
    let args = Args::parse();

    let operations: Vec<FilterOperation> = match args.filters.iter()
        .map(|s| FilterOperation::from_str(s))
        .collect::<Result<Vec<_>, _>>() {
            Ok(ops) => ops,
            Err(e) => {
                eprintln!("Error {e}");
                return;
            }
        };


    let mut image: DynamicImage = match image::open(&args.input_path) {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to load image {}: {}", args.input_path, e);
            return;
        }
    };

    let mut gray_image_option: Option<GrayImage> = None;

    for op in operations {
        println!("Applying {:?}...", op);

        match op {
            FilterOperation::Palette => {
                if gray_image_option.is_some() {
                    let gray: ImageBuffer<Luma<u8>, Vec<u8>> = gray_image_option.take().unwrap();
                    image = DynamicImage::ImageLuma8(gray).into();
                }
                let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                    apply_palette(&image, "palette.json");
                image = DynamicImage::ImageRgb8(rgb_image);
                gray_image_option = None;
            }
            FilterOperation::Pixelate(size) => {
                if gray_image_option.is_some() {
                    let gray: ImageBuffer<Luma<u8>, Vec<u8>> = gray_image_option.take().unwrap();
                    image = DynamicImage::ImageLuma8(gray).into();
                }
                let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> = pixelate(&image, size);
                image = DynamicImage::ImageRgb8(rgb_image);
                gray_image_option = None;
            }
            FilterOperation::FloydSteinberg => {
                let gray_image: ImageBuffer<Luma<u8>, Vec<u8>> =
                    apply_floyd_steinberg_dithering(&image);
                gray_image_option = Some(gray_image);
            }
            FilterOperation::Reverse => {
                if gray_image_option.is_some() {
                    let gray: ImageBuffer<Luma<u8>, Vec<u8>> = gray_image_option.take().unwrap();
                    image = DynamicImage::ImageLuma8(gray).into();
                }
                let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> = reverse(&image);
                image = DynamicImage::ImageRgb8(rgb_image);
                gray_image_option = None;
            }
        }
    }

    if let Some(gray_image) = gray_image_option {
        save(&args.output_path, gray_image);
    } else {
        match image.save(&args.output_path) {
            Ok(_) => println!("The image is saved: {}", args.output_path),
            Err(e) => println!("Failed to save image {}: {}", args.output_path, e),
        }
    }
}
