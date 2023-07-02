#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_snake_case)]

extern crate pest;
extern crate pest_derive;

use crate::types::{BinaryOp, Class, Expr, FieldDecl, MethodDecl, Stmt, StmtExpr, Type};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use tracing::debug;

#[derive(Parser)]
#[grammar = "../lib/src/parser/JavaGrammar.pest"]
struct JavaParser;
#[allow(clippy::result_large_err)]
pub fn parse_programm(file: &str) -> Result<Vec<Class>, Error<Rule>> {
    let prg: Pair<Rule> = JavaParser::parse(Rule::Program, file)?.next().unwrap();

    if prg.as_rule() != Rule::Program {
        panic!();
    }
    let pased_clases = prg.into_inner().map(parse_class).collect();
    println!("Parsed program successfully!🎉✍️");
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
                    _ => {
                        unreachable!()
                    }
                };
            }
            Class {
                name: other_name,
                fields,
                methods,
            }
        }
        _ => unreachable!(),
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
            for p in inners {
                match p.as_rule() {
                    Rule::ParamDeclList => {
                        let mut inner_param = p.into_inner();
                        for parm in inner_param {
                            assert_eq!(parm.as_rule(), Rule::ParamDecl);
                            let mut inTheParm = parm.into_inner();
                            let param_type = parse_Type(inTheParm.next().unwrap());
                            let param_name = next_id(&mut inTheParm);
                            params.push((param_type, param_name));
                        }
                    }
                    Rule::BlockStmt => body = Some(parse_BlockStmt(p)),
                    _ => {
                        dbg!("REGEL NICHT ABGEFANGEN: ");
                        dbg!(p.as_rule());
                        unreachable!()
                    }
                };
            }

            MethodDecl {
                ret_type,
                name: method_name,
                params,
                body: Stmt::Block(body.unwrap()),
            }
        }
        _ => {
            dbg!("REGEL NICHT ABGEFANGEN: ");
            dbg!(pair.as_rule());
            unreachable!()
        }
    }
}

fn parse_BlockStmt(pair: Pair<Rule>) -> Vec<Stmt> {
    /*println!(
        "parse_BlockStmt: rule = {:?}, str = {}",
        pair.as_rule(),
        pair.as_str()
    );*/
    let rule = pair.as_rule();
    let mut inner = pair.into_inner();
    match rule {
        Rule::BlockStmt => {
            let mut result = vec![];

            for curStmt in inner {
                result.append(&mut parse_Stmt(curStmt));
            }

            result
        }
        _ => {
            unreachable!()
        }
    }
}
fn parse_Stmt(pair: Pair<Rule>) -> Vec<Stmt> {
    debug!(
        "parse_Stmt: rule = {:?}, str = {}",
        pair.as_rule(),
        pair.as_str()
    );
    match pair.as_rule() {
        Rule::Stmt => parse_Stmt(pair.into_inner().next().unwrap()),
        Rule::WhileStmt => {
            let mut inners = pair.into_inner();
            let Expr = parse_expr(inners.next().unwrap());
            let Stmt = parse_Stmt(inners.next().unwrap());
            vec![Stmt::While(Expr, Box::new(Stmt::Block(Stmt)))]
        }
        Rule::IfElseStmt => {
            let mut inners = pair.into_inner();

            let mut firstif = inners.next().unwrap().into_inner();
            let Expr = parse_expr(firstif.next().unwrap());
            let Stmt = parse_Stmt(firstif.next().unwrap());
            let elsePart = parse_Stmt(inners.next().unwrap());

            vec![Stmt::If(
                Expr,
                Box::new(Stmt::Block(Stmt)),
                Some(Box::new(Stmt::Block(elsePart))),
            )]
        }
        Rule::IfStmt => {
            let mut inners = pair.into_inner();

            let Expr = parse_expr(inners.next().unwrap());
            let Stmt = parse_Stmt(inners.next().unwrap());
            vec![Stmt::If(Expr, Box::new(Stmt::Block(Stmt)), None)]
        }
        Rule::ReturnStmt => {
            let mut inners = pair.into_inner();

            let Expr = parse_expr(inners.next().unwrap());
            vec![Stmt::Return(Expr)]
        }
        Rule::LocalVarDeclStmt => {
            let mut inners = pair.into_inner();

            let mut result = vec![];
            let typeJ = parse_Type(inners.next().unwrap());

            let mut last_var_name = None;
            for inner in inners {
                match inner.as_rule() {
                    Rule::Identifier => {
                        if last_var_name.is_some() {
                            result.push(Stmt::LocalVarDecl(typeJ.clone(), last_var_name.unwrap()));
                        }
                        last_var_name = Some(inner.as_str().trim().to_string());
                    }
                    Rule::Expr => {
                        result.push(Stmt::LocalVarDecl(
                            typeJ.clone(),
                            last_var_name.as_ref().unwrap().clone(),
                        ));
                        result.push(Stmt::StmtExprStmt(StmtExpr::Assign(
                            Expr::LocalOrFieldVar(last_var_name.unwrap()),
                            parse_expr(inner),
                        )));
                        last_var_name = None;
                    }
                    _ => unreachable!(),
                }
            }
            if last_var_name.is_some() {
                result.push(Stmt::LocalVarDecl(typeJ, last_var_name.unwrap()));
            }
            result
        }
        Rule::StmtExpr => {
            vec![Stmt::StmtExprStmt(parse_StmtExpr(
                pair.into_inner().next().unwrap(),
            ))]
        }
        Rule::BlockStmt => parse_BlockStmt(pair),
        _ => {
            dbg!(pair.as_rule());
            unreachable!()
        }
    }
}

