# Final seven-round stability result

Source: `results/stability-summary.csv`

Source SHA-256: `2391777c4872def6ecb903b5666b5ec0b47b87fa6bff79249d366e073c201232`

| log2 N | N | Kernel median (ms) | Stwo median (ms) | Median K/Stwo | Min | Max | Within ±5% |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 18 | 262144 | 1.536916 | 1.542792 | 0.996191 | 0.994730 | 1.003536 | 7/7 |
| 19 | 524288 | 3.218542 | 3.307583 | 0.974495 | 0.969027 | 0.978603 | 7/7 |
| 20 | 1048576 | 6.924750 | 7.058000 | 0.979510 | 0.976630 | 0.996149 | 7/7 |

Interpretation: all observed round-level aggregate ratios lie inside the
predeclared ±5% parity band for the reported sizes. The paper therefore
uses the conservative claim **stable native-transform parity**.
