use std::env;
use bitflags::bitflags;



pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

bitflags! {
    pub struct Flags: u8 {
        const R = 0b00000001;
        const L = 0b00000010;
        const P = 0b00000100;
    }
}

fn parse_url(args: Vec<String>) -> (Flags, u32, String, String) {
    let mut flags = Flags::empty();
    let mut url = String::new();
    let mut lim: u32 = 0;

    for i in &args {
        match (*i).as_str() {
            "-r" => flags.insert(Flags::R),
            "-l" => {
                flags.insert(Flags::L);
                // let y = i + 1;
                // lim = y.as_str().parse();
            }
            "-p" => flags.insert(Flags::P),
            s if s.starts_with('-') => {
                for c in s.chars().skip(1) {
                    match c {
                        'r' => flags.insert(Flags::R),
                        'l' => flags.insert(Flags::L),
                        'p' => flags.insert(Flags::P),
                        _ => {
                            eprintln!("Usage: ./spider [-rlp] [N] [PATH] URL");
                            std::process::exit(1);
                        }
                    }
                }
            }
            _ => url.push_str((*i).as_str()),
        }
    }

    if flags.contains(Flags::L) && !flags.contains(Flags::R) {
        eprintln!("Usage: flag -l cannot be used without -r");
        std::process::exit(1);
    }
    if url.is_empty() {
        eprintln!("Usage: ./spider [-rlp] [N] [PATH] URL");
        std::process::exit(1);
    }
    
    (flags, lim, String::new(), url)

}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    let url = parse_url(args);

    //debug

    if url.0.contains(Flags::R) {
        println!("Flag -r is set");
    }
    if url.0.contains(Flags::L) {
        println!("Flag -l is set");
    }
    if url.0.contains(Flags::P) {
        println!("Flag -p is set");
    }
    println!("Url to scrap: {}", url.3);

    Ok(())
}
