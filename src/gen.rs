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

// パスワードの種類に対応する文字種のベクタ
fn get_charkind_vector(passkind: PassKind) -> Vec<CharKind> {
    match passkind {
        Alphanum => vec![Number, Lowercase, Uppercase], // alphabets and numbers
        Alphabets => vec![Lowercase, Uppercase],        // alphabets and numbers
        Numbers => vec![Number],                        // only numbers
        All => vec![Number, Lowercase, Uppercase, Sign],
    }
}

// 引数で与えられた文字種の中からランダムに１つ返す
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

// 引数に文字種とその長さ、そしてパスワードの文字列をとり、引数で与えられた長さ分の文字種からのランダムな値をパスワードに追加する。
fn put_char_to_password(mut password: String, length: u32, kind: CharKind) -> String {
    if length <= 0 {
        return password;
    }
    for _n in 1..=length {
        let c: char = get_char_by_charkind(kind);
        password.push(c);
    }

    password
}

pub fn get_password(passkind: PassKind, length: u32) -> String {
    let mut rng = thread_rng();
    let mut password: String = String::new();

    let charkind_array: Vec<CharKind> = get_charkind_vector(passkind); // 文字種のベクタ
    let charkind_len = charkind_array.len(); // 文字種の種類数

    // パスワードに各文字種が少なくとも一文字以上にするために、
    // 初めに各文字種の一文字分パスワードに追加する。
    for kind in &charkind_array {
        password = put_char_to_password(password, 1, *kind);
    }

    // 残りのパスワードの文字数を記憶する変数を定義。
    // 初期値はパスワードの長さからその文字種の種類を引いたものである。
    let mut remaining_len = length - charkind_len as u32;

    // それぞれの文字種に対して、0以上で残った文字数以下の個数の値をパスワードに追加する。
    for (i, kind) in charkind_array.into_iter().enumerate() {
        // iと文字種の種類数が等しい時、つまり繰り返しの最後は残った文字数分追加する。
        if i + 1 == charkind_len {
            password = put_char_to_password(password, remaining_len, kind);
        } else {
            let random_number = rng.gen_range(0..=remaining_len);
            remaining_len -= random_number;

            password = put_char_to_password(password, random_number, kind);
        }
    }

    // 生成されたパスワードをシャッフルしてよりランダムなものにする
    let mut bytes = password.into_bytes();
    bytes.shuffle(&mut rng);

    String::from_utf8(bytes).expect("Error: Fail to change bytes to utf8")
}
