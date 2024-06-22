use image::{GenericImageView};

fn get_ascii(intensity: u8, inverted: bool) -> char { 
    let ascii_values = [' ', '.', ',', '-', '+', '=', '{', '$'];
    
    // intensity 0 - 255, but want indexes from 0 - 8. thus /32
    let index = intensity / 32;

    if inverted {
        return ascii_values[(7 - index) as usize];
    }
    return ascii_values[index as usize];
}

fn process_image(dir: &str, scale: u32, inverted: bool) -> Vec<Vec<char>> {
    let img = image::open(dir).unwrap();
    let (width, height) = img.dimensions();

    let mut ascii_image = Vec::new();

    for y in (0..height).step_by((scale * 2) as usize) {
        let mut row = Vec::new();
        for x in (0..width).step_by(scale as usize) {
            let pixel = img.get_pixel(x, y);
            let intensity = pixel[0]/3 + pixel[1]/3 + pixel[2]/3;
            row.push(get_ascii(intensity, inverted));
        }
        ascii_image.push(row);
    }
    return ascii_image;
}

fn animate_image(ascii_image: Vec<Vec<char>>, frames: usize, delay: u64) {
    // clean screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for row in &ascii_image {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn display_image(ascii_image: Vec<Vec<char>>) {
    for row in &ascii_image {
        for &ch in row {
            print!("{}", ch); 
        }
        println!();
    }
}
fn main() {
    display_image(process_image("pepe.png", 8, true))
    //process_image("pepe.png", 8, true);
}
