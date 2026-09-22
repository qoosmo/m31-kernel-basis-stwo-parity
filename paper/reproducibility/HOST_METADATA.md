# Benchmark host metadata

This file records the host state collected after the frozen v0.20.1 parity
result. It does **not** rerun the benchmark.

## Frozen experiment identity

- Algorizk repository commit: `00523858bd3f4f399d3a26bd24a7d5642ea6215a`
- Algorizk source SHA-256: `a3a5a14353bd038e5a1f2ccf44d42ca880f2f137040a40ff9851479541ba2c7c`
- stability CSV SHA-256: `2391777c4872def6ecb903b5666b5ec0b47b87fa6bff79249d366e073c201232`
- pinned Stwo commit: `826591c6c371376810ca8213b5812d5daf6d5092`
- canonical stability result folder:
  `$HOME/algorizk-rd/benchmarks/kernel-vs-stwo-m31/results/kernel-fused-neon-tail2-stability-20260901-082551`
- canonical result folder status: `present`
- canonical result folder filesystem timestamp: `2026-09-01T08:26:14+0300`
- folder-name run timestamp: `2026-09-01 08:25:51` (local benchmark-host naming convention)

## Hardware

- architecture: `arm64`
- hardware model: `MacBookPro17,1`
- chip: `Apple M1`
- memory bytes: `17179869184`
- physical CPU cores: `8`
- logical CPU cores: `8`
- performance-cluster physical cores: `4`
- efficiency-cluster physical cores: `4`

## Operating system

- product: `macOS`
- version: `26.6.2`
- build: `25G83`
- kernel / uname: `Darwin Alis-MacBook-Pro-2.local 25.6.0 Darwin Kernel Version 25.6.0: Fri Jul 31 19:17:12 PDT 2026; root:xnu-12377.161.14~5/RELEASE_ARM64_T8103 arm64`

## Rust toolchain

```text
rustc 1.96.0-nightly (55e86c996 2026-04-02)
binary: rustc
commit-hash: 55e86c996809902e8bbad512cfb4d2c18be446d9
commit-date: 2026-04-02
host: aarch64-apple-darwin
release: 1.96.0-nightly
LLVM version: 22.1.2
```

```text
cargo 1.96.0-nightly (888f67534 2026-03-30)
```

## Native compiler

```text
Apple clang version 21.0.0 (clang-2100.1.1.101)
Target: arm64-apple-darwin25.6.0
Thread model: posix
```

- Xcode developer path: `/Library/Developer/CommandLineTools`
- Homebrew: `Homebrew 6.0.20`

## Power state captured now

This is **post hoc metadata**, not proof of the exact power/thermal state during
the frozen benchmark.

```text
Battery Power:
 Sleep On Power Button 1
 lowpowermode         1
 standby              1
 ttyskeepawake        1
 hibernatemode        3
 powernap             1
 hibernatefile        /var/vm/sleepimage
 displaysleep         10
 womp                 0
 networkoversleep     0
 sleep                1
 lessbright           1
 tcpkeepalive         1
 disksleep            10
AC Power:
 Sleep On Power Button 1
 lowpowermode         1
 standby              1
 ttyskeepawake        1
 hibernatemode        3
 powernap             1
 hibernatefile        /var/vm/sleepimage
 displaysleep         10
 womp                 1
 networkoversleep     0
 sleep                1
 tcpkeepalive         1
 disksleep            10
```

```text
Now drawing from 'Battery Power'
 -InternalBattery-0 (id=36569187)	77%; discharging; 4:55 remaining present: true
```

```text
Note: No thermal warning level has been recorded
Note: No performance warning level has been recorded
Note: No CPU power status has been recorded
```

## Build policy used for the final benchmark

The final benchmark methodology records:

- Rust release build;
- `-C target-cpu=native`;
- release codegen units = 1;
- ThinLTO;
- `RAYON_NUM_THREADS=1`;
- kernel timing includes the input copy;
- domain/twiddle preparation is excluded;
- Boolean-zeta/native-coordinate preparation is excluded;
- output canonicalization for correctness is excluded from timing.

## Reproducibility limitation

The exact instantaneous clock frequency, die temperature, fan state and
background-process state at the original frozen run were not independently
logged. The paper must not imply otherwise.
