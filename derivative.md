```{=mediawiki}
{{Short description|Function defined on formal languages in computer science}}
```
![Brzozowski derivative (on red background) of a dictionary string set with respect to the string \"con\"](Brzozowski_derivative.gif "Brzozowski derivative (on red background) of a dictionary string set with respect to the string "con""){width="100"}

In [theoretical computer science](theoretical_computer_science "theoretical computer science"){.wikilink}, particularly in [formal language theory](formal_language_theory "formal language theory"){.wikilink}, the **Brzozowski derivative** $u^{-1}S$ of a [set](set_(mathematics) "set"){.wikilink} $S$ of [strings](word_(formal_languages) "strings"){.wikilink} and a string $u$ is the set of all strings obtainable from a string in $S$ by cutting off the [prefix](prefix_(computer_science) "prefix"){.wikilink} $u$. Formally:

$$u^{-1}S = \{v \in \Sigma^* \mid uv \in S\}$$.

For example,

$$\text{c}^{-1}\{\text{cat}, \text{cow}, \text{dog}\} = \{\text{at}, \text{ow}\}.$$

The Brzozowski derivative was introduced under various different names since the late 1950s.[^1][^2][^3] Today it is named after the computer scientist [Janusz Brzozowski](Janusz_Brzozowski_(computer_scientist) "Janusz Brzozowski"){.wikilink} who investigated its properties and gave an [algorithm](algorithm "algorithm"){.wikilink} to compute the derivative of a generalized [regular expression](regular_expression "regular expression"){.wikilink}.[^4]

## Definition

Even though originally studied for regular expressions, the definition applies to arbitrary formal languages. Given any [formal language](formal_language "formal language"){.wikilink} $S$ over an alphabet $\Sigma$ and any string $u \in \Sigma^*$, the derivative of $S$ with respect to $u$ is defined as:[^5]

$$u^{-1}S = \{v \in \Sigma^* \mid uv \in S\}$$

The Brzozowski derivative is a special case of [left quotient](quotient_of_a_formal_language "left quotient"){.wikilink} by a singleton set containing only $u$: $\ u^{-1}S = \{u\} \;\backslash\; S$.

Equivalently, for all $u,v \in \Sigma^*$:

$$v \in u^{-1}S \;\Leftrightarrow\; uv \in S.$$

From the definition, for all $u, v \in \Sigma^*$:

$$(uv)^{-1}S = v^{-1}(u^{-1}S)$$

since for all $w \in \Sigma^*$, we have `{{nowrap|<math>w \in (uv)^{-1}S \Leftrightarrow uvw \in S \Leftrightarrow vw \in u^{-1}S \Leftrightarrow w \in v^{-1}(u^{-1}S)</math>.}}`{=mediawiki}

The derivative with respect to an arbitrary string reduces to successive derivatives over the symbols of that string, since for all $a \in \Sigma, u \in \Sigma^*$: $\begin{align}
(ua)^{-1}S
&= a^{-1}(u^{-1}S) \\
\varepsilon^{-1}S
&= S
\end{align}$

A language $S \subseteq \Sigma^*$ is called *nullable* if it contains the empty string $\varepsilon$. Each language $S$ is uniquely determined by nullability of its derivatives:

$$w \in S \ \Leftrightarrow\ \varepsilon \in w^{-1}S$$

A language can be viewed as a (potentially infinite) boolean-labelled [tree](tree_(automata_theory) "tree"){.wikilink} (see also [tree (set theory)](tree_(set_theory) "tree (set theory)"){.wikilink} and [infinite-tree automaton](infinite-tree_automaton "infinite-tree automaton"){.wikilink}). Each possible string $w \in \Sigma^*$ denotes a node in the tree, with label *true* when $w \in S$ and *false* otherwise. In this interpretation, the derivative with respect to a symbol $a$ corresponds to the subtree obtained by following the edge $a$ from the root. Decomposing a tree into the root and the subtrees $a^{-1}S$ corresponds to the following equality, which holds for every language $S \subseteq \Sigma^*$:

$$S = (\{\varepsilon\} \cap S) \cup \bigcup_{a \in \Sigma} a(a^{-1}S).$$

## Derivatives of generalized regular expressions {#derivatives_of_generalized_regular_expressions}

When a language is given by a regular expression, the concept of derivatives leads to an algorithm for deciding whether a given word belongs to the regular expression.

Given a finite [alphabet](alphabet "alphabet"){.wikilink} *A* of symbols,[^6] a **generalized regular expression** *R* denotes a possibly infinite set of finite-length strings over the alphabet *A*, called the **language** of *R*, denoted *L*(*R*).

A generalized regular expression can be one of the following (where *a* is a symbol of the alphabet *A*, and *R* and *S* are generalized regular expressions):

- \"∅\" denotes the [empty set](empty_set "empty set"){.wikilink}: *L*(∅) = {},
- \"ε\" denotes the singleton set containing the [empty string](empty_string "empty string"){.wikilink}: *L*(ε) = {ε},
- \"*a*\" denotes the singleton set containing the single-symbol string *a*: *L*(*a*) = {*a*},
- \"*R*∨*S*\" denotes the union of *R* and *S*: *L*(*R*∨*S*) = *L*(*R*) ∪ *L*(*S*),
- \"*R*∧*S*\" denotes the intersection of *R* and *S*: *L*(*R*∧*S*) = *L*(*R*) ∩ *L*(*S*),
- \"¬*R*\" denotes the complement of *R* (with respect to *A*\*, the set of all strings over *A*): *L*(¬*R*) = *A*\* \\ *L*(*R*),
- \"*RS*\" denotes the concatenation of *R* and *S*: *L*(*RS*) = *L*(*R*) · *L*(*S*),
- \"*R*\*\" denotes the [Kleene closure](Kleene_closure "Kleene closure"){.wikilink} of *R*: *L*(*R*\*) = *L*(*R*)\*.

In an ordinary regular expression, neither ∧ nor ¬ is allowed.

### Computation

For any given generalized regular expression *R* and any string *u*, the derivative *u*^−1^*R* is again a generalized regular expression (denoting the language *u*^−1^*L*(*R*)).[^7] It may be computed recursively as follows.[^8]

  --------------- ----------------------- -----------------------------------------
  (*ua*)^−1^*R*   = *a*^−1^(*u*^−1^*R*)         for a symbol *a* and a string *u*
  *ε*^−1^*R*      = *R*                   
  --------------- ----------------------- -----------------------------------------

Using the previous two rules, the derivative with respect to an arbitrary string is explained by the derivative with respect to a single-symbol string *a*. The latter can be computed as follows:[^9]

  ------------------ --------------------------------------
  *a*^−1^*a*         = *ε*
  *a*^−1^*b*         = ∅
  *a*^−1^*ε*         = ∅
  *a*^−1^∅           = ∅
  *a*^−1^(*R*\*)     = (*a*^−1^*R*)*R*\*
  *a*^−1^(*RS*)      = (*a*^−1^*R*)*S* ∨ ν(*R*)*a*^−1^*S*
  *a*^−1^(*R*∧*S*)   = (*a*^−1^*R*) ∧ (*a*^−1^*S*)
  *a*^−1^(*R*∨*S*)   = (*a*^−1^*R*) ∨ (*a*^−1^*S*)
  *a*^−1^(¬*R*)      = ¬(*a*^−1^*R*)
  ------------------ --------------------------------------

Here, `{{math|ν(''R'')}}`{=mediawiki} is an [auxiliary function](auxiliary_function "auxiliary function"){.wikilink} yielding a generalized regular expression that evaluates to the empty string *ε* if *R*{{\'s}} language contains *ε*, and otherwise evaluates to ∅. This function can be computed by the following rules:[^10]

  -------------- ------------------- --------------------
  ν(*a*)         = ∅                 for any symbol *a*
  ν(*ε*)         = *ε*               
  ν(∅)           = ∅                 
  ν(*R*\*)       = *ε*               
  ν(*RS*)        = ν(*R*) ∧ ν(*S*)   
  ν(*R* ∧ *S*)   = ν(*R*) ∧ ν(*S*)   
  ν(*R* ∨ *S*)   = ν(*R*) ∨ ν(*S*)   
  ν(¬*R*)        = *ε*               if ν(*R*) = ∅
  ν(¬*R*)        = ∅                 if ν(*R*) = *ε*
  -------------- ------------------- --------------------

### Properties

A string *u* is a member of the string set denoted by a generalized regular expression *R* if and only if ε is a member of the string set denoted by the derivative *u*^−1^*R*.[^11]

Considering all the derivatives of a fixed generalized regular expression *R* results in only finitely many different languages. If their number is denoted by *d*~*R*~, all these languages can be obtained as derivatives of *R* with respect to strings of length less than *d*~*R*~.[^12] Furthermore, there is a complete [deterministic finite automaton](deterministic_finite_automaton "deterministic finite automaton"){.wikilink} with *d*~*R*~ states that recognises the [regular language](regular_language "regular language"){.wikilink} given by *R*, as stated by the [Myhill--Nerode theorem](Myhill–Nerode_theorem "Myhill–Nerode theorem"){.wikilink}.

## Derivatives of context-free languages {#derivatives_of_context_free_languages}

Derivatives are also effectively computable for recursively defined equations with regular expression operators, which are equivalent to [context-free grammars](context-free_grammar "context-free grammar"){.wikilink}. This insight was used to derive [parsing](parsing "parsing"){.wikilink} algorithms for [context-free languages](context-free_language "context-free language"){.wikilink}.[^13] Implementation of such algorithms have shown to have cubic [time complexity](time_complexity "time complexity"){.wikilink},[^14] corresponding to the complexity of the [Earley parser](Earley_parser "Earley parser"){.wikilink} on general context-free grammars.

## See also {#see_also}

- [Quotient of a formal language](Quotient_of_a_formal_language "Quotient of a formal language"){.wikilink}

## References

```{=mediawiki}
{{reflist}}
```
[Category:Formal languages](Category:Formal_languages "Category:Formal languages"){.wikilink}

[^1]:
    ```{=mediawiki}
    {{cite journal | url=https://dl.acm.org/doi/10.1145/320924.320930 | author=George N. Raney | title=Sequential functions | journal=Journal of the ACM | volume=5 | number=2 | pages=177&ndash;180 | date=Apr 1958 | doi=10.1145/320924.320930 | s2cid=1611992 }}
    ```

[^2]:
    ```{=mediawiki}
    {{cite journal | url=http://www.cse.chalmers.se/~coquand/AUTOMATA/rs.pdf | author=[[Dana Scott]] and [[Michael O. Rabin]] | title=Finite Automata and Their Decision Problems | journal=IBM Journal of Research and Development | volume=3 | number=2 | pages=114&ndash;125 | date=Apr 1959 | doi=10.1147/rd.32.0114 }}
    ```

[^3]:
    ```{=mediawiki}
    {{cite book |author=C. C. Elgot |author2=J. D. Rutledge | title=2nd Annual Symposium on Switching Circuit Theory and Logical Design (SWCT 1961) | chapter=Operations on finite automata | doi=10.1109/FOCS.1961.26 | editor=Robert S. Ledley  | volume= | pages=129&ndash;132 | date=Oct 1961 }}
    ```

[^4]:
    ```{=mediawiki}
    {{cite journal| author=Janusz A. Brzozowski| title=Derivatives of Regular Expressions| journal=J ACM| year=1964| volume=11| issue=4| pages=481–494| doi=10.1145/321239.321249| s2cid=14126942| doi-access=free}}
    ```

[^5]:

[^6]: Brzozowski (1964), p.481, required *A* to consist of the 2^*n*^ combinations of *n* [bits](bit "bit"){.wikilink}, for some *n*.

[^7]: Brzozowski (1964), p.483, Theorem 4.1

[^8]: Brzozowski (1964), p.483, Theorem 3.2

[^9]: Brzozowski (1964), p.483, Theorem 3.1

[^10]: Brzozowski (1964), p.482, Definition 3.2

[^11]: Brzozowski (1964), p.483, Theorem 4.2

[^12]: Brzozowski (1964), p.484, Theorem 4.3

[^13]:
    ```{=mediawiki}
    {{cite conference
    | author1=Matthew Might
    | author2=David Darais
    | author3=Daniel Spiewak
    | title=Parsing with derivatives: a functional pearl
    | conference=Proceeding of the 16th ACM SIGPLAN international conference on Functional Programming (ICFP)
    | year=2011
    | pages=189–195
    | doi=10.1145/2034773.2034801}}
    ```

[^14]:
    ```{=mediawiki}
    {{cite conference
    | author1=Michael D. Adams
    | author2=Celeste Hollenbeck
    | author3=Matthew Might
    | title=Proceedings of the 37th ACM SIGPLAN Conference on Programming Language Design and Implementation
    | chapter=On the complexity and performance of parsing with derivatives
    | conference=Proceedings of the 37th ACM SIGPLAN Conference on Programming Language Design and Implementation (PLDI)
    | year=2016
    | pages=224–236
    | doi=10.1145/2908080.2908128| isbn=9781450342612
    | doi-access=free
    | arxiv=1604.04695
    }}
    ```
