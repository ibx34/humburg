use common::{HumburgCursor, LexResult};
use std::fmt::Debug;

const PRIM_TYPES: [&str; 2] = ["int", "str"];

#[derive(Debug)]
pub enum TyExpr {
    Int,
    Str,
}

impl TyExpr {
    pub fn try_from_option(from: &str) -> Option<TyExpr> {
        match from {
            "int" => Some(TyExpr::Int),
            "str" => Some(TyExpr::Str),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Exprs {
    Lambda,
    Ty(TyExpr),
    TyList(Vec<Box<Exprs>>),
}

#[derive(Debug)]
pub struct HumburgParser<A>
where
    A: Iterator<Item = LexResult> + Debug,
{
    pub cursor: HumburgCursor<A, LexResult>,
    pub res: Vec<Exprs>,
}

impl<A> HumburgParser<A>
where
    A: Iterator<Item = LexResult> + Debug,
{
    pub fn push_back(&mut self, l_res: Exprs, should_advance: bool) {
        self.res.push(l_res);
        if should_advance {
            self.cursor.advance();
        }
    }

    pub fn parse_expr(&mut self, to_parse: Option<LexResult>) -> Option<Exprs> {
        let to_parse = if let Some(ref to_parse) = to_parse {
            to_parse
        } else {
            self.cursor.peek().to_owned()?
        };
        match to_parse {
            LexResult::Identifier(ident) => {
                let ident = ident.to_owned();
                let lowered_and_str = ident.to_lowercase();
                if PRIM_TYPES.contains(&lowered_and_str.as_str()) {
                    let r#type = TyExpr::try_from_option(&lowered_and_str)?;
                    return Some(Exprs::Ty(r#type));
                }
                self.cursor.advance();
                let next_peeked = self.cursor.peek()?.to_owned();
                let type_list = self.parse_expr(Some(next_peeked));
                println!(
                    "The types in the type list for the assignment {ident:?} are: {type_list:?}"
                );

                None
            }
            LexResult::OpenSquare => {
                let mut till_close_square = self
                    .cursor
                    .iter
                    .by_ref()
                    .take_while(|e| e != &LexResult::CloseSquare)
                    .collect::<Vec<LexResult>>()
                    .into_iter()
                    .peekable();
                let mut types_in_type_list: Vec<Box<Exprs>> = Vec::new();
                while let Some(n) = till_close_square.peek() {
                    let n = n.to_owned();
                    match n {
                        LexResult::Dash => {
                            till_close_square.next();
                            if let Some(n) = till_close_square.peek() {
                                if n == &LexResult::GreatherThan {
                                    till_close_square.next();
                                    continue;
                                }
                            }
                        }
                        ident @ LexResult::Identifier(_) => {
                            if let ty_expr @ Some(Exprs::Ty(_)) = self.parse_expr(Some(ident)) {
                                types_in_type_list.push(Box::new(ty_expr?))
                            }
                            till_close_square.next();
                        }
                        _ => _ = till_close_square.next(),
                    }
                }
                Some(Exprs::TyList(types_in_type_list))
            }
            _ => None,
        }
    }
}
