use crate::tokens::tokens::{Token, TokenType};

pub struct Program {
    pub statements: Vec<Statement>,
}

pub fn program_to_string(program: &Program) -> String {
    let mut output = String::new();
    for statement in program.statements.iter() {
        output += &statement_to_string(&statement);
    }

    output
}

#[derive(Debug)]
pub enum Statement {
    Let(Identifier, Expr),
    Return(Expr),
    BlockStatement(Vec<Box<Statement>>),
    Expr(Expr),
    None,
}

pub fn statement_to_string(statement: &Statement) -> String {
    match &*statement {
        Statement::Let(ident, expr) => {
            format!("let {} = {}", ident.value, expression_to_string(expr))
        }
        Statement::Return(expr) => format!("(return {})", expression_to_string(expr)),
        Statement::Expr(expr) => format!("{}", expression_to_string(expr)),
        Statement::BlockStatement(vec) => format!("{:?}", vec),
        Statement::None => "None".to_string(),
    }
}

#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    Identifier(Identifier),
    IntegerLiteral(i32),
    Prefix(String, Box<Expr>),
    Infix(TokenType, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Statement>, Option<Box<Statement>>),
    FunctionLiteral(Vec<Identifier>, Box<Statement>),
    CallExpresion{function: Box<Expr>, arguments: Vec<Box<Expr>>}, // Function can be either func_literal or identifier
    None,
}

pub fn expression_to_string(expression: &Expr) -> String {
    match expression {
        Expr::Bool(val) => format!("{}", val),
        Expr::Identifier(id) => format!("{}", id.value),
        Expr::IntegerLiteral(i) => format!("{}", i),
        Expr::Prefix(op, boxed_expr) => format!("({}{})", op, expression_to_string(boxed_expr)),
        Expr::Infix(op, boxed_left, boxed_right) => format!(
            "({} {} {})",
            expression_to_string(boxed_left),
            op,
            expression_to_string(boxed_right)
        ),
        Expr::If(condition, if_block, else_block) => match else_block {
            Some(box_else) => format!(
                "if {} {} else {}",
                expression_to_string(condition),
                statement_to_string(if_block),
                statement_to_string(box_else)
            ),
            None => format!(
                "if {} {}",
                expression_to_string(condition),
                statement_to_string(if_block)
            ),
        },
        Expr::FunctionLiteral(parameters, body) => format!("fn ({:?}) {}", parameters, statement_to_string(body)),
        Expr::CallExpresion{function: boxed_func, arguments: args} => {
        	let mut arg_string = String::new();
        	for (index, expr) in args.iter().enumerate() {
        		arg_string.push_str(&expression_to_string(expr));
        		if args.len() - (index+1) > 0{
        			println!("len {:?} index {}", args.len(), index);
        			arg_string.push_str(", ");
        		}
        	}

        	format!("{}({})", expression_to_string(boxed_func), arg_string)
        },
        Expr::None => "none".to_string(),
    }
}
#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}
