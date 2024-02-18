use std::io;

struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    fn count_words(&self) -> Result<usize, &'static str> {
        if self.text.trim().is_empty() {
            return Err("Error: Text is empty");
        }

        let words: Vec<&str> = self.text.split_whitespace().collect();
        Ok(words.len())
    }
}

fn main() {
    println!("Enter a text:");

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text)
        .expect("Failed to read line");

    let word_counter = WordCounter::new(&input_text);

    let word_count = word_counter.count_words();
    println!("Number of words in the text: {}", word_count);
}
