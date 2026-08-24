// SYNTAX TEST "source.rust" "Rust documentation comments"

    /// outer docs
//  ^^^^^^^^^^^^^^ comment.line.documentation.rust
    //! inner docs
//  ^^^^^^^^^^^^^^ comment.line.documentation.rust
    //// not docs
//  ^^^^^^^^^^^^^ comment.line.double-slash.rust
    /** outer docs */
//  ^^^^^^^^^^^^^^^^^ comment.block.documentation.rust
    /*! inner docs */
//  ^^^^^^^^^^^^^^^^^ comment.block.documentation.rust
    /*** not docs */
//  ^^^^^^^^^^^^^^^^^ comment.block.rust
