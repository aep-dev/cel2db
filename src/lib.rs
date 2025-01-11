mod cel2db;
use cel2db::convert_expression;
use cel_parser::parse;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn cel_to_sql(cel_expr: *const c_char, sql_dialect: *const c_char) -> *mut c_char {
    let cel_str = unsafe {
        if cel_expr.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(cel_expr).to_str().unwrap_or("")
    };

    // TODO: use the dialect when writing the expression. It doesn't seem
    // like there is a way to specify the dialct when writing to_str today.
    let _ = unsafe {
        if sql_dialect.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(sql_dialect).to_str().unwrap_or("")
    };

    let result = match parse(cel_str) {
        Ok(cel_ast) => match convert_expression(cel_ast) {
            Ok(sql_ast) => sql_ast.to_string(),
            Err(_) => String::from(""),
        },
        Err(_) => String::from(""),
    };

    CString::new(result)
        .map(|c_str| c_str.into_raw())
        .unwrap_or(std::ptr::null_mut())
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
