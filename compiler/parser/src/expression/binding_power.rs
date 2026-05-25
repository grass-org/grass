use std::collections::HashMap;

use interfaces::BinaryOperator;

pub struct BindingPowers {
    // TODO: This could be simplified into a Vec instead to avoid hashing
    map: HashMap<BinaryOperator, BindingPower>,
}

impl BindingPowers {
    pub fn new() -> Self {
        Self::from_order(operator_order())
    }

    fn from_order(operator_order: Vec<Vec<(BinaryOperator, Associativity)>>) -> Self {
        let mut map = HashMap::new();

        for (index, operators) in operator_order.into_iter().rev().enumerate() {
            for (operator, associativity) in operators.into_iter() {
                let binding_power = binding_power(index, associativity);
                map.insert(operator, binding_power);
            }
        }

        Self { map }
    }

    pub fn binding_power(&self, operator: BinaryOperator) -> Option<BindingPower> {
        self.map.get(&operator).cloned()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct BindingPower {
    pub left: u32,
    pub right: u32,
}

fn operator_order() -> Vec<Vec<(BinaryOperator, Associativity)>> {
    vec![
        vec![
            (BinaryOperator::Multiply, Associativity::Left),
            (BinaryOperator::Divide, Associativity::Left),
            (BinaryOperator::Remainder, Associativity::Left),
        ],
        vec![
            (BinaryOperator::Add, Associativity::Left),
            (BinaryOperator::Subtract, Associativity::Left),
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
