use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

// monster regex and unwrap fest! yeah don't do this in production code ;-)
fn match_bag_line(line: &str) -> Result<(String, HashMap<String, u32>)> {
    let re = Regex::new(
        r"\A(?P<subject>.+)\sbags contain (?P<contains>(?P<nr>\d+) (?P<b2>.+)\sbags?[,.]+)+",
    )
    .unwrap();
    let mut items = HashMap::new();
    match re.captures(&line) {
        Some(caps) => {
            let subject: String = caps.name("subject").unwrap().as_str().into();
            let contains = caps.name("contains").unwrap();

            let re = Regex::new(r"(?U)((\d+) (.+) bags?[,.])+").unwrap();
            for caps in re.captures_iter(contains.as_str()) {
                let bag_amount = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let bag_type = caps.get(3).unwrap().as_str().to_owned();
                items.insert(bag_type, bag_amount);
            }

            Ok((subject, items))
        }
        None => {
            let re = Regex::new(r"\A(?P<subject>.+)\sbags contain no other bags").unwrap();
            let caps = re.captures(&line).unwrap();
            let subject: String = caps.name("subject").unwrap().as_str().into();
            Ok((subject, items))
        }
    }
}

fn find_unique_bag_colors(
    needle: &str,
    bags: &HashMap<String, HashMap<String, u32>>,
) -> HashSet<String> {
    let mut total: HashSet<String> = HashSet::new();

    for (bag, containing_bags) in bags.to_owned() {
        if containing_bags.contains_key(needle) && bag != needle {
            // let mut bags_without_current = bags.clone();
            // bags_without_current.remove(bag).unwrap();
            total.insert(bag.to_owned());
            total = total
                .union(&find_unique_bag_colors(&bag, &bags))
                .cloned()
                .collect();
        }
    }
    total
}

fn find_total_containing_bags(needle: &str, bags: &HashMap<String, HashMap<String, u32>>) -> u32 {
    if let Some(containing_bags) = bags.get(needle) {
        let mut totals = 0;
        for (bag_name, amount) in containing_bags {
            totals += amount + (amount * find_total_containing_bags(&bag_name, &bags));
        }
        totals
    } else {
        0
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut bags = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (name, contains) = match_bag_line(line).unwrap();
        bags.insert(name, contains);
    }

    println!(
        "part1 = {:?}",
        find_unique_bag_colors("shiny gold", &bags).len()
    );

    println!(
        "part2 = {:?}",
        find_total_containing_bags("shiny gold", &bags)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_strs() {
        let input = "light lime bags contain 4 pale lime bags, 5 plaid green bags, 1 clear turquoise bag, 3 plaid yellow bags.";
        let mut expected_items = HashMap::new();
        expected_items.insert("pale lime".to_string(), 4);
        expected_items.insert("plaid green".to_string(), 5);
        expected_items.insert("clear turquoise".to_string(), 1);
        expected_items.insert("plaid yellow".to_string(), 3);

        assert_eq!(
            match_bag_line(&input).unwrap(),
            ("light lime".to_string(), expected_items)
        );
    }

    #[test]
    fn test_empty_bags() {
        let input = "dotted black bags contain no other bags.";
        let mut expected_items: HashMap<String, u32> = HashMap::new();
        assert_eq!(
            match_bag_line(&input).unwrap(),
            ("dotted black".to_string(), HashMap::new())
        );
    }
}
