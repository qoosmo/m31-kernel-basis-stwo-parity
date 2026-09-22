# Full manuscript consistency audit

Audit version: v0.1.3

Correction history: v0.1.1 fixed the over-broad `fastest` rule; v0.1.3 adds macOS Bash 3.2 compatibility;
only unsupported marketing claims such as `fastest FFT/transform` are rejected.

- checks: 43
- passed: 43
- failed: 0

## Checks

- [x] `section:## Abstract`
- [x] `section:## 1. Introduction`
- [x] `section:## 2. Mathematical setting`
- [x] `section:## 3. Boolean-zeta coordinates`
- [x] `section:## 4. Bit-reversed execution schedule`
- [x] `section:## 5. ARM64 implementation`
- [x] `section:## 6. Negative results`
- [x] `section:## 7. Experimental methodology`
- [x] `section:## 8. Results`
- [x] `section:## 9. Discussion`
- [x] `section:## 10. Related work`
- [x] `section:## 11. Future work`
- [x] `section:## 12. Conclusion`
- [x] `notation:B^m`
- [x] `notation:|a|_2`
- [x] `notation:K_y`
- [x] `notation:Z_1`
- [x] `notation:D,C`
- [x] `forbidden:S_b(`
- [x] `forbidden:T^{\otimes`
- [x] `forbidden:fully sheared basis`
- [x] `prior:BooleanKernel citation`
- [x] `prior:KernelPCS citation`
- [x] `bib:BooleanKernel`
- [x] `bib:KernelPCS`
- [x] `guard:notation lock`
- [x] `guard:prior alignment`
- [x] `evidence:0.979510`
- [x] `evidence:0.976630`
- [x] `evidence:0.996149`
- [x] `evidence:6.924750`
- [x] `evidence:7.058000`
- [x] `evidence:headline parity`
- [x] `evidence:native boundary`
- [x] `claim:first` — \bwe\s+(?:introduce|present|give|provide)\s+the\s+first\b
- [x] `claim:fastest-transform` — \b(?:the|a|our|world'?s)?\s*fastest\s+(?:fft|transform|implementation|kernel|prover)\b
- [x] `claim:state-of-art` — \bstate[- ]of[- ]the[- ]art\b
- [x] `claim:beats-stwo` — \bbeats?\s+Stwo\b
- [x] `claim:outperforms-stwo` — \boutperforms?\s+Stwo\b
- [x] `audit:benign-fastest-to-slowest-allowed` — occurrences=1
- [x] `frontmatter:no TODO`
- [x] `conclusion:no TODO`
- [x] `title:no-shear-branding`

## Frozen parity result

- median kernel/Stwo: `0.97951000`
- range: `0.97663000 .. 0.99614900`
- median kernel: `6.924750 ms`
- median Stwo: `7.058000 ms`
- rounds inside ±5%: `7/7`

## Decision

**PASS — reconciled manuscript is internally consistent at this gate.**

Remaining work:
- exact benchmark-host metadata;
- figures generated from frozen evidence;
- targeted exact-formula prior-art check;
- human mathematical proof audit;
- submission-format conversion.
