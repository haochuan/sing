extern crate clap;
extern crate sing;
use clap::{App, Arg};
use spinner::SpinnerBuilder;
use std::fs;

fn main() {
    let matches = App::new("sing")
        .version("0.1")
        .about("Generate music based on text/file. Enjoy!")
        .arg(
            Arg::with_name("text")
                .short("t")
                .long("text")
                .value_name("text")
                .help("string/text you wanna listen. E.g sing -t Hello")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("file")
                .help("file you wanna listen. E.g sing -f ./input.txt")
                .takes_value(true),
        )
        .get_matches();

    let mut input: Vec<u8> = vec![];

    // check for text input
    if let Some(text) = matches.value_of("text") {
        input = String::from(text).into_bytes();
    }

    // checkfor file input
    if let Some(path) = matches.value_of("file") {
        let content = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(_) => {
                println!("Cannot read {}, use 'Hello World' to generate.", path);
                String::from("Hello World")
            }
        };
        input = content.into_bytes();
    };

    if !input.is_empty() {
        let _sp = SpinnerBuilder::new("Playing music...".into()).start();
        sing::play_music(input);
    } else {
        println!("There's nothing we can use to generate music :(");
        println!("Please run sing -h to see the available usage.");
    }
}
