# Verified negative optimization branches at N=2^20

| Version | Tested branch | Kernel ms | Stwo SIMD ms | K/Stwo | Decision |
|---|---|---:|---:|---:|---|
| v0.14 | naive cache blocking | 11.248 | 7.105 | 1.583 | KILL |
| v0.15 | generic PackedM31 true radix-8 | 10.777 | 7.046 | 1.529 | KILL |
| v0.17 | PackedM31 radix-8 + exact Stwo twiddle mul | 10.855 | 6.978 | 1.556 | KILL |
