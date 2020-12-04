use crate::utils;
use regex::Regex;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        return Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
    }

    fn is_valid(&self) -> bool {
        return self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
    }

    fn is_strict_valid(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        if !utils::is_numeric_string_in_range(self.byr.as_ref().unwrap(), 1920, 2002) {
            return false;
        }

        if !utils::is_numeric_string_in_range(self.iyr.as_ref().unwrap(), 2010, 2020) {
            return false;
        }

        if !utils::is_numeric_string_in_range(self.eyr.as_ref().unwrap(), 2020, 2030) {
            return false;
        }

        if let Some(height) = self.hgt.as_ref() {
            if height.ends_with("cm") {
                if let Some(height_value_str) = height.strip_suffix("cm") {
                    if !utils::is_numeric_string_in_range(height_value_str, 150, 193) {
                        return false;
                    }
                } else {
                    return false;
                }
            } else if height.ends_with("in") {
                if let Some(height_value_str) = height.strip_suffix("in") {
                    if !utils::is_numeric_string_in_range(height_value_str, 59, 76) {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(hair_colour) = self.hcl.as_ref() {
            let re = Regex::new(r"#[0-9a-f]").unwrap();
            if !re.is_match(hair_colour) {
                return false;
            }
        }

        if let Some(eye_colour) = self.ecl.as_ref() {
            match eye_colour.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => return false,
            }
        }

        if let Some(passport_id) = self.pid.as_ref() {
            if passport_id.len() != 9 {
                return false;
            }

            if let Err(_) = passport_id.parse::<i32>() {
                return false;
            }
        }

        return true;
    }
}

#[allow(dead_code)]
pub fn problem1() {
    println!("running problem 4.1:");

    if let Ok(lines) = utils::read_lines("data/day4.txt") {
        let mut passport = Passport::new();
        let mut num_valid = 0;

        for res in lines {
            if let Ok(line) = res {
                // end of passport, so check if we got everything
                if line == "" {
                    if passport.is_valid() {
                        num_valid += 1;
                    }

                    passport = Passport::new();
                    continue;
                }

                for data in line.split(" ") {
                    let pair: Vec<&str> = data.split(":").collect();

                    let value = String::from(pair[1]);
                    match pair[0] {
                        "byr" => passport.byr = Some(value),
                        "iyr" => passport.iyr = Some(value),
                        "eyr" => passport.eyr = Some(value),
                        "hgt" => passport.hgt = Some(value),
                        "hcl" => passport.hcl = Some(value),
                        "ecl" => passport.ecl = Some(value),
                        "pid" => passport.pid = Some(value),
                        "cid" => passport.cid = Some(value),
                        _ => {}
                    }
                }
            }
        }

        // check the last passport
        if passport.is_valid() {
            num_valid += 1;
        }
        println!("Found valid: {}", num_valid);
    }
}

#[allow(dead_code)]
pub fn problem2() {
    println!("running problem 4.2:");

    if let Ok(lines) = utils::read_lines("data/day4.txt") {
        let mut passport = Passport::new();
        let mut num_valid = 0;

        for res in lines {
            if let Ok(line) = res {
                // end of passport, so check if we got everything
                if line == "" {
                    if passport.is_strict_valid() {
                        num_valid += 1;
                    }

                    passport = Passport::new();
                    continue;
                }

                for data in line.split(" ") {
                    let pair: Vec<&str> = data.split(":").collect();

                    let value = String::from(pair[1]);
                    match pair[0] {
                        "byr" => passport.byr = Some(value),
                        "iyr" => passport.iyr = Some(value),
                        "eyr" => passport.eyr = Some(value),
                        "hgt" => passport.hgt = Some(value),
                        "hcl" => passport.hcl = Some(value),
                        "ecl" => passport.ecl = Some(value),
                        "pid" => passport.pid = Some(value),
                        "cid" => passport.cid = Some(value),
                        _ => {}
                    }
                }
            }
        }

        // check the last passport
        if passport.is_valid() {
            num_valid += 1;
        }
        println!("Found valid: {}", num_valid);
    }
}
