use regex::Regex;
struct Passport<'a> {
    attrs: Vec<&'a str>,
}

impl Passport<'_> {
    fn parse_from(input: &str) -> Vec<Passport> {
        let mut v: Vec<Passport> = Vec::new();
        let mut current_passport = Vec::new();
        for line in input.lines() {
            if line.trim() == "" {
                v.push(Passport {
                    attrs: current_passport,
                });
                current_passport = Vec::new();
                current_passport.clear();
            } else {
                let mut passport_attrs = line.split(" ").collect::<Vec<&str>>();
                current_passport.append(&mut passport_attrs);
            }
        }
        v.push(Passport {
            attrs: current_passport,
        });
        v
    }

    fn part1_is_valid(&self) -> bool {
        let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // "cid"
        fields.iter().fold(0, |acc, field| {
            let field_in_attrs = self
                .attrs
                .iter()
                .find(|attr| attr.contains(field))
                .is_some();

            if field_in_attrs {
                acc + 1
            } else {
                acc
            }
        }) == fields.len()
    }

    fn part2_is_valid(&self) -> bool {
        let fields: Vec<(&str, Box<dyn Fn(&str) -> bool>)> = vec![
            ("byr", Box::new(|v| Passport::is_valid_byr(v))),
            ("iyr", Box::new(|v| Passport::is_valid_iyr(v))),
            ("eyr", Box::new(|v| Passport::is_valid_eyr(v))),
            ("hgt", Box::new(|v| Passport::is_valid_hgt(v))),
            ("hcl", Box::new(|v| Passport::is_valid_hcl(v))),
            ("ecl", Box::new(|v| Passport::is_valid_ecl(v))),
            ("pid", Box::new(|v| Passport::is_valid_pid(v))),
        ]; // "cid"

        fields.iter().fold(0, |acc, (field, validator)| {
            let field_in_attrs = self.attrs.iter().find(|attr| attr.contains(field));
            match field_in_attrs {
                Some(field) => {
                    if validator(field.split(":").nth(1).unwrap()) {
                        acc + 1
                    } else {
                        acc
                    }
                }
                None => acc,
            }
        }) == fields.len()
    }

    fn is_valid_byr(v: &str) -> bool {
        v.parse::<i32>()
            .map(|byr| (1920..=2002).contains(&byr))
            .unwrap_or(false)
    }
    fn is_valid_iyr(v: &str) -> bool {
        v.parse::<i32>()
            .map(|iyr| (2010..=2020).contains(&iyr))
            .unwrap_or(false)
    }
    fn is_valid_eyr(v: &str) -> bool {
        v.parse::<i32>()
            .map(|eyr| (2020..=2030).contains(&eyr))
            .unwrap_or(false)
    }
    fn is_valid_hgt(v: &str) -> bool {
        if v.ends_with("cm") {
            v.replace("cm", "")
                .parse::<i32>()
                .map(|hgt| (150..=193).contains(&hgt))
                .unwrap_or(false)
        } else if v.ends_with("in") {
            v.replace("in", "")
                .parse::<i32>()
                .map(|hgt| (59..=76).contains(&hgt))
                .unwrap_or(false)
        } else {
            false
        }
    }
    fn is_valid_hcl(v: &str) -> bool {
        let color_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        color_regex.is_match(v)
    }
    fn is_valid_ecl(v: &str) -> bool {
        let color_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        color_regex.is_match(v)
    }
    fn is_valid_pid(v: &str) -> bool {
        let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();
        pid_regex.is_match(v)
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let passports = Passport::parse_from(&input);
    println!("nr of total passports = {}", passports.len());
    println!(
        "part1 nr of \"valid\" passports = {}",
        passports.iter().filter(|pp| pp.part1_is_valid()).count()
    );
    println!(
        "part2 nr of \"valid\" passports = {}",
        passports.iter().filter(|pp| pp.part2_is_valid()).count()
    );
}
