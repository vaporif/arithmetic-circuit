use std::collections::HashMap;

pub enum Gate {
    Input(String),
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Const(i32),
}

pub struct Node {
    gate: Gate,
    value: Option<i32>,
}

impl Node {
    pub fn new(gate: Gate) -> Self {
        Node { gate, value: None }
    }

    pub fn evaluate(&mut self, inputs: &HashMap<String, i32>) -> i32 {
        if let Some(val) = self.value {
            return val;
        }

        let result = match &mut self.gate {
            Gate::Input(name) => *inputs.get(name).unwrap(),
            Gate::Add(l, r) => l.evaluate(inputs) + r.evaluate(inputs),
            Gate::Mul(l, r) => l.evaluate(inputs) * r.evaluate(inputs),
            Gate::Const(val) => *val,
        };

        self.value = Some(result);
        result
    }
}

pub struct ArithmeticCircuit {
    root: Node,
}

impl ArithmeticCircuit {
    pub fn new(root: Node) -> Self {
        Self { root }
    }

    pub fn from_expr(expr: &str) -> Self {
        todo!()
    }

    pub fn evaluate(&mut self, inputs: &[(&str, i32)]) -> i32 {
        let input_map: HashMap<String, i32> = inputs
            .iter()
            .cloned()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        self.root.evaluate(&input_map)
    }
}
