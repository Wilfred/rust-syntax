// SYNTAX TEST "source.rust" "C string literals"

// C strings: the c prefix must be scoped as part of the string
let cs = c"hello";
//       ^ string.quoted.double.rust string.quoted.byte.raw.rust
//        ^ string.quoted.double.rust punctuation.definition.string.rust
//         ^^^^^ string.quoted.double.rust
//              ^ string.quoted.double.rust punctuation.definition.string.rust

// Raw C strings: cr prefix and # delimiters scoped as part of the string
let raw = cr#"raw c"#;
//        ^^ string.quoted.double.rust string.quoted.byte.raw.rust
//          ^ string.quoted.double.rust punctuation.definition.string.raw.rust
//           ^ string.quoted.double.rust punctuation.definition.string.rust
//            ^^^^^ string.quoted.double.rust
//                 ^ string.quoted.double.rust punctuation.definition.string.rust
//                  ^ string.quoted.double.rust punctuation.definition.string.raw.rust

// Regression: byte strings
let b = b"bytes";
//      ^ string.quoted.double.rust string.quoted.byte.raw.rust
//       ^ string.quoted.double.rust punctuation.definition.string.rust
//        ^^^^^ string.quoted.double.rust
//             ^ string.quoted.double.rust punctuation.definition.string.rust

// Regression: raw strings
let r = r"no \n escapes";
//      ^ string.quoted.double.rust string.quoted.byte.raw.rust
//       ^ string.quoted.double.rust punctuation.definition.string.rust
//        ^^^^^^^^^^^^^ string.quoted.double.rust
//                     ^ string.quoted.double.rust punctuation.definition.string.rust

// Regression: raw byte strings
let br = br#"raw bytes"#;
//       ^^ string.quoted.double.rust string.quoted.byte.raw.rust
//         ^ string.quoted.double.rust punctuation.definition.string.raw.rust
//          ^ string.quoted.double.rust punctuation.definition.string.rust
//           ^^^^^^^^^ string.quoted.double.rust
//                    ^ string.quoted.double.rust punctuation.definition.string.rust
//                     ^ string.quoted.double.rust punctuation.definition.string.raw.rust
