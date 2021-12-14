use crate::models::Input;

pub fn count_1_4_7_8(input: &Input) -> usize {
    let mut count = 0;

    for entry in &input.data {
        for value in entry.values.clone() {
            if value == entry.patterns[entry.value_to_pattern_index[1]]
                || value == entry.patterns[entry.value_to_pattern_index[4]]
                || value == entry.patterns[entry.value_to_pattern_index[7]]
                || value == entry.patterns[entry.value_to_pattern_index[8]]
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::part_1::count_1_4_7_8;

    #[test]
    fn part1_example_case() {
        let content =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"
            .to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(26, count_1_4_7_8(&input));
    }
}
