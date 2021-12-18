use std::ops::Add;

pub struct SnailfishMathProblem {
    pub values: Vec<SnailfishNumber>,
}

impl SnailfishMathProblem {
    pub fn parse_string(content: String) -> SnailfishMathProblem {
        let values = content
            .lines()
            .map(|line| SnailfishNumber::parse_str(line, 0).0)
            .collect::<_>();

        SnailfishMathProblem {
            values
        }
    }

    pub fn sum(&self) -> SnailfishNumber {
        let mut n = self.values[0].clone();

        for i in 1..self.values.len() {
            n = n + self.values[i].clone();
            n.reduce();
        }

        n
    }

    pub fn solve_part_1(&self) -> i32 {
        magnitude(&self.sum())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SnailfishNumber {
    Literal {
        value: i32
    },
    Pair {
        left: Box<SnailfishNumber>,
        right: Box<SnailfishNumber>,
    },
}

impl SnailfishNumber {
    fn parse_str(number_str: &str, index: usize) -> (SnailfishNumber, usize) {
        match number_str.chars().nth(index).unwrap() {
            '[' => {
                let (left, index) = SnailfishNumber::parse_str(number_str, index + 1);
                let (right, index) = SnailfishNumber::parse_str(number_str, index);

                (SnailfishNumber::Pair { left: Box::new(left), right: Box::new(right) }, index + 1)
            }
            n => {
                let value = (n as u8 - '0' as u8) as i32;
                (SnailfishNumber::Literal { value }, index + 2)
            }
        }
    }

    fn explode(&mut self) -> bool {
        let mut queue = Vec::new();

        queue.push((Box::new(self), 0));

        let mut last_literal: Option<Box<&mut SnailfishNumber>> = None;
        let mut explosion = None;
        while queue.len() > 0 {
            let (current_node, current_depth) = queue.pop().unwrap();

            match *current_node {
                SnailfishNumber::Literal { value } => {
                    if explosion.is_some() {
                        let (left_value, right_value) = explosion.unwrap();
                        match last_literal {
                            None => {}
                            Some(last_literal) => {
                                match *last_literal {
                                    SnailfishNumber::Literal { value } => { *value += left_value; }
                                    _ => {}
                                }
                            }
                        }
                        *value += right_value;
                        return true;
                    } else {
                        last_literal = Some(current_node);
                    }
                }
                SnailfishNumber::Pair { left, right } => {
                    if current_depth == 3 && explosion.is_none() {
                        // If this is a pair, start an explosion
                        match &**left {
                            SnailfishNumber::Pair { left: left_literal, right: right_literal } => {
                                // Extract literal values
                                let left_value = match *left_literal.clone() {
                                    SnailfishNumber::Literal { value } => value,
                                    _ => panic!()
                                };
                                let right_value = match *right_literal.clone() {
                                    SnailfishNumber::Literal { value } => value,
                                    _ => panic!()
                                };

                                explosion = Some((left_value, right_value));

                                *left = Box::from(SnailfishNumber::Literal { value: 0 });
                                queue.push((Box::new(right), current_depth + 1));
                            }
                            _ => {
                                last_literal = Some(Box::new(left));
                                // If this is a pair, start an explosion
                                match &**right {
                                    SnailfishNumber::Pair { left: left_literal, right: right_literal } => {
                                        let left_value = match *left_literal.clone() {
                                            SnailfishNumber::Literal { value } => value,
                                            _ => panic!()
                                        };
                                        let right_value = match *right_literal.clone() {
                                            SnailfishNumber::Literal { value } => value,
                                            _ => panic!()
                                        };
                                        explosion = Some((left_value, right_value));
                                        *right = Box::from(SnailfishNumber::Literal { value: 0 });
                                    }
                                    _ => {
                                        last_literal = Some(Box::new(right));
                                    }
                                }
                            }
                        }
                    } else {
                        queue.push((Box::new(right), current_depth + 1));
                        queue.push((Box::new(left), current_depth + 1));
                    }
                }
            }
        }

        match explosion {
            None => {}
            Some((left_value, _)) => {
                match last_literal {
                    None => {}
                    Some(node) => {
                        match *node {
                            SnailfishNumber::Literal { value } => {
                                *value += left_value;
                                return true;
                            }
                            SnailfishNumber::Pair { .. } => {}
                        }
                    }
                }
            }
        }

        return false;
    }

    fn split(&mut self) -> bool {
        let mut queue = Vec::new();

        queue.push(Box::new(self));
        while queue.len() > 0 {
            let current_node = queue.pop().unwrap();

            match *current_node {
                SnailfishNumber::Literal { value } => {
                    if *value > 9 {
                        **current_node = SnailfishNumber::Pair {
                            left: Box::new(SnailfishNumber::Literal { value: *value / 2 }),
                            right: Box::new(SnailfishNumber::Literal { value: *value / 2 + 1 }),
                        };
                        return true;
                    }
                }
                SnailfishNumber::Pair { left, right } => {
                    queue.push(Box::new(right));
                    queue.push(Box::new(left));
                }
            }
        }

        return false;
    }

    fn reduce(&mut self) {
        loop {
            let mut exploded_or_splitted = false;
            while self.explode() {
                exploded_or_splitted = true;
            }
            exploded_or_splitted |= self.split();

            if !exploded_or_splitted {
                break;
            }
        }
    }
}

fn magnitude(number: &SnailfishNumber) -> i32 {
    match number {
        SnailfishNumber::Literal { value } => { *value }
        SnailfishNumber::Pair { left, right } => { 3 * magnitude(left) + 2 * magnitude(right) }
    }
}

fn print_debug(number: &SnailfishNumber) {
    match number {
        SnailfishNumber::Literal { value } => {
            eprint!("{}", value);
        }
        SnailfishNumber::Pair { left, right } => {
            eprint!("[");
            print_debug(left);
            eprint!(",");
            print_debug(right);
            eprint!("]");
        }
    }
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        SnailfishNumber::Pair {
            left: Box::new(self.clone()),
            right: Box::new(rhs.clone()),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::models::{magnitude, print_debug, SnailfishNumber};
    use crate::models::SnailfishNumber::{Literal, Pair};
    use crate::SnailfishMathProblem;

    #[test]
    fn parse_example_case_1_2() {
        let n = SnailfishNumber::parse_str("[1,2]", 0).0;
        assert_eq!(n, SnailfishNumber::Pair {
            left: Box::new(SnailfishNumber::Literal { value: 1 }),
            right: Box::new(SnailfishNumber::Literal { value: 2 }),
        });

        let n = SnailfishNumber::parse_str("[[1,2],3]", 0).0;
        assert_eq!(n, SnailfishNumber::Pair {
            left: Box::new(SnailfishNumber::Pair {
                left: Box::new(SnailfishNumber::Literal { value: 1 }),
                right: Box::new(SnailfishNumber::Literal { value: 2 }),
            }),
            right: Box::new(SnailfishNumber::Literal { value: 3 }),
        });
    }

    #[test]
    fn parse_example_case_3_4() {
        let n = SnailfishNumber::parse_str("[9,[8,7]]", 0).0;
        assert_eq!(n, SnailfishNumber::Pair {
            left: Box::new(SnailfishNumber::Literal { value: 9 }),
            right: Box::new(SnailfishNumber::Pair {
                left: Box::new(SnailfishNumber::Literal { value: 8 }),
                right: Box::new(SnailfishNumber::Literal { value: 7 }),
            }),
        });

        let n = SnailfishNumber::parse_str("[[1,9],[8,5]]", 0).0;
        assert_eq!(n, SnailfishNumber::Pair {
            left: Box::new(SnailfishNumber::Pair {
                left: Box::new(SnailfishNumber::Literal { value: 1 }),
                right: Box::new(SnailfishNumber::Literal { value: 9 }),
            }),
            right: Box::new(SnailfishNumber::Pair {
                left: Box::new(SnailfishNumber::Literal { value: 8 }),
                right: Box::new(SnailfishNumber::Literal { value: 5 }),
            }),
        });
    }

    #[test]
    fn parse_example_case_7() {
        let n = SnailfishNumber::parse_str("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]", 0).0;
        assert_eq!(n, SnailfishNumber::Pair {
            left: Box::new(SnailfishNumber::Pair {
                left: Box::new(SnailfishNumber::Pair {
                    left: Box::new(SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Literal { value: 1 }),
                        right: Box::new(SnailfishNumber::Literal { value: 3 }),
                    }),
                    right: Box::new(SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Literal { value: 5 }),
                        right: Box::new(SnailfishNumber::Literal { value: 3 }),
                    }),
                }),
                right: Box::new(SnailfishNumber::Pair {
                    left: Box::new(SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Literal { value: 1 }),
                        right: Box::new(SnailfishNumber::Literal { value: 3 }),
                    }),
                    right: Box::new(SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Literal { value: 8 }),
                        right: Box::new(SnailfishNumber::Literal { value: 7 }),
                    }),
                }),
            }),
            right: Box::new(SnailfishNumber::Pair {
                left: Box::new(SnailfishNumber::Pair {
                    left: Box::new(SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Literal { value: 4 }),
                        right: Box::new(SnailfishNumber::Literal { value: 9 }),
                    }),
                    right: Box::new(SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Literal { value: 6 }),
                        right: Box::new(SnailfishNumber::Literal { value: 9 }),
                    }),
                }),
                right: Box::new(SnailfishNumber::Pair {
                    left: Box::new(SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Literal { value: 8 }),
                        right: Box::new(SnailfishNumber::Literal { value: 2 }),
                    }),
                    right: Box::new(SnailfishNumber::Pair {
                        left: Box::new(SnailfishNumber::Literal { value: 7 }),
                        right: Box::new(SnailfishNumber::Literal { value: 3 }),
                    }),
                }),
            }),
        });
    }

    #[test]
    fn explode_1() {
        let mut n = SnailfishNumber::parse_str("[[[[[9,8],1],2],3],4]", 0).0;
        n.explode();
        assert_eq!(n,
                   SnailfishNumber::Pair {
                       left: Box::new(SnailfishNumber::Pair {
                           left: Box::new(SnailfishNumber::Pair {
                               left: Box::new(SnailfishNumber::Pair {
                                   left: Box::new(SnailfishNumber::Literal { value: 0 }),
                                   right: Box::new(SnailfishNumber::Literal { value: 9 }),
                               }),
                               right: Box::new(SnailfishNumber::Literal { value: 2 }),
                           }),
                           right: Box::new(SnailfishNumber::Literal { value: 3 }),
                       }),
                       right: Box::new(SnailfishNumber::Literal { value: 4 }),
                   }
        );
    }

    #[test]
    fn explode_2() {
        let mut n = SnailfishNumber::parse_str("[7,[6,[5,[4,[3,2]]]]]", 0).0;
        n.explode();
        assert_eq!(n,
                   SnailfishNumber::Pair {
                       left: Box::new(SnailfishNumber::Literal { value: 7 }),
                       right: Box::new(SnailfishNumber::Pair {
                           left: Box::new(SnailfishNumber::Literal { value: 6 }),
                           right: Box::new(SnailfishNumber::Pair {
                               left: Box::new(SnailfishNumber::Literal { value: 5 }),
                               right: Box::new(SnailfishNumber::Pair {
                                   left: Box::new(SnailfishNumber::Literal { value: 7 }),
                                   right: Box::new(SnailfishNumber::Literal { value: 0 }),
                               }),
                           }),
                       }),
                   }
        );
    }

    #[test]
    fn explode_3() {
        let mut n = SnailfishNumber::parse_str("[[6,[5,[4,[3,2]]]],1]", 0).0;
        n.explode();
        assert_eq!(n,
                   SnailfishNumber::Pair {
                       left: Box::new(SnailfishNumber::Pair {
                           left: Box::new(SnailfishNumber::Literal { value: 6 }),
                           right: Box::new(SnailfishNumber::Pair {
                               left: Box::new(SnailfishNumber::Literal { value: 5 }),
                               right: Box::new(SnailfishNumber::Pair {
                                   left: Box::new(SnailfishNumber::Literal { value: 7 }),
                                   right: Box::new(SnailfishNumber::Literal { value: 0 }),
                               }),
                           }),
                       }),
                       right: Box::new(SnailfishNumber::Literal { value: 3 }),
                   }
        );
    }

    #[test]
    fn explode_4() {
        let mut n = SnailfishNumber::parse_str("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", 0).0;
        n.explode();
        assert_eq!(n,
                   Pair {
                       left: Box::new(Pair {
                           left: Box::new(Pair {
                               left: Box::new(Pair {
                                   left: Box::new(Literal { value: 0 }),
                                   right: Box::new(Literal { value: 7 }),
                               }),
                               right: Box::new(Literal { value: 4 }),
                           }),
                           right: Box::new(Pair {
                               left: Box::new(Literal { value: 15 }),
                               right: Box::new(Pair {
                                   left: Box::new(Literal { value: 0 }),
                                   right: Box::new(Literal { value: 13 }),
                               }),
                           }),
                       }),
                       right: Box::new(Pair {
                           left: Box::new(Literal { value: 1 }),
                           right: Box::new(Literal { value: 1 }),
                       }),
                   }
        )
    }

    #[test]
    fn explode_5() {
        let mut n = SnailfishNumber::parse_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", 0).0;
        n.explode();
        print_debug(&n);
    }

    #[test]
    fn split_1() {
        let mut n = SnailfishNumber::Pair {
            left: Box::new(SnailfishNumber::Literal { value: 15 }),
            right: Box::new(SnailfishNumber::Literal { value: 16 }),
        };
        n.split();
        assert_eq!(n,
                   SnailfishNumber::Pair {
                       left: Box::new(SnailfishNumber::Pair {
                           left: Box::new(SnailfishNumber::Literal { value: 7 }),
                           right: Box::new(SnailfishNumber::Literal { value: 8 }),
                       }),
                       right: Box::new(SnailfishNumber::Literal { value: 16 }),
                   }
        );
    }

    #[test]
    fn reduce_1() {
        let n1 = SnailfishNumber::parse_str("[[[[4,3],4],4],[7,[[8,4],9]]]", 0).0;
        let n2 = SnailfishNumber::parse_str("[1,1]", 0).0;

        let mut n = n1 + n2;

        n.reduce();

        assert_eq!(n,
                   SnailfishNumber::Pair {
                       left: Box::new(SnailfishNumber::Pair {
                           left: Box::new(SnailfishNumber::Pair {
                               left: Box::new(SnailfishNumber::Pair {
                                   left: Box::new(SnailfishNumber::Literal { value: 0 }),
                                   right: Box::new(SnailfishNumber::Literal { value: 7 }),
                               }),
                               right: Box::new(SnailfishNumber::Literal { value: 4 }),
                           }),
                           right: Box::new(SnailfishNumber::Pair {
                               left: Box::new(SnailfishNumber::Pair {
                                   left: Box::new(SnailfishNumber::Literal { value: 7 }),
                                   right: Box::new(SnailfishNumber::Literal { value: 8 }),
                               }),
                               right: Box::new(SnailfishNumber::Pair {
                                   left: Box::new(SnailfishNumber::Literal { value: 6 }),
                                   right: Box::new(SnailfishNumber::Literal { value: 0 }),
                               }),
                           }),
                       }),
                       right: Box::new(SnailfishNumber::Pair {
                           left: Box::new(SnailfishNumber::Literal { value: 8 }),
                           right: Box::new(SnailfishNumber::Literal { value: 1 }),
                       }),
                   }
        );
    }

    #[test]
    fn magnitude_1() {
        let n = SnailfishNumber::parse_str("[[1,2],[[3,4],5]]", 0).0;
        assert_eq!(magnitude(&n), 143);
    }

    #[test]
    fn small_add() {
        let problem = SnailfishMathProblem {
            values: vec![
                SnailfishNumber::parse_str("[1,1]", 0).0,
                SnailfishNumber::parse_str("[2,2]", 0).0,
                SnailfishNumber::parse_str("[3,3]", 0).0,
                SnailfishNumber::parse_str("[4,4]", 0).0,
                SnailfishNumber::parse_str("[5,5]", 0).0,
                SnailfishNumber::parse_str("[6,6]", 0).0,
            ]
        };

        print_debug(&problem.sum());
    }

    #[test]
    fn add_two() {
        let n1 = SnailfishNumber::parse_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", 0).0;
        let n2 = SnailfishNumber::parse_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", 0).0;

        let mut n = n1 + n2;
        n.reduce();

        print_debug(&n);
    }

    #[test]
    fn big_add() {
        let problem = SnailfishMathProblem {
            values: vec![
                SnailfishNumber::parse_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", 0).0,
                SnailfishNumber::parse_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", 0).0,
                SnailfishNumber::parse_str("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]", 0).0,
                SnailfishNumber::parse_str("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]", 0).0,
                SnailfishNumber::parse_str("[7,[5,[[3,8],[1,4]]]]", 0).0,
                SnailfishNumber::parse_str("[[2,[2,2]],[8,[8,1]]]", 0).0,
                SnailfishNumber::parse_str("[2,9]", 0).0,
                SnailfishNumber::parse_str("[1,[[[9,3],9],[[9,0],[0,7]]]]", 0).0,
                SnailfishNumber::parse_str("[[[5,[7,4]],7],1]", 0).0,
                SnailfishNumber::parse_str("[[[[4,2],2],6],[8,7]]", 0).0,
            ]
        };

        print_debug(&problem.sum());

        assert_eq!(problem.solve_part_1(), 4140);
    }

    #[test]
    fn part_1_solve() {
        let problem = SnailfishMathProblem {
            values: vec![
                SnailfishNumber::parse_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]", 0).0,
                SnailfishNumber::parse_str("[[[5,[2,8]],4],[5,[[9,9],0]]]", 0).0,
                SnailfishNumber::parse_str("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]", 0).0,
                SnailfishNumber::parse_str("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]", 0).0,
                SnailfishNumber::parse_str("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]", 0).0,
                SnailfishNumber::parse_str("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]", 0).0,
                SnailfishNumber::parse_str("[[[[5,4],[7,7]],8],[[8,3],8]]", 0).0,
                SnailfishNumber::parse_str("[[9,3],[[9,9],[6,[4,9]]]]", 0).0,
                SnailfishNumber::parse_str("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]", 0).0,
                SnailfishNumber::parse_str("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]", 0).0,
            ]
        };

        assert_eq!(problem.solve_part_1(), 4140);
    }
}
