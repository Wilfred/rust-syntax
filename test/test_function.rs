// SYNTAX TEST "source.rust" "Simple function highlighting"

    fn add(left: u32, right: u32) -> u32 {
//  ^^ keyword.other.fn.rust
//     ^^^ entity.name.function.rust
//        ^ punctuation.brackets.round.rust
//         ^^^^ variable.parameter.rust
//             ^ keyword.operator.key-value.rust
//               ^^^ storage.type.numeric.rust
//                  ^ punctuation.comma.rust
//                    ^^^^^ variable.parameter.rust
//                         ^ keyword.operator.key-value.rust
//                           ^^^ storage.type.numeric.rust
//                                ^^ keyword.operator.arrow.skinny.rust
//                                   ^^^ storage.type.numeric.rust
//                                       ^ punctuation.brackets.curly.rust
        left + right
//      ^^^^ variable.other.rust
//           ^ keyword.operator.math.rust
//             ^^^^^ variable.other.rust
    }
//  ^ punctuation.brackets.curly.rust

    fn transform<T>(mut value: T, r#type: bool) -> T {
//                      ^^^^^ variable.parameter.rust
//                                ^^^^^^ variable.parameter.rust
        value
//      ^^^^^ variable.other.rust
    }