fn parse_StmtExpr(pair: Pair<Rule>) -> StmtExpr {
    debug!(
        "parse_StmtExpr: rule = {:?}, str = {}",
        pair.as_rule(),
        pair.as_str()
    );
    let rule = pair.as_rule();
    match rule {
        Rule::AssignExpr => {
            let mut inners = pair.into_inner();

            let mut name = inners.next().unwrap();
            let var = match name.as_rule() {
                Rule::Identifier => Expr::LocalOrFieldVar(name.as_str().trim().to_string()),
                Rule::InstVarExpr => parse_expr(name),
                _ => {
                    unreachable!()
                }
            };
            let Expr = parse_expr(inners.next().unwrap());

            StmtExpr::Assign(var, Expr)
        }
        Rule::NewExpr => {
            let mut inners = pair.into_inner();

            let id_name = parse_Type(inners.next().unwrap());
            let paramList = inners.next().unwrap().into_inner();
            let mut exprList: Vec<Expr> = vec![];
            for param in paramList {
                exprList.push(parse_expr(param));
            }

            StmtExpr::New(id_name, exprList)
        }
        Rule::MethodCallExpr => {
            let mut inners = pair.into_inner();
            let mut identifORinstVar = inners.next().unwrap();
            let String_name;
            let MethodExpr;
            match identifORinstVar.as_rule() {
                Rule::Identifier => {
                    String_name = identifORinstVar.as_str().trim().to_string();
                    MethodExpr = Expr::This;
                }
                Rule::InstVarExpr => {
                    let Expr::InstVar(a, b) = parse_expr(identifORinstVar) else { unreachable!() };
                    MethodExpr = *a;
                    String_name = b;
                }
                _ => {
                    dbg!(identifORinstVar.as_rule());
                    unreachable!()
                }
            }

            let mut exprList: Vec<Expr> = vec![];
            if let Some(paramList) = inners.next() {
                for param in paramList.into_inner() {
                    exprList.push(parse_expr(param));
                }
            }

            StmtExpr::MethodCall(MethodExpr, String_name, exprList)
        }
        _ => {
            dbg!(pair);
            unreachable!()
        }
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

        _ => {
            dbg!(pair.as_rule());
            unreachable!()
        }
    }
}

