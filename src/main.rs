use std::env::args;

use rand::{distributions::DistString, prelude::*};

#[derive(Debug, Clone, Copy)]
pub struct PasswordCharacters;

impl Distribution<u8> for PasswordCharacters {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10 + 32;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789\
                -_!\"#$%&'*+,./:;=?@\\^`|~[]{}()<>";
        loop {
            let var = rng.next_u32() >> (32 - 7);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

impl DistString for PasswordCharacters {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        unsafe {
            let v = string.as_mut_vec();
            v.extend(self.sample_iter(rng).take(len));
        }
    }
}
fn main() {
    let arg: Vec<String> = args().skip(1).take(2).collect();

    let size = match &arg[..] {
        [name, value, ..] if name == "--strength" => value.parse::<usize>().unwrap(),
        _ => 24,
    };

    println!("generating...");
    let mut r = rand_isaac::Isaac64Rng::from_entropy();
    let pass = PasswordCharacters.sample_string(&mut r, size);
    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_text(&pass).unwrap();
    println!("password copied in your clipboard");
}
