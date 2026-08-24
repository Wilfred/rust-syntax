// SYNTAX TEST "source.rust" "Static variable highlighting"

static GLOBAL_COUNT: u32 = 0;
// <------ storage.modifier.rust
//     ^^^^^^^^^^^^ variable.other.static.rust

static mut counter: u32 = 0;
// <------ storage.modifier.rust
//     ^^^ storage.modifier.mut.rust
//         ^^^^^^^ variable.other.static.rust

static r#type: bool = false;
//     ^^^^^^ variable.other.static.rust