fn parse_field_var_decl_list(jtype: Type, pair: Pair<Rule>) -> Vec<FieldDecl> {
    assert_eq!(pair.as_rule(), Rule::FieldVarDeclList);
    let mut inners = pair.into_inner();
    let mut var_decl = inners.next().unwrap().into_inner();
    let name = next_id(&mut var_decl);
    let val = var_decl.next().map(parse_expr);
    let mut out = vec![FieldDecl {
        field_type: jtype.clone(),
        name,
        val,
    }];
    if let Some(p) = inners.next() {
        out.append(&mut parse_field_var_decl_list(jtype, p));
    }
    out
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
        _ => {
            dbg!(pair.as_rule());
            unreachable!()
        }
    }
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    let rule = pair.as_rule();
    /* println!(
        "parse_expr: rule = {:?}, str = {}",
        pair.as_rule(),
        pair.as_str()
    );*/
    match rule {
        Rule::Expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::ThisExpr => Expr::This,
        Rule::JNull => Expr::Jnull,
        Rule::InstVarExpr => {
            let mut pairs = pair.into_inner();
            let x = pairs.next().unwrap();
            let mut obj = match x.as_rule() {
                Rule::Identifier => Expr::LocalOrFieldVar(x.as_str().trim().to_string()),
                Rule::ThisExpr => Expr::This,
                _ => {
                    dbg!(x.as_rule());
                    unreachable!()
                }
            };
            for p in pairs {
                assert_eq!(p.as_rule(), Rule::Identifier);
                obj = Expr::InstVar(Box::new(obj), p.as_str().trim().to_string());
            }
            obj
        }
        Rule::UnaryExpr => {
            let mut inners = pair.into_inner();
            let unaryOP = next_id(&mut inners);
            let noBinExpr = parse_expr(inners.next().unwrap());
            Expr::Unary(unaryOP, Box::new(noBinExpr))
        }
        Rule::ParanthesizedExpr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::IntLiteral => Expr::Integer(pair.as_str().trim().parse().unwrap()),
        Rule::BoolLiteral => Expr::Bool(pair.as_str().parse().unwrap()),
        Rule::CharLiteral => Expr::Char(get_str_content(pair.as_str()).parse().unwrap()),
        Rule::StrLiteral => Expr::String(get_str_content(pair.as_str()).to_string()),
        Rule::StmtExpr => {
            Expr::StmtExprExpr(Box::new(parse_StmtExpr(pair.into_inner().next().unwrap())))
        }
        Rule::NonBinaryExpr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::Prec4BinExpr
        | Rule::Prec3BinExpr
        | Rule::Prec2BinExpr
        | Rule::Prec1BinExpr
        | Rule::Prec0BinExpr => {
            let pc = pair.clone();
            let mut inners = pair.into_inner();
            let left = inners.next().unwrap();
            let left = parse_expr(left);
            match inners.next() {
                None => left,
                Some(Op) => {
                    let opStr = Op.as_str().trim().to_string();
                    let right = inners.next().unwrap();
                    let right = parse_expr(right);

                    match right {
                        Expr::Binary(op, rl, rr) => {
                            if BinaryOp::prec(&op) == BinaryOp::prec(&opStr) {
                                Expr::Binary(
                                    op,
                                    Box::new(Expr::Binary(opStr, Box::new(left), rl)),
                                    rr,
                                )
                            } else {
                                Expr::Binary(
                                    opStr,
                                    Box::new(left),
                                    Box::new(Expr::Binary(op, rl, rr)),
                                )
                            }
                        }
                        _ => Expr::Binary(opStr, Box::new(left), Box::new(right)),
                    }
                }
            }
        }
        Rule::Identifier => Expr::LocalOrFieldVar(pair.as_str().trim().to_string()),

        _ => {
            dbg!(pair);
            unreachable!()
        }
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
