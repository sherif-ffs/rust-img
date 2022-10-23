use image::{GenericImageView, ImageBuffer, Pixel, Rgba, DynamicImage};
use rand::Rng; // 0.8.5

fn main() {
  let img = image::open("./smok.jpg").expect("File not found!");
  let (w, h) = img.dimensions();
  let mut output = ImageBuffer::new(w, h); // create a new buffer for our output
  let (w, h) = img.dimensions();
  
  let mut blur = |x, y| {
    let test_rand = rand::thread_rng().gen_range(1..250);
    let new_x = if x+test_rand < w { x+test_rand } else { 0 };
    let new_y = if y+test_rand < h { y+test_rand } else { 0 };
  
    output.put_pixel(x, y,img.get_pixel(new_x, new_y));
  };

  for (x, y, pixel) in img.pixels() {
    blur(x,y);
  }

  // blue
  for (x, y, pixel) in img.pixels() {
    let rand_r= rand::thread_rng().gen_range(20..30);
    let rand_g= rand::thread_rng().gen_range(60..80);
    let rand_b= rand::thread_rng().gen_range(100..150);
    let new_pixel = image::Rgba([rand_r, rand_g, rand_b, 255]);

    if x % 2 == 0 && y % 2 != 0 {
      output.put_pixel(x, y, new_pixel);
    }
  }
  
  // red
  // for (x, y, pixel) in img.pixels() {
  //   let rand_r= rand::thread_rng().gen_range(180..210);
  //   let rand_g= rand::thread_rng().gen_range(0..10);
  //   let rand_b= rand::thread_rng().gen_range(10..40);
  //   let new_pixel = image::Rgba([rand_r, rand_g, rand_b, 255]);

  //   if x % 2 == 0 && y % 2 != 0 {
  //     output.put_pixel(x, y, new_pixel);
  //   }
  // }

  output.save("output4.png").unwrap();
}