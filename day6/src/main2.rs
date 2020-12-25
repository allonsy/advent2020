use std::collections::HashSet;
use util;

struct Group {
    questions: HashSet<char>,
    is_new: bool,
}

impl Group {
    fn new() -> Group {
        Group {
            questions: HashSet::new(),
            is_new: true,
        }
    }

    fn add_question_line(&mut self, questions: &str) {
        let mut new_answers: HashSet<char> = HashSet::new();

        for ch in questions.chars() {
            new_answers.insert(ch);
        }

        if self.is_new {
            self.questions = new_answers;
            self.is_new = false;
            return;
        }

        self.questions = new_answers
            .intersection(&self.questions)
            .map(|c| c.clone())
            .collect();
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
            this_group.add_question_line(line);
        }
    }

    println!("{}", total);
}
