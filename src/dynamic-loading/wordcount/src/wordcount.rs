use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct WordCount {
    counts: HashMap<PathBuf, Vec<usize>>,
}

impl WordCount {
    pub fn new() -> WordCount {
        WordCount::default()
    }

    pub fn count_file(&mut self, filename: PathBuf, contents: &str) {
        let count = contents.split(" ").count();
        self.counts.entry(filename)
            .or_insert_with(Default::default)
            .push(count);
    }

    pub fn report(&self) {
        println!("Word count for {} files", self.counts.len());

        for (key, value) in &self.counts {
            println!("{}", key.display());
            
            for count in value {
                println!("\t{}", count);
            }
        }
    }
}
