use image::{GenericImageView};
use rand::Rng;
 use rand::prelude::SliceRandom;
use std::{thread, time::Duration};

fn get_ascii(intensity: u8, inverted: bool) -> char { 
    let ascii_values = [' ', '.', ',', '-', '+', '=', '{', '$'];
    
    // intensity 0 - 255, but want indexes from 0 - 8. thus /32
    let index = intensity / 32;

    if inverted {
        return ascii_values[(7 - index) as usize];
    }
    return ascii_values[index as usize];
}

fn get_next_state(ch: char) -> char {
    let chars = match ch {
        '{' | '}' => vec!['{', '}', '(', ')'],
        '$' => vec!['$', '€', '£', '¥'],
        '=' => vec!['=', '≠', '≈', '≡'],
        '+' => vec!['+', '-', '*', 'o'],
        '.' => vec!['.', ':', ';', ','],
        '-' => vec!['-', '_', '~'], 
        _ => vec![ch],
    };
    let mut rng = rand::thread_rng();
    return *chars.choose(&mut rng).unwrap();
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

fn animate_image(ascii_image: Vec<Vec<char>>, frames: usize, fps: u64) {

    // remember: delay = 1000/FPS
    let delay = 1000/fps;
    for _ in 0..frames {
        // clean screen
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        //print!("{esc}[2J{esc}[H", esc = 27 as char);
       let mut frame = String::new();

        for row in &ascii_image {
            for &ch in row {
                frame.push(get_next_state(ch));
            }
            frame.push('\n');
        }
        print!("\x1B[2J\x1B[1;1H{}", frame);

        thread::sleep(Duration::from_millis(delay));
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
    let ascii_image = process_image("pepe.png", 8, true);
    animate_image(ascii_image, 10000, 15);
}
