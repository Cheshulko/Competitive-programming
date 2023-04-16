use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

type OpResult = std::result::Result<Vec<Value>, Error>;

type UnaryOp = Box<dyn Fn(Value) -> OpResult>;
type BinaryOp = Box<dyn Fn(Value, Value) -> OpResult>;

pub enum OperationType {
    Unary(UnaryOp),
    Binary(BinaryOp),
}

pub struct Operation {
    pub op_type: OperationType,
}

pub struct Forth {
    pub st: Vec<Value>,
    pub ops: HashMap<String, Vec<Operation>>,
    pub difs: HashMap<String, Vec<String>>,
    pub redefines: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            st: Vec::new(),
            difs: HashMap::new(),
            redefines: HashMap::new(),
            ops: HashMap::from([
                // Binary
                (
                    "+".to_string(),
                    vec![Operation {
                        op_type: OperationType::Binary(Box::new(|a, b| Ok(vec![a + b]))),
                    }],
                ),
                (
                    "-".to_string(),
                    vec![Operation {
                        op_type: OperationType::Binary(Box::new(|a, b| Ok(vec![a - b]))),
                    }],
                ),
                (
                    "*".to_string(),
                    vec![Operation {
                        op_type: OperationType::Binary(Box::new(|a, b| Ok(vec![a * b]))),
                    }],
                ),
                (
                    "/".to_string(),
                    vec![Operation {
                        op_type: OperationType::Binary(Box::new(|a, b| {
                            if b == 0 {
                                return Err(Error::DivisionByZero);
                            } else {
                                return Ok(vec![a / b]);
                            }
                        })),
                    }],
                ),
                (
                    "swap".to_string(),
                    vec![Operation {
                        op_type: OperationType::Binary(Box::new(|a, b| Ok(vec![b, a]))),
                    }],
                ),
                (
                    "over".to_string(),
                    vec![Operation {
                        op_type: OperationType::Binary(Box::new(|a, b| Ok(vec![a, b, a]))),
                    }],
                ),
                // Unary
                (
                    "dup".to_string(),
                    vec![Operation {
                        op_type: OperationType::Unary(Box::new(|a| Ok(vec![a, a]))),
                    }],
                ),
                (
                    "drop".to_string(),
                    vec![Operation {
                        op_type: OperationType::Unary(Box::new(|_| Ok(vec![]))),
                    }],
                ),
            ]),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.st
    }

    fn run(&mut self, x: &String) -> Result {
        match self.ops.get(x) {
            Some(ops) => ops
                .iter()
                .map(|op| match &op.op_type {
                    OperationType::Unary(fnc) => {
                        if let Some(value) = self.st.pop() {
                            match fnc(value) {
                                Ok(res) => {
                                    res.iter().for_each(|x| self.st.push(*x));
                                    Ok(())
                                }
                                Err(e) => Err(e),
                            }
                        } else {
                            Err(Error::StackUnderflow)
                        }
                    }
                    OperationType::Binary(fnc) => {
                        let right = self.st.pop();
                        let left = self.st.pop();
                        if let (Some(left), Some(right)) = (left, right) {
                            match fnc(left, right) {
                                Ok(res) => {
                                    res.iter().for_each(|x| self.st.push(*x));
                                    Ok(())
                                }
                                Err(e) => Err(e),
                            }
                        } else {
                            Err(Error::StackUnderflow)
                        }
                    }
                })
                .filter(|x| x.is_err())
                .last()
                .or_else(|| Some(Ok(())))
                .unwrap(),
            None => Err(Error::UnknownWord),
        }
    }

    fn find_redefine_and_update(&self, x: &String) -> String {
        let mut x = x.clone();
        while let Some(x_redef) = self.redefines.get(&x) {
            x = x_redef.clone();
        }
        return x;
    }

    fn match_token(&mut self, x: &String) -> Result {
        match x.parse() {
            Ok(value) => {
                self.st.push(value);
                Ok(())
            }
            Err(_) => {
                if let Some(difs) = self.difs.get(x) {
                    difs.clone()
                        .iter()
                        .map(|y| self.match_token(y))
                        .filter(|x| x.is_err())
                        .last()
                        .or_else(|| Some(Ok(())))
                        .unwrap()
                } else {
                    self.run(&x)
                }
            }
        }
    }

    pub fn eval(&mut self, input_outer: &str) -> Result {
        let splited_input = input_outer
            .split([' '])
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        let mut token_index = 0;
        let mut result = Result::Ok(());

        while token_index < splited_input.len() {
            let input = splited_input[token_index];

            if input == ":" {
                let mut token_index_end = token_index + 1;
                while token_index_end < splited_input.len() && splited_input[token_index_end] != ";"
                {
                    token_index_end += 1;
                }
                if token_index_end >= splited_input.len() {
                    result = Err(Error::InvalidWord);
                }

                let splited = &splited_input[token_index + 1..token_index_end];

                let mut word: String = String::new();
                let mut difs: Vec<String> = vec![];

                for (ind, x) in splited.iter().enumerate() {
                    if ind == 0 {
                        word = String::from(*x).to_lowercase();
                        if word.parse::<i32>().is_ok() {
                            result = Err(Error::InvalidWord);
                        }
                    } else {
                        let mut dif = String::from(*x).to_lowercase();

                        while let Some(dif_redef) = self.redefines.get(&dif) {
                            dif = dif_redef.clone();
                        }

                        difs.push(dif);
                    }
                }
                if self.difs.contains_key(&word) {
                    if let Some(redef) = self.redefines.get(&word) {
                        word = redef.clone();
                    }
                    let word_upd = word.clone() + "#";
                    self.redefines.insert(word.clone(), word_upd.clone());
                    self.difs.insert(word_upd, difs);
                } else {
                    self.difs.insert(word, difs);
                }

                // upd
                token_index = token_index_end + 1;
            } else {
                let p = input
                    .split(" ")
                    .map(|x| x.to_lowercase())
                    .map(|x| {
                        let x = self.find_redefine_and_update(&x);
                        self.match_token(&x)
                    })
                    .filter(|x| x.is_err())
                    .last();

                if let Some(e) = p {
                    result = e;
                }

                // upd
                token_index += 1;
            }
        }

        result
    }
}
