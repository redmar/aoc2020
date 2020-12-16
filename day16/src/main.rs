mod part1;

use part1::Part1;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = Part1::parse(input);

    let mut invalid_fields: Vec<u32> = Vec::new();

    for ticket in &part1.nearby_tickets {
        for field in ticket {
            if !part1.is_inside_all_ranges(*field) {
                invalid_fields.push(*field);
            }
        }
    }
    let part1_solution: u32 = invalid_fields.iter().sum();
    println!("part1_solution = {}", part1_solution);

    // part2
    let part2 = part1.clone();

    // make a mapping of field_name to all valid columns (columns go vertical over all tickets)
    // col0 is ticket1[0],ticket2[0],ticket3[0],...
    // col1 is ticket1[1],ticket2[1],ticket3[1],...
    let mut field_to_valid_cols: HashMap<String, Vec<usize>> = HashMap::new();
    for fields in part2.rules.keys().enumerate() {
        let mut columns_valid: Vec<usize> = Vec::new();
        for idx in 0..20 {
            if part2.column_idx_all_valid_for_field(idx, fields.1) {
                columns_valid.push(idx);
            }
        }
        field_to_valid_cols.insert(fields.1.clone(), columns_valid);
    }

    // find field mappings by reducing it based on a first single field to idx mapping
    let mut mapping: HashMap<String, usize> = HashMap::new();
    loop {
        if let Some((field_name, columns)) = field_to_valid_cols.clone().iter().find(|(_,v)| v.len() == 1) {
            let col_idx = columns.iter().last().unwrap();

            for entry in field_to_valid_cols.iter_mut() {
                entry.1.retain(|value| value != col_idx);
            }
            mapping.insert(field_name.clone(), *col_idx);
            field_to_valid_cols.remove(field_name);
        } else {
            break;
        }
    }

    let departure_fields_sum: u64 = mapping
        .iter()
        .filter_map(|(k,v)| if k.starts_with("departure") { Some(v) } else { None })
        .map(|idx| part2.ticket[*idx] as u64)
        .product();

    println!("part2 solution = {}", departure_fields_sum);
}
