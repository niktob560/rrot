use std::io::{self, BufRead};
use std::string::String;
use std::env;

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
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 || args.get(1).unwrap().to_string() == "--help" {
        println!("Usage:");
        println!("\trrot");
        println!("\trrot $shift");
        println!("\techo $text | rrot $shift");
        return Ok(());
    }

    let rot_shift: usize;

    let stdin = io::stdin();
    let en_alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    let alphabet_len = en_alphabet.len();

    if args.len() == 1 {
        rot_shift = 13;
    }
    else {

        let rot_str: String = match args.get(1) {
            Some(x) => x.to_string(),
            _ => "13".to_string()
        };
        
        let rot_shift_raw = match rot_str.parse::<i32>() {
            Ok(x) => x,
            Err(_) => 13
        };

        if rot_shift_raw < 0 {
            rot_shift = (alphabet_len as i32 + (rot_shift_raw % alphabet_len as i32)) as usize;
        }
        else {
            rot_shift = (rot_shift_raw % alphabet_len as i32) as usize;
        }
    }


    for line in stdin.lock().lines() {
        println!("{}", rot(line?, en_alphabet.clone(), rot_shift));
    }

    Ok(())
}
