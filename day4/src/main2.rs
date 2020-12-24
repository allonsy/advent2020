use std::collections::HashSet;
use util;

struct Passport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

struct PassportBuilder {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl PassportBuilder {
    fn new() -> PassportBuilder {
        PassportBuilder {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn set_birth_year(&mut self, year: String) {
        let parsed_year = str::parse::<i32>(&year);
        if parsed_year.is_err() {
            return;
        }
        let parsed_year = parsed_year.unwrap();
        if parsed_year >= 1920 && parsed_year <= 2002 {
            self.birth_year = Some(year);
        }
    }

    fn set_issue_year(&mut self, year: String) {
        let parsed_year = str::parse::<i32>(&year);
        if parsed_year.is_err() {
            return;
        }
        let parsed_year = parsed_year.unwrap();
        if parsed_year >= 2010 && parsed_year <= 2020 {
            self.issue_year = Some(year);
        }
    }

    fn set_expiration_year(&mut self, year: String) {
        let parsed_year = str::parse::<i32>(&year);
        if parsed_year.is_err() {
            return;
        }
        let parsed_year = parsed_year.unwrap();
        if parsed_year >= 2020 && parsed_year <= 2030 {
            self.expiration_year = Some(year);
        }
    }

    fn set_height(&mut self, mut height: String) {
        let hgt_len = height.len();
        let hgt_unit = height.split_off(hgt_len - 2);
        let parsed_height = str::parse::<i32>(&height);
        if parsed_height.is_err() {
            return;
        }

        let parsed_height = parsed_height.unwrap();
        match hgt_unit.as_str() {
            "cm" => {
                if parsed_height < 150 || parsed_height > 193 {
                    return;
                }
            }
            "in" => {
                if parsed_height < 59 || parsed_height > 76 {
                    return;
                }
            }
            _ => return,
        }
        self.height = Some(height);
    }

    fn set_hair_color(&mut self, mut hair_color: String) {
        let color_code = hair_color.split_off(1);
        if hair_color != "#" {
            return;
        }

        if color_code.len() != 6 {
            return;
        }

        let valid_chars: HashSet<char> = vec![
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ]
        .into_iter()
        .collect();

        for digit in color_code.chars() {
            if !valid_chars.contains(&digit) {
                return;
            }
        }

        self.hair_color = Some(color_code);
    }

    fn set_eye_color(&mut self, eye_color: String) {
        let valid_colors: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .collect();
        if !valid_colors.contains(eye_color.as_str()) {
            return;
        }
        self.eye_color = Some(eye_color);
    }

    fn set_passport_id(&mut self, passport_id: String) {
        if passport_id.len() != 9 {
            return;
        }
        let parsed_passport_id = str::parse::<usize>(&passport_id);
        if parsed_passport_id.is_err() {
            return;
        }
        self.passport_id = Some(passport_id);
    }

    fn set_country_id(&mut self, country_id: String) {
        self.country_id = Some(country_id);
    }

    fn build(self) -> Option<Passport> {
        Some(Passport {
            birth_year: self.birth_year?,
            issue_year: self.issue_year?,
            expiration_year: self.expiration_year?,
            height: self.height?,
            hair_color: self.hair_color?,
            eye_color: self.eye_color?,
            passport_id: self.passport_id?,
            country_id: self.country_id,
        })
    }
}

fn main() {
    let input = util::read_file_string("input.txt");

    let mut num_valid = 0;
    let mut builder = PassportBuilder::new();

    for line in input.lines() {
        if line.is_empty() {
            let passport = builder.build();
            if passport.is_some() {
                num_valid += 1;
            }
            builder = PassportBuilder::new();
        } else {
            let words: Vec<&str> = line.split(" ").collect();

            for word in words {
                let map: Vec<&str> = word.split(":").collect();
                let code = map[0];
                let val = map[1];
                match code {
                    "byr" => builder.set_birth_year(val.to_string()),
                    "iyr" => builder.set_issue_year(val.to_string()),
                    "eyr" => builder.set_expiration_year(val.to_string()),
                    "hgt" => builder.set_height(val.to_string()),
                    "hcl" => builder.set_hair_color(val.to_string()),
                    "ecl" => builder.set_eye_color(val.to_string()),
                    "pid" => builder.set_passport_id(val.to_string()),
                    "cid" => builder.set_country_id(val.to_string()),
                    catch => panic!("Unknown code: {}", catch),
                }
            }
        }
    }

    println!("{}", num_valid);
}
