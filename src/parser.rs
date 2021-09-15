// auto-generated: "lalrpop 0.19.6"
// sha3: 5da8b9321afb95383afc4268882b6a975843c7e4a1c5ac27253475e544c0
use crate::lexer;
use crate::tokens;
use crate::config::LangConfig;
use crate::ast::*;
use tokens::Token::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::lexer;
    use crate::tokens;
    use crate::config::LangConfig;
    use crate::ast::*;
    use tokens::Token::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(tokens::Token),
        Variant1(core::option::Option<tokens::Token>),
        Variant2(AST),
        Variant3(Expr),
        Variant4(Stmt),
        Variant5(Vec<Stmt>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 28, 0, 0, 0,
        // State 3
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 28, 0, 0, 0,
        // State 4
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 28, 0, 0, 0,
        // State 5
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 28, 0, 0, 0,
        // State 6
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 28, 0, 0, 0,
        // State 7
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 28, 0, 0, 0,
        // State 8
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 27, 0, 0, 28, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, -22, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, -20, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, -21, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, -19, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, -24, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, -23, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 30, 0, 6, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, -8, 8, -8, 0, -8, 9, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, -8, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, -27, -27, -27, 0, -27, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, -27, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, -26, -26, -26, 0, -26, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, -26, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, -28, -28, -28, 0, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, -28, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, -11, -11, -11, 0, -11, -11, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, -11, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, -12, -12, -12, 0, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, -12, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, -14, -14, -14, 0, -14, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, -14, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, -25, -25, -25, 0, -25, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, -25, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 6, 0, 7, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, -13, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 32, -17, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 37, 0, 6, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, -16, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, -6, 8, -6, 0, -6, 9, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, -6, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, -7, 8, -7, 0, -7, 9, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, -9, -9, -9, 0, -9, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, -10, -10, -10, 0, -10, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, -10, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, -15, -15, -15, 0, -15, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, -15, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 40 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -18,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        -22,
        // State 10
        -20,
        // State 11
        -5,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -21,
        // State 15
        -19,
        // State 16
        -24,
        // State 17
        -23,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -8,
        // State 21
        -27,
        // State 22
        -26,
        // State 23
        -28,
        // State 24
        -11,
        // State 25
        -12,
        // State 26
        -14,
        // State 27
        -25,
        // State 28
        -13,
        // State 29
        -17,
        // State 30
        0,
        // State 31
        -16,
        // State 32
        -6,
        // State 33
        -7,
        // State 34
        -9,
        // State 35
        -10,
        // State 36
        -15,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => match state {
                3 => 28,
                4 => 30,
                _ => 19,
            },
            4 => match state {
                5 => 32,
                6 => 33,
                _ => 20,
            },
            5 => 21,
            6 => match state {
                1 => 16,
                _ => 9,
            },
            7 => 22,
            8 => match state {
                1 => 17,
                _ => 10,
            },
            9 => 11,
            10 => 1,
            11 => 23,
            12 => match state {
                7 => 34,
                8 => 35,
                _ => 24,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""{""###,
            r###""||""###,
            r###""}""###,
            r###"ElseAux1"###,
            r###"ElseStart"###,
            r###"ForAux1"###,
            r###"ForAux2"###,
            r###"ForAux3"###,
            r###"ForAux4"###,
            r###"ForStart"###,
            r###"ID"###,
            r###"IfAux1"###,
            r###"IfAux2"###,
            r###"IfStart"###,
            r###"In"###,
            r###"LetStart"###,
            r###"Num"###,
            r###"PrintEnd"###,
            r###"PrintStart"###,
            r###"StrLit"###,
            r###"WhileAux1"###,
            r###"WhileAux2"###,
            r###"WhileStart"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'__0>
    where 
    {
        lc: &'__0 LangConfig,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<'__0> __state_machine::ParserDefinition for __StateMachine<'__0>
    where 
    {
        type Location = usize;
        type Error = lexer::LexerError;
        type Token = tokens::Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = AST;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 40 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.lc,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &tokens::Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            Not if true => Some(0),
            NotEq if true => Some(1),
            And if true => Some(2),
            OpenRoundBrack if true => Some(3),
            CloseRoundBrack if true => Some(4),
            Mul if true => Some(5),
            Plus if true => Some(6),
            Comma if true => Some(7),
            Minus if true => Some(8),
            Div if true => Some(9),
            SemiColon if true => Some(10),
            LessThan if true => Some(11),
            LessThanEq if true => Some(12),
            Eq if true => Some(13),
            EqEq if true => Some(14),
            GreaterThan if true => Some(15),
            GreaterThanEq if true => Some(16),
            OpenCurlyBrack if true => Some(17),
            Or if true => Some(18),
            CloseCurlyBrack if true => Some(19),
            ElseAux1 if true => Some(20),
            ElseStart if true => Some(21),
            ForAux1 if true => Some(22),
            ForAux2 if true => Some(23),
            ForAux3 if true => Some(24),
            ForAux4 if true => Some(25),
            ForStart if true => Some(26),
            ID(_) if true => Some(27),
            IfAux1 if true => Some(28),
            IfAux2 if true => Some(29),
            IfStart if true => Some(30),
            In if true => Some(31),
            LetStart if true => Some(32),
            Number(_) if true => Some(33),
            PrintEnd if true => Some(34),
            PrintStart if true => Some(35),
            StringVal(_) if true => Some(36),
            WhileAux1 if true => Some(37),
            WhileAux2 if true => Some(38),
            WhileStart if true => Some(39),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: tokens::Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    pub struct programParser {
        _priv: (),
    }

    impl programParser {
        pub fn new() -> programParser {
            programParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            lc: &LangConfig,
            __tokens0: __TOKENS,
        ) -> Result<AST, __lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    lc,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        lc: &LangConfig,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<AST,__lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(lc, __sym0);
                return Some(Ok(__nt));
            }
            5 => {
                __reduce5(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(lc, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AST, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Stmt>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<tokens::Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, tokens::Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? = ";" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? =  => ActionFn(24);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action24::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PrintEnd? = PrintEnd => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PrintEnd? =  => ActionFn(22);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action22::<>(lc, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce5<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // expr = expr, "+", factor => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce6<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // expr = expr, "-", factor => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // expr = factor => ActionFn(10);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce8<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // factor = factor, "*", value => ActionFn(11);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce9<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // factor = factor, "/", value => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce10<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // factor = value => ActionFn(13);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce11<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // id = ID => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce12<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // let_stmt = LetStart, ID, "=", expr => ActionFn(7);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action7::<>(lc, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 6)
    }
    pub(crate) fn __reduce13<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // num = Num => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce14<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // num = "(", expr, ")" => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(lc, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 7)
    }
    pub(crate) fn __reduce15<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // print_stmt = PrintStart, "(", expr, ")", PrintEnd => ActionFn(29);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action29::<>(lc, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 8)
    }
    pub(crate) fn __reduce16<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // print_stmt = PrintStart, "(", expr, ")" => ActionFn(30);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action30::<>(lc, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 8)
    }
    pub(crate) fn __reduce17<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // program = stmts => ActionFn(1);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmts = print_stmt, ";" => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce19<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmts = print_stmt => ActionFn(26);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce20<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmts = let_stmt, ";" => ActionFn(27);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action27::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce21<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmts = let_stmt => ActionFn(28);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce22<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmts = stmts, print_stmt => ActionFn(4);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action4::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce23<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // stmts = stmts, let_stmt => ActionFn(5);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action5::<>(lc, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce24<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // str_lit = StrLit => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // value = num => ActionFn(14);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // value = id => ActionFn(15);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce27<
    >(
        lc: &LangConfig,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // value = str_lit => ActionFn(16);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(lc, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
}
pub use self::__parse__program::programParser;

#[allow(unused_variables)]
fn __action0<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, AST, usize),
) -> AST
{
    __0
}

#[allow(unused_variables)]
fn __action1<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Vec<Stmt>, usize),
) -> AST
{
    AST{top_block:Block{stmts:__0}}
}

#[allow(unused_variables)]
fn __action2<
>(
    lc: &LangConfig,
    (_, p, _): (usize, Stmt, usize),
    (_, _, _): (usize, core::option::Option<tokens::Token>, usize),
) -> Vec<Stmt>
{
    vec![p]
}

#[allow(unused_variables)]
fn __action3<
>(
    lc: &LangConfig,
    (_, l, _): (usize, Stmt, usize),
    (_, _, _): (usize, core::option::Option<tokens::Token>, usize),
) -> Vec<Stmt>
{
    vec![l]
}

#[allow(unused_variables)]
fn __action4<
>(
    lc: &LangConfig,
    (_, mut s, _): (usize, Vec<Stmt>, usize),
    (_, p, _): (usize, Stmt, usize),
) -> Vec<Stmt>
{
    {s.push(p);s}
}

#[allow(unused_variables)]
fn __action5<
>(
    lc: &LangConfig,
    (_, mut s, _): (usize, Vec<Stmt>, usize),
    (_, l, _): (usize, Stmt, usize),
) -> Vec<Stmt>
{
    {s.push(l);s}
}

#[allow(unused_variables)]
fn __action6<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, v, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, e, _): (usize, core::option::Option<tokens::Token>, usize),
) -> Stmt
{
    {
        if !lc.is_optional(PrintEnd) && e.is_none(){
            panic!("Error, print end required");
        }
        return Stmt::Print(PrintStmt{arg:v});
    }
}

