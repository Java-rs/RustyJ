extern crate pest;
extern crate pest_derive;

use crate::types::{Class, Expr, FieldDecl, MethodDecl, Stmt, StmtExpr, Type};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parser/JavaGrammar.pest"]
struct ExampleParser;

pub fn parse_Programm(file: &str) -> Result<Vec<Class>, Error<Rule>> {
    let example: Pair<Rule> = ExampleParser::parse(Rule::Program, file)?.next().unwrap();

    if example.as_rule() != Rule::Program {
        panic!();
    }
    let pased_clases = example.into_inner().map(parse_class).collect();
    Ok(pased_clases)
}

fn parse_class(pair: Pair<Rule>) -> Class {
    match pair.as_rule() {
        Rule::ClassDecl => {
            let mut inners = pair.into_inner();
            let other_name = next_id(&mut inners);
            let mut fields = vec![];
            let mut methods = vec![];
            for fieldOrMethod in inners {
                match fieldOrMethod.as_rule() {
                    Rule::FieldDecl => {
                        fields.append(&mut parse_field(fieldOrMethod));
                    }
                    Rule::MethodDecl => {
                        methods.push(parse_method(fieldOrMethod));
                    }
                    _ => unreachable!(),
                };
            }
            Class {
                name: other_name,
                fields,
                methods,
            }
        }
        _ => todo!(),
    }
}
fn next_id(inners: &mut Pairs<Rule>) -> String {
    // ".as_str().to_string()" might look weird but is legitimate
    // calling to_string() immediately would return "Identifier(<location>)",
    // while as_str() returns the actual str-slice captured by the Identifier-rule
    // We still have to copy that str-slice into its own string object with to_string() though
    inners.next().unwrap().as_str().to_string()
}

fn parse_method(pair: Pair<Rule>) -> MethodDecl {
    match pair.as_rule() {
        Rule::MethodDecl => {
            let mut inners = pair.into_inner();
            let ret_type = parse_Type(inners.next().unwrap());
            let method_name = next_id(&mut inners);
            let mut params = vec![];
            let mut body = None;
            while let Some(p) = inners.next() {
                match p.as_rule() {
                    Rule::ParamDecl => {
                        let mut inner_param = p.into_inner();
                        let param_type = parse_Type(inner_param.next().unwrap());
                        let param_name = next_id(&mut inner_param);
                        params.push((param_type, param_name));
                    }
                    Rule::BlockStmt => body = Some(parse_block_stmt(p)),
                    _ => unreachable!(),
                };
            }

            MethodDecl {
                ret_type,
                name: method_name,
                params,
                body: body.unwrap(),
            }
        }
        _ => unreachable!(),
    }
}

// TODO
fn parse_block_stmt(pair: Pair<Rule>) -> Stmt {
    Stmt::Block(vec![])
}

fn parse_Stmt(pair: Pair<Rule>) -> Vec<Stmt> {
    match pair.as_rule() {
        Rule::BlockStmt => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            match first.as_rule() {
                Rule::JType => {
                    let jtype = parse_Type(first);
                    let var_decels = inner.next().unwrap().into_inner();
                    var_decels
                        .map(|x| {
                            let mut inner = x.into_inner();
                            let other_name = next_id(&mut inner);
                            match inner.next() {
                                None => vec![Stmt::LocalVarDecl(jtype.clone(), other_name)],
                                Some(expresion) => vec![
                                    Stmt::LocalVarDecl(jtype.clone(), other_name.clone()),
                                    Stmt::StmtExprStmt(StmtExpr::Assign(
                                        other_name,
                                        parse_expr(expresion),
                                    )),
                                ],
                            }
                        })
                        .flatten()
                        .collect()
                }
                Rule::Stmt => {
                    vec![]
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
fn parse_statement(pair: Pair<Rule>) -> Stmt {
    match pair.as_rule() {
        _ => unreachable!(),
    }
    todo!()
}

//fn parse_variabledeclarators(pair: Pair<Rule>)->

fn parse_field(pair: Pair<Rule>) -> Vec<FieldDecl> {
    match pair.as_rule() {
        Rule::FieldDecl => {
            let mut inners = pair.into_inner();
            let JType = parse_Type(inners.next().unwrap());
            let varDecels = inners.next().unwrap().into_inner();

            varDecels
                .map(|x| {
                    let mut inner = x.into_inner();
                    let other_name = next_id(&mut inner);
                    match inner.next() {
                        None => FieldDecl {
                            field_type: JType.clone(),
                            name: other_name,
                            val: None,
                        },
                        Some(val) => FieldDecl {
                            field_type: JType.clone(),
                            name: other_name,
                            val: Some(parse_expr(val)),
                        },
                    }
                })
                .collect()
        }

        _ => unreachable!(),
    }
}

fn parse_Type(pair: Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::JType => parse_Type(pair.into_inner().next().unwrap()),
        Rule::PrimitiveType => match pair.as_str() {
            "boolean" => Type::Bool,
            "int" => Type::Int,
            "char" => Type::Char,
            "String" => Type::String,
            "void" => Type::Void,
            "null" => Type::Null,
            _ => unreachable!(),
        },
        Rule::Identifier => Type::Class(next_id(&mut pair.into_inner())),
        _ => unreachable!(),
    }
}
fn parse_expr(pair: Pair<Rule>) -> Expr {
    todo!()
}
fn parse_value(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        // Rule::ID => Example::ID(String::from(pair.as_str())),
        // Rule::Name => Name(pair),
        // Rule::List => Example::List(pair.into_inner().map(parse_value).collect()),
        Rule::Identifier => Expr::String(String::from(pair.as_str())),

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
