// SYNTAX TEST "source.rust" "trailing-dot float literals"

// Trailing-dot float literals should include the dot in the numeric literal scope
let f = 2.;
//      ^^ constant.numeric.decimal.rust
//       ^ punctuation.separator.dot.decimal.rust

// Standard decimal floats should keep dot in literal scope (regression)
let g = 2.5;
//      ^^^ constant.numeric.decimal.rust
//       ^ punctuation.separator.dot.decimal.rust

// Range operators should not include dots in numeric literals
let r = 1..5;
//      ^ constant.numeric.decimal.rust
//       ^^ keyword.operator.range.rust
//         ^ constant.numeric.decimal.rust

// Trailing dots in tuples (each on separate line for clarity)
let a = 2.;
//      ^^ constant.numeric.decimal.rust
let b = 3.;
//      ^^ constant.numeric.decimal.rust

// Method calls on integer literals: the dot was already scoped as part of the
// numeric literal before this change (a separate pre-existing quirk); this pins
// that the behavior is unchanged by the trailing-dot fix.
let m = 4.max(5);
//      ^ constant.numeric.decimal.rust
//       ^ constant.numeric.decimal.rust punctuation.separator.dot.decimal.rust

// Trailing-dot floats inside a tuple
let t = (2., 3.);
//       ^^ constant.numeric.decimal.rust
//           ^^ constant.numeric.decimal.rust
