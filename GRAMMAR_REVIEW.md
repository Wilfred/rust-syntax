# Grammar review: correctness, coverage, resilience, theme-friendliness

A review of `syntaxes/rust.tmLanguage.yml` (and the generated JSON). All findings below
were verified empirically by tokenizing probe files with `vscode-textmate` +
`vscode-oniguruma` — the same engine VS Code uses — against the current grammar.

**Overall:** the grammar is well organized, the YAML source and generated JSON are in
sync, the scope tests pass, and scope naming is largely standard and theme-friendly.
The issues fall into four groups below, followed by a suggested priority order.

## 1. Correctness bugs (reproduced)

### 1.1 Uppercase identifier followed by `<` opens a runaway generic context

`if A<b { ... }` (or `CONST<x`, or any un-spaced comparison whose left side starts with
a capital letter) matches the `parameterized types` begin rule
(`\b(_?[A-Z][A-Za-z0-9_]*)(<)`), which stays open until the next `>` — potentially many
lines later. Because that rule's inner patterns don't include `#strings`, `#constants`,
or `#functions`, everything in between loses string, number, and function highlighting.
Since the rule has no `name`, the breakage is invisible in the scope inspector.

This is also the biggest resilience problem while typing: entering `let m: HashMap<`
degrades all code below the cursor until the closing `>` is typed.

Possible fixes: also terminate the rule on characters that cannot occur in a type
argument list (e.g. `[;{]`), and/or include `#strings` and `#constants` in its patterns.

### 1.2 Keywords directly followed by `(` are scoped as function calls

`if(x)`, `match(y)`, `while(x)`, `for(i)`, `return(v)` render the keyword as
`entity.name.function`, because `#functions` is included before `#keywords` at top level
and the call rule (`[A-Za-z0-9_]+\(`) has no keyword exclusion. Un-spaced keywords are
legal Rust. Fix: negative lookahead for keywords in the function-call begin pattern.

### 1.3 Short Unicode escapes are mis-highlighted as interpolations

The escape regex requires `u\{[\da-fA-F]{4,6}\}`, but Rust allows 1–6 hex digits (plus
underscores). `"\u{7f}"` tokenizes as backslash + bare `u` escape, and `{7f}` is then
scoped `meta.interpolation`. Fix: `[\da-fA-F_]{1,6}`.

### 1.4 Escaped braces in strings scoped as interpolation

In `"{{literal}}"`, the inner `{literal}` gets `meta.interpolation`, although `{{`/`}}`
escape to literal braces. A rule matching `\{\{|\}\}` before `#interpolations` fixes it.

### 1.5 Byte escapes above `\x7F` half-match

The escape pattern restricts `\x` to `[0-7][\da-fA-F]` — correct for `char`/`str`, wrong
for byte and byte-string literals: in `b'\xff'`, the `ff` is left unscoped.

### 1.6 Path segments ending in `self`/`super` lose their namespace scope

In `myself::thing` or `unsuper::thing`, the `(?<!super|self)` lookbehind (meant to
exempt the keywords) also rejects identifiers that merely *end* with those strings, so
`myself` falls through to `variable.other`. Use word-anchored lookbehinds.

### 1.7 Doc-comment classification inverted at both edges

- `//!` and `/*!` (inner docs — the first lines of most `lib.rs` files) are scoped as
  plain comments, not documentation.
- `////…` (explicitly *not* rustdoc) gets `comment.line.documentation.rust`.

### 1.8 Smaller confirmed nits

- **Trailing-dot floats**: in `2.`, the dot is scoped `keyword.operator.access.dot`
  instead of being part of the numeric literal.
- **`f16`/`f128` suffixes** (unstable, but stabilizing): unknown suffix breaks the whole
  literal — in `1.5f16`, the `5f16` ends up with no scope at all.
- **Const-generic arguments**: in `ArrayVec<u8, 4>`, the `4` is scoped
  `variable.other.rust` — the parameterized-types rule includes `#variables` (whose
  regex `[a-z0-9_]+` matches pure digits) but not `#constants`.
- **Raw identifiers in type positions**: `struct r#Weird;` shatters into `r` +
  unscoped `#` + generic type. (`fn r#async()` works fine.)
- **Dead rule**: the `pub (\()` "restricted visibility" rule is unreachable — the plain
  `\b(pub)\b` rule above it always wins at the same position.
