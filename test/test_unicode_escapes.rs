// SYNTAX TEST "source.rust" "Unicode escape highlighting"

    let short = "\u{7f}";
//                ^^^^^ constant.character.escape.unicode.rust
    let underscored = "\u{1_0_0_0}";
//                      ^^^^^^^^^^ constant.character.escape.unicode.rust
