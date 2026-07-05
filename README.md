# regex-engine

Un moteur d'expressions régulières écrit **from scratch en Rust**, fondé sur les
**dérivées de Brzozowski**.

L'idée : plutôt que de compiler la regex en automate (NFA/DFA) via la construction
de Thompson, on manipule directement l'expression régulière comme une structure de
données et on la « dérive » caractère par caractère. Une chaîne est reconnue si,
après avoir consommé tous ses caractères, l'expression obtenue accepte le mot vide.

## Le principe en deux fonctions

- **`nullable(r)`** — est-ce que l'expression `r` reconnaît le mot vide `ε` ?
- **`derivative(r, c)`** — l'expression qui reconnaît « tout ce que `r` reconnaît,
  mais après avoir consommé le caractère `c` ».

Reconnaître une chaîne `c₀c₁…cₙ` revient alors à calculer :

```
nullable( derivative( … derivative(derivative(r, c₀), c₁) … , cₙ) )
```

## L'AST actuel

```rust
pub enum Reg {
    Empty,                    // ∅  — ne reconnaît rien
    Eps,                      // ε  — reconnaît le mot vide
    Chr(char),                // 'a' — un caractère
    Alt(Box<Reg>, Box<Reg>),  // r | s  — alternative
    Seq(Box<Reg>, Box<Reg>),  // r s    — concaténation
    Star(Box<Reg>),           // r*     — étoile de Kleene
}
```

## État d'avancement

- [x] AST des expressions régulières (`Reg`)
- [x] Fonction `nullable`

C'est le tout début : les fondations sont posées, le moteur de reconnaissance
reste à écrire (voir la roadmap).

## Construire & lancer

```bash
cargo build      # compiler
cargo run        # exécuter le binaire de démo
cargo test       # lancer les tests (à venir)
```

Prérequis : une toolchain Rust récente (edition 2024).

## Roadmap

### Phase 1 — Cœur du moteur par dérivées
- [ ] `derivative(r, c)` — dérivée de Brzozowski
- [ ] `matches(r, input)` — reconnaissance d'une chaîne complète
- [ ] Constructeurs « intelligents » (smart constructors) pour normaliser/simplifier
      les expressions et éviter l'explosion de leur taille
- [ ] Premiers tests unitaires

### Phase 2 — Opérateurs étendus (sucre syntaxique)
- [ ] `r+` (une ou plusieurs), `r?` (zéro ou un)
- [ ] `.` (n'importe quel caractère)
- [ ] Classes de caractères `[a-z]`, `[^…]`
- [ ] Quantificateurs bornés `{n}`, `{n,m}`

### Phase 3 — Parsing
- [ ] Lexer + parser d'une chaîne (`"a(b|c)*"`) vers l'AST `Reg`
- [ ] Gestion des priorités et des parenthèses
- [ ] Échappements (`\.`, `\*`, `\(`, …)
- [ ] Messages d'erreur clairs sur regex invalide

### Phase 4 — Compilation en automate
- [ ] Construction d'un DFA directement à partir des dérivées (approche Brzozowski)
- [ ] Mémoïsation des états dérivés (cache) pour la performance
- [ ] (Alternative) NFA de Thompson + simulation, pour comparer les approches

### Phase 5 — Recherche & extraction
- [ ] Recherche de sous-chaîne, pas seulement match complet : `find`, `find_all`
- [ ] Ancres `^`, `$` et limites de mot `\b`
- [ ] Groupes de capture

### Phase 6 — Ergonomie & qualité
- [ ] Séparer bibliothèque (`lib.rs`) et binaire CLI (`main.rs`)
- [ ] Petit outil en ligne de commande façon `grep`
- [ ] Documentation (`cargo doc`) et exemples
- [ ] Benchmarks (`criterion`)
- [ ] Property testing / fuzzing en comparant les résultats au crate `regex`

### Phase 7 — Pour aller plus loin (optionnel)
- [ ] Support Unicode complet et classes Unicode
- [ ] Traitement UTF-8 au niveau octet
- [ ] Optimisations (détection de préfixes littéraux, etc.)

## Références

- Janusz A. Brzozowski, *Derivatives of Regular Expressions* (1964)
- Owens, Reppy, Turon, *Regular-expression derivatives re-examined* (2009)

## Licence

Projet personnel — à définir.
