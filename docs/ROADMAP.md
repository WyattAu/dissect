# dissect ROADMAP

Threat model zero is our own supply chain: upstream already had a
trojanized copy circulate (6.3.0 archive, per upstream advisories). Trust
story first, features second.

## R0: Baseline + supply-chain posture (this fork, week one)

- [ ] Upstream install + provider readiness on CachyOS; record in
      docs/BASELINE.md with dates and commands
- [ ] Release policy: git tags signed, install script checksummed, checksum
      published in the release body; document the verification steps in
      README before the first fork release
- [ ] LICENSE + NOTICE audit across the vendored tree (upstream ships a
      licenses/ dir; keep it intact)

## R1: Provider health gates

- [ ] `dissect doctor --json`: per-provider (Ghidra/IDA/Hopper/JADX/Binwalk)
      installed version, license, readiness, last-probe result; exit codes
      CI can gate on
- [ ] Investigation preflight: refuse to start when a required provider is
      degraded, list exactly what is missing
- [ ] Version-pinned provider manifests; no floating installs

## R2: Sandboxed analysis (estate clawdius)

- [ ] clawdius sandbox provider: targets are copied into a disposable root,
      engines run against the copy, grants for outside-path access are
      explicit and logged
- [ ] Network policy for dynamic tools (browser capture, PTY process runs)
      default-deny with per-investigation allow rules
- [ ] Poisoned-sample tests: hostile ELFs/ASARs in the corpus must not
      escape the sandbox root

## R3: Tamper-evident evidence

- [ ] Evidence store chains record hashes (estate auditlog/tamper-audit):
      each engine report appended to a hash chain keyed by investigation ID
- [ ] `dissect verify <investigation>` re-derives the chain and diffs
      findings against stored evidence
- [ ] Findings manual generator stamps every claim with its evidence-chain
      position; confirmed/inferred/unknown tags preserved from upstream

## R4: Depth + ecosystem

- [ ] Extra providers: r2/r2ghidra via the estate RE toolchain skills
      (re-toolchain, re-triage) as first-class workflows
- [ ] Corpus CI: the upstream demo binaries become regression fixtures with
      expected-findings snapshots
- [ ] Website parity decision: upstream's website/ dir vs estate starlight
      docs; pick one, do not carry both
