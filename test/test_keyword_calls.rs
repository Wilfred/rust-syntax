// SYNTAX TEST "source.rust" "Keywords next to parentheses"

    if(x) {}
//  ^^ keyword.control.rust
//    ^ punctuation.brackets.round.rust
    match(x) {}
//  ^^^^^ keyword.control.rust
    while(x) {}
//  ^^^^^ keyword.control.rust
    for(i) {}
//  ^^^ keyword.control.rust
    return(value);
//  ^^^^^^ keyword.control.rust
    r#if();
//  ^^^^ entity.name.function.rust
