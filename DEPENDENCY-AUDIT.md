# Dependency Audit Configuration and Security Guide

## Overview

This document describes the dependency security and audit configuration for SysAdmin Tamagotchi.

## Automated Tools

### 1. Dependabot (GitHub)

**Configuration**: `.github/dependabot.yml`

**Features**:
- Weekly automated dependency updates
- Separate PRs for Rust (Cargo) and JavaScript (npm) dependencies
- Monthly GitHub Actions updates
- Automatic grouping of patch updates
- Configurable PR limits to avoid spam

**Schedule**:
- Rust dependencies: Every Monday at 09:00 UTC
- npm dependencies: Every Monday at 09:00 UTC
- GitHub Actions: Monthly

**Usage**:
1. Dependabot automatically creates PRs for updates
2. Review the changelog and breaking changes
3. CI pipeline runs tests automatically
4. Merge if tests pass and changes are acceptable
5. Close PR if update is not desired (with comment explaining why)

**Configuration customization**:
```yaml
# To ignore specific dependencies
ignore:
  - dependency-name: "dependency-name"
    update-types: ["version-update:semver-major"]
```

### 2. cargo-deny

**Configuration**: `deny.toml`

**Features**:
- Security vulnerability scanning
- License compliance checking
- Duplicate dependency detection
- Source verification (crates.io only)

**Installation**:
```bash
cargo install cargo-deny
```

**Usage**:
```bash
# Check all advisories, licenses, bans, and sources
cargo deny check

# Check only security advisories
cargo deny check advisories

# Check only licenses
cargo deny check licenses

# Check for duplicate dependencies
cargo deny check bans

# Check dependency sources
cargo deny check sources
```

**Integration with CI**:
Add to `.github/workflows/ci.yml`:
```yaml
- name: Run cargo-deny
  run: |
    cargo install cargo-deny
    cargo deny check
```

### 3. cargo-audit

**Purpose**: Scan Rust dependencies for known security vulnerabilities

**Installation**:
```bash
cargo install cargo-audit
```

**Usage**:
```bash
# Scan for vulnerabilities
cargo audit

# Deny build if vulnerabilities found
cargo audit --deny warnings

# Generate JSON report
cargo audit --json > audit-report.json
```

**Already integrated**: ✅ GitHub Actions CI workflow

### 4. cargo-outdated

**Purpose**: Check for outdated Rust dependencies

**Installation**:
```bash
cargo install cargo-outdated
```

**Usage**:
```bash
# Check for outdated dependencies
cargo outdated

# Exit with error code if outdated dependencies found
cargo outdated --exit-code 1

# Show only direct dependencies
cargo outdated --root-deps-only
```

## Security Policies

### Vulnerability Response

**Severity Levels and Response Times**:

| Severity | Response Time | Action |
|----------|---------------|--------|
| Critical | 24 hours | Immediate patch, emergency release |
| High | 7 days | Priority patch, expedited release |
| Medium | 30 days | Include in next regular release |
| Low | 90 days | Consider for future release |

**Process**:
1. **Detection**: Automated via `cargo audit` in CI pipeline
2. **Assessment**: Review vulnerability details and impact
3. **Patching**: Update dependency to patched version
4. **Testing**: Run full test suite
5. **Release**: Deploy updated version
6. **Communication**: Notify users via release notes

### Dependency Update Policy

**Automatic Updates** (via Dependabot):
- Patch versions (0.1.1 → 0.1.2): Auto-merge if tests pass
- Minor versions (0.1.0 → 0.2.0): Manual review required
- Major versions (0.x → 1.x): Manual review + testing required

**Manual Review Checklist**:
- [ ] Read changelog and migration guide
- [ ] Check for breaking changes
- [ ] Run full test suite
- [ ] Test application manually
- [ ] Review license changes (if any)
- [ ] Update documentation if needed
- [ ] Verify binary size impact (<10% increase acceptable)

### License Compliance

**Allowed Licenses**:
- MIT
- Apache-2.0
- BSD-2-Clause / BSD-3-Clause
- ISC
- Zlib
- Unicode-DFS-2016

**Denied Licenses**:
- GPL-3.0 (copyleft incompatible with MIT)
- AGPL-3.0 (network copyleft)

**Policy**:
- All dependencies must have explicit, approved licenses
- Copyleft licenses trigger warnings for manual review
- Unknown/unlicensed dependencies are denied

**Checking licenses**:
```bash
# Install cargo-license
cargo install cargo-license

# List all dependency licenses
cargo license --json > licenses.json

# Check with cargo-deny
cargo deny check licenses
```

## Supply Chain Security

### Trusted Sources

**Allowed**:
- crates.io (official Rust package registry)
- npm registry (official Node.js package registry)

**Denied**:
- Git dependencies (unless explicitly whitelisted)
- Unknown registries
- Local path dependencies (development only)

**Exceptions**:
If a git dependency is required (e.g., unpublished patch):
1. Fork the repository to your own GitHub account
2. Apply necessary patches
3. Add to `deny.toml` under `[sources.allow-git]`
4. Document the reason in `DEPENDENCY-NOTES.md`
5. Plan migration to crates.io version

