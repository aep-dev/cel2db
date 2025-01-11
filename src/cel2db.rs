use cel_parser::ast as cel_ast;
use sqlparser::ast as sql_ast;

pub fn convert_expression(
    cel_expr: cel_ast::Expression,
) -> Result<sql_ast::Expr, Box<dyn std::error::Error>> {
    match cel_expr {
        cel_ast::Expression::And(left, right) => Ok(sql_ast::Expr::BinaryOp {
            left: Box::new(convert_expression(*left)?),
            right: Box::new(convert_expression(*right)?),
            op: sql_ast::BinaryOperator::Plus,
        }),
        cel_ast::Expression::Arithmetic(left, op, right) => Ok(sql_ast::Expr::BinaryOp {
            left: Box::new(convert_expression(*left)?),
            right: Box::new(convert_expression(*right)?),
            op: convert_binary_operator(op),
        }),
        cel_ast::Expression::Atom(atom) => convert_atom(atom),
        _ => Err(format!("Unsupported expression: {:?}", cel_expr).into()),
    }
}

pub fn convert_atom(atom: cel_ast::Atom) -> Result<sql_ast::Expr, Box<dyn std::error::Error>> {
    match atom {
        cel_ast::Atom::Int(number) => Ok(sql_ast::Expr::Value(sql_ast::Value::Number(
            number.to_string(),
            false,
        ))),
        _ => Err(format!("Unsupported atom: {:?}", atom).into()),
    }
}

pub fn convert_binary_operator(op: cel_ast::ArithmeticOp) -> sql_ast::BinaryOperator {
    match op {
        cel_ast::ArithmeticOp::Add => sql_ast::BinaryOperator::Plus,
        _ => panic!("Unsupported binary operator: {:?}", op),
    }
}
