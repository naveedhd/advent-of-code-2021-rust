use counter::Counter;
use itertools::Itertools;
use std::collections::HashMap;

type Pair = (char, char);
type Template = Counter<Pair, u128>;
type InsertionRules = HashMap<Pair, char>;
type CharCounter = Counter<char, u128>;

pub fn solve(filestring: &str) {
    let (template_str, insertion_rules_str) = filestring.split_once("\n\n").unwrap();
    let mut template = template_str.chars().tuple_windows::<(_, _)>().collect();

    let parse_insertion_line = |line: &str| -> (Pair, char) {
        let (key_str, val_str) = line.split_once(" -> ").unwrap();
        let key_str_bytes = key_str.as_bytes();

        (
            (key_str_bytes[0] as char, key_str_bytes[1] as char),
            val_str.chars().nth(0).unwrap(),
        )
    };

    let insertion_rules = insertion_rules_str
        .lines()
        .map(parse_insertion_line)
        .collect();
    let mut char_counter: CharCounter = template_str.chars().collect();

    let step = |template: &mut Template,
                char_counter: &mut CharCounter,
                insertion_rules: &InsertionRules| {
        let mut new_template = Template::new();
        for (pair, count) in template.iter() {
            if let Some(&val) = insertion_rules.get(&pair) {
                new_template[&(pair.0, val)] += *count;
                new_template[&(val, pair.1)] += *count;

                char_counter[&val] += *count;
            }
        }

        *template = new_template;
    };

    for _ in 0..10 {
        step(&mut template, &mut char_counter, &insertion_rules);
    }
    let most_common_ordered = char_counter.most_common_ordered();
    println!(
        "{}",
        most_common_ordered.first().unwrap().1 - most_common_ordered.last().unwrap().1
    );

    for _ in 0..30 {
        step(&mut template, &mut char_counter, &insertion_rules);
    }
    let most_common_ordered = char_counter.most_common_ordered();
    println!(
        "{}",
        most_common_ordered.first().unwrap().1 - most_common_ordered.last().unwrap().1
    );
}
