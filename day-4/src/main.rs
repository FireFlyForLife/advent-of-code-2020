mod puzzle_input;

use validator::{Validate, ValidationError};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::hash_map::HashMap;
use std::error::Error;

lazy_static! {
    static ref RE_COLOR_HEX: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
}

#[derive(Debug, Validate)]
struct Passport{
    #[validate(range(min=1920, max=2002))]
    birth_year: i32,
    #[validate(range(min=2010, max=2020))]
    issue_year: i32,
    #[validate(range(min=2020, max=2030))]
    expiration_year: i32,
    #[validate(custom = "validate_height")]
    height: &'static str,
    #[validate(regex = "RE_COLOR_HEX")]
    hair_color: &'static str,
    #[validate(custom = "validate_eye_color")]
    eye_color: &'static str,
    #[validate(length(equal = 9))]
    passport_id: &'static str,
    country_id: Option<&'static str>, 
}
fn validate_height(height: &str) -> Result<(), ValidationError>{
    let height_num = height[0..height.len()-2].parse::<i32>().map_err(|_err| ValidationError::new("Height number parsing failed"))?;
    
    if height.ends_with("cm") {
        if height_num >= 150 && height_num <= 193 {
            Ok(())
        } else {
            Err(ValidationError::new("Height is out of bounds"))
        }
    } else if height.ends_with("in") {
        if height_num >= 59 && height_num <= 76 {
            Ok(())
        } else {
            Err(ValidationError::new("Height is out of bounds"))
        }
    } else {
        Err(ValidationError::new("Not a known height format"))
    }
}

fn validate_eye_color(eye_color: &str) -> Result<(), ValidationError>{
    const VALID_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry" ,"grn", "hzl", "oth"]; 

    if VALID_COLORS.iter().any(|valid_color| eye_color == *valid_color){
        Ok(())
    } else {
        Err(ValidationError::new("Eye colour is not valid!"))
    }
}

fn parse_passport_properties(properties_str: &'static str) -> Result<Passport, Box<dyn Error>> {
    let properties: HashMap<&'static str, &'static str> = properties_str.split_whitespace().map(|property_str: & str| { 
        let mut property_str_it = property_str.split(":");

        (property_str_it.next().unwrap(), property_str_it.next().unwrap())
    }).collect();

    let passport = Passport{
        birth_year:         properties.get("byr").ok_or(ValidationError::new("Missing Property"))?.parse()?,
        issue_year:         properties.get("iyr").ok_or(ValidationError::new("Missing Property"))?.parse()?,
        expiration_year:    properties.get("eyr").ok_or(ValidationError::new("Missing Property"))?.parse()?,
        height:             properties.get("hgt").ok_or(ValidationError::new("Missing Property"))?,
        hair_color:         properties.get("hcl").ok_or(ValidationError::new("Missing Property"))?,
        eye_color:          properties.get("ecl").ok_or(ValidationError::new("Missing Property"))?,
        passport_id:        properties.get("pid").ok_or(ValidationError::new("Missing Property"))?,
        country_id:         properties.get("cid").map(|val| *val),
    };

    passport.validate()?;

    Ok(passport)
}

fn main() {
    let passports: Vec<_> = puzzle_input::PASSPORTS.split("\n\n").map(parse_passport_properties).collect();

    let valid_passports = passports.iter().filter(|result| Result::is_ok(result)).count();
    println!("Valid passports count: {}", valid_passports);
}
