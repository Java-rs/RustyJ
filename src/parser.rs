extern crate pest;
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "JavaGrammar.pest"]
struct ExampleParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Example {
    ID(String),
    Name(Box<Example>, Box<Example>),
    List(Vec<Example>),
}

pub fn parse_Example(file: &str) -> Result<Example, Error<Rule>> {
    let example: Pair<Rule> = ExampleParser::parse(Rule::List, file)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> Example {
        match pair.as_rule() {
            Rule::ID => Example::ID(String::from(pair.as_str())),
            Rule::Name => {
                let mut pairs = pair.into_inner();
                let a = parse_value(pairs.next().unwrap());
                let b = parse_value(pairs.next().unwrap());
                Example::Name(Box::new(a), Box::new(b))
            }
            Rule::List => Example::List(pair.into_inner().map(parse_value).collect()),
        }
    }

    Ok(parse_value(example))
}
