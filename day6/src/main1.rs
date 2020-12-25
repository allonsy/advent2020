use std::collections::HashSet;
use util;

struct Group {
    questions: HashSet<char>,
}

impl Group {
    fn new() -> Group {
        Group {
            questions: HashSet::new(),
        }
    }

    fn add_question(&mut self, question: char) {
        self.questions.insert(question);
    }

    fn get_score(&self) -> usize {
        self.questions.len()
    }
}

fn main() {
    let input = util::read_file_string("input.txt");
    let mut this_group = Group::new();
    let mut total = 0;

    for line in input.lines() {
        if line.is_empty() {
            total += this_group.get_score();
            this_group = Group::new();
        } else {
            for ch in line.chars() {
                this_group.add_question(ch);
            }
        }
    }

    println!("{}", total);
}
