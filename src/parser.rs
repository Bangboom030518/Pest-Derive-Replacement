use crate::{Pair, Pairs, Rule};
use pest::{
    error::{Error, ErrorVariant},
    Span,
};

pub enum Expression {
    Choice(Vec<Expression>),
}

impl<'a> From<Pair<'a>> for Expression {
    fn from(pair: Pair) -> Self {
        // let mut choice_operator = false;
        // let mut sequence_operator = false;
        let inner = pair.into_inner();
        match inner.count() {
            // TODO format error using span
            0 => panic!("Rulesets should not be empty"),
            1 => (),
            number if number + 1 % 2 == 0 => {}
        };
    }
}

#[derive(PartialEq)]
enum BinaryOperator {
    Choice,
    Sequence,
}

impl<'a> From<Pair<'a>> for BinaryOperator {
    fn from(pair: Pair<'a>) -> Self {
        let rule = pair.as_rule();
        assert_eq!(
            rule,
            Rule::infix_operator,
            "Binary Operator pair should be rule 'infix_operator'. Found '{:?}'",
            rule
        );
        let pair = expect_single_child(pair);
        match pair.as_rule() {
            Rule::choice_operator => Self::Choice,
            Rule::sequence_operator => Self::Sequence,
            rule => unreachable!(
                "'infix_operator' child should not be '{:?}'",
                pair.as_rule()
            ),
        }
    }
}

fn error(message: &str, span: Span) -> ! {
    let err = Error::new_from_span(
        ErrorVariant::CustomError::<Rule> {
            message: message.to_string(),
        },
        span,
    );

    panic!("{}", err)
}

fn parse_binary_expression(pairs: Pairs) -> Vec<Expression> {
    let mut operator: Option<BinaryOperator> = None;
    pairs.collect::<Vec<Pair>>().split(|pair| {
        let splitting = pair.as_rule() == Rule::infix_operator;
        if splitting {
            let current_operator = BinaryOperator::from(*pair);
            match operator {
                None => operator = Some(current_operator),
                Some(operator) => {
                    if operator != current_operator {
                        error("Expressions should only have 1 type of operator.", span)
                    }
                }
            };
        }
        splitting
    });

    pairs
        .enumerate()
        .filter_map(|(index, pair)| {
            if index % 2 == 0 {
                Some(Expression::from(pair))
            } else {
                let rule = pair.as_rule();
                assert_eq!(
                    rule,
                    Rule::infix_operator,
                    "Alternate terms in expressions should be infix operators. Found '{:?}'",
                    rule
                );
                let operator = expect_single_child(pair);
                match operator.as_rule() {
                    Rule::choice_operator => (),
                };
                None
            }
        })
        .collect()
}

fn expect_single_child(pair: Pair) -> Pair {
    let rule = pair.as_rule();
    let mut pairs = pair.into_inner();
    pairs
        .next()
        .and_then(|pair| {
            if pairs.next().is_none() {
                Some(pair)
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("Rule '{:?}' should have exactly 1 child", rule))
}

fn expect_structure<'a>(
    mut pairs: Pairs<'a>,
    rules: &'a [Rule],
    name: &'a str,
    index: usize,
) -> Pair {
    let all_pairs = pairs.clone().collect::<Vec<Pair>>();
    let length = all_pairs.len();
    for (index, &rule) in rules.iter().enumerate() {
        let pair = pairs
            .next()
            .unwrap_or_else(|| panic!("Rule '{}' should have a child at index {}", name, index));
        assert_eq!(
            pair.as_rule(),
            rule,
            "Child {} should be the rule '{:?}'. Found '{:?}'",
            index,
            rule,
            pair.as_rule()
        );
    }
    all_pairs
        .get(index)
        .unwrap_or_else(|| {
            panic!(
                "Index should be in range. Trying to access index '{}' in list of length '{}'",
                index, length
            )
        })
        .clone()
}
