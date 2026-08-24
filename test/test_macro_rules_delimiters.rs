// SYNTAX TEST "source.rust" "macro_rules delimiters"

    macro_rules! braces { () => {} }
//               ^^^^^^ entity.name.function.macro.rust
    macro_rules! parens ( () => {} );
//               ^^^^^^ entity.name.function.macro.rust
    macro_rules! brackets [ () => {} ];
//               ^^^^^^^^ entity.name.function.macro.rust
