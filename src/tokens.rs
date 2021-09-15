use std::convert::TryFrom;

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum Token {
    ID(String),
    Number(String),
    StringVal(String),

    OpenRoundBrack,
    CloseRoundBrack,
    OpenCurlyBrack,
    CloseCurlyBrack,
    SemiColon,
    Comma,

    Plus,
    Minus,
    Mul,
    Div,
    Eq,
    EqEq,

    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    Not,
    NotEq,
    Or,
    And,

    PrintStart,
    PrintEnd,

    LetStart,

    ForStart,
    ForAux1,
    ForAux2,
    In,
    ForAux3,
    ForAux4,

    IfStart,
    IfAux1,
    IfAux2,
    ElseStart,
    ElseAux1,

    WhileStart,
    WhileAux1,
    WhileAux2,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ID(s) => {
                write!(f, "ID( {}  )", s)
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = String;

    fn try_from(t: &str) -> Result<Self, Self::Error> {
        use Token::*;
        match t {
            "PrintStart" => Ok(PrintStart),
            "PrintEnd" => Ok(PrintEnd),
            "ForStart" => Ok(ForStart),
            "ForAux1" => Ok(ForAux1),
            "ForAux2" => Ok(ForAux2),
            "In" => Ok(In),
            "ForAux3" => Ok(ForAux3),
            "ForAux4" => Ok(ForAux4),
            "IfStart" => Ok(IfStart),
            "IfAux1" => Ok(IfAux1),
            "IfAux2" => Ok(IfAux2),
            "LetStart" => Ok(LetStart),
            "Or" => Ok(Or),
            "And" => Ok(And),
            "WhileStart" => Ok(WhileStart),
            "WhileAux1" => Ok(WhileAux1),
            "WhileAux2" => Ok(WhileAux2),
            "ElseStart" => Ok(ElseStart),
            "ElseAux1" => Ok(ElseAux1),
            _ => Err(format!("Unknown Token Type in config : {}", t)),
        }
    }
}
