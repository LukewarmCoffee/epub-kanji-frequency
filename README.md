# Command Line Kanji Frequency Tool

Takes in an epub book and prints the kanji to a file, sorted by highest frequency back.  

The kanji are compared to the [kanjidic](https://www.edrdg.org/wiki/index.php/KANJIDIC_Project) dictionary. The output format can be easily imported into Anki.  

Field 1 (front) is the kanji, 2 is meaning, 3 is on'yomi (represented in katakana), and field 4 is kun'yomi in hiragana.  

example: `cargo run example.epub output.txt`  

In my testing, some books could take over a minute.  

## Running the app  

Install [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and cargo.  

clone, build, and run via a development build:    

`git clone 'https://github.com/LukewarmCoffee/epub-kanji-frequency.git'`  

`cd epub-kanji-frequency`  

`cargo build`  

`cargo run [your epub file].epub`  
