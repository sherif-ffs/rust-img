use image::{GenericImageView, ImageBuffer, save_buffer_with_format};
use rand::Rng; // 0.8.5
use image::ImageFormat::Png;
use std::cmp::{ max };

fn main() {
  let img = image::open("./input_photos/license.jpg").expect("File not found!");
  let (w, h) = img.dimensions();

  // create a new buffer for our output
  let mut output = ImageBuffer::new(w, h); 
  
  let pixel_count = img.pixels().count();
  println!("{:?}", pixel_count);
  println!("{:?}", w);
  println!("{:?}", h);
  let mut blur = |x, y| {
    let test_rand = rand::thread_rng().gen_range(1..max(w, h) /10);
    let new_x = if x+test_rand < w { x+test_rand } else { 0 };
    let new_y = if y+test_rand < h { y+test_rand } else { 0 };
  
    output.put_pixel(x, y, img.get_pixel(new_x, new_y));
  };

  for (x, y, _pixel) in img.pixels() {
    blur(x,y);
  }

  for (x, y, _pixel) in img.pixels() {
    blur(x,y);
  }

  // blue
  for (x, y, _pixel) in img.pixels() {
    let rand_r= rand::thread_rng().gen_range(20..30);
    let rand_g= rand::thread_rng().gen_range(60..80);
    let rand_b= rand::thread_rng().gen_range(100..150);
    let new_pixel = image::Rgba([rand_r, rand_g, rand_b, 255]);

    if x % 2 == 0 {
      output.put_pixel(x, y, new_pixel);
    }
  }
  
    // redish
    // for (x, y, _pixel) in img.pixels() {
    //   let rand_r= rand::thread_rng().gen_range(220..250);
    //   let rand_g= rand::thread_rng().gen_range(100..120);
    //   let rand_b= rand::thread_rng().gen_range(80..110);
    //   let new_pixel = image::Rgba([rand_r, rand_g, rand_b, 255]);
  
    //   if x % 2 == 0 && y % 2 == 0 {
    //     output.put_pixel(x, y, new_pixel);
    //   }
    // }

  save_buffer_with_format("./output_photos/output_1.png", &output, w, h, image::ColorType::Rgba8, Png).unwrap_or_else(|err| println!("{:?}", err)) ;
}