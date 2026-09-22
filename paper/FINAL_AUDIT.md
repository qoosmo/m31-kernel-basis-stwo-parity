# Final proof, citation, provenance, and figure audit

Audit version: v0.1.1

Correction from v0.1.0: `claims:no PCS` is now checked case-insensitively
against the claims ledger. No manuscript claim or evidence logic changed.

- checks: 75
- passed: 75
- failed: 0

## Checks

- [x] `sha:source`
- [x] `sha:final-csv`
- [x] `sha:optimization-evidence`
- [x] `sha:stage-evidence`
- [x] `section-prefix:## 1.`
- [x] `section-prefix:## 2.`
- [x] `section-prefix:## 3.`
- [x] `section-prefix:## 4.`
- [x] `section-prefix:## 5.`
- [x] `section-prefix:## 6.`
- [x] `section-prefix:## 7.`
- [x] `section-prefix:## 8.`
- [x] `section-prefix:## 9.`
- [x] `section-prefix:## 10.`
- [x] `section-prefix:## 11.`
- [x] `section-prefix:## 12.`
- [x] `abstract`
- [x] `notation:\mathbb B^m`
- [x] `notation:|a|_2`
- [x] `notation:K_y(X)`
- [x] `notation:Z_1`
- [x] `notation:(D,C)`
- [x] `forbidden:S_b(`
- [x] `forbidden:T^{\otimes`
- [x] `forbidden:fully sheared basis`
- [x] `prior:BooleanKernel`
- [x] `prior:KernelPCS`
- [x] `prior:alignment-file`
- [x] `prior:exact-search-note`
- [x] `claims:no PCS` — case-insensitive claims-ledger guardrail
- [x] `claims:no end-to-end` — claims ledger must preserve end-to-end boundary
- [x] `figure-ref:negative-ablations.svg`
- [x] `figure-file:negative-ablations.svg`
- [x] `figure-provenance:negative-ablations.svg`
- [x] `figure-ref:optimization-trajectory.svg`
- [x] `figure-file:optimization-trajectory.svg`
- [x] `figure-provenance:optimization-trajectory.svg`
- [x] `figure-ref:stability-parity.svg`
- [x] `figure-file:stability-parity.svg`
- [x] `figure-provenance:stability-parity.svg`
- [x] `figure-ref:stage-profile-v0.19.svg`
- [x] `figure-file:stage-profile-v0.19.svg`
- [x] `figure-provenance:stage-profile-v0.19.svg`
- [x] `final-evidence:0.979510`
- [x] `final-evidence:0.976630`
- [x] `final-evidence:0.996149`
- [x] `final-evidence:6.924750`
- [x] `final-evidence:7.058000`
- [x] `stage:3.484`
- [x] `stage:paper-value`
- [x] `bib-key:BCKL21`
- [x] `bib-key:BCKL22`
- [x] `bib-key:CGHLLPS26`
- [x] `bib-key:GaoMateer10`
- [x] `bib-key:HLN23`
- [x] `bib-key:HLP24`
- [x] `bib-key:Habock24GFFT`
- [x] `bib-key:LiX24`
- [x] `bib-key:LinAHC16`
- [x] `bib-key:Mkhida2026BooleanKernel`
- [x] `bib-key:Mkhida2026KernelPCS`
- [x] `bib-key:Plonky3Software`
- [x] `bib-key:SamantaBadakhshanGong26`
- [x] `bib-key:StwoSoftware`
- [x] `claim:first` — \bwe\s+(?:introduce|present|give|provide)\s+the\s+first\b
- [x] `claim:fastest` — \bfastest\s+(?:fft|transform|implementation|kernel|prover)\b
- [x] `claim:state-of-art` — \bstate[- ]of[- ]the[- ]art\b
- [x] `claim:beats-stwo` — \bbeats?\s+Stwo\b
- [x] `claim:outperforms-stwo` — \boutperforms?\s+Stwo\b
- [x] `claim:parity`
- [x] `claim:conversion-boundary`
- [x] `figure-number:1`
- [x] `figure-number:2`
- [x] `figure-number:3`
- [x] `figure-number:4`

## Decision

**PASS — manuscript is ready for submission-format conversion.**

This gate establishes internal consistency with the frozen source/results,
prior Algorizk notation, bibliography, verified figures, and conservative
claim boundaries. It is not a substitute for external peer review.
