use anyhow::Result;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;

fn main() -> Result<()> {
  let input_path = "assets/1280.png";
  let output_path = "assets/1280_output.png";
  let radius = 72;
  let image = image::open(input_path)?;
  let rounded_image = add_rounded_corners(&image, radius);
  rounded_image.save(output_path)?;
  Ok(())
}

fn add_rounded_corners(image: &DynamicImage, radius: u32) -> RgbaImage {
  let (width, height) = image.dimensions();
  let mut rounded_image = RgbaImage::new(width, height);

  // Copy the original image to the new image
  for y in 0..height {
    for x in 0..width {
      let pixel = image.get_pixel(x, y);
      rounded_image.put_pixel(x, y, pixel);
    }
  }

  // Create a mask with rounded corners
  let mut mask = RgbaImage::new(width, height);

  // Draw rectangles
  draw_filled_rect_mut(
    &mut mask,
    Rect::at(radius as i32, 0).of_size(width - 2 * radius, height),
    Rgba([0, 0, 0, 255]),
  );
  draw_filled_rect_mut(
    &mut mask,
    Rect::at(0, radius as i32).of_size(width, height - 2 * radius),
    Rgba([0, 0, 0, 255]),
  );

  // Draw corner circles
  draw_filled_circle_mut(
    &mut mask,
    (radius as i32, radius as i32),
    radius as i32,
    Rgba([0, 0, 0, 255]),
  );
  draw_filled_circle_mut(
    &mut mask,
    ((width - radius) as i32, radius as i32),
    radius as i32,
    Rgba([0, 0, 0, 255]),
  );
  draw_filled_circle_mut(
    &mut mask,
    (radius as i32, (height - radius) as i32),
    radius as i32,
    Rgba([0, 0, 0, 255]),
  );
  draw_filled_circle_mut(
    &mut mask,
    ((width - radius) as i32, (height - radius) as i32),
    radius as i32,
    Rgba([0, 0, 0, 255]),
  );

  // Apply the mask to the rounded image
  for y in 0..height {
    for x in 0..width {
      let pixel = rounded_image.get_pixel_mut(x, y);
      let mask_pixel = mask.get_pixel(x, y);
      pixel.0[3] = mask_pixel.0[3];
    }
  }

  rounded_image
}
