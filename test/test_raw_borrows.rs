// SYNTAX TEST "source.rust" "Raw borrow highlighting"

    let pointer = &raw const value;
//                ^ keyword.operator.borrow.and.rust
//                 ^^^ keyword.operator.borrow.raw.rust
//                     ^^^^^ storage.modifier.rust
    let pointer = & raw mut value;
//                  ^^^ keyword.operator.borrow.raw.rust
//                      ^^^ storage.modifier.rust
    let raw = value;
//      ^^^ variable.other.rust
