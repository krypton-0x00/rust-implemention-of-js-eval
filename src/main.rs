use std::collections::{HashMap, VecDeque};

pub fn eval(exp: String) -> i128 {
    let mut output_queue: VecDeque<i128> = VecDeque::new();
    let mut operator_stack: Vec<char> = Vec::new();
    let mut precedence: HashMap<char, u8> = HashMap::new();

    precedence.insert('^', 3);
    precedence.insert('*', 2);
    precedence.insert('/', 2);
    precedence.insert('+', 1);
    precedence.insert('-', 1);

    for c in exp.chars() {
        if c.is_numeric() {
            output_queue.push_back(c.to_digit(10).unwrap() as i128);
        } else {
            while let Some(&top) = operator_stack.last() {
                if precedence[&top] >= precedence[&c] {
                    operator_stack.pop();
                    output_queue.push_back(top as i128);
                } else {
                    break;
                }
            }
            operator_stack.push(c);
        }
    }

    while let Some(opr) = operator_stack.pop() {
        output_queue.push_back(opr as i128);
    }

    let mut eval_stack: Vec<i128> = Vec::new();
    while let Some(token) = output_queue.pop_front() {
        if (0..=9).contains(&token) {
            eval_stack.push(token);
        } else {
            let b = eval_stack.pop().unwrap();
            let a = eval_stack.pop().unwrap();
            let result = match token as u8 as char {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                '^' => a.pow(b as u32),
                _ => panic!("Unknown operator"),
            };
            eval_stack.push(result);
        }
    }

    eval_stack.pop().unwrap()
}

fn main() {
    let result = eval("1+2*3-4".to_string());
    println!("{}", result);
}
