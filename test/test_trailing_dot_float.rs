// SYNTAX TEST "source.rust" "Trailing-dot float highlighting"

    let float = 2.;
//              ^^ constant.numeric.decimal.rust
//               ^ punctuation.separator.dot.decimal.rust
    let field = 2.foo;
//              ^ constant.numeric.decimal.rust
//               ^ keyword.operator.access.dot.rust
