extern crate pest;
extern crate pest_derive;

use super::*;
use crate::types::{Class, Expr, FieldDecl, Type};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::iter::Map;
use std::path::Iter;

#[derive(Parser)]
#[grammar = "src/parser/JavaGrammar.pest"]
struct ExampleParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Example {
    ID(String),
    Name(Box<Example>, Box<Example>),
    List(Vec<Example>),
}

pub fn parse_Programm(file: &str) -> Result<Vec<Class>, Error<Rule>> {
    let example: Pair<Rule> = ExampleParser::parse(Rule::compilationunit, file)?
        .next()
        .unwrap();

    if (example.as_rule() != Rule::compilationunit) {
        panic!();
    }
    let pasedClases = example.into_inner().map(parse_class).collect();
    Ok(pasedClases)
}

fn parse_class(pair: Pair<Rule>) -> Class {
    match pair.as_rule() {
        Rule::classdeclaration => {
            let mut inners = pair.into_inner();
            let name = next_id(&mut inners);
            let mut fields = vec![];
            let mut methods = vec![];
            for fieldOrMethod in inners {
                match fieldOrMethod.as_rule() {
                    Rule::fielddeclaration => {
                        // add to fields list
                    }
                    Rule::methoddeclaration => {
                        // add to method list
                    }
                    _ => unreachable!(),
                }
            }
            Class {
                name,
                fields,
                methods,
            }
        }
        _ => todo!(),
    }
}
fn next_id(inners: &mut Pairs<Rule>) -> String {
    inners.next().unwrap().to_string()
}

fn parse_field(pair: Pair<Rule>) -> FieldDecl {
    match pair.as_rule() {
        Rule::fielddeclaration => {
            let mut inners = pair.into_inner();
            let typeJ = parse_Type(inners.next().unwrap());
            let varDecels = vec![];

            todo!()
        }

        _ => unreachable!(),
    }
}

fn parse_Type(pair: Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::primitivetype => match pair.as_str() {
            "boolean" => Type::Bool,
            "int" => Type::Int,
            "char" => Type::Char,
            "String" => Type::String,
            "void" => Type::Void,
            "null" => Type::Null,
            _ => unreachable!(),
        },
        Rule::IDENTIFIER => Type::Class(next_id(&mut pair.into_inner())),
        _ => unreachable!(),
    }
}

fn parse_value(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        // Rule::ID => Example::ID(String::from(pair.as_str())),
        // Rule::Name => Name(pair),
        // Rule::List => Example::List(pair.into_inner().map(parse_value).collect()),
        Rule::IDENTIFIER => Expr::String(String::from(pair.as_str())),

        _ => Expr::String(String::from(pair.as_str())),
    }
}
/*
pub fn Name(pair: Pair<Rule>) -> Example {
    let mut pairs = pair.into_inner();
    let a = parse_value(pairs.next().unwrap());
    let b = parse_value(pairs.next().unwrap());
    return Example::Name(Box::new(a), Box::new(b));
}

 */
