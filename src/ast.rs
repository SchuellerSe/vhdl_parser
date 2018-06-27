// AST Definiton
// Author: Sebastian Schüller <schueller@ti.uni-bonn.de>

use std;
use token::{Token, TokenKind, TokenKind::*};
use SrcPos;

#[derive(Debug, Clone, Default)]
pub struct NodeId(u32);


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Exp,  // **
    Abs,  // abs
    Not,  // not
    Mul,  // *
    Div,  // /
    Mod,  // mod
    Rem,  // rem
    Add,  // +
    Sub,  // -
    Cat,  // & (concatenation)
    Sll,  // sll
    Srl,  // srl
    Sla,  // sla
    Sra,  // sra
    Rol,  // rol
    Ror,  // ror
    Eq,   // =
    NEq,  // /=
    Gt,   // >
    GEq,  // >=
    Lt,   // <
    LEq,  // <=
    QEq,  // ?=
    QNEq, // ?/=
    QLt,  // ?<
    QLEq, // ?<=
    QGt,  // ?>
    QGEq, // ?>=
    And,  // and
    Or,   // or
    Nand, // nand
    Nor,  // nor
    Xor,  // xor
    Xnor, // xnor
    QQ,   // ??
    UnPos,// + <>
    UnNeg,// - <>
    UnAnd,//
    UnOr, //
    UnNand,//
    UnNor,//
    UnXor,//
    UnXnor,
}


impl Op {
    pub fn from_op_symbol(s: &str) -> Option<Op> {
        match s {
            "abs"  => Some(Op::Abs),
            "not"  => Some(Op::Not),
            "mod"  => Some(Op::Mod),
            "rem"  => Some(Op::Rem),
            "sll"  => Some(Op::Sll),
            "srl"  => Some(Op::Srl),
            "sla"  => Some(Op::Sla),
            "sra"  => Some(Op::Sra),
            "rol"  => Some(Op::Rol),
            "ror"  => Some(Op::Ror),
            "and"  => Some(Op::And),
            "or"   => Some(Op::Or),
            "nand" => Some(Op::Nand),
            "nor"  => Some(Op::Nor),
            "xor"  => Some(Op::Xor),
            "xnor" => Some(Op::Xnor),
            "="    => Some(Op::Eq),
            "/="   => Some(Op::NEq),
            "<"    => Some(Op::Lt),
            ">"    => Some(Op::Gt),
            ">="   => Some(Op::GEq),
            "<="   => Some(Op::LEq),
            "+"    => Some(Op::Add),
            "-"    => Some(Op::Sub),
            "&"    => Some(Op::Cat),
            "?="   => Some(Op::QEq),
            "?/="  => Some(Op::QNEq),
            "?<"   => Some(Op::QLt),
            "?<="  => Some(Op::QLEq),
            "?>"   => Some(Op::QGt),
            "?>="  => Some(Op::QGEq),
            "*"    => Some(Op::Mul),
            "**"   => Some(Op::Exp),
            "/"    => Some(Op::Div),
            _      => None,
        }
    }

    // If a token can represent both a binary and unary
    // op, we default here to get the binary op.
    pub fn from_token(t: &Token) -> Option<Op> {
        match t.kind {
            StarStar => Some(Op::Exp),
            Abs      => Some(Op::Abs),
            Not      => Some(Op::Not),
            Star     => Some(Op::Mul),
            Slash    => Some(Op::Div),
            Mod      => Some(Op::Mod),
            Rem      => Some(Op::Rem),
            Plus     => Some(Op::Add),
            Minus    => Some(Op::Sub),
            Amp      => Some(Op::Cat),
            Sll      => Some(Op::Sll),
            Sla      => Some(Op::Srl),
            Sra      => Some(Op::Sla),
            Srl      => Some(Op::Sra),
            Rol      => Some(Op::Rol),
            Ror      => Some(Op::Ror),
            Eq       => Some(Op::Eq),
            SlashEq  => Some(Op::NEq),
            Gt       => Some(Op::Gt),
            GEq      => Some(Op::GEq),
            Lt       => Some(Op::Lt),
            LEq      => Some(Op::LEq),
            QEq      => Some(Op::QEq),
            QSlashEq => Some(Op::QNEq),
            QLt      => Some(Op::QLt),
            QLEq     => Some(Op::QLEq),
            QGt      => Some(Op::QGt),
            QGEq     => Some(Op::QGEq),
            And      => Some(Op::And),
            Or       => Some(Op::Or),
            Xor      => Some(Op::Nand),
            Nand     => Some(Op::Nor),
            Nor      => Some(Op::Xor),
            Xnor     => Some(Op::Xnor),
            QQ       => Some(Op::QQ),
            _ => None,
        }
    }

