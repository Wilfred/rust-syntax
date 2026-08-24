// SYNTAX TEST "source.rust" "Byte escape highlighting"

    let byte = b'\xff';
//               ^^^^ constant.character.escape.rust
    let bytes = b"\x80\xff";
//                ^^^^^^^^ constant.character.escape.rust
