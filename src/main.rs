mod cel2db;
use cel2db::convert_expression;
use cel_parser::parse;

fn main() {
    let cel_expr = "1 + 2";
    let cel_ast = parse(cel_expr).unwrap();
    match convert_expression(cel_ast) {
        Ok(sql_ast) => println!("Converted SQL AST: {:?}", sql_ast),
        Err(e) => eprintln!("Error converting CEL to SQL AST: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cel_to_sql_ast_basic() {
        let cel_expr = "1 + 2";
        let cel_ast = parse(cel_expr).unwrap();
        let result = convert_expression(cel_ast).unwrap();
        assert_eq!(result.to_string(), "1 + 2");
    }
}
