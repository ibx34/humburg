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
    // todo: add a peek function please... This has gotta exist in order to fill the struct's field.
    pub fn push_back(&mut self, l_res: Exprs, should_advance: bool) {
        self.res.push(l_res);
        if should_advance {
            self.cursor.advance();
        }
    }

    pub fn parse_lambda(&mut self) -> Option<Exprs> {
        None
    }

    pub fn parse_expr(&mut self, to_parse: Option<LexResult>) -> Option<Exprs> {
        let to_parse = if let Some(ref to_parse) = to_parse {
            to_parse
        } else {
            self.cursor.peek().to_owned()?
        };
        match to_parse {
            LexResult::At => {
                self.parse_lambda()
            }
            LexResult::Identifier(ident) => {
                let ident = ident.to_owned();
                let lowered_and_str = ident.to_lowercase();
                // This gets us passed the identifer we are currently on
                self.cursor.advance();
                // This gets us the next character.
                let next_peeked = self.cursor.advance()?.to_owned();
                if PRIM_TYPES.contains(&lowered_and_str.as_str()) {
                    let r#type = TyExpr::try_from_option(&lowered_and_str)?;
                    return Some(Exprs::Ty(r#type));
                }
                match next_peeked {
                    LexResult::OpenSquare => {
                        let type_list = self.parse_expr(Some(next_peeked));
                        println!(
                            "The types in the type list for the assignment {ident:?} are: {type_list:?}"
                        );
                    }
                    _ => {}
                }
                //     }
                //     _ => {}
                // }
                None
            }
            // todo: the problem is with the advances
            LexResult::OpenSquare => {
                // This will get us past the open square we are currently on.
                let mut types_in_type_list: Vec<Box<Exprs>> = Vec::new();
                while let Some(next) = self.cursor.peek() {
                    let next = next.to_owned();
                    match next {
                        LexResult::CloseSquare => break,
                        ref os @ LexResult::OpenSquare | ref os @ LexResult::Identifier(_) => {
                            if os == &LexResult::OpenSquare {
                                self.cursor.advance();
                            }
                            let new_expr = self.parse_expr(Some(os.to_owned()))?;
                            types_in_type_list.push(Box::new(new_expr));
                        }
                        LexResult::Space => _ = self.cursor.advance(),
                        unkown @ _ => {
                            println!("Unkown: {unkown:?}");
                            self.cursor.advance();
                        }
                    }
                }
                Some(Exprs::TyList(types_in_type_list))
            }
            _ => None,
        }
    }
}
