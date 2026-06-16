# Comparison with Other Legal-Reasoning Frameworks

Legalis-RS sits in the field of **computational law** — encoding legal rules so a
machine can analyze, check, and apply them. It is not the only approach. This page
gives a fair, factual comparison with the most relevant alternatives and explains
the design trade-offs behind Legalis-RS's choices.

The goal here is orientation, not advocacy: each of these projects makes
reasonable choices for its own goals. Where they overlap with Legalis-RS, the
project usually *interoperates* with them (via `legalis-interop`) rather than
competing.

## The landscape at a glance

| Project / approach | Primary goal | Form | Typical strength |
|--------------------|--------------|------|------------------|
| **Legalis-RS** | Author, verify, simulate, and export legal rules across jurisdictions | Rust library + DSL + CLI/API | End-to-end pipeline; multi-jurisdiction; explicit deterministic-vs-discretion model |
| **Catala** | Faithfully translate tax/benefit law into reliable code | DSL (compiler/library) | Literate programming; default-logic base-case/exception fidelity |
| **OpenFisca** | Microsimulation of tax-and-benefit systems | Python framework | Population-level fiscal microsimulation; established government use |
| **General rule engines** (Drools, business-rules systems) | Execute business/decision rules | Engine + rule format (often DMN) | Mature tooling; throughput; integration with enterprise stacks |
| **Akoma Ntoso** | Represent legislative/judicial documents | XML standard (OASIS) | Document structure, citation, semantic markup |
| **LegalRuleML** | Represent legal rules and their context | XML rule-markup standard (OASIS) | Deontic/defeasible rule semantics; standardized interchange |

---

## Catala