    pub fn unary_from_token(t: &Token) -> Option<Op> {
        match t.kind {
            QQ    => Some(Op::QQ),
            Plus  => Some(Op::UnPos),
            Minus => Some(Op::UnNeg),
            And   => Some(Op::UnAnd),
            Or    => Some(Op::UnOr),
            Nand  => Some(Op::UnNand),
            Nor   => Some(Op::UnNor),
            Xor   => Some(Op::UnXnor),
            Xnor  => Some(Op::UnXnor),
            _ => None,
        }

    }

    pub fn precedence(&self) -> u32 {
        match self {
            Op::Exp | Op::Abs  | Op::Not              => 7,
            Op::Mul | Op::Div  | Op::Mod | Op::Rem    => 6,
            Op::UnPos | Op::UnNeg                     => 5,
            Op::Add | Op::Sub  | Op::Cat              => 4,
            Op::Sll | Op::Srl  | Op::Sla |
            Op::Sra | Op::Rol  | Op::Ror              => 3,
            Op::Eq  | Op::NEq  | Op::Gt   |
            Op::GEq | Op::Lt   | Op::LEq  |
            Op::QEq | Op::QNEq | Op::QLt  | Op::QLEq |
            Op::QGt | Op::QGEq                        => 2,
            Op::UnAnd | Op::UnOr | Op::UnXor | Op::UnNand | Op::UnNor | Op::UnXnor |
            Op::And  | Op::Or   | Op::Nand| Op::Nor  |
            Op::Xor  | Op::Xnor                       => 1,
            Op::QQ                                    => 0,
        }
    }

    pub fn assoc(&self) -> Assoc {
        if *self == Op::Exp {
            return Assoc::Right;
        }
        return Assoc::Left;
    }

}

#[derive(Debug, Clone)]
pub struct NumericLit {
    pub pos: SrcPos,
}


#[derive(Debug, Clone)]
pub struct StringLit {
    pub pos: SrcPos,
}

#[derive(Debug, Clone)]
pub struct CharLit {
    pub pos: SrcPos,
}

#[derive(Debug, Clone)]
pub enum Direction {
    To,
    Downto,
}

