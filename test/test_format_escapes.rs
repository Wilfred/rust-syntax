// SYNTAX TEST "source.rust" "Escaped format string braces"

    let value = "{{literal}}";
//               ^^       ^^ constant.character.escape.format.rust
//                 ^^^^^^^ - meta.interpolation.rust
