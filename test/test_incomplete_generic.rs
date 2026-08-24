// SYNTAX TEST "source.rust" "Incomplete generic highlighting"

    if A<b {
        call("ok", 42);
//      ^^^^ entity.name.function.rust
//           ^^^^ string.quoted.double.rust
//                 ^^ constant.numeric.decimal.rust
    }
