# Command Line Kanji Frequency Tool

Takes in an epub book and prints the kanji, sorted by highest frequency back.  

example: `cargo run example.epub output.txt`  

The frequency tool will print its results to the terminal if no output text file is given.

In my testing, some books could take over a minute.  

## Running the app  

Install [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and cargo.  

clone, build, and run  

`git clone 'https://github.com/LukewarmCoffee/epub-kanji-frequency.git'`  

`cd epub-kanji-frequency`  

`cargo build`  

`cargo run [your epub file].epub`  
