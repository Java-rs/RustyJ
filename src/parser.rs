use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use pest_derive::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;


//  Name, Name , Name

#[derive(Parser)]
#[grammar = "JavaGrammar.pest"]
struct JavaParser;

#[derive(Debug, Clone, Deserialize, Serialize , Disply)]
pub enum Example
{
    ID(* str),
    Name(Example::ID(), Example::ID()),
    List([Name]),
}


pub fn parse_Example(file: &str) -> Result<Example, Error<Rule>>
{
    let expamle = JavaParser::parse(Rule::List, file)?.next().unwrap();
    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> Example
    {
        match pair.as_rule() {
            Rule::ID => Example::ID(pair.into_inner().next().unwrap().as_str()),
            Rule::Name => Example::Name(pair.into_inner().next().unwrap(), pair.into_inner().next().unwrap()),
            Rule::List => Example::List(pair.into_inner().map(parse_value).collect()),
        }
    }

    Ok(parse_value(expamle))
}





