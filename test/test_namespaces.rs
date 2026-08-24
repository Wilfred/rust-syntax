// SYNTAX TEST "source.rust" "Namespace names ending in keywords"

    myself::thing();
//  ^^^^^^ entity.name.namespace.rust
    unsuper::thing();
//  ^^^^^^^ entity.name.namespace.rust
    self::thing();
//  ^^^^ variable.language.self.rust
//      ^^ keyword.operator.namespace.rust
