// SYNTAX TEST "source.rust" "String prefix scopes"

    let bytes = b"bytes";
//              ^ string.quoted.byte.rust
    let raw = r#"raw"#;
//            ^ string.quoted.raw.rust
    let raw_bytes = br#"bytes"#;
//                  ^ string.quoted.byte.rust
//                   ^ string.quoted.raw.rust
    let byte = b'x';
//             ^ string.quoted.byte.rust
