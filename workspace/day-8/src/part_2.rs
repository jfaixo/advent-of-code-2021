use crate::models::Input;

pub fn sum_output(input: &Input) -> usize {
    let mut sum = 0;

    for entry in &input.data {
        let mut value = 0;
        for digit_pattern in entry.values.clone() {
            value *= 10;
            value += entry.pattern_to_digit(digit_pattern);
        }
        sum += value;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::part_2::sum_output;

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

        assert_eq!(61229, sum_output(&input));
    }
}
