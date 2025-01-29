use std::{env, process::exit, collections::HashSet, fs::File, io::copy};
use bitflags::bitflags;
use scraper;
use reqwest;

#[derive(Debug)]
enum ParsingError {
    UnknownFlag(String),
    FlagWithMissingArg(String),
    MissingUrl(String),
    FlagWithInvArg(String),
    FlagLWithoutR(String),
}

// enum ScrapError {

// }

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

struct  Scraper {
    flags: Flags,
    limit: u32,
    path: String,
    url: String,
    visited: HashSet<String>,
}
/*
struct Image {
    url: Option<String>,
    name: Option<String>,
}*/

impl Scraper {
    pub fn new(scraper: (Flags, u32, String, String)) -> Self {
        let mut a = Scraper {
            flags: scraper.0,
            limit: scraper.1,
            path: scraper.2,
            url: scraper.3,
            visited: HashSet::new(),
        };
        a
    }
}

fn parse_url(args: Vec<String>) -> Result<(Flags, u32, String, String), ParsingError> {
    let mut flags = Flags::empty();
    let mut url = String::new();
    let mut lim: u32 = 0;
    let mut path = String::new();
    let mut args_iter = args.iter();

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-r" => {
                flags.insert(Flags::R);
                lim = 5;
            }
            "-l" => {
                flags.insert(Flags::L);
                if let Some(next_arg) = args_iter.next() {
                    lim = next_arg.parse().unwrap();
                }
                else {
                    return Err(ParsingError::FlagWithMissingArg("Usage: flag -l needs a value".to_string()));
                }
            }
            "-p" => {
                flags.insert(Flags::P);
                if let Some(next_arg) = args.iter().next() {
                    path = next_arg.to_string();
                }
                else {
                    return Err(ParsingError::FlagWithMissingArg("Usage: flag -p needs a value".to_string()));
                }
            }
            _ => url.push_str(arg.as_str()),
        }
    }

    if flags.contains(Flags::L) && !flags.contains(Flags::R) {
        return Err(ParsingError::FlagLWithoutR("Usage: flag -l cannot be used without -r".to_string())); 
    }
    if url.is_empty() {
        return Err(ParsingError::MissingUrl("Usage: ./spider [-rlp] [N] [PATH] URL".to_string()));
    }
    
    Ok((flags, lim, path, url))

}

fn init(args: Vec<String>) -> Result<Scraper, ParsingError> {
    let url_result = parse_url(args);
    
    let scrap = match url_result {
        Ok((flags, lim, path, url)) => (flags, lim, path, url),
        Err(ParsingError) => {
            eprintln!("Error: {:?}", ParsingError);
            exit(1);
        }
    };
    
    Ok(Scraper::new(scrap))
}

fn recursive_web_scraping(scrap: &mut Scraper) -> Result<(), String> {

    let filename = "new.jpeg";

    let mut response = reqwest::blocking::get(scrap.url.as_str()).expect("Failed to dl img");
    let mut file = File::create(filename).expect("Failed to create file");
    copy(&mut response, &mut file).expect("Failed to save image");
    Ok(())
}

fn scrap_process(scrap: &mut Scraper) -> Result<(), String> {
    //if scrap.flags.contains(Flags::R) {
    recursive_web_scraping(scrap);
    //}
    //else {
    //    single_web_scraping(scrap);
    //}
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut scrap = match init(args) {
        Ok(new_scrap) => new_scrap,
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown")),
    };
    scrap_process(&mut scrap);
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parsing_basic_with_flags() {
//         let test = vec!["-r".to_string(), "-l".to_string(), "3".to_string(), "-p".to_string(), "./tests".to_string(), "aboulore.42.fr".to_string()];
//         let res = Scraper {
//             flags: Flags::R | Flags::L | Flags::P,
//             limit: 3,
//             path: String::from("./tests".to_string()),
//             url: String::from("aboulore.42.fr".to_string()),
//         };
//         let test_res = init(test);
//         assert_eq!(init(test), Ok(res));
//     }
// }
