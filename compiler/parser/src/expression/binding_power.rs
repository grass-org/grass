use crate::ParseExpressionError;
use interfaces::Operator;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub(super) struct BindingPowers {
    // TODO: This could be simplified into a Vec instead to avoid hashing
    map: HashMap<Operator, BindingPower>,
}

impl BindingPowers {
    pub fn new() -> Self {
        Self::from_order(operator_order())
    }

    fn from_order(operator_order: Vec<Vec<(Operator, Associativity)>>) -> Self {
        let mut map = HashMap::new();

        for (index, operators) in operator_order.into_iter().rev().enumerate() {
            for (operator, associativity) in operators.into_iter() {
                let binding_power = binding_power(index, associativity);
                map.insert(operator, binding_power);
            }
        }

        Self { map }
    }

    pub fn binding_power(
        &self,
        operator: &Operator,
    ) -> Result<BindingPower, UndefinedBindingPowerError> {
        self.map.get(operator).cloned().ok_or_else(|| {
            let operator = operator.clone();
            UndefinedBindingPowerError { operator }
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub(crate) struct BindingPower {
    pub left: u32,
    pub right: u32,
}

// TODO: These are temporarily hardcoded, at least until we implement Grass operators
fn operator_order() -> Vec<Vec<(Operator, Associativity)>> {
    vec![
        vec![
            (Operator::multiply(), Associativity::Left),
            (Operator::divide(), Associativity::Left),
            (Operator::remainder(), Associativity::Left),
        ],
        vec![
            (Operator::add(), Associativity::Left),
            (Operator::subtract(), Associativity::Left),
        ],
    ]
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum Associativity {
    Left,
    Right,
}

const fn binding_power(reverse_index: usize, associativity: Associativity) -> BindingPower {
    let base_binding_power = base_binding_power(reverse_index);

    BindingPower {
        left: left_binding_power(associativity, base_binding_power),
        right: right_binding_power(associativity, base_binding_power),
    }
}

const fn base_binding_power(reverse_index: usize) -> u32 {
    (reverse_index as u32 + 1) * 10
}

const fn left_binding_power(associativity: Associativity, base_binding_power: u32) -> u32 {
    match associativity {
        Associativity::Left => base_binding_power,
        Associativity::Right => base_binding_power + 1,
    }
}

const fn right_binding_power(associativity: Associativity, base_binding_power: u32) -> u32 {
    match associativity {
        Associativity::Left => base_binding_power + 1,
        Associativity::Right => base_binding_power,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct UndefinedBindingPowerError {
    pub operator: Operator,
}

impl From<UndefinedBindingPowerError> for ParseExpressionError {
    fn from(value: UndefinedBindingPowerError) -> Self {
        Self::UndefinedBindingPower(value)
    }
}

impl Display for UndefinedBindingPowerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "undefined binding power for operator {}", self.operator)
    }
}

impl Error for UndefinedBindingPowerError {}
