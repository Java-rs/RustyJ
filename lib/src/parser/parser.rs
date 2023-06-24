extern crate pest;
extern crate pest_derive;

use super::*;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "src/parser/ExampleGrammar.pest"]
struct ExampleParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Example {
    ID(String),
    Name(Box<Example>, Box<Example>),
    List(Vec<Example>),
}

pub fn parse_Example(file: &str) -> Result<Example, Error<Rule>> {
    let example: Pair<Rule> = ExampleParser::parse(Rule::List, file)?.next().unwrap();

    Ok(parse_value(example))
}

fn parse_value(pair: Pair<Rule>) -> Example {
    match pair.as_rule() {
        Rule::ID => Example::ID(String::from(pair.as_str())),
        Rule::Name => Name(pair),
        Rule::List => Example::List(pair.into_inner().map(parse_value).collect()),
    }
}

pub fn Name(pair: Pair<Rule>) -> Example {
    let mut pairs = pair.into_inner();
    let a = parse_value(pairs.next().unwrap());
    let b = parse_value(pairs.next().unwrap());
    return Example::Name(Box::new(a), Box::new(b));
}
