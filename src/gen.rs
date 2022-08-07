use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use CharKind::*;
use PassKind::*;

pub enum PassKind {
    Alphanum,  // alphabets and numbers
    Alphabets, // alphabets
    Numbers,   // only numbers
    All,       // all kind of Char
}

#[derive(Copy, Clone)]
pub enum CharKind {
    Number,
    Lowercase,
    Uppercase,
    Sign,
}

fn get_charkind_vector(passkind: PassKind) -> Vec<CharKind> {
    match passkind {
        Alphanum => vec![Number, Lowercase, Uppercase], // alphabets and numbers
        Alphabets => vec![Lowercase, Uppercase],        // alphabets and numbers
        Numbers => vec![Number],                        // only numbers
        All => vec![Number, Lowercase, Uppercase, Sign],
    }
}

fn get_char_by_charkind(kind: CharKind) -> char {
    let mut rng = thread_rng();
    let ascii: u8 = match kind {
        Number => rng.gen_range(48..=57),
        Lowercase => rng.gen_range(97..=122),
        Uppercase => rng.gen_range(65..=90),
        Sign => rng.gen_range(58..=64),
    };

    ascii as char
}

fn put_char_to_password(mut password: String, length: i32, kind: CharKind) -> String {
    for _n in 1..=length {
        let c: char = get_char_by_charkind(kind);
        password.push(c);
    }

    password
}

pub fn get_password(passkind: PassKind, length: i32) -> String {
    let mut rng = thread_rng();
    let mut password: String = String::new();
    let mut len = length;
    let array: Vec<CharKind> = get_charkind_vector(passkind);

    let mut max: i32;
    let mut random_number: i32;
    let cklen = array.len() as i32;

    for (i, kind) in array.into_iter().enumerate() {
        let i = i as i32;
        max = len - cklen - 1 + i;
        if cklen == i + 1 {
            random_number = len;
        } else {
            random_number = rng.gen_range(1..=max);
        }
        password = put_char_to_password(password, random_number, kind);
        len -= random_number;
    }

    let mut bytes = password.into_bytes();
    bytes.shuffle(&mut rng);
    String::from_utf8(bytes).expect("Error in changing bytes to utf8")
}
