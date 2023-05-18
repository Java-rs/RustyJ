extern crate pest;
extern crate pest_derive;
extern crate serde;
use serde::Deserialize;
use serde::Serialize;
use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use pest_derive::Parser;
use Example::*;



//  Name, Name , Name

#[derive(Parser)]
#[grammar = "JavaGrammar.pest"]
struct JavaParser;

#[derive(Debug, Clone, Deserialize, Serialize )]
pub enum Example
{
    ID ( String),
    Name(Box<Example>, Box<Example>),
    List(Vec<Example>),
}
/*
pub enum Example<'a>
{
    ID(&'static str),
    Name(Box<Example<'a>>, Box<Example<'a>>),
    List(&'a[Box<Example<'a>>]),
}*/


pub fn parse_Example(file: &str) -> Result<Example, Error<Rule>>
{
    let expamle = JavaParser::parse(Rule::List, file)?.next().unwrap();
    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> Example
    {
        match pair.as_rule() {
            Rule::ID => Example::ID( pair.into_inner().next().unwrap().as_str()),
            Rule::Name => Example::Name(Box::new(parse_value(pair.clone().into_inner().next().unwrap()) ), Box::new( parse_value(pair.into_inner().next().unwrap()))),
            Rule::List => Example::List(pair.into_inner().map(parse_value).collect()),
        }
    }

    Ok(parse_value(expamle))
}





