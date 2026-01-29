# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in Legalis-RS, please report it by emailing security@cooljapan.eu. Please do not create public GitHub issues for security vulnerabilities.

We take all security reports seriously and will respond within 48 hours.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Known Security Issues

### Medium Severity: RSA Timing Sidechannel (RUSTSEC-2023-0071)

**Affected Component**: `rsa 0.9.10` (transitive dependency via `sqlx-mysql`)

**Details**:
- **Advisory**: [RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071)
- **Issue**: Marvin Attack - potential key recovery through timing sidechannels in RSA PKCS#1 v1.5 decryption
- **Severity**: 5.9/10 (Medium, CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:N/A:N)
- **Status**: No fixed upgrade available as of 2026-01-28

**Dependency Chain**:
```
rsa 0.9.10
└── sqlx-mysql 0.9.0-alpha.1
    └── sqlx 0.9.0-alpha.1
        └── legalis-registry 0.1.4 (optional feature)
```

**Impact Assessment**:
- **Scope**: Only affects `legalis-registry` when MySQL database support is enabled
- **Core Impact**: Does NOT affect core legal computation engine or jurisdiction modules
- **Attack Requirements**: Requires attacker to:
  - Have network access to perform timing measurements
  - Be able to send multiple crafted RSA ciphertexts
  - Have precise timing measurement capabilities
- **Risk Level**: LOW for typical deployment scenarios (legal computation is primary use case, not database encryption)

**Mitigation**:
- MySQL feature is optional - most users won't use it
- Consider alternative databases (PostgreSQL, SQLite) if MySQL is needed
- Monitor upstream for `sqlx` updates that remove or update `rsa` dependency
- For production deployments requiring MySQL, implement additional network-level security controls

**Monitoring**:
We actively monitor this issue and will update dependencies when fixes become available.

### Unmaintained Dependencies (Low Priority)

The following dependencies are flagged as unmaintained but have low security impact:

1. **fxhash 0.2.1** (RUSTSEC-2025-0057)
   - Dependency chain: `fxhash → selectors → kuchiki → printpdf`
   - Impact: Used in PDF generation feature only
   - Alternative: `rustc-hash` or `ahash` (migration planned for v0.2.0)

2. **kuchiki 0.8.1** (RUSTSEC-2023-0019)
   - Dependency chain: `kuchiki → printpdf`
   - Impact: HTML parsing for PDF generation
   - Alternative: Consider replacing `printpdf` with `typst` or `tectonic` (migration planned)

3. **proc-macro-error 1.0.4** (RUSTSEC-2024-0370)
   - Dependency chain: `proc-macro-error → ouroboros_macro → ouroboros → allsorts → printpdf`
   - Impact: Build-time only, no runtime security impact

**Plan**: These dependencies are in the PDF generation feature tree (`printpdf`). We are evaluating alternatives for v0.2.0, including:
- Migrating to actively maintained PDF libraries
- Making PDF features fully optional with feature flags
- Using external PDF generation tools via CLI interface

## Security Best Practices

When using Legalis-RS:

1. **Keep Dependencies Updated**: Regularly run `cargo audit` and `cargo update`
2. **Feature Minimization**: Only enable features you need (e.g., disable database features if not used)
3. **Input Validation**: Always validate and sanitize legal text inputs
4. **Sensitive Data**: Be cautious with personal data in legal documents - use anonymization features
5. **Production Deployments**: 
   - Use release builds (`--release`)
   - Enable all security hardening compiler flags
   - Run behind appropriate network security controls
   - Follow GDPR/data protection requirements for legal data

## Dependency Auditing

We use the following tools to maintain security:

- **cargo-audit**: Automated vulnerability scanning
- **RustSec Advisory Database**: Up-to-date vulnerability information
- **Dependabot**: Automated dependency updates (GitHub)

Run security audit yourself:
```bash
cargo install cargo-audit
cargo audit
```

## Security Roadmap

- **v0.1.5**: Document all optional features and their security implications
- **v0.2.0**: Migrate away from unmaintained dependencies (printpdf alternatives)
- **v0.3.0**: Comprehensive security audit by external firm
- **v1.0.0**: Full security certification for production use

## Disclosure Timeline

When vulnerabilities are reported and fixed:

1. **Day 0**: Vulnerability reported to security@cooljapan.eu
2. **Day 1-2**: Initial assessment and confirmation
3. **Day 3-14**: Develop and test fix
4. **Day 14**: Coordinated disclosure with security advisory
5. **Day 14+**: Patch release published

## Contact

- **Security Reports**: security@cooljapan.eu
- **General Issues**: https://github.com/cool-japan/legalis/issues
- **Website**: https://cooljapan.eu

---

**Last Updated**: 2026-01-28  
**Next Review**: 2026-04-28 (quarterly)
