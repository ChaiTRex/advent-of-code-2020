use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct PasswordRule {
    positions: (u8, u8),
    char: u8,
}

impl PasswordRule {
    fn new(positions: (u8, u8), char: u8) -> PasswordRule {
        PasswordRule { positions, char }
    }

    fn old_policy_allows(&self, password: &[u8]) -> bool {
        (self.positions.0..=self.positions.1)
            .contains(&(password.iter().copied().filter(|&x| x == self.char).count() as u8))
    }

    fn new_policy_allows(&self, password: &[u8]) -> bool {
        password
            .get(self.positions.0 as usize - 1)
            .map_or(false, |&x| x == self.char)
            ^ password
                .get(self.positions.1 as usize - 1)
                .map_or(false, |&x| x == self.char)
    }
}

fn main() {
    let values = {
        let mut contents = String::new();
        let _ = File::open("../02.txt")
            .unwrap()
            .read_to_string(&mut contents);

        contents
            .lines()
            .map(|line| {
                let mut iter = line.split(|c| matches!(c, '-' | ':' | ' '));
                let rule = PasswordRule::new(
                    (
                        iter.next().unwrap().parse().unwrap(),
                        iter.next().unwrap().parse().unwrap(),
                    ),
                    iter.next().unwrap().as_bytes()[0],
                );
                let password = iter.nth(1).unwrap().as_bytes().to_vec();
                (rule, password)
            })
            .collect::<Vec<_>>()
    };

    println!(
        "{}",
        values
            .iter()
            .filter(|(rule, password)| rule.old_policy_allows(&password))
            .count()
    );

    println!(
        "{}",
        values
            .iter()
            .filter(|(rule, password)| rule.new_policy_allows(&password))
            .count()
    );
}
