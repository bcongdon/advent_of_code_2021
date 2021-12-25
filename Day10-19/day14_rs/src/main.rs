use std::collections::BTreeMap;

fn parse_line(line: &str) -> (char, char, char) {
    let mut pieces = line.split(" -> ");
    let input = pieces.next().unwrap();
    let output = pieces.next().unwrap();
    let (a, b) = (input.chars().next().unwrap(), input.chars().nth(1).unwrap());
    let c = output.chars().next().unwrap();
    (a, b, c)
}

fn expand_template(
    templ_counter: BTreeMap<(char, char), i64>,
    rules: &BTreeMap<(char, char), char>,
) -> BTreeMap<(char, char), i64> {
    let mut out = BTreeMap::new();
    for (&pair @ (a, c), count) in templ_counter.iter() {
        if let Some(&b) = rules.get(&pair) {
            *out.entry((a, b)).or_default() += count;
            *out.entry((b, c)).or_default() += count;
        } else {
            *out.entry(pair).or_default() += count;
        }
    }
    out
}

fn calculate_score(templ_counter: &BTreeMap<(char, char), i64>) -> i64 {
    let mut char_counts = BTreeMap::<char, (i64, i64)>::new();
    for ((a, b), count) in templ_counter.iter() {
        char_counts.entry(*a).or_default().0 += count;
        char_counts.entry(*b).or_default().1 += count;
    }
    let char_counts = char_counts
        .values()
        .map(|(a, b)| *a.max(b))
        .collect::<Vec<_>>();
    let diff = char_counts.iter().max().unwrap() - char_counts.iter().min().unwrap();
    diff
}

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let template = lines.next().unwrap();
    let rules = lines
        .skip(1)
        .map(parse_line)
        .map(|(a, b, c)| ((a, b), c))
        .collect::<BTreeMap<_, _>>();

    let mut template_counter = BTreeMap::<(char, char), i64>::new();
    for pair in template.chars().zip(template.chars().skip(1)) {
        *template_counter.entry(pair).or_default() += 1;
    }

    for idx in 0..40 {
        template_counter = expand_template(template_counter, &rules);
        if idx + 1 == 10 {
            println!("Part 1: {}", calculate_score(&template_counter));
        } else if idx + 1 == 40 {
            println!("Part 2: {}", calculate_score(&template_counter));
        }
    }
}
