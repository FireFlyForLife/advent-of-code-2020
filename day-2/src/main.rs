mod puzzle_input;

use lazy_static::lazy_static;
use regex::Regex;



fn count_valid_passwords(database_passwords: &Vec<DatabasePassword>) -> i32 {
    let mut valid_password_count = 0;
    
    for database_password in database_passwords {
        let mut char_count = 0;
        for c in database_password.password.chars() {
            if c == database_password.character {
                char_count += 1;
            }
        }

        if char_count >= database_password.char_count_min && char_count <= database_password.char_count_max {
            valid_password_count += 1;
        }
    }

    valid_password_count
}

fn count_valid_passwords_new_policy(database_passwords: &Vec<DatabasePassword>) -> i32 {
    let mut valid_password_count = 0;
    
    for database_password in database_passwords {
        let first_char_valid = match database_password.password.chars().nth((database_password.char_count_min-1) as usize) {
            Some(c) => { c == database_password.character }
            None => { false }
        };

        let second_char_valid = match database_password.password.chars().nth((database_password.char_count_max-1) as usize){
            Some(c) => { c == database_password.character }
            None => { false }
        };

        if first_char_valid as i32 + second_char_valid as i32 == 1 {
            valid_password_count += 1;
        }
    }

    valid_password_count
}

struct DatabasePassword{
    character: char,
    char_count_min: i32,
    char_count_max: i32,

    password: String
}

fn parse_database_password(line: &'static str) -> DatabasePassword
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<cnt_min>\d{1,2})-(?P<cnt_max>\d{1,2}) (?P<char>[a-z]): (?P<password>[a-z]+)")
            .expect("Regex for parsing database_passwords failed to compile");
    }

    let captures = RE.captures(line).unwrap();

    DatabasePassword{
        character: captures["char"].chars().nth(0).unwrap(),
        char_count_min: captures["cnt_min"].parse().unwrap(),
        char_count_max: captures["cnt_max"].parse().unwrap(),
        password: captures["password"].to_owned(),
    }
}

fn main() {
    let database_passwords: Vec<_> = puzzle_input::PASSWORDS.lines().map(parse_database_password).collect();

    let valid_passwords = count_valid_passwords(&database_passwords);
    let valid_passwords_new_policy = count_valid_passwords_new_policy(&database_passwords);
    println!("Valid password count: {}", valid_passwords);
    println!("Valid password count (New policy): {}", valid_passwords_new_policy);
}
