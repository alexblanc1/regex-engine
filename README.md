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
- [x] `derivative` function + `eta` (ν)
- [x] First unit tests (membership via derivatives)

The core derivative engine works; next up are smart constructors, then the
extended operators and the parser (see the roadmap).

## Project structure

Current and planned layout — files marked with a phase number don't exist yet:

```
regex-engine/
├── Cargo.toml
├── README.md
├── derivative.md        # notes on Brzozowski derivatives
└── src/
    ├── main.rs          # today: everything lives here (AST, nullable, derivative, tests)
    │                    # later (Phase 6): thin CLI on top of the library
    ├── lib.rs           # Phase 6 — library root, public API
    ├── ast.rs           # Phase 1 — Reg enum + smart constructors (simplification)
    ├── derivative.rs    # Phase 1 — nullable, eta, derivative, matches
    ├── parser.rs        # Phase 3 — lexer + parser: "a(b|c)*" → Reg
    ├── dfa.rs           # Phase 4 — DFA built from derivatives, state memoization
    ├── search.rs        # Phase 5 — find / find_all, anchors, capture groups
    └── bin/
        └── rgrep.rs     # Phase 6 — small grep-like CLI tool
```

## Build & run

```bash
cargo build      # compile
cargo run        # run the demo binary
cargo test       # run the tests
```

Requirements: a recent Rust toolchain (edition 2024).

## Roadmap

### Phase 1 — Core derivative engine
- [x] `derivative(r, c)` — Brzozowski derivative
- [ ] `matches(r, input)` — full-string matching (exists as a test helper for now)
- [ ] Smart constructors to normalize/simplify expressions and prevent their size
      from blowing up
- [x] First unit tests

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
- [ ] Build a Regex playground with the Ratatui API

## References

- Janusz A. Brzozowski, *Derivatives of Regular Expressions* (1964)
- Owens, Reppy, Turon, *Regular-expression derivatives re-examined* (2009)

## License

Personal project — to be defined.
