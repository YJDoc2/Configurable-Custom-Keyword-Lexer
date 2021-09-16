# Custrom Configurable Lexer-Parser

---

### Note

This is still very experimental, and for any syntax error it will just panic giving very unhelpful error message. The error recovery and more helpful error messages are possible, but have not been implemented yet. Also this is more of a proof-of-concept, and the language itself only supports variable declaration,assignment, while and for loop, if-else statements and baked-in print statement

If someone is actually interested in this, I can add more examples, write up a better documentation etc, let me know in issues.

---

Inspired by [a tweet](https://twitter.com/Felienne/status/1435864809963130883) , I made this lexer-parser pair along with an evaluator, which allows you to set custom tokens for the language using a config file, and according to that config, this runs the program. To show in short, what this makes possible:

```sh
# English
let v1 = 1234;
if (v1>12){
    print(v1+(123 - (25*6)));
} else {
    print('Hello');
}
v1 = 1;
if(v1>12){
    print(v1+(123 - (25*6)));
} else{
    print('Hello');
}
let i = 0;
while (i<5){
    print('आता i ची value आहे '+i);
    i = i+1;
}
for k in [i,v1]{
    print(k);
}

```

And

```sh
# Marathi
नवीन v1 = १२३४;
जर (v1>12) तर{
    हे(v1+(123 - (25*6))) दाखवा ;
} नाहीतर {
    हे('Hello') दाखवा;
}
v1 = १;
जर (v1>12) तर{
    हे(v1+(123 - (25*6))) दाखवा;
} नाहीतर {
    हे('Hello') दाखवा;
}
नवीन i = 0;
जोपर्यंत (i<5) तोपर्यंत{
    हे('आता i ची value आहे '+i)दाखवा;
    i = i+1;
}
for k in [i,v1]{
    हे(k)दाखवा;
}
```

both are a valid programs and can be run by <strong>the same binary</strong>, with their own config-english and config-marathi config files. Moreover, you can create custom config files to write code in your own language.

## How this works

This is made of three parts : A handwritten lexer, taken much after the amazing [Crafting Interpreters](https://craftinginterpreters.com/), A Parser , generated using [Lalrpop crate](https://crates.io/crates/lalrpop), and the a evaluator.

the commandline interface is :

```
USAGE:
    config_lex --config <CONFIG> --file <FILE>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -c, --config <CONFIG>    Token Configuration file
    -f, --file <FILE>        Source code file
```

This takes in a token config file, which specify the token mappings, which are then used by the lexer to determine the tokens. The lexer then emits the tokens, which are used by parser to generate AST. This AST is evaluated using a recursive tree walker, again taken after [Crafting Interpreters](https://craftinginterpreters.com/).

The point to note here is that the major changes here from other lexer-parsers are confined to lexer and parser only. This can still emit a convenient Intermediate Representation, such as AST, which can then be transplied to some other language.

### Language Specification

Currently following tokens are configurable, the brackets are default english fallback values, in case tokens are not specified in config file :

- PrintStart ( print )
- PrintEnd
- ForStart ( for )
- ForAux1
- ForAux2
- In ( in )
- ForAux3
- ForAux4
- IfStart ( if )
- IfAux1
- IfAux2
- ElseStart ( else )
- ElseAux1
- LetStart ( let )
- Or ( || )
- And ( && )
- WhileStart ( while )
- WhileAux1
- WhileAux2

Where \*Aux tokens are optional, and other are required. These auxillary tokens are provided so that the constructs, such as : if, while can be made more "organic", as some languages can use extra tokens to make the constructs more "coherent"/"natural" for that language.

Currently the structure of this language is as (can be seen in parser.lalrpop file):

```sh
-> This supports only string and numerical datatypes, and is dynamically typed
-> expr is any expression containing +-/*() and literal values or variables.
-> condition is value/variables compared using <,>,<=,=>,==,!=
    and such conditions joined by And,Or tokens.
-> While comparing string, it only == and != do a char-by-char comparison,
    others compare by string length
-> strings can be only added with other strings or numbers (like java)
    and other arithmetic operations are invalid for strings.
-> block is statements inside { and }

print => PrintStart ( Expr ) PrintEnd
let => LetStart ID = Expr
if => IfStart IfAux1 (condition) IfAux2 block ElseStart ElseAux1 block # else section is optional
while => WhileStart WhileAux1 (condition) WhileAux2 block
for => ForStart ForAux1 ID ForAux2 ForAux3 In array ForAux4 block
array => [ comma-separated-expr ]

```

---

## Licence

This code is released under GNU GPL V3, see [License](https://github.com/YJDoc2/Configurable-Custom-Token-Lexer/blob/main/License) file for more info.