[Catala](https://catala-lang.org/) is a domain-specific language from Inria for
turning law — especially tax and social-benefit law that is already computed
automatically — into reliable code. Its defining ideas are **literate
programming** (the code lives next to the legal text it implements) and a basis in
**prioritized default logic**, which lets authors mirror the base-case/exception
structure that pervades legislation.
([Catala: A Programming Language for the Law](https://arxiv.org/pdf/2103.03198).)

**How it compares.**

- *Shared ground:* both Catala and Legalis-RS believe law should be written in a
  purpose-built language rather than buried in general-purpose code, and both keep
  the encoding close to the source legal structure (Legalis-RS supports
  `EXCEPTION` clauses and statute relationships; Catala makes exceptions a
  first-class semantic via default logic).
- *Scope:* Catala focuses tightly on faithful *translation and execution* of a
  body of law. Legalis-RS spans a wider pipeline — authoring **plus** static
  verification, population simulation, visualization, diffing, audit, and export —
  and explicitly models multiple jurisdictions.
- *Semantics:* Catala's prioritized-default-logic core gives it a rigorous,
  well-studied account of exceptions. Legalis-RS uses an algebraic
  `Condition`/`Effect` model with recursive `And`/`Or`/`Not` and a separate
  verification layer; its distinctive semantic move is the explicit
  `LegalResult` separation of deterministic outcomes from judicial discretion.
- *Ecosystem fit:* Legalis-RS's interop layer can import from and export to
  Catala, so the two are complementary rather than mutually exclusive.

---

## OpenFisca

[OpenFisca](https://openfisca.org/doc/) is an open-source **microsimulation**
framework, written in Python, for modeling tax-and-benefit systems. It lets public
administrations reuse a common engine instead of rebuilding tax/benefit
calculators from scratch, and it excels at running rules over representative
populations to estimate fiscal and distributional effects.

**How it compares.**

- *Shared ground:* simulation over a population is a core capability of both.
  Legalis-RS's `legalis-sim` runs statutes against a generated population and
  reports outcome breakdowns, which is conceptually close to OpenFisca's
  microsimulation.
- *Language and safety:* OpenFisca is Python; Legalis-RS is Rust. The Rust choice
  buys compile-time exhaustiveness on the rule model, a no-panic library policy,
  and a pure-Rust, dependency-light build — at the cost of Python's
  approachability and its rich data-science ecosystem.
- *Emphasis:* OpenFisca is squarely a fiscal microsimulation tool. Legalis-RS's
  simulation is one stage of a broader pipeline and is paired with *static
  verification* (finding logical defects before simulating) and the
  deterministic-vs-discretion accounting, which OpenFisca does not aim to provide.
- *Breadth:* Legalis-RS targets many legal domains and traditions
  (civil/common/socialist/Islamic/supranational), not only tax-and-benefit.

---

## General-purpose rule engines

Business rule engines (for example Drools and the broader family of
production-rule / decision systems, often driven by the **DMN** standard) execute
declarative rules at scale and are deeply integrated into enterprise software.

**How it compares.**

- *Shared ground:* at heart, applying conditions to facts to produce effects is
  what both do, and Legalis-RS can export to DMN/BPMN via `legalis-interop`.
- *Domain modeling:* general rule engines are domain-agnostic — they have no
  built-in notion of a statute, a jurisdiction, temporal validity, an amendment,
  a citation, or judicial discretion. Legalis-RS bakes these legal concepts into
  the core types, so the model speaks the language of law rather than the language
  of generic "if/then."
- *Verification:* rule engines focus on *execution*. Legalis-RS adds a dedicated
  verification layer (conflict/contradiction detection, dead-rule and
  circular-reference checks, optional SMT satisfiability) aimed at catching
  *legal* defects before deployment.
- *The discretion boundary:* a generic engine will happily compute an answer for
  every input. Legalis-RS deliberately refuses to: `LegalResult` forces
  acknowledgment of cases that require human judgment.

---

## Standards: Akoma Ntoso and LegalRuleML

[Akoma Ntoso](https://en.wikipedia.org/wiki/Akoma_Ntoso) and **LegalRuleML** are
OASIS standards rather than execution engines. Akoma Ntoso is an XML vocabulary
for the *structure* of legislative and judicial documents (parts, sections,
citations, semantic markup). LegalRuleML is an XML markup for *rules* and their
legal context, with support for deontic and defeasible semantics, designed for
standardized interchange.

**How it compares.**

- *Different layer:* these are *representation/interchange* standards; Legalis-RS
  is a *processing* system with its own in-memory model. They are not
  alternatives so much as formats Legalis-RS reads and writes.
- *Interoperability, not competition:* `legalis-interop` supports Akoma Ntoso and
  LegalRuleML (among others). The intended pattern is to ingest a document or rule
  set expressed in a standard, work with it in the Legalis-RS model (verify,
  simulate, visualize), and export back out.
- *Executable vs. descriptive:* Akoma Ntoso describes *documents* and LegalRuleML
  describes *rules*; neither runs a simulation or proves a rule set
  contradiction-free on its own. That executable, analytical layer is what
  Legalis-RS contributes.

---

## Where Legalis-RS is distinctive

Pulling the threads together, the design trade-offs that set Legalis-RS apart:

1. **An end-to-end pipeline in one stack.** Authoring (DSL), the core model,
   static verification, simulation, visualization, diffing, audit, porting, and
   export to many targets all live in one coherent Rust workspace, sharing one
   model. Most alternatives specialize in one of these stages.

2. **The deterministic-vs-discretion split is a first-class type.**
   `LegalResult<T>` makes "this requires a human" a value the system carries and
   measures, rather than something an engine quietly papers over. This is the
   project's central philosophical commitment
   ([ADR-0004](adr/0004-legalresult-deterministic-vs-discretion.md)).

3. **Verification before execution.** A dedicated verifier looks for logical
   defects — conflicts, contradictions, dead rules, optionally unsatisfiable
   conditions via a pure-Rust SMT solver — treating legislative bugs like compile
   errors ([ADR-0006](adr/0006-static-verification-and-conflict-detection.md)).

4. **Multi-jurisdiction by construction.** A generic engine plus one crate per
   country (23 operational jurisdictions across several legal traditions), rather
   than a tool scoped to a single body of law
   ([ADR-0007](adr/0007-jurisdiction-crate-per-country.md)).

5. **Pure-Rust, offline-first, no-panic.** The default build needs no C/C++
   toolchain and no external services; library code does not crash on bad input;
   heavy capabilities are feature-gated. This targets the air-gapped, audited
   environments where legal systems often run
   ([ADR-0001](adr/0001-pure-rust-no-mandatory-c-dependencies.md),
   [ADR-0008](adr/0008-no-panic-error-handling-policy.md),
   [ADR-0009](adr/0009-offline-first-no-mandatory-services.md)).

## Where the alternatives may fit better

In the interest of a fair comparison:

- If your *only* need is to faithfully encode and execute a specific body of
  tax/benefit law with rigorous exception semantics, **Catala** offers a focused,
  research-backed answer with strong fidelity guarantees.
- If you need established, government-grade **fiscal microsimulation** in a
  Python data-science workflow, **OpenFisca** is purpose-built for exactly that.
- If you are embedding decision logic into an existing enterprise Java/.NET stack
  and need mature operational tooling, a **general rule engine** (often via DMN)
  may integrate more readily.
- If your task is primarily about *documents and interchange* — marking up
  legislation or exchanging rules between systems — the **Akoma Ntoso /
  LegalRuleML** standards are the right substrate, and Legalis-RS can sit on top
  of them via its interop layer.

---

### Sources

- Catala — Law to Code: <https://catala-lang.org/>
- *Catala: A Programming Language for the Law* (Merigoux et al., Inria):
  <https://arxiv.org/pdf/2103.03198>
- OpenFisca documentation: <https://openfisca.org/doc/>
