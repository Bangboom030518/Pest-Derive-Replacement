use crate::Pair;

pub enum Expression {
    Choice(Vec<Expression>)
}

impl<'a> From<Pair<'a>> for Expression {
    fn from(_: Pair) -> Self {
        let mut operators = Vec::<(usize, BinaryOperator)>::new();
        // let mut choice_operator = false;
        // let mut sequence_operator = false;
        let inner = pair.into_inner();
        match inner.count() {
            // TODO format error using
            0 => panic!("Rulesets should not be empty"),
            1 => 
        }    
    }
}