impl From<TokenKind> for Direction {
    fn from(k: TokenKind) -> Direction {
        match k {
            To     => Direction::To,
            Downto => Direction::Downto,
            _ => {
                panic!("Internal Compiler Error {}, {}: Tried to build a range direction from token other than To or Downto (namely this: {:?})", file!(), line!(), k);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RangeExpr {
    pub lhs: Box<Expr>,
    pub dir: Direction,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum DiscreteRange {
    SubtypeIndication(SubtypeIndication),
    Range(RangeExpr),
    Attribute(Name),
}

#[derive(Debug, Clone)]
pub struct BinOpExpr {
    pub lhs: Box<Expr>,
    pub op: Op,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct UnOpExpr {
    pub op: Op,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct QualifiedExpr {
    pub qualifier: Name,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct TypeCastExpr {
    pub target_type: Name,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct AssocExpr {
    pub choices: Box<Expr>,
    pub designator: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Paren{ lvl: u32, expr: Box<Expr> },
    BinOp(BinOpExpr),
    UnOp(UnOpExpr),
    Qualified(QualifiedExpr),
    TypeCast(TypeCastExpr),
    Name(Name),
    Range(RangeExpr),
    Assoc(AssocExpr),
    List(Vec<Expr>),
    Inertial(Box<Expr>),
    NumLit(NumericLit),
    StrLit(StringLit),
    ChrLit(CharLit),
    SubtypeIndication(SubtypeIndication),
    Other,
    Open,
}

impl ExprKind {
    pub fn new_unop(op: Op, rhs: Expr) -> ExprKind {
        ExprKind::UnOp(UnOpExpr {
            op,
            rhs: Box::new(rhs),
        })
    }
}

impl PartialEq<Self> for ExprKind {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub pos: SrcPos,
    pub kind: ExprKind,
}

impl From<Vec<Expr>> for Expr {
    fn from(mut val: Vec<Expr>) -> Expr {
        assert!(val.len() >= 1);
        if val.len() == 1 {
            return val.pop().unwrap();
        }
        return Expr::new(
            val.first().unwrap().pos.to(&val.last().unwrap().pos),
            ExprKind::List(val)
        );
    }
}

impl Expr {
    pub fn new(pos: SrcPos, kind: ExprKind) -> Expr {
        Expr {
            pos,
            kind,
        }
    }

    pub fn new_paren(pos: SrcPos, inner: Expr) -> Expr {
        Expr::new(pos, ExprKind::Paren {
            lvl: 1 + inner.nesting_lvl(),
            expr: Box::new(inner),
        })
    }

    pub fn is_valid_choices(&self) -> bool {
        match self.kind {
            ExprKind::List(ref vec) => {
                vec.iter().map(|e| match e.kind {
                    ExprKind::Name(_) => true,
                    ExprKind::Range(_) => true,
                    _ => false,
                }).fold(true, |acc, x| acc && x)
            },
            ExprKind::Range(_) => true,
            ExprKind::Other    => true,
            _ => false,
        }
    }

    pub fn is_name(&self) -> bool {
        match self.kind {
            ExprKind::Name(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_name(self) -> Name {
        match self.kind {
            ExprKind::Name(name) => name,
            _ => panic!(),
        }

    }

    pub fn without_parens(&self) -> &Expr {
        let mut paren_expr = self;
        loop {
            match paren_expr.kind {
                ExprKind::Paren{lvl: _, ref expr} => paren_expr = expr,
                _ => return paren_expr,
            }
        }
    }

    pub fn is_valid_formal_part(&self) -> bool {
        match self.without_parens().kind {
            ExprKind::Name(_) => true,
            _ => false,
        }
    }

    pub fn nesting_lvl(&self) -> u32 {
        match self.without_parens().kind {
            ExprKind::Paren{lvl, ..} => lvl,
            _ => 0,
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct Signature {
    pub pos: SrcPos,
    pub parameter_typenames: Vec<Name>,
    pub return_typename: Option<Name>,
}

#[derive(Debug, Clone)]
pub enum SegmentKind {
    QualifiedExpr(Box<Expr>),
    UnparsedBlob(Vec<Token>),
    AttachedExpression(Box<Expr>),
    AttachedRange(Box<Range>),
    CharLiteral(char),
    Signature(Box<Signature>),
    Attribute,
    AllQualifier,
    OperatorSymbol(Op),
    Identifier,
}

impl SegmentKind {
    pub fn unwrap_qualified_expr(self) -> Box<Expr> {
        match self {
            SegmentKind::QualifiedExpr(x) => x,
            _ => panic!("Unwrapped Wrong SegmentKind!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameSegment {
    pub pos: SrcPos,
    pub kind: SegmentKind,
}

#[derive(Debug, Clone, Default)]
pub struct Name {
    pub pos: SrcPos,
    pub segments: Vec<NameSegment>,
}

impl Name {
    pub fn add_segment(&mut self, seg: NameSegment) {
        self.pos = self.pos.to(&seg.pos);
        self.segments.push(seg);
    }

    pub fn is_qualifiend_expr(&self) -> bool {
        if self.segments.len() < 2 {
            return false;
        }
        match self.segments.last().map(|ref seg| &seg.kind) {
            Some(SegmentKind::QualifiedExpr(_)) => true,
            _ => false,
        }
    }

    pub fn is_attribute(&self) -> bool {
        match self.segments.last() {
            Some(NameSegment {pos: _, kind: SegmentKind::Attribute }) => true,
            _ => false,
        }
    }

    pub fn is_simple(&self) -> bool {
        self.segments.len() == 1 &&
            match self.segments.first().map(|ref seg| &seg.kind) {
                Some(SegmentKind::Identifier) => true,
                _ => false,
        }
    }

    pub fn pop_constraint(&mut self) -> Option<Constraint> {
        match self.segments.last() {
            Some(NameSegment {pos: _, kind: SegmentKind::AttachedExpression(_)}) => (),
            _ => return None,
        }

        if let Some(NameSegment {pos: _, kind: SegmentKind::AttachedExpression(expr)}) = self.segments.last() {
            // Try to convert the expression to a valid segment constraint

            Constraint::try_from(expr)
        } else {
            None
        }
    }

}

#[derive(Debug, Clone)]
pub enum ResolutionIndication {
    Function(Box<Name>),
    ArrayIndication{
        pos: SrcPos,
        lvl: u32,
        resolution: Box<ResolutionIndication>
    },
    RecordIndication{
        pos: SrcPos,
        resolutions: Vec<(Name, ResolutionIndication)>
    },
}

impl ResolutionIndication {
    pub fn is_function(&self) -> bool {
        match self {
            ResolutionIndication::Function(_) => true,
            _ => false,
        }
    }

    pub fn pos(&self) -> SrcPos {
        match self {
            ResolutionIndication::Function(name) => name.pos.clone(),
            ResolutionIndication::ArrayIndication {pos, ..} => pos.clone(),
            ResolutionIndication::RecordIndication {pos, ..} => pos.clone(),
        }
    }

    pub fn lvl(&self) -> u32 {
        match self {
            ResolutionIndication::ArrayIndication{lvl, ..}  => lvl.clone(),
            _ => 0,
        }
    }

    pub fn try_into_name(self) -> Option<Name> {
        match self {
            ResolutionIndication::Function(name) => Some(*name),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Range {
    Name(Name),
    Expr(RangeExpr),
}

impl Range {
    pub fn pos(&self) -> SrcPos {
        match self {
            Range::Name(name) => name.pos,
            Range::Expr(expr) => expr.lhs.pos.to(&expr.rhs.pos),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElementConstraint {
    ArrayOpen{pos: SrcPos, next: Option<Box<ElementConstraint>>},
    Array    {pos: SrcPos, constraints: Vec<DiscreteRange>, next: Option<Box<ElementConstraint>>},
    Record   {pos: SrcPos, constraints: Vec< (Box<Name>, Box<ElementConstraint>) >},
}

impl ElementConstraint {
    pub fn pos(&self) -> SrcPos {
        match self {
            ElementConstraint::ArrayOpen{pos, ..} => pos.clone(),
            ElementConstraint::Array{pos, ..}     => pos.clone(),
            ElementConstraint::Record{pos, ..}    => pos.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constraint {
    Range   {pos: SrcPos, constraint: Box<Range>},
    Element {pos: SrcPos, constraint: Box<ElementConstraint>},
}

impl Constraint {
    pub fn new_range(pos: SrcPos, range: Range) -> Constraint{
        Constraint::Range {
            pos: pos,
            constraint: Box::new(range),
        }
    }

    fn try_from(_expr: &Expr) -> Option<Constraint> {
        /*
        let pos = expr.pos;
        if let ExprKind::Paren{expr: expr, ..} = expr.kind {
            if let ExprKind::Range(_) = expr.kind {
                let constraint = Constraint::new_range(*expr.clone());
                return Some(constraint);
            }

            if let ExprKind::Open = expr.kind {
                return Some(Constraint::Open{pos});
            }

            if let ExprKind::List(exprs) = expr.kind {

            }

            None
        } else {
            None
        }
        */
        unimplemented!();


    }

}


#[derive(Debug, Clone, Default)]
pub struct SubtypeIndication {
    pub pos: SrcPos,
    pub typemark: Name,
    pub resolution: Option<ResolutionIndication>,
    pub constraint: Option<Box<Constraint>>,
}

#[derive(Debug, Clone, Default)]
pub struct Identifier {
    pub pos: SrcPos,
}

#[derive(Debug, Clone, Default)]
pub struct EntityDecl {
    pub pos: SrcPos,
    pub name: Identifier,
    //pub generics: Vec<InterfaceDecl>,
    pub ports: Vec<PortDecl>,
}

#[derive(Debug, Clone, Default)]
pub struct PortDecl {
    pub pos: SrcPos,
    pub idents: Vec<Identifier>,
    pub mode: PortMode,
    pub typemark: SubtypeIndication,
    pub is_bus: bool,
    pub default_expr: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub enum PortMode {
    In,
    Out,
    Inout,
    Buffer,
    Linkage
}

impl Default for PortMode {
    fn default() -> PortMode {
        PortMode::In
    }
}

impl PortMode {
    pub fn try_from_tokenkind(kind: TokenKind) -> Option<PortMode> {
        match kind {
            TokenKind::In =>      Some(PortMode::In),
            TokenKind::Out =>     Some(PortMode::Out),
            TokenKind::Inout =>   Some(PortMode::Inout),
            TokenKind::Buffer =>  Some(PortMode::Buffer),
            TokenKind::Linkage => Some(PortMode::Linkage),
            _ => None,
        }
    }
}
