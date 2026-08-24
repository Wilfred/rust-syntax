// SYNTAX TEST "source.rust" "Tuple constructor highlighting"

    let value = MyStruct(5);
//              ^^^^^^^^ entity.name.type.rust
//              ^^^^^^^^ - entity.name.function.rust
    let color = Color::Red(255);
//                     ^^^ entity.name.type.rust
//                     ^^^ - entity.name.function.rust
