use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Part1 {
    pub rules: HashMap<String, Vec<Range<u32>>>,
    pub ticket: Vec<u32>,
    pub nearby_tickets: Vec<Vec<u32>>,
}

impl Part1 {
    fn parse_rule(line: &str) -> (String, Vec<Range<u32>>) {
        let v: Vec<&str> = line.split(':').collect();
        let name = v[0].to_string();
        let ranges: Vec<Range<u32>> = v[1].split("or").map(|r| {
            let range: Vec<&str> = r.split('-').collect();
            Range {
                start: range[0].trim().parse::<u32>().unwrap(),
                end: range[1].trim().parse::<u32>().unwrap() + 1,
            }
        }).collect();
        (name, ranges)
    }

    fn parse_ticket(line: &str) -> Vec<u32> {
        line.split(',').map(|x| x.parse::<u32>().unwrap()).collect()
    }

    pub fn parse(input: &str) -> Self {
        let mut rules = HashMap::new();
        let mut nearby_tickets = Vec::new();

        // parse rules
        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }
            let x = Self::parse_rule(line);
            rules.insert(x.0, x.1);
        }

        // parse your ticket
        let _empty_line = lines.next();
        let ticket: Vec<u32> = Self::parse_ticket(lines.next().unwrap());
        let _empty_line = lines.next();
        let _empty_line = lines.next();
        while let Some(line) = lines.next() {
            let nearby_ticket: Vec<u32> = Self::parse_ticket(line);
            nearby_tickets.push(nearby_ticket);
        }

        Self {
            rules,
            ticket,
            nearby_tickets,
        }
    }

    pub fn all_ranges(&self) -> Vec<&Range<u32>> {
        self.rules.values().flatten().collect()
    }

    pub fn is_inside_all_ranges(&self, value: u32) -> bool {
        self.all_ranges().iter().any(|range| range.contains(&value))
    }

    pub fn is_valid_ticket(&self, ticket: &Vec<u32>) -> bool {
        for field in ticket {
            if !self.is_inside_all_ranges(*field) {
                return false;
            }
        }
        true
    }

    fn valid_tickets(&self) -> Vec<&Vec<u32>> {
        self
            .nearby_tickets
            .iter()
            .filter(|ticket| self.is_valid_ticket(ticket))
            .collect()
    }

    pub fn ticket_values_for_column_idx(&self, column_idx: usize) -> Vec<u32> {
        self.valid_tickets().iter().fold(Vec::new(), |mut acc, ticket| {
            acc.push(ticket[column_idx]);
            acc
        })
    }

    pub fn column_idx_all_valid_for_field(&self, column_idx: usize, field: &str) -> bool {
        let field_ranges = self.rules.get(field).unwrap();
        let column_values = self.ticket_values_for_column_idx(column_idx);

        for value in column_values {
            if !field_ranges.iter().any(|range| range.contains(&value)) {
                return false;
            }
        }
        true

        // column_values.iter().all(|field| {
        //     field_ranges.iter().any(|range| range.contains(&field))
        // })
    }
}
