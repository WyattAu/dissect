# dissect release and supply-chain policy

Threat model zero is this project's own supply chain. Upstream rea already
saw a trojanized copy circulate (a fake 6.3.0 release archive, per upstream
advisories). dissect treats that as the design constraint.

## Release artifacts

Every dissect release ships exactly:

1. **A signed git tag** (`git tag -s vX.Y.Z`, GPG or SSH signature). The
   signing key is published in this repository's SECURITY.md and on the
   maintainer's profile. Releases without a verifiable tag signature do not
   exist.
2. **A source tarball** (`git archive` from the tag, never from a dirty
   worktree).
3. **A SHA-256 checksums file** covering the tarball and any bundled
   installer, itself covered by a detached signature
   (`SHA256SUMS.asc`, `gpg --clearsign`).

## Install verification (user-facing contract)

The documented install path is:

```sh
# fetch the tag's checksums + signature, then:
gpg --verify SHA256SUMS.asc SHA256SUMS
sha256sum -c SHA256SUMS
# only then run the installer
```

`dissect doctor` re-checks provider integrity after install: pinned provider
versions (from the provider manifest), engine binaries on PATH, and
`licenses/` completeness. Any drift is reported with the expected and actual
hashes.

## Pinning rules

- Provider manifests (Ghidra, Hopper, IDA, JADX, Binwalk, Unblob) pin exact
  versions. No floating installs, no `@latest`.
- Third-party components keep their license texts in `licenses/` (upstream
  ships 22 files; audited intact 2026-10-31, see docs/BASELINE.md).
- CI installs nothing outside the lockfile/manifest set.

## Why no PyPI/npm distribution for the agent skill layer

Registry names are squattable (upstream's own install docs warn that the
PyPI `agent-reach`-style name collision happened to peers in this space;
rea's trojanized copy was a direct archive impersonation). Until a
reproducible-build story exists for registry artifacts, dissect distributes
from signed tags only.
