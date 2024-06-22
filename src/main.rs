use image::{GenericImageView};
use rand::prelude::SliceRandom;
use std::{thread, time::Duration, io::{self, Write}};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ascii_animator")]
struct Opt {
    #[structopt(short, long, default_value = "pepe.png")]
    image: String,

    #[structopt(short, long, default_value = "20")]
    fps: u64,

    #[structopt(short, long)]
    evil: bool, //default is false. if --evil is used ascii will be inversed
}

fn get_ascii(intensity: u8, inverted: bool) -> char { 
    let ascii_values = [' ', '.', ',', '-', '+', '=', '{', '$'];
    
    // intensity 0 - 255, but want indexes from 0 - 8, thus /32
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

fn animate_image(ascii_image: &mut Vec<Vec<char>>, frames: usize, fps: u64) {
    let delay = 1000 / fps; 
    for _ in 0..frames {
        let mut frame = String::new();

        for row in &mut *ascii_image {
            for &mut ch in row {
                frame.push(get_next_state(ch));
            }
            frame.push('\n');
        }

        print!("{esc}[2J{esc}[H{}", frame, esc = 27 as char);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(delay as u64));
    }
}

fn main() {
    let opt = Opt::from_args();
    let mut ascii_image = process_image(&opt.image, 8, opt.evil);
    animate_image(&mut ascii_image, 100000, opt.fps); 
}
