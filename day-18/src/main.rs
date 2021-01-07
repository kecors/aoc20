use std::io::{stdin, Read};

#[derive(Debug, Clone)]
enum Token {
    LeftParenthesis,
    RightParenthesis,
    Plus,
    Times,
    Number(u64),
}

#[derive(Debug)]
enum State {
    Empty,
    Operand(u64),
    Add(u64),
    Multiply(u64),
}

#[derive(Debug)]
struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn new(line: &str) -> Expression {
        let tokens: Vec<Token> = line
            .chars()
            .filter(|x| *x != ' ')
            .map(|x| match x {
                '(' => Token::LeftParenthesis,
                ')' => Token::RightParenthesis,
                '+' => Token::Plus,
                '*' => Token::Times,
                _ => Token::Number(x.to_digit(10).unwrap().into()),
            })
            .collect();

        Expression { tokens }
    }

    fn evaluate(&self) -> u64 {
        let mut stream: Vec<Token> = self.tokens.clone().into_iter().rev().collect();
        let mut stack: Vec<State> = Vec::new();
        let mut state = State::Empty;

        while let Some(token) = stream.pop() {
            match token {
                Token::Number(n) => match state {
                    State::Empty => {
                        state = State::Operand(n);
                    }
                    State::Operand(x) => {
                        panic!("unexpected Number {} in Operand {} state", n, x);
                    }
                    State::Add(x) => {
                        state = State::Operand(x + n);
                    }
                    State::Multiply(x) => {
                        state = State::Operand(x * n);
                    }
                },
                Token::Plus => match state {
                    State::Operand(x) => {
                        state = State::Add(x);
                    }
                    _ => {
                        panic!("unexpected Plus in state {:?}", state);
                    }
                },
                Token::Times => match state {
                    State::Operand(x) => {
                        state = State::Multiply(x);
                    }
                    _ => {
                        panic!("unexpected Times in state {:?}", state);
                    }
                },
                Token::LeftParenthesis => {
                    stack.push(state);
                    state = State::Empty;
                }
                Token::RightParenthesis => match state {
                    State::Operand(n) => {
                        stream.push(Token::Number(n));
                        state = stack.pop().unwrap();
                    }
                    _ => {
                        panic!("unexpected RightParenthesis in state {:?}", state);
                    }
                },
            }
        }

        match state {
            State::Operand(n) => n,
            _ => {
                panic!("ended processing with state {:?}", state);
            }
        }
    }
}

#[derive(Debug)]
struct Engine {
    expressions: Vec<Expression>,
}

impl Engine {
    fn new(input: &str) -> Engine {
        let expressions: Vec<Expression> = input.lines().map(|x| Expression::new(x)).collect();

        Engine { expressions }
    }

    fn add_results(&self) -> u64 {
        let mut sum = 0;

        for expression in self.expressions.iter() {
            sum += expression.evaluate();
        }

        sum
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let engine = Engine::new(&input);

    let sum = engine.add_results();
    println!("Part 1: the sum of the results is {}", sum);
}
