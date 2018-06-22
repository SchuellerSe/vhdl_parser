//#[macro_use]
//extern crate bitflags;

macro_rules! _matches_tt_as_expr_hack {
    ($value:expr) => ($value)
}

macro_rules! matches {
    ($expression:expr, $($pattern:tt)+) => {
        _matches_tt_as_expr_hack! {
            match $expression {
                $($pattern)+ => true,
                _ => false
            }
        }
    }
}

pub mod token;
pub mod lexer;
pub mod parser;
pub mod ast;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SrcPos(pub u32, pub u32);

impl SrcPos {
    pub fn from_src_pos(pos: &SrcPos) -> SrcPos {
        pos.clone()
    }

    pub fn invalid() -> SrcPos {
        SrcPos(0,0)
    }

    pub fn to(&self, pos: &SrcPos) -> SrcPos {
        SrcPos(self.0, pos.1)
    }

    pub fn as_range(&self) -> std::ops::Range<usize> {
        std::ops::Range { start: self.0 as usize, end: self.1 as usize }
    }
}


#[derive(Debug, Clone)]
pub enum ParseError {
    ExprChoicesWithoutDesignator,
    InvalidOpSymbolString,
    UnexpectedToken(token::Token, String),
    UnexpectedEoF,
    Internal,
}

pub type PResult<T>=Result<T, ParseError>;



#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
