# regex-engine

A regular-expression engine written **from scratch in Rust**, based on
**Brzozowski derivatives**.

The idea: instead of compiling the regex into an automaton (NFA/DFA) via Thompson's
construction, we manipulate the regular expression directly as a data structure and
"differentiate" it one character at a time. A string is accepted if, after consuming
all of its characters, the resulting expression accepts the empty word.

## The principle in two functions

- **`nullable(r)`** — does the expression `r` accept the empty word `ε`?
- **`derivative(r, c)`** — the expression that accepts "everything `r` accepts, but
  after consuming the character `c`".

Matching a string `c₀c₁…cₙ` then boils down to computing:

```
nullable( derivative( … derivative(derivative(r, c₀), c₁) … , cₙ) )
```

## Current AST

```rust
pub enum Reg {
    Empty,                    // ∅  — matches nothing
    Eps,                      // ε  — matches the empty word
    Chr(char),                // 'a' — a single character
    Alt(Box<Reg>, Box<Reg>),  // r | s  — alternation
    Seq(Box<Reg>, Box<Reg>),  // r s    — concatenation
    Star(Box<Reg>),           // r*     — Kleene star
}
```

## Status

- [x] Regular-expression AST (`Reg`)
- [x] `nullable` function

This is the very beginning: the foundations are in place, but the matching engine
itself is still to be written (see the roadmap).

## Build & run

```bash
cargo build      # compile
cargo run        # run the demo binary
cargo test       # run the tests (coming soon)
```

Requirements: a recent Rust toolchain (edition 2024).

## Roadmap

### Phase 1 — Core derivative engine
- [ ] `derivative(r, c)` — Brzozowski derivative
- [ ] `matches(r, input)` — full-string matching
- [ ] Smart constructors to normalize/simplify expressions and prevent their size
      from blowing up
- [ ] First unit tests

### Phase 2 — Extended operators (syntactic sugar)
- [ ] `r+` (one or more), `r?` (zero or one)
- [ ] `.` (any character)
- [ ] Character classes `[a-z]`, `[^…]`
- [ ] Bounded quantifiers `{n}`, `{n,m}`

### Phase 3 — Parsing
- [ ] Lexer + parser from a string (`"a(b|c)*"`) to the `Reg` AST
- [ ] Operator precedence and parentheses handling
- [ ] Escapes (`\.`, `\*`, `\(`, …)
- [ ] Clear error messages on invalid regex

### Phase 4 — Automaton compilation
- [ ] Build a DFA directly from derivatives (Brzozowski approach)
- [ ] Memoize derived states (cache) for performance
- [ ] (Alternative) Thompson NFA + simulation, to compare approaches

### Phase 5 — Search & extraction
- [ ] Substring search, not just full match: `find`, `find_all`
- [ ] Anchors `^`, `$` and word boundaries `\b`
- [ ] Capture groups

### Phase 6 — Ergonomics & quality
- [ ] Split library (`lib.rs`) and CLI binary (`main.rs`)
- [ ] Small `grep`-like command-line tool
- [ ] Documentation (`cargo doc`) and examples
- [ ] Benchmarks (`criterion`)
- [ ] Property testing / fuzzing, cross-checking results against the `regex` crate

### Phase 7 — Going further (optional)
- [ ] Full Unicode support and Unicode classes
- [ ] Byte-level UTF-8 handling
- [ ] Optimizations (literal prefix detection, etc.)

## References

- Janusz A. Brzozowski, *Derivatives of Regular Expressions* (1964)
- Owens, Reppy, Turon, *Regular-expression derivatives re-examined* (2009)

## License

Personal project — to be defined.
