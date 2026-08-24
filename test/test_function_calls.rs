// SYNTAX TEST "source.rust" "Function call highlighting"

    add(1, 2);
//  ^^^ variable.function.rust
//     ^ punctuation.brackets.round.rust

    calculator.add(1, 2);
//  ^^^^^^^^^^ variable.other.rust
//            ^ keyword.operator.access.dot.rust
//             ^^^ variable.function.rust
//                ^ punctuation.brackets.round.rust

    parse::<u32>(value);
//  ^^^^^ variable.function.rust
//       ^^ keyword.operator.namespace.rust
//         ^ punctuation.brackets.angle.rust
//          ^^^ storage.type.numeric.rust
//             ^ punctuation.brackets.angle.rust
//              ^ punctuation.brackets.round.rust

    r#match();
//  ^^^^^^^ variable.function.rust
//         ^ punctuation.brackets.round.rust
