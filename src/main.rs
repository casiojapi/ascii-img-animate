use image::{GenericImageView};

fn get_ascii(intensity: u8, inverted: bool) -> &'static str{ 
    let ascii_values = [" ", ".", ",", "-", "+", "=", "{", "$"];
    
    // intensity 0 - 255, but want indexes from 0 - 8. thus /32
    let index = intensity / 32;

    if inverted {
        return ascii_values[(7 - index) as usize];
    }
    return ascii_values[index as usize];
}

fn process_image(dir: &str, scale: u32, inverted: bool) {
    let img = image::open(dir).unwrap();
    let (width, height) = img.dimensions();
    println!("width: {}, height: {}.", width, height);
    for y in 0..height {
        for x in 0..width {
            if y % scale == 0 && x % scale == 0 {
                let pixel = img.get_pixel(x, y);
                let mut intensity = pixel[0]/3 + pixel[1]/3 + pixel[2]/3;
                print!("{}", get_ascii(intensity, inverted));
            } 
        }
        if y % scale == 0 {
            println!("");
        }
    }
}
fn main() {
    process_image("donuts.png", 4, false);
    process_image("pepe.png", 8, true);
}
