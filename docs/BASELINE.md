# dissect baseline: upstream toolchain surface on CachyOS (wyattdeskacercachy)

Date: 2026-10-31. Seed commit: `60149282fcfc`.

## Vendored license audit

Upstream ships `licenses/` with 22 third-party license/notice files
(Ghidra 12.1.4 LICENSE/DISCLAIMER/NOTICE, frida-17.23.2, dotnet-runtime,
cfworker, GCC runtime exception, and others). Audited intact after seed:
22/22 present, unmodified.

## Toolchain readiness (upstream contract)

`npx rea-agents setup` registers dissect's engines with an agent. Native
analysis requires a configured provider (Ghidra, Hopper, or IDA). This host
has the estate RE toolchain partially provisioned (r2pm, GhidraMCP tracked
by fleet `sys-check`) — full provider readiness lands with ROADMAP R1's
`dissect doctor --json`, which turns readiness into a machine-checkable gate
instead of a README promise.

## Not yet measured (honest gaps)

- No provider probe run at baseline (doctor is R1's first deliverable)
- No sample investigation executed yet; the upstream demo corpus becomes the
  regression fixture set in R4 corpus CI

These are recorded as gaps by design: ROADMAP R1 exists to convert them into
gates, and this file will carry the numbers when it does.