- **Inert `patterns`**: the two macro-metavariable rules carry
  `patterns: [include #keywords]` on `match` rules; only `begin`/`end` rules support
  `patterns`, so these are ignored.
- **Misnamed capture**: exponent digits are scoped
  `constant.numeric.decimal.exponent.mantissa.rust`, but the mantissa is the part
  *before* `e`.

## 2. Language coverage gaps

- **C string literals** (stable since Rust 1.77): `c"…"` and `cr#"…"#` are
  unrecognized — the `c`/`cr` prefix is scoped as a variable and `#` delimiters are
  bare. The string rules only know `b`, `r`, `br`.
- **`f16`/`f128` suffixes** — see 1.8; worth future-proofing since the failure mode
  un-scopes the whole literal.
- **`&raw const` / `&raw mut`** (stable since 1.82): `raw` is scoped as a variable.
- **Raw lifetimes** (`'r#foo`, stable since 1.84) are not handled.
- **`macro_rules!` with `(` or `[` delimiters**: the rule requires `{`, so
  `macro_rules! name ( … );` loses the macro-name scope.
- **Shebang** (`#!/usr/bin/env cargo` — relevant for cargo-script): renders as bare `#`
  plus a logical-operator `!`.
- **Contextual keywords**: `union` is unconditionally `keyword.other`, so legal
  identifier uses (`let union = 5;`) mis-highlight. Same tradeoff class as highlighting
  reserved words (`do`, `abstract`, `become`, …), which is reasonable.

Coverage that is notably good: macro metavariables with fragment specifiers (incl.
`expr_2021`, `pat_param`), turbofish, nested generics (`Vec<Vec<u8>>` — the `>>` closes
correctly), labeled loops vs. chars vs. lifetimes, raw strings correctly suppressing
escapes, digit separators, and tuple-field access `tup.0.1`.

## 3. Resilience to incomplete code

- The **worst case is 1.1** (runaway generic context) — severe and avoidable.
- An **unclosed attribute** `#[derive(Debug` swallows following lines into
  `meta.attribute` until any `]`. Keywords/punctuation stay colored, but identifiers and
  numbers go bare (no `#variables`/`#constants` inside). Adding those includes would
  soften the degradation.
- An **unterminated `"`** runs the string to the next quote — correct, since Rust
  strings are legitimately multi-line.
- `fn`-definition and `use` rules span to `{`/`;` but include essentially everything, so
  they degrade gracefully. A half-typed `fn foo` (no parens) falls back to sensible
  keyword + variable scoping.

## 4. Theme friendliness

Genuinely good overall: everything receives a scope (catch-all `variable.other.rust`),
punctuation is exhaustively scoped, `meta.*` containers let themes style regions, and
dual scopes on declaration keywords (`keyword.declaration.struct.rust storage.type.rust`)
serve both modern and classic themes. Notes:

- `Some`/`None`/`Ok`/`Err` as `entity.name.type.option/result` and the ALL-CAPS constant
  heuristic are reasonable choices themes can target or ignore. (The `[A-Z]{2}`
  heuristic correctly rejects `IOError` thanks to the trailing `\b`.)
- The `b`/`r` string prefixes are scoped `string.quoted.byte.raw.rust` even for
  non-raw, non-byte combinations — a misleading scope name, though harmless since it
  nests under the string scope.
- Bitwise/shift operators (`^ | << >>`) share `keyword.operator.logical` with `&& !`;
  a `keyword.operator.bitwise` split would be more expressive.
- Tuple structs and enum variants with parens (`MyStruct(5)`, `Color::Red(255)`) render
  as function calls — already tracked in the file's own TODO comments.
- `#interpolations` applies to *every* double-quoted string, not just format-macro
  strings — an inherent TextMate tradeoff worth documenting.

## 5. Housekeeping

- The YAML source and generated JSON are structurally identical (verified). ✓
- `npm test` passes. ✓
- The scope-test suite covers only imports, float exponents, metavariables, and one
  comment case — the bugs above would all make good regression tests.
- CI uses `actions/checkout@v2` (deprecated); bump to `@v4`.

## Suggested priorities

1. Runaway generic context (1.1) — biggest blast radius, hits people mid-typing.
2. Keyword-as-function-call (1.2) and short `\u{…}` escapes (1.3) — wrong on stable,
   everyday code.
3. C-string literals and inner doc comments — the most visible coverage gaps in modern
   codebases.
4. The remaining nits and dead rules — cheap cleanups, each easily locked in with a
   `vscode-tmgrammar-test` case.
