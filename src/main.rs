/// Takes in an EPUB file and parses it, returning a list of Kanji  
/// sorted by frequency.  
/// The kanji found are compared to a dictionary from the kanjidic project.
/// (link)[https://www.edrdg.org/wiki/index.php/KANJIDIC_Project]
/// And finally the .txt file is formatted so that it can be easily uploaded to
/// Anki.  
///  
/// Usage:  
/// cargo run japanesebook.epub myfile.txt  
///     - output is printed to `myfile.txt`  
/// cargo run mybook.epub  
///     - output is printed to 'output.txt'  

#[macro_use]
extern crate lazy_static;
extern crate epub;
extern crate regex;

use epub::doc::EpubDoc;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let kanjidict = "kanjidic2.xml";

    let book = EpubDoc::new(&args[1]);
    assert!(book.is_ok());

    //determines whether the a file should be written to
    let shouldprint = if args.len() < 3 { true } else { false };

    let mut file = if !shouldprint {
        std::fs::File::create(&args[2]).unwrap()
    } else {
        std::fs::File::create("output.txt").unwrap()
    };

    let mut book = book.unwrap();

    assert_eq!(0, book.get_current_page());
    assert_eq!("application/xhtml+xml", book.get_current_mime().unwrap());

    //dictionary parsing
    let mut kanjis: Vec<Kanji> = Vec::new();
    let mut kanji = String::from(""); //.to_string();
    let mut onyomi: String = "".to_string();
    let mut kunyomi: String = "".to_string();
    let mut meaning: String = "".to_string();

    if let Ok(lines) = read_lines(kanjidict) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(mut ip) = line {
                if ip.contains("<literal>") {
                    kanji.push_str(&ip.remove(9).to_string());
                }

                if ip.contains("<reading r_type=\"ja_on\">") {
                    if onyomi != "" {
                        onyomi.push_str(", ");
                    }
                    onyomi.push_str(
                        &ip.replace("<reading r_type=\"ja_on\">", "")
                            .replace("</reading>", ""),
                    );
                }

                if ip.contains("<reading r_type=\"ja_kun\">") {
                    if kunyomi != "" {
                        kunyomi.push_str(", ");
                    }
                    kunyomi.push_str(
                        &ip.replace("<reading r_type=\"ja_kun\">", "")
                            .replace("</reading>", ""),
                    );
                }

                if ip.contains("<meaning>") {
                    if meaning != "" {
                        meaning.push_str(", ");
                    }
                    meaning.push_str(&ip.replace("<meaning>", "").replace("</meaning>", ""));
                }

                if ip.contains("</character>") {
                    let word = &kanji;
                    kanjis.push(Kanji {
                        kanji: word.to_string(),
                        onyomi: onyomi.to_string(),
                        kunyomi: kunyomi.to_string(),
                        meaning: meaning.to_string(),
                    });
                    kanji.clear();
                    onyomi.clear();
                    kunyomi.clear();
                    meaning.clear();
                }
            }
        }
    }
    //end dictionary parsing

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

    //sorts by frequency
    kanjifreq.sort_by(|b, a| (a.1).cmp(&b.1));

    //compares dictionary to book lists
    let mut merged_list: Vec<&Kanji> = Vec::new();
    let mut leftovers: Vec<&(char, u64)> = Vec::new();

    for kanji in &kanjifreq {
        let mut found = false;

        for dict_kanji in &kanjis {
            if dict_kanji.kanji.contains(&kanji.0.to_string()) {
                merged_list.push(dict_kanji);
                found = true;
            }
        }

        if !found {
            leftovers.push(&kanji);
        }
    }
    //end comparisons

    //prints to console or writes to file
    for kanji in merged_list {
        write!(
            file,
            "{}; {}; {}; {}\n",
            kanji.kanji, kanji.meaning, kanji.onyomi, kanji.kunyomi
        )
        .expect("fail");
    }

    if leftovers.len() > 0 {
        println!("These kanjis were not found in the dictionary:");
    }
    for kanji in leftovers {
        println!("{:?}", kanji);
    }
}

//makes sure regex only compiles once
fn regex_speed_helper(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\p{Han}").unwrap();
    }
    RE.is_match(text)
}

#[derive(Debug)]
struct Kanji {
    kanji: String,
    onyomi: String,
    kunyomi: String,
    meaning: String,
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
