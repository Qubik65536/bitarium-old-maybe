# Verification plan

![Proof scope: Experiment first](assets/badges/proof.svg) ![Builds / proofs: No evidence](assets/badges/unverified.svg)

[Documentation](README.md) · [Progress](progress.md) · [Experiments](experiments/README.md)

Scope policy: **accepted**. Perform one proof experiment, then decide what formal verification is required for the first full MVP. Tool choices and the candidate targets below remain proposed. No property has been proved yet.

## First proof experiment

RV32I is selected. Choose one small implementation function and one precisely stated property for the first proof experiment. Record the proof attempt, assumptions, tool/version, result, and effort. Use that evidence to decide the release proof scope. The table below is a list of candidates, not a promise to prove every property before release.

## Distinguish the evidence

- Tests check concrete examples. Property tests and fuzzing explore many inputs.
- Differential tests compare implementations on exercised inputs.
- Formal verification establishes a stated property under explicit assumptions and a tool/model's supported semantics.
- Proving the OCaml model does not automatically prove the Rust CPU or native memory library.

## Initial targets

| Target | Property | Proposed route |
| --- | --- | --- |
| OCaml instruction codec | Decode(encode(i)) = i for one supported instruction with valid operands | Gospel/Cameleer through Why3, or a small WhyML component extracted to OCaml |
| Rust register writes | x0 remains zero | Kani proof harness |
| Rust ADD/ADDI logic | Result matches specified 32-bit modular arithmetic | Kani proof harness |
| C3 memory library | Bounds, permissions, byte order, and error behavior | Tests/fuzzing initially; a direct proof route remains open |
| Integrated execution | State agrees with an independent reference model | Differential tests; not a proof of equivalence |

Evaluate each proposed proof tool on one tiny function before designing the project around it. Gospel contract checking alone is not an implementation proof. C3 contracts and language safety checks are also not general formal proofs.

## Record for every proof

Property; function/model covered; input assumptions; integer representation; memory assumptions; loop bounds if applicable; tool and version; reproduction command; result; excluded behavior.

Foreign-function assumptions require particular care: a Rust proof using a modeled memory API does not cover the actual native implementation. Whole-system correctness is a later, separate goal.

## Primary references

- [Gospel](https://github.com/ocaml-gospel/gospel)
- [Cameleer](https://github.com/ocaml-gospel/cameleer)
- [Why3](https://why3.org/)
- [WhyML execution and extraction](https://why3.org/doc/exec.html)
- [Kani](https://model-checking.github.io/kani/)
- [Kani loop unwinding](https://model-checking.github.io/kani/tutorial-loop-unwinding.html)
