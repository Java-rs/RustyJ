#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_snake_case)]

extern crate pest;
extern crate pest_derive;

use crate::types::{Class, Expr, FieldDecl, MethodDecl, Stmt, StmtExpr, Type};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../lib/src/parser/JavaGrammar.pest"]
struct JavaParser;

pub fn parse_programm(file: &str) -> Result<Vec<Class>, Error<Rule>> {
    let prg: Pair<Rule> = JavaParser::parse(Rule::Program, file)?.next().unwrap();

    if prg.as_rule() != Rule::Program {
        panic!();
    }
    let pased_clases = prg.into_inner().map(parse_class).collect();
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
    inners.next().unwrap().as_str().trim().to_string()
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

fn parse_BlockStmt(pair: Pair<Rule>) -> Vec<Stmt> {
    let rule = pair.as_rule().clone();
    let mut inner = pair.into_inner();
    match rule {
        Rule::BlockStmt => {
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
                Rule::Stmt => parse_Stmt(inner.next().unwrap()),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
fn parse_Stmt(pair: Pair<Rule>) -> Vec<Stmt> {
    match pair.as_rule() {
        Rule::WhileStmt => {
            let mut inners = pair.into_inner();

            let Expr = parse_expr(inners.next().unwrap());
            let Stmt = parse_Stmt(inners.next().unwrap()).get(0).unwrap().clone();
            vec![Stmt::While(Expr, Box::new(Stmt))]
        }
        Rule::IfElseStmt => {
            let mut inners = pair.into_inner();

            let mut firstif = inners.next().unwrap().into_inner();
            let Expr = parse_expr(firstif.next().unwrap());
            let Stmt = parse_Stmt(firstif.next().unwrap()).get(0).unwrap().clone();

            let elsePart = parse_Stmt(inners.next().unwrap()).get(0).unwrap().clone();
            vec![Stmt::If(Expr, Box::new(Stmt), Some(Box::new(elsePart)))]
        }
        Rule::IfStmt => {
            let mut inners = pair.into_inner();

            let Expr = parse_expr(inners.next().unwrap());
            let Stmt = parse_Stmt(inners.next().unwrap()).get(0).unwrap().clone();
            vec![Stmt::If(Expr, Box::new(Stmt), None)]
        }
        Rule::ReturnStmt => {
            let mut inners = pair.into_inner();

            let Expr = parse_expr(inners.next().unwrap());
            vec![Stmt::Return(Expr)]
        }
        Rule::LocalVarDeclStmt => {
            let mut inners = pair.into_inner();

            let typeJ = parse_Type(inners.next().unwrap());
            let var_name = next_id(&mut inners);
            //   StmtExprStmt
            let lVD = Stmt::LocalVarDecl(typeJ, var_name.clone());

            match inners.next() {
                None => vec![lVD],
                Some(expr_pair) => {
                    let expr = StmtExpr::Assign(var_name, parse_expr(expr_pair));
                    vec![lVD, Stmt::StmtExprStmt(expr)]
                }
            }
        }
        Rule::StmtExpr => {
            vec![Stmt::StmtExprStmt(parse_StmtExpr(pair))]
        }
        Rule::BlockStmt => parse_BlockStmt(pair),
        _ => unreachable!(),
    }
}

fn parse_StmtExpr(pair: Pair<Rule>) -> StmtExpr {
    let rule = pair.as_rule().clone();
    let mut inners = pair.into_inner();
    match rule {
        Rule::AssignExpr => {
            let mut name = inners.next().unwrap();
            let String_name;
            match name.as_rule() {
                Rule::Identifier => {
                    String_name = name.as_str().trim().to_string();
                }
                Rule::InstVarExpr => {
                    todo!() // until further notice ignored
                }
                _ => unreachable!(),
            }
            let Expr = parse_expr(inners.next().unwrap());

            StmtExpr::Assign(String_name, Expr)
        }
        Rule::NewExpr => {
            todo!()
        }
        Rule::MethodCallExpr => {
            todo!()
        }
        _ => unreachable!(),
    }
}

//fn parse_variabledeclarators(pair: Pair<Rule>)->

fn parse_field(pair: Pair<Rule>) -> Vec<FieldDecl> {
    match pair.as_rule() {
        Rule::FieldDecl => {
            let mut inners = pair.into_inner();
            let jtype = parse_Type(inners.next().unwrap());
            parse_field_var_decl_list(jtype, inners.next().unwrap())
        }

        _ => unreachable!(),
    }
}

fn parse_field_var_decl_list(jtype: Type, pair: Pair<Rule>) -> Vec<FieldDecl> {
    assert_eq!(pair.as_rule(), Rule::FieldVarDeclList);
    let mut inners = pair.into_inner();
    let mut var_decl = inners.next().unwrap().into_inner();
    let name = next_id(&mut var_decl);
    let val = var_decl.next().and_then(|expr| Some(parse_expr(expr)));
    let mut out = vec![FieldDecl {
        field_type: jtype.clone(),
        name,
        val,
    }];
    if let Some(p) = inners.next() {
        out.append(&mut parse_field_var_decl_list(jtype, p));
    }
    return out;
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
    match pair.as_rule() {
        Rule::Expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::NonBinaryExpr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::IntLiteral => Expr::Integer(pair.as_str().parse().unwrap()),
        Rule::BoolLiteral => Expr::Bool(pair.as_str().parse().unwrap()),
        Rule::CharLiteral => Expr::Char(get_str_content(pair.as_str()).parse().unwrap()),
        Rule::StrLiteral => Expr::String(get_str_content(pair.as_str()).to_string()),
        Rule::JNull => Expr::Jnull,
        Rule::ThisExpr => Expr::This,
        Rule::InstVarExpr => {
            let mut pairs = pair.into_inner();
            let x = pairs.next().unwrap();
            let mut obj = match x.as_rule() {
                Rule::Identifier => Expr::LocalOrFieldVar(x.as_str().trim().to_string()),
                Rule::ThisExpr => Expr::This,
                _ => unreachable!(),
            };
            while let Some(p) = pairs.next() {
                assert_eq!(p.as_rule(), Rule::Identifier);
                obj = Expr::InstVar(Box::new(obj), p.as_str().trim().to_string());
            }
            obj
        }
        _ => todo!(),
    }
}

fn get_str_content(s: &str) -> &str {
    &s[1..s.len() - 1]
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
