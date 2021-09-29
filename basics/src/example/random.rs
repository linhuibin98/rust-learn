extern crate rand;

use rand::prelude::*;

pub fn print_rand() {
    let rand_num: u8 = random();
    println!("random value: {}", rand_num);
}