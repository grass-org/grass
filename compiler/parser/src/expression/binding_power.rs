use crate::ParseExpressionError;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

type OperatorSymbol = String;

pub(super) struct BindingPowers {
    // TODO: This could be simplified into a Vec instead to avoid hashing
    prefix_map: HashMap<OperatorSymbol, u32>,
    infix_map: HashMap<OperatorSymbol, BindingPower>,
    postfix_map: HashMap<OperatorSymbol, u32>,
}

impl BindingPowers {
    pub fn from_order(operator_order: Vec<Vec<(OperatorSymbol, Binding)>>) -> Self {
        let mut binding_powers = Self::new();

        for (reverse_index, operators) in operator_order.into_iter().rev().enumerate() {
            for (operator, binding) in operators.into_iter() {
                binding_powers.add_operator(operator, binding, reverse_index);
            }
        }

        binding_powers
    }

    fn new() -> Self {
        Self {
            prefix_map: HashMap::new(),
            infix_map: HashMap::new(),
            postfix_map: HashMap::new(),
        }
    }

    fn add_operator(&mut self, operator: OperatorSymbol, binding: Binding, reverse_index: usize) {
        match binding {
            Binding::Prefix => self.add_prefix(operator, reverse_index),
            Binding::Infix(associativity) => self.add_infix(operator, reverse_index, associativity),
            Binding::Postfix => self.add_postfix(operator, reverse_index),
        }
    }

    fn add_prefix(&mut self, operator: OperatorSymbol, reverse_index: usize) {
        let binding_power = binding_power(reverse_index);
        self.prefix_map.insert(operator, binding_power);
    }

    fn add_infix(
        &mut self,
        operator: OperatorSymbol,
        reverse_index: usize,
        associativity: Associativity,
    ) {
        let binding_power = associative_binding_power(reverse_index, associativity);
        self.infix_map.insert(operator, binding_power);
    }

    fn add_postfix(&mut self, operator: OperatorSymbol, reverse_index: usize) {
        let binding_power = binding_power(reverse_index);
        self.postfix_map.insert(operator, binding_power);
    }

    pub fn prefix_binding_power(&self, operator: &str) -> Result<u32, UndefinedBindingPowerError> {
        self.prefix_map.get(operator).cloned().ok_or_else(|| {
            let operator = operator.into();
            UndefinedBindingPowerError { operator }
        })
    }

    pub fn infix_binding_power(
        &self,
        operator: &str,
    ) -> Result<BindingPower, UndefinedBindingPowerError> {
        self.infix_map.get(operator).cloned().ok_or_else(|| {
            let operator = operator.into();
            UndefinedBindingPowerError { operator }
        })
    }

    pub fn postfix_binding_power(&self, operator: &str) -> Result<u32, UndefinedBindingPowerError> {
        self.postfix_map.get(operator).cloned().ok_or_else(|| {
            let operator = operator.into();
            UndefinedBindingPowerError { operator }
        })
    }
}

impl Default for BindingPowers {
    fn default() -> Self {
        Self::from_order(operator_order())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub(super) struct BindingPower {
    pub left: u32,
    pub right: u32,
}

// TODO: These are temporarily hardcoded, at least until we implement Grass operators
fn operator_order() -> Vec<Vec<(OperatorSymbol, Binding)>> {
    vec![
        vec![("-".into(), Binding::Prefix)],
        vec![
            ("*".into(), Binding::Infix(Associativity::Left)),
            ("/".into(), Binding::Infix(Associativity::Left)),
            ("%".into(), Binding::Infix(Associativity::Left)),
        ],
        vec![
            ("+".into(), Binding::Infix(Associativity::Left)),
            ("-".into(), Binding::Infix(Associativity::Left)),
        ],
    ]
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub(super) enum Binding {
    Prefix,
    Infix(Associativity),
    Postfix,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub(super) enum Associativity {
    Left,
    Right,
}

const fn binding_power(reverse_index: usize) -> u32 {
    (reverse_index as u32 + 1) * 10
}

const fn associative_binding_power(
    reverse_index: usize,
    associativity: Associativity,
) -> BindingPower {
    let base_binding_power = binding_power(reverse_index);

    BindingPower {
        left: left_binding_power(associativity, base_binding_power),
        right: right_binding_power(associativity, base_binding_power),
    }
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
    pub operator: OperatorSymbol,
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
