mod ast;

lalrpop_util::lalrpop_mod!(parse, "/math/parse.rs");

pub use crate::math::ast::Math;
use crate::math::parse::{ExprMulParser, ExprParser};

impl Math {
    pub fn new_neutral(s: &str) -> Result<Math, String> {
        ExprParser::new().parse(s).map_err(|x| format!("{}", x))
    }

    pub fn new_add_first(s: &str) -> Result<Math, String> {
        ExprMulParser::new().parse(s).map_err(|x| format!("{}", x))
    }
}
