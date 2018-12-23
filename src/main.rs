#[macro_use]
extern crate clap;
extern crate regex;
extern crate syllables;

use clap::App;

use std::fs::File;
use std::io::Read;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut text = String::from("");

    // File args take priority
    let path = matches.value_of("file").unwrap_or("");
    if path != "" {
        text = handle_file_input(path).unwrap_or(String::from(""));
    }

    // If nothing was read from a file, get input through command line
    if text == "" {
        text = match matches.value_of("INPUT") {
            Some(v) => v.to_string(),
            None => {
                println!("Enter a line of words to count syllables in: \n");

                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                input.trim().to_string()
            }
        };
    }

    print_syllable_info(&text);
}

/// Safely attempts to read the text in a file at a given path.
fn handle_file_input(path: &str) -> Result<String, std::io::Error> {
    let mut text = String::new();

    match File::open(path) {
        Ok(mut f) => {
            f.read_to_string(&mut text)?;
            Ok(text.trim().to_string())
        }
        Err(e) => Err(e),
    }
}

/// Count the number of syllables in each word in a single line of text.
fn count_line_syllables_per_word(line: &str) -> Vec<(&str, usize)> {
    let words = line.split_whitespace();

    let mut syllable_counts: Vec<(&str, usize)> = vec![];
    for word in words {
        let count = syllables::syllables_in_word(word);

        syllable_counts.push((word, count));
    }

    syllable_counts
}

/// Count the number of syllables in each "line" of text,
/// split by newline characters.
fn count_line_syllables(text: &str) -> Vec<(&str, usize)> {
    let lines = text.split('\n');

    let mut syllables: Vec<(&str, usize)> = vec![];
    for line in lines {
        let count = syllables::syllables_in_words(&line);

        syllables.push((&line, count));
    }

    syllables
}

/// Calculates the number of syllables in each word and line,
/// and pretty prints the results.
fn print_syllable_info(text: &str) {
    let lines = text.split('\n');

    let counts = count_line_syllables(&text);

    // Find length of longest line
    let mut longest = 4; // min spacing for header
    for count in counts {
        if count.0.len() > longest {
            longest = count.0.len();
        }
    }

    print!(
        "\n{}   {:>spaces$}",
        "Text",
        "Syllables",
        spaces = longest + 5
    );
    print!(
        "\n{:-<width1$}   {:-<width2$}\n",
        "",
        "",
        width1 = longest,
        width2 = "Syllables".len()
    );

    for line in lines {
        let word_counts = count_line_syllables_per_word(line);
        print_syllable_counts_per_word(&word_counts);

        let total = count_line_syllables(&line)[0].1;

        // Don't print anything if syllable count is 0 (i. e. empty line)
        if total == 0 { continue}

        let spacing = longest - line.len() + 1;
        print!("{}   {:spaces$}", line, total, spaces = spacing);

        //print!("{}", total[0].1);

        print!("\n\n");
    }
}

/// Prints the number of syllables in each word in input.
/// The numbers are spaced so as to be centered above their
/// respective word if the line was printed below.
///
/// Takes one line at a time.
fn print_syllable_counts_per_word(counts: &Vec<(&str, usize)>) {
    for count in counts {
        let word_len = count.0.len();

        print!("{:^1$} ", count.1, word_len);
    }

    println!();
}
