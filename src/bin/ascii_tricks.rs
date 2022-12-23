

fn main() {

    let s = "This is a string!";
    let abc = "abcdefghijklmnopqrstuvwxyz";

    to_alphabetic_ordinals(s);
    to_alphabetic_ordinals(abc);
    ()
}

fn to_alphabetic_ordinals(s: &str) {

    let ords: Vec<u32> = s.chars().filter(char::is_ascii_alphabetic).map(|c| {
        (u32::from(c) & 0x1f) - 1
    }).collect();

    println!("This:\n\t{}\nIs:\n\t{:?}", s, ords);
}