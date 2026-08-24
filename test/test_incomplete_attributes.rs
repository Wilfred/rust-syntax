// SYNTAX TEST "source.rust" "Incomplete attribute highlighting"

    #[attribute(value = 42
//    ^^^^^^^^^ variable.other.rust
//              ^^^^^ variable.other.rust
//                      ^^ constant.numeric.decimal.rust
    next_line
//  ^^^^^^^^^ variable.other.rust