### Dependency Pinning

**Rust (Cargo)**:
- ✅ `Cargo.lock` committed to version control
- Ensures reproducible builds
- All developers and CI use exact same versions

**JavaScript (npm)**:
- ✅ `package-lock.json` committed to version control
- Use `npm ci` instead of `npm install` in CI
- Ensures consistent frontend builds

### Verification

**Checksum Verification**:
- Cargo automatically verifies checksums from crates.io
- npm verifies package integrity via SHA-512

**Signature Verification**:
- Consider using `cargo-vet` for additional verification
- Manual review of critical dependencies

## Monitoring and Reporting

### Weekly Report

Run weekly security check:
```bash
# Create weekly-security-check.ps1
cargo audit
cargo outdated --root-deps-only
cargo deny check advisories
```

**Expected output**: No vulnerabilities, no critical outdated dependencies

### Monthly Audit

Full dependency audit:
```bash
# Run comprehensive checks
cargo deny check
cargo license --json > licenses-$(date +%Y%m).json
cargo tree --depth 1 > dependency-tree-$(date +%Y%m).txt
```

**Review**:
- New dependencies added
- License changes
- Dependency bloat (excessive transitive dependencies)

### Metrics to Track

| Metric | Target | Current |
|--------|--------|---------|
| Known vulnerabilities | 0 | 0 |
| Outdated dependencies | <5% | TBD |
| License violations | 0 | 0 |
| Duplicate dependencies | <5 | TBD |
| Dependency count | <100 | ~30 (direct + transitive) |

## Emergency Procedures

### Critical Vulnerability Discovered

1. **Immediate Actions**:
   ```bash
   # Check if vulnerability affects project
   cargo audit

   # Identify affected dependency
   cargo tree -i <vulnerable-crate>

   # Check for patched version
   cargo outdated | grep <vulnerable-crate>
   ```

2. **Patching**:
   ```bash
   # Update to patched version
   cargo update -p <vulnerable-crate>

   # Run tests
   cargo test

   # Build release
   cargo build --release
   ```

3. **Emergency Release**:
   ```bash
   # Bump patch version
   # Update version in Cargo.toml and tauri.conf.json

   # Create emergency release
   git tag -a v0.1.1 -m "Security patch: CVE-XXXX-XXXX"
   git push origin master --tags
   ```

4. **Communication**:
   - Update CHANGELOG.md with security notice
   - Create GitHub Security Advisory
   - Notify users via release notes

### Compromised Dependency

If a dependency is compromised (malicious code injected):

1. **Immediately**:
   - Remove dependency from `Cargo.toml`
   - Find alternative or implement functionality in-house
   - Alert team and users

2. **Investigation**:
   - Review when dependency was added
   - Check if compromised version was used in any release
   - Analyze potential impact

3. **Recovery**:
   - Rebuild all releases after compromised version
   - Revoke affected releases
   - Issue security advisory

## Tools Installation Guide

### Quick Setup

```powershell
# Windows PowerShell
# Install all recommended security tools

# Rust security tools
cargo install cargo-audit
cargo install cargo-deny
cargo install cargo-outdated
cargo install cargo-license

# Optional: Advanced tools
cargo install cargo-vet       # Supply chain verification
cargo install cargo-geiger    # Unsafe code detection
```

```bash
# Linux/macOS
# Install all recommended security tools

cargo install cargo-audit
cargo install cargo-deny
cargo install cargo-outdated
cargo install cargo-license
cargo install cargo-vet
cargo install cargo-geiger
```

### Verification

```bash
# Verify installations
cargo audit --version
cargo deny --version
cargo outdated --version
cargo license --version
```

## Best Practices

### Do's

✅ **DO** keep dependencies up to date
✅ **DO** review changelogs before updating
✅ **DO** run security audits regularly
✅ **DO** use `Cargo.lock` and `package-lock.json`
✅ **DO** minimize dependency count
✅ **DO** prefer mature, well-maintained crates
✅ **DO** check crate download counts and GitHub stars
✅ **DO** verify licenses before adding dependencies

### Don'ts

❌ **DON'T** blindly accept Dependabot PRs
❌ **DON'T** use git dependencies without review
❌ **DON'T** ignore security warnings
❌ **DON'T** add dependencies for trivial functionality
❌ **DON'T** use abandoned crates
❌ **DON'T** skip testing after updates
❌ **DON'T** commit secrets or API keys

## Resources

### Official Documentation
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [cargo-deny](https://embarkstudios.github.io/cargo-deny/)
- [Dependabot](https://docs.github.com/en/code-security/dependabot)
- [RustSec Advisory Database](https://rustsec.org/)

### Security Advisories
- [GitHub Advisory Database](https://github.com/advisories)
- [npm Security Advisories](https://www.npmjs.com/advisories)
- [Rust CVE Database](https://cve.mitre.org/)

### Learning Resources
- [Rust Security Best Practices](https://anssi-fr.github.io/rust-guide/)
- [Supply Chain Security Guide](https://github.com/ossf/scorecard)

---

**Last Updated**: 2026-01-13
**Next Review**: Before v1.0.0 release
**Maintainer**: Infra/DevOps Agent
