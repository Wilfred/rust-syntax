// SYNTAX TEST "source.rust" "Raw identifiers in type positions"

    struct r#Weird;
//         ^^^^^^^ entity.name.type.struct.rust
    let value: r#Weird<u8>;
//             ^^^^^^^ entity.name.type.rust
