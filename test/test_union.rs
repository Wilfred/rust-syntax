// SYNTAX TEST "source.rust" "Contextual union keyword"

    union Value { integer: i32 }
//  ^^^^^ keyword.declaration.union.rust
//        ^^^^^ entity.name.type.union.rust
    let union = Value { integer: 1 };
//      ^^^^^ variable.other.rust
    union();
//  ^^^^^ entity.name.function.rust
