use util;

struct PasswordLine {
    lower_bound: i32,
    upper_bound: i32,
    letter: char,
    password: String,
}

impl PasswordLine {
    fn parse(line: &str) -> PasswordLine {
        let word_split: Vec<&str> = line.split(":").collect();
        let password = word_split[1].trim().to_string();
        let conditions: Vec<&str> = word_split[0].split(" ").collect();
        let letter = conditions[1].chars().collect::<Vec<char>>()[0];

        let bounds: Vec<&str> = conditions[0].split("-").collect();
        let lower_bound = str::parse::<i32>(bounds[0]).unwrap();
        let upper_bound = str::parse::<i32>(bounds[1]).unwrap();

        PasswordLine {
            lower_bound,
            upper_bound,
            letter,
            password,
        }
    }

    fn is_valid(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let is_lower = chars[(self.lower_bound - 1) as usize] == self.letter;
        let is_upper = chars[(self.upper_bound - 1) as usize] == self.letter;
        is_lower != is_upper
    }
}

fn main() {
    let lines = util::read_file_lines("input.txt");

    let mut num_valid = 0;

    for line in lines.iter() {
        let this_pwd = PasswordLine::parse(line);
        if this_pwd.is_valid() {
            num_valid += 1;
        }
    }

    println!("{}", num_valid);
}
