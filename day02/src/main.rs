use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
struct PasswordWithPolicy<'a> {
    password: &'a str,
    range: Range<usize>,
    letter: &'a str,
}

impl PasswordWithPolicy<'_> {
    pub fn password_is_valid(&self) -> bool {
        let letter_count = self.password.matches(self.letter).count();
        self.range.contains(&letter_count)
    }

    pub fn toboggan_password_is_valid(&self) -> bool {
        // saving into a Range wasn't the best choice for this policy :-)
        let pos1 = self.range.start - 1; // Toboggan Corporate Policies have no concept of "index zero"!
        let pos2 = self.range.end - 2; // we added one and have to cope for the index starting at 1
        let pass: Vec<char> = self.password.chars().collect();
        let letter: char = self.letter.chars().next().unwrap();
        (pass[pos1] == letter) ^ (pass[pos2] == letter)
    }
}

// tried to this without the regex crate
// and we assume the parse can't go wrong for now
fn line_to_password_with_policy(line: &str) -> PasswordWithPolicy {
    let v: Vec<&str> = line.split(':').collect();
    let password = v[1].trim();

    let pp: Vec<&str> = v[0].split(' ').collect();
    let vrange: Vec<usize> = pp[0]
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let range = std::ops::Range {
        start: vrange[0],
        end: vrange[1] + 1, // this is exclusive so we need to add 1 to it
    };
    let letter = pp[1];

    PasswordWithPolicy {
        password,
        range,
        letter,
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "#passwords = {}",
        input.lines().collect::<Vec<&str>>().len()
    );

    let valid_passwords: Vec<PasswordWithPolicy> = input
        .lines()
        .map(|line| line_to_password_with_policy(line))
        .filter(|pwp| pwp.password_is_valid())
        .collect();

    println!("#valid_passwords = {:?}", valid_passwords.len());

    let valid_toboggan_passwords: Vec<PasswordWithPolicy> = input
        .lines()
        .map(|line| line_to_password_with_policy(line))
        .filter(|pwp| pwp.toboggan_password_is_valid())
        .collect();

    println!(
        "#valid_toboggan_passwords = {:?}",
        valid_toboggan_passwords.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_password_policy() {
        let input = "10-12 g: gggggggggmggg";
        let expected = PasswordWithPolicy {
            password: &"gggggggggmggg",
            range: Range { start: 10, end: 13 },
            letter: &"g",
        };

        assert_eq!(line_to_password_with_policy(&input), expected);
    }

    #[test]
    fn test_parse_password_is_valid() {
        let input = "10-12 g: gggggggggmggg";
        let pwp = line_to_password_with_policy(&input);

        assert!(pwp.password_is_valid());
    }

    #[test]
    fn test_toboggan_password_policy() {
        let p1 = line_to_password_with_policy("1-3 a: abcde");
        assert_eq!(p1.toboggan_password_is_valid(), true);

        let p2 = line_to_password_with_policy("1-3 b: cdefg");
        assert_eq!(p2.toboggan_password_is_valid(), false);

        let p3 = line_to_password_with_policy("2-9 c: ccccccccc");
        assert_eq!(p3.toboggan_password_is_valid(), false);
    }
}