#[allow(unused_variables)]
fn __action7<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, name, _): (usize, tokens::Token, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, v, _): (usize, Expr, usize),
) -> Stmt
{
    {
        if let ID(name) = name{
            return Stmt::Decl(Decl{name:name,value:v});
        }else{
            unreachable!();
        }
        
    }
}

#[allow(unused_variables)]
fn __action8<
>(
    lc: &LangConfig,
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, f, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Op(Box::new(e),Operator::Add,Box::new(f))
}

#[allow(unused_variables)]
fn __action9<
>(
    lc: &LangConfig,
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, f, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Op(Box::new(e),Operator::Sub,Box::new(f))
}

#[allow(unused_variables)]
fn __action10<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action11<
>(
    lc: &LangConfig,
    (_, f, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, n, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Op(Box::new(f),Operator::Mul,Box::new(n))
}

#[allow(unused_variables)]
fn __action12<
>(
    lc: &LangConfig,
    (_, f, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
    (_, n, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Op(Box::new(f),Operator::Div,Box::new(n))
}

#[allow(unused_variables)]
fn __action13<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action14<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action15<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action16<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action17<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> Expr
{
    {
        if let ID(t) = __0 {
            return Expr::Val(Value::Id(t));
        }else{
            return Expr::Val(Value::Id("".to_owned()));
        }
    }
}

#[allow(unused_variables)]
fn __action18<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> Expr
{
    {
        if let StringVal(t) = __0 {
            return Expr::Val(Value::Sval(t));
        }else{
            return Expr::Val(Value::Sval("".to_owned()));
        }
    }
}

#[allow(unused_variables)]
fn __action19<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> Expr
{
    {
        if let Number(t) = __0 {
            return Expr::Val(Value::Nval(t.parse().unwrap()));
        }else{
            return Expr::Val(Value::Nval(0.0));
        }
    }
}

#[allow(unused_variables)]
fn __action20<
>(
    lc: &LangConfig,
    (_, _, _): (usize, tokens::Token, usize),
    (_, __0, _): (usize, Expr, usize),
    (_, _, _): (usize, tokens::Token, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action21<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action22<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action23<
>(
    lc: &LangConfig,
    (_, __0, _): (usize, tokens::Token, usize),
) -> core::option::Option<tokens::Token>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action24<
>(
    lc: &LangConfig,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<tokens::Token>
{
    None
}

#[allow(unused_variables)]
fn __action25<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
    __1: (usize, tokens::Token, usize),
) -> Vec<Stmt>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action23(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
) -> Vec<Stmt>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action27<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
    __1: (usize, tokens::Token, usize),
) -> Vec<Stmt>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action23(
        lc,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action28<
>(
    lc: &LangConfig,
    __0: (usize, Stmt, usize),
) -> Vec<Stmt>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        lc,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action29<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Expr, usize),
    __3: (usize, tokens::Token, usize),
    __4: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action21(
        lc,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action30<
>(
    lc: &LangConfig,
    __0: (usize, tokens::Token, usize),
    __1: (usize, tokens::Token, usize),
    __2: (usize, Expr, usize),
    __3: (usize, tokens::Token, usize),
) -> Stmt
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action22(
        lc,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        lc,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,tokens::Token,usize), __lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>>;
}

impl<> __ToTriple<> for (usize, tokens::Token, usize) {
    fn to_triple(value: Self) -> Result<(usize,tokens::Token,usize), __lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, tokens::Token, usize), lexer::LexerError> {
    fn to_triple(value: Self) -> Result<(usize,tokens::Token,usize), __lalrpop_util::ParseError<usize, tokens::Token, lexer::LexerError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
