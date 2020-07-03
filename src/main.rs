use std::io::{self, BufRead};
use std::string::String;

fn rot (text: String, alphabet: String, rot: usize) -> String {
    let res_alphabet = get_result_alphabet(alphabet.clone(), rot);
    text.chars().fold(String::from(""), |acc, x| {
        let  mut a = acc; 
        a.push_str(""); 
        a.push_str(match res_alphabet.chars().nth(alphabet.chars().position(|c| c == x).unwrap()) {
            Some(ret) => ret,
            _ => x
        }.to_string().as_str());
        a
    })
}

fn get_begining(rot: usize, len: usize) -> usize {
    rot % len
}

fn get_result_alphabet (alphabet: String, rot: usize) -> String {
    let split = alphabet.split_at(get_begining(rot, alphabet.len()));
    let mut res = String::from(split.1);
    res.push_str(split.0);
    res
}

fn main() -> io::Result<()> {
    
    let stdin = io::stdin();
    let en_alphabet = String::from("abcdefghijklmnopqrstuvwxyz");

    for i in 0..en_alphabet.len() {
        println!("{}: {}", i, rot(String::from("text"), en_alphabet.clone(), i));
    }

    Ok(())
}
