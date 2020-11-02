/// Takes in an EPUB file and parses it, returning a list of Kanji
/// sorted by frequency

#[macro_use]
extern crate lazy_static;
extern crate epub;
extern crate regex;

use epub::doc::EpubDoc;
use regex::Regex;
use std::env;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let book = EpubDoc::new(&args[1]);
    assert!(book.is_ok());

    //determines whether the a file should be written to
    let shouldprint = if args.len() < 3 { true } else { false };

    let mut file = if !shouldprint {
        std::fs::File::create(&args[2]).unwrap()
    } else {
        std::fs::File::create("dummy.txt").unwrap()
    };

    let mut book = book.unwrap();

    assert_eq!(0, book.get_current_page());
    assert_eq!("application/xhtml+xml", book.get_current_mime().unwrap());

    //gets frequencies of all chars in book
    let mut charsfreq: Vec<(char, u64)> = Vec::new();

    while book.go_next().is_ok() {
        let page = book.get_current_str().unwrap();

        for c in page.chars() {
            let mut matchfound = false;

            for value in charsfreq.iter_mut() {
                if c == value.0 {
                    value.1 = value.1 + 1;
                    matchfound = true;
                }
            }

            if !matchfound {
                charsfreq.push((c, 1));
            }
        }
    }
    //end book parsing

    //reduces the characters to only kanji
    let mut kanjifreq: Vec<(char, u64)> = Vec::new();

    for value in charsfreq.iter() {
        if regex_speed_helper(&value.0.to_string()) {
            kanjifreq.push((value.0, value.1));
        }
    }

    kanjifreq.sort_by(|b, a| (a.1).cmp(&b.1));

    for freq in kanjifreq.iter() {
        if shouldprint {
            print!("{:?}\n", freq);
        } else {
            write!(file, "{:?}\n", freq).expect("fail");
        }
    }
}

//makes sure regex only compiles once
fn regex_speed_helper(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\p{Han}").unwrap();
    }
    RE.is_match(text)
}
