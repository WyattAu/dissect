# dissect

Reverse engineering as an agent workflow: your coding agent interrogates
binaries, apps, and sites through professional tools, with evidence for every
claim. A bridge, not a decompiler.

dissect is a fork of [rea (Reverse Engineer Anything)](https://github.com/morluto/rea)
(MIT), seeded from upstream commit `60149282fcfc` (see
[docs/UPSTREAM.md](docs/UPSTREAM.md)).

## Why this fork exists

Upstream's architecture is right and restrained: the agent reasons, the
bridge schedules Ghidra/IDA/Hopper/JADX, findings carry
confirmed/inferred/unknown tags. This fork hardens the trust story:

1. **Sandboxed by default.** Analysis targets run inside the WyattAu clawdius
   sandbox; the agent cannot point the toolchain at arbitrary system paths
   without an explicit, logged grant.
2. **Tamper-evident evidence.** The evidence store chains hashes (estate
   auditlog), so a findings manual can be checked against what the engines
   actually reported, after the fact.
3. **Signed releases, verified installs.** Upstream already suffered a
   trojanized copy circulating in the wild. dissect treats its own supply
   chain as threat model zero: signed tags, checksummed install scripts,
   `dissect doctor` verifying provider integrity.
4. **Provider health gates.** Per-engine readiness, versions, and licenses in
   `doctor --json`, so an investigation never starts half-armed.

## Status

Seeded, upstream-toolchain buildable. See [docs/ROADMAP.md](docs/ROADMAP.md).

## License

MIT, preserved in [LICENSE](LICENSE). Upstream copyright morluto and
contributors; this fork's changes are (c) 2026 WyattAu, same license.
Use only on software you are authorized to analyze.
