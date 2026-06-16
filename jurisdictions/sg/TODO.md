# TODO - Singapore Jurisdiction Implementation

## COMPLETED (2026-06-14 — additional legal domains batch 2: SFA + Property)

The final two "Additional Legal Domains" implemented to the established quality
bar (typed models, validators, builders, accurate statute/case citations,
bilingual English + 华语 errors, SGD money in u64 cents, comprehensive tests).
All additive and backward-compatible; nothing already marked DEFERRED was
touched. Structure mirrors `src/contract/` and `src/competition/` exactly.

**Domains delivered:**

1. **Securities and Futures Act 2001 (SFA)** — `src/securities/`
   - Capital markets products (s.2(1)): securities, units in a collective
     investment scheme, derivatives contracts, leveraged spot FX; CIS
     authorisation (s.286) / recognition (s.287).
   - Investor classification (s.4A): institutional / accredited / retail, with
     the accredited-investor wealth/income thresholds (net personal assets
     SGD 2m with the SGD 1m primary-residence cap, net financial assets SGD 1m,
     income SGD 300k; corporation net assets SGD 10m).
   - Offers of investments — prospectus regime (Part 13): s.240 registration
     requirement, s.246, s.253 defective prospectus; exemptions: small offers
     (s.272A, SGD 5m / 12 months), private placement (s.272B, ≤ 50 persons),
     institutional (s.274), accredited (s.275).
   - Market conduct (Part 12): insider trading — connected person s.218 / other
     person s.219, with inside information s.214-216 (not generally available +
     material price effect); false trading & market rigging s.197; employment of
     manipulative/deceptive devices s.201; false/misleading statements s.199;
     fraudulent inducement s.200. Civil penalty cap s.232 (3× profit, min
     SGD 50k individual / SGD 100k corporation; SGD 2m where no profit); criminal
     maxima s.204/s.221 (SGD 250k / 7 years).
   - Licensing (Part 4): Capital Markets Services licence (s.82-83), regulated
     activities (Second Schedule), appointed representatives on the MAS public
     register (s.99B/s.99D).
   - Files: error.rs (336), types.rs (682), offerings.rs (361), misconduct.rs
     (437), validator.rs (627), mod.rs (185). 65 inline + 15 integration tests.

2. **Property Law (Land Titles Act 1993 — Torrens)** — `src/property/`
   - Indefeasibility of title (s.46): registered title paramount (s.46(1)),
     exceptions incl. fraud/forgery to which the proprietor was party or privy
     (s.46(2)) and the in personam exception (*UOB v Bebe* [2006] SGCA 30);
     overriding interests (short leases / easements) bind without defeating.
   - Caveats (s.115): caveatable (proprietary) interest requirement; prohibition
     on registration of inconsistent dealings.
   - Conveyancing: writing requirement (Civil Law Act s.6(d)); the
     option-to-purchase practice (1% option fee, 14-day option period, 4%
     balance deposit — market conventions); completion; Buyer's Stamp Duty
     (Stamp Duties Act 1929, residential & non-residential marginal scales as at
     2023).
   - Leases: 7-year registration threshold (s.45 / s.46(1) overriding short
     leases; *Walsh v Lonsdale* for unregistered long leases), covenants
     (express/implied), determination (effluxion/surrender/merger/notice to
     quit/forfeiture), forfeiture & relief (CLPA s.18).
   - Interests: mortgages take effect as a charge (s.68) with power of sale on
     default; easements per *Re Ellenborough Park* [1956] Ch 131 (four
     characteristics).
   - Files: error.rs (273), types.rs (640), leases.rs (388), conveyancing.rs
     (415), validator.rs (535), mod.rs (207). 56 inline + 16 integration tests.

**Gates:**
- `cargo nextest run -p legalis-sg`: **711 passed / 0 skipped** (was 559).
- `cargo test -p legalis-sg --doc`: 62 passed / 11 ignored (incl. new SFA &
  property doc examples).
- `cargo clippy -p legalis-sg --all-targets -- -D warnings`: **clean** (zero
  warnings). No unwrap/expect/panic/todo/unimplemented/unreachable in non-test
  code (expect()/panic! only inside #[test]).
- Every source file < 2000 lines (largest new file: securities/types.rs, 682).
- Workspace dep policy respected (no new external crates, no inline versions).
- lib.rs wiring is collision-free: `securities` and `property` re-exported with
  curated lists; `Result` aliased to `SecuritiesResult` / `PropertyResult`. No
  type-name clashes with existing crate-root exports (verified: Security,
  Prospectus, Lease, Mortgage, Caveat, Easement, Completion, etc. are all new).
- New integration test files: securities_tests.rs (15), property_tests.rs (16).
  Performance tests assert typical validation < 1ms; serde JSON roundtrip tests
  for all new aggregate types (offerings, licences, claims, titles, leases, OTPs,
  reports).

## COMPLETED (2026-06-14 — additional legal domains batch 1)

Four new "Additional Legal Domains" implemented to the established quality bar
(typed models, validators, builders, accurate statute/case citations, bilingual
errors, comprehensive tests). All additive and backward-compatible; nothing
already marked DEFERRED was touched.

**Domains delivered:**

1. **Contract Law (Singapore common law)** — `src/contract/`
   - Formation: offer/acceptance (mirror image, postal vs receipt rule),
     consideration (executory/executed/past/existing-duty with the *Williams v
     Roffey* practical-benefit gloss), intention to create legal relations
     (commercial vs social-domestic presumptions). Four requirements per
     *Gay Choon Ing v Loh Sze Ti* [2009] SGCA 3.
   - Terms: condition / warranty / innominate (*Hongkong Fir*; *RDC Concrete v
     Sato Kogyo* [2007] SGCA 1).
   - Vitiating factors: misrepresentation (fraudulent *Derry v Peek* /
     negligent Misrepresentation Act 1967 s.2(1) / innocent s.2(2)), mistake
     (common *Great Peace Shipping* / mutual *Raffles v Wichelhaus* / unilateral
     *Chwee Kin Keong v Digilandmall.com* [2005] SGCA 2), duress (incl. economic),
     undue influence (actual / presumed, *RBS v Etridge*).
   - Discharge: performance / agreement / breach / frustration (*Davis
     Contractors v Fareham UDC*; Frustrated Contracts Act 1959).
   - Remedies: expectation damages with *Hadley v Baxendale* two-limb remoteness
     + mitigation (*British Westinghouse*), specific performance, termination.
   - Files: error.rs (304), types.rs (911), remedies.rs (270), validator.rs
     (565), mod.rs (132). 34 inline + 13 integration + 2 cross-domain tests.

2. **Tort Law (Singapore common law)** — `src/tort/`
   - Negligence: *Spandeck Engineering v DSTA* [2007] SGCA 37 two-stage duty
     test (factual foreseeability threshold + proximity + policy), breach
     (reasonable person / *Bolam*; risk calculus), causation ("but for"
     *Barnett*; novus actus), remoteness (*The Wagon Mound (No 1)*).
   - Defamation: Defamation Act 1957 — libel (per se) vs slander (special damage
     save ss.5–6), defences (justification s.8, fair comment, absolute/qualified
     privilege defeated by malice, offer of amends s.7).
   - Nuisance: private (standing/substantial/unreasonable, *Hunter v Canary
     Wharf*) and public (special damage, *Tate & Lyle v GLC*).
   - Occupiers' liability: duty by entrant status (visitor vs trespasser).
   - Defences: contributory negligence apportionment (CNPI Act 1953 s.3),
     volenti, illegality.
   - Files: error.rs (235), types.rs (589), nuisance.rs (452), validator.rs
     (520), mod.rs (109). 41 inline + 13 integration tests.

3. **Insolvency, Restructuring and Dissolution Act 2018 (IRDA)** —
   `src/insolvency/`
   - Corporate winding up: compulsory (s.125(1) grounds incl. (e) inability to
     pay; s.125(2) statutory-demand deeming, 3 weeks / SGD 15,000) and voluntary
     (members' / creditors').
   - Judicial management: s.89(1) statutory purposes + moratorium.
   - Schemes of arrangement: majority-in-number AND 75%-in-value test
     (overflow-safe), s.64 30-day moratorium + extension, s.70 cram-down.
   - Bankruptcy: SGD 15,000 debt threshold, Debt Repayment Scheme (≤ SGD
     150,000).
   - Files: error.rs (312), types.rs (1013), validator.rs (894), mod.rs (232).
     60 inline tests.

4. **Competition Act 2004** — `src/competition/`
   - s.34 prohibition: anti-competitive agreements (price fixing s.34(2)(a),
     output limitation (b), market sharing (c), discrimination (d), tying (e),
     bid rigging); by-object vs by-effect.
   - s.47 prohibition: abuse of dominance (indicative 60% share threshold).
   - s.54: mergers causing a substantial lessening of competition (40% / 70%
     CR3 / 20% indicative thresholds).
   - CCCS enforcement: penalty up to 10% of turnover for up to 3 years (s.69(4)),
     leniency programme, Third Schedule exclusions (net economic benefit, etc.).
   - Files: error.rs (219), types.rs (837), validator.rs (660), mod.rs (200).
     42 inline tests.

**Gates:**
- `cargo nextest run -p legalis-sg`: **559 passed / 0 skipped** (was 354).
- `cargo test -p legalis-sg --doc`: 58 passed / 11 ignored (incl. new contract &
  tort doc examples).
- `cargo clippy -p legalis-sg --all-targets -- -D warnings`: **clean** (zero
  warnings). No unwrap/expect/panic/todo/unimplemented/unreachable in non-test
  code (expect()/panic! only inside #[test]).
- Every source file < 2000 lines (largest new file: insolvency/types.rs, 1013).
- Workspace dep policy respected (no new external crates, no inline versions).
- 2 new runnable examples: contract_breach_analysis, negligence_claim_analysis.
- New integration test files: contract_law_tests.rs, tort_law_tests.rs,
  contract_tort_cross_tests.rs. Performance tests assert typical validation
  < 1ms; serde JSON roundtrip tests for all new aggregate types.

**Intentionally left for a follow-up pass** (down-list, lower priority): the
Securities and Futures Act 2001 (SFA) and Property Law (Land Titles Act /
conveyancing). Stopped here to keep all four delivered domains at the full depth
and citation-accuracy bar rather than spreading thinner.

## COMPLETED (2026-06-14 — PDPA + cross-domain)

Deep, citation-accurate rewrite of the PDPA module and addition of cross-domain
integration tests. The prior PDPA implementation was a thin "simplified" stub
(539-line single types.rs, 140-line validator with `// Simplified:` shortcuts and
several inaccurate citations/thresholds); it has been fully re-implemented.

**Delivered:**
- PDPA `types.rs` split into 6 focused submodules under `src/pdpa/types/` (all < 2000 lines).
- Consent-centric model: express (s.14) vs deemed consent — by conduct (s.15(1)),
  contractual necessity/pass-through (s.15(3)-(8)), and by notification (s.15A,
  with mandatory adverse-effect assessment + opt-out window).
- Withdrawal of consent (s.16) incl. the s.16(2) "explain consequences" duty and
  s.16(4) cessation effect.
- Purpose limitation (s.18) via a conservative compatibility matrix (marketing
  never silently expands).
- Notifiable data breach (s.26B) two-limb test: significant harm (reg.3(1):
  name/ID + financial/health, OR account credentials) and significant scale
  (500, reg.4); internal-only carve-out (s.26B(4)).
- **3-CALENDAR-DAY** PDPC notification — corrected to **s.26D(1)** (clock runs
  from the s.26C assessment, not discovery); calendar-day arithmetic; individual
  notification (s.26D(2)) + exceptions (s.26D(5)-(7)).
- DNC Registry (Part 9): three registers (voice/text/fax, s.39),
  check-before-marketing (s.43), 21-day confirmation validity (reg.15).
- DPO — corrected: designation is **MANDATORY** (s.11(3)), public BCI (s.11(5));
  advisory `DpoStaffingRecommendation` models scale only.
- Cross-border transfer (s.26 + PDP Regs 2021 reg.10-12).
- Access (s.21 + reg.5 30-day) and correction (s.22) requests.
- Business contact information exemption — corrected citation to **s.4(5)/s.2(1)**.
- Max financial penalty (s.48J(3)): SGD 1M, or 10% of SG turnover if > SGD 10M.
- 4 examples (consent_management, data_breach_notification, dnc_registry_check,
  dpo_requirement_assessment) — all run.
- Tests: pdpa_consent_tests (18), pdpa_breach_tests (16, covering 3-calendar-day
  timing exhaustively), cross_domain_tests (5: Company+Employment, Company+PDPA,
  Employment+PDPA, Consumer+PDPA).
- `cargo nextest run -p legalis-sg`: 354 passed, 0 failed (was 99).
- `cargo clippy -p legalis-sg --all-targets -- -D warnings`: clean. No
  unwrap/expect/panic in non-test code.

## Phase 1: Foundation ✅

- [x] Create Cargo.toml with dependencies
- [x] Create src/lib.rs with comprehensive documentation
- [x] Create src/citation.rs (Singapore legal citation system)
- [x] Create README.md
- [x] Create TODO.md

## Phase 2: Companies Act (Cap. 50) ✅

### Core Files
- [x] src/companies/mod.rs - Module documentation and re-exports
- [x] src/companies/types.rs - Company, Director, ShareCapital, Shareholder types (800 lines)
- [x] src/companies/validator.rs - Formation and compliance validation (650 lines)
- [x] src/companies/error.rs - CompaniesError enum with statute references (400 lines)
- [x] src/companies/acra.rs - ACRA registration logic, UEN handling (400 lines)
- [x] src/companies/governance.rs - AGM, annual return, board meetings (400 lines)

### Examples
- [x] examples/acra_company_registration.rs - Pte Ltd formation (reconciled: already present)
- [x] examples/director_compliance_check.rs - S145 resident director + s.148/149/154/155 disqualification (new)
- [x] examples/annual_compliance_checklist.rs - AGM (s.175) / annual return (s.197) / secretary (s.171) deadlines (new)
- [x] examples/share_issuance.rs - Share allotment, ownership, dilution (new)

### Tests
- [x] tests/companies_validation_tests.rs - Formation, UEN classification, capital, director, secretary (new, 11 tests)
- [x] tests/companies_governance_tests.rs - AGM/annual-return deadlines, notice, resolutions, quorum (new, 11 tests)

### Key Features
- [x] UEN (Unique Entity Number) validation (reconciled + enhanced: `classify_uen`/`UenFormat` for ACRA's 3 documented formats)
- [x] Resident director requirement (s. 145) (reconciled: `validate_resident_director_requirement`)
- [x] Share capital structures (par/no-par value) (reconciled: `ShareCapital`/`ShareClass`, s. 62A)
- [x] AGM deadline calculation (s. 175) (reconciled + added current FYE-based `calculate_agm_deadline_from_fye`, 4/6 months)
- [x] Annual return filing deadline (s. 197) (reconciled + added `calculate_annual_return_deadline_from_fye`, 5/7 months)
- [x] Company secretary requirement (s. 171) (reconciled + new `validate_company_secretary_requirement`: 6-month vacancy, sole-director s.171(1E), public s.171(1AA))
- [x] Director disqualification checking (s. 148/149/154/155) (reconciled + new expiry-aware `validate_director_disqualification`, `DisqualificationStatus::is_active`/`statute_section`)

## Phase 3: Employment Act (Cap. 91) ✅

### Core Files
- [x] src/employment/mod.rs - Module documentation and re-exports (183 lines)
- [x] src/employment/types.rs - EmploymentContract, WorkingHours, LeaveEntitlement (565 lines)
- [x] src/employment/validator.rs - Contract, working hours, leave validation (448 lines)
- [x] src/employment/error.rs - EmploymentError enum (210 lines)
- [x] Integrated CPF contribution calculations by age into types.rs
- [x] Integrated leave entitlement calculations (7→14 days) into types.rs
- [x] Integrated termination notice calculation (s. 10/11) into types.rs

### Examples
- [x] examples/employment_contract_validation.rs - Full contract validation (313 lines)
- [x] examples/cpf_contribution_calculator.rs - CPF by age groups (348 lines)
- [x] examples/leave_entitlement_calculator.rs - Leave progression by service years (298 lines)
- [x] examples/termination_notice_checker.rs - Notice period calculation (351 lines)

### Tests
- [x] tests/employment_contract_tests.rs - Contract, working hours, overtime, notice, leave, EA coverage (new, 8 tests)
- [x] tests/employment_cpf_tests.rs - CPF rate accuracy across all age brackets + wage ceiling (new, 6 tests)

### Key Features
- [x] EA coverage determination (Part IV s. 35) (new: `determine_ea_coverage`/`is_covered_by_part_iv`, `EmployeeCategory`/`EaCoverage`; workman ≤ SGD 4,500, non-workman ≤ SGD 2,600 named constants)
- [x] Working hours validation (44h/week max for non-shift) (reconciled: `validate_working_hours`, s. 38)
- [x] Overtime calculation (1.5x rate) (reconciled: `validate_overtime_payment`, s. 38(4))
- [x] CPF contribution rates by age (17%/20% for ≤55) (reconciled: `CpfContribution::rates_by_age`, all brackets tested)
- [x] CPF wage ceiling (SGD 6,000/month ordinary wage) (reconciled: `ORDINARY_WAGE_CEILING_CENTS`; see Known Limitations re annual updates)
- [x] Annual leave progression (7→14 days by years of service) (reconciled: `LeaveEntitlement::new`, s. 43)
- [x] Sick leave entitlement (14 outpatient + 60 hospitalization) (reconciled: `LeaveEntitlement`, s. 89)
- [x] Maternity leave (16 weeks for citizens) (reconciled: `LeaveEntitlement::with_maternity_leave`)
- [x] Termination notice periods (1 day → 4 weeks by service length) (reconciled: `TerminationNotice::required_notice_days`, s. 10/11)

## Phase 4: PDPA (Personal Data Protection Act 2012) ✅

### Core Files
- [x] src/pdpa/mod.rs - Module documentation and re-exports (33 lines, rewritten)
- [x] src/pdpa/types/ - split into focused submodules (was a single 539-line file):
  - [x] src/pdpa/types/mod.rs - re-exports + GDPR comparison (53 lines)
  - [x] src/pdpa/types/consent.rs - ConsentRecord, ConsentMethod, DeemedConsentBasis, PurposeOfCollection, business contact info exemption (512 lines)
  - [x] src/pdpa/types/breach.rs - DataBreachNotification, s.26B limbs, s.26D timing (383 lines)
  - [x] src/pdpa/types/dnc.rs - DncRegisterKind, DncRegistration, DncCheckConfirmation (182 lines)
  - [x] src/pdpa/types/organisation.rs - PdpaOrganisation, DpoContact, s.48J penalties (262 lines)
  - [x] src/pdpa/types/transfer.rs - DataTransfer, TransferMechanism (161 lines)
  - [x] src/pdpa/types/access.rs - DataSubjectRequest (s.21/s.22) (195 lines)
- [x] src/pdpa/validator.rs - validators + builders + PdpaValidationReport (610 lines, rewritten)
- [x] src/pdpa/error.rs - PdpaError enum, accurate section refs, quadrilingual (196 lines, rewritten)
- [x] Consent management (express s.14 vs deemed s.15(1)/15(3)-(8)/15A) in types/consent.rs
- [x] Breach notification workflow (s.26B determination / s.26C assess / s.26D notify) in types/breach.rs
- [x] Do Not Call Registry (Part 9, three registers) in types/dnc.rs
- [x] DPO accountability assessment (s.11) in validator.rs

### Examples
- [x] examples/consent_management.rs - express/deemed consent, purpose limitation, withdrawal (177 lines)
- [x] examples/data_breach_notification.rs - s.26B determination + 3-calendar-day s.26D(1) workflow (185 lines)
- [x] examples/dnc_registry_check.rs - three registers + check-before-marketing + 21-day validity (133 lines)
- [x] examples/dpo_requirement_assessment.rs - mandatory s.11(3) + advisory staffing + s.48J penalties (110 lines)

### Tests
- [x] tests/pdpa_consent_tests.rs - express/deemed consent, purpose limitation, withdrawal, BCI exemption, DNC (18 tests)
- [x] tests/pdpa_breach_tests.rs - s.26B limbs + 3-calendar-day s.26D(1) timing + individual notification (16 tests)

### Key Features
- [x] Consent-centric model (vs GDPR's 6 lawful bases) - documented in types/mod.rs, ConsentMethod
- [x] Explicit vs deemed consent (s. 15) - ConsentMethod + DeemedConsentBasis (conduct s.15(1), contract s.15(3)-(8), notification s.15A)
- [x] Purpose limitation (s. 18) - PurposeOfCollection::is_compatible_with + validate_purpose_limitation
- [x] Notifiable data breach determination (s. 26B) - two limbs: significant harm (reg.3) + significant scale (500, reg.4)
- [x] 3 calendar day breach notification deadline - CORRECTED to s. 26D(1) (runs from assessment, not discovery); s.26C is the duty to assess
- [x] DNC Registry types (voice call, text message, fax) - DncRegisterKind, s.39; 21-day confirmation validity (reg.15)
- [x] DPO criteria - CORRECTED: designation is MANDATORY (s.11(3)), public BCI (s.11(5)); advisory DpoStaffingRecommendation models scale only
- [x] Cross-border transfer validation (s. 26) - TransferMechanism + reg.10-12 (clauses, BCR, certification, consent)
- [x] Access request 30-day deadline (s. 21) - DataSubjectRequest + reg.5; correction (s.22)
- [x] Business contact information exemption - CORRECTED to s. 4(5) / s. 2(1) (was mis-cited as s.4(b)); is_business_contact_information

## Phase 5: Consumer Protection ✅

### Core Files
- [x] src/consumer/mod.rs - Module documentation and re-exports (187 lines)
- [x] src/consumer/types.rs - ConsumerContract, UnfairPractice, SaleOfGoods, ImpliedTerm (668 lines)
- [x] src/consumer/validator.rs - Contract validation, unfair practice detection (363 lines)
- [x] src/consumer/error.rs - ConsumerError enum (201 lines)
- [x] Integrated sale_of_goods logic into types.rs and validator.rs
- [x] Integrated unfair_practices detection into validator.rs

### Examples
- [x] examples/consumer_contract_analysis.rs - Full contract with risk scoring (398 lines)
- [x] examples/sale_of_goods_validation.rs - Implied terms checking (373 lines)

### Tests
- [x] 21 unit tests in types.rs (all passing)
- [x] 11 unit tests in validator.rs (all passing)
- [x] 3 unit tests in error.rs (all passing)

### Key Features
- [x] Implied term: Corresponds to description (SOGA s. 13)
- [x] Implied term: Merchantable quality (SOGA s. 14(2))
- [x] Implied term: Fitness for purpose (SOGA s. 14(3))
- [x] Implied term: Sale by sample (SOGA s. 15)
- [x] Unfair practice: False representation (CPFTA s. 4)
- [x] Unfair practice: Unconscionable conduct (CPFTA s. 5)
- [x] Unfair practice: Bait advertising (CPFTA s. 6)
- [x] Unfair practice: Harassment/coercion (CPFTA s. 7)
- [x] Unfair practice: Pyramid schemes (CPFTA s. 7A)
- [x] Contract term risk scoring (0-100 scale)
- [x] Small Claims Tribunal thresholds (SGD 20,000)
- [x] Lemon law (6 months after purchase)
- [x] Warranty validation
- [x] Remedy recommendations

## Phase 6: Integration & Polish ✅

### Quality Assurance
- [x] Run `cargo build --package legalis-sg` - ensure compilation ✅
- [x] Run `cargo nextest run --package legalis-sg` - all tests pass (99/99) ✅
- [x] Run `cargo nextest run --no-run --package legalis-sg` - **ZERO WARNINGS** ✅
- [x] Run `cargo clippy --package legalis-sg -- -D warnings` - clippy clean ✅
- [x] Run `cargo fmt --package legalis-sg -- --check` - formatting check ✅
- [x] Run all 7 examples successfully ✅

### Integration Tests
- [x] Cross-domain scenario: Company + Employment contract (tests/cross_domain_tests.rs)
- [x] Cross-domain scenario: Company + PDPA (corporate data controller, mandatory DPO s.11)
- [x] Cross-domain scenario: Employment + PDPA (employee data, screening consent + HR breach)
- [x] Cross-domain scenario: Consumer contract + PDPA (e-commerce: SOGA/CPFTA + consent + DNC + transfer)
  - All 4 scenarios in tests/cross_domain_tests.rs (5 tests, 261 lines)

### Documentation
- [x] All public items have documentation (verified for the 2026-06-14 batch 1 domains; pre-existing modules already documented)
- [x] All modules have overview documentation (contract/tort/insolvency/competition each have rich `//!` module docs)
- [x] Code examples in documentation compile (doctests: 58 passed / 11 ignored, incl. new contract & tort doc examples)
- [ ] README examples are accurate and tested
- [ ] Statute references accurate (verified against Singapore Statutes Online)

### Performance
- [x] Validation functions are efficient (< 1ms for typical cases) (contract/tort tests assert < 1ms over 10,000-iteration loops; insolvency/competition include perf-style tests)
- [ ] No unnecessary allocations in hot paths
- [x] Serialization/deserialization works correctly (serde JSON roundtrip tests added for all new aggregate types)

## Future Enhancements (Post-v0.1.1)

### Additional Legal Domains
- [x] Contract Law - Common law principles, remedies for breach (2026-06-14 batch 1: src/contract/)
- [x] Tort Law - Negligence, defamation, nuisance, occupiers' liability (2026-06-14 batch 1: src/tort/)
- [x] Insolvency Act (IRDA 2018) - Winding up, judicial management, schemes of arrangement, bankruptcy (2026-06-14 batch 1: src/insolvency/)
- [x] Competition Act 2004 - Anti-competitive agreements (s.34), abuse of dominance (s.47), mergers, CCCS (2026-06-14 batch 1: src/competition/)
- [x] Intellectual Property - Patents Act, Copyright Act, Trade Marks Act, Designs Act (already implemented: src/ip/)
- [x] Banking Act - MAS regulations, banking licenses (already implemented: src/banking/)
- [x] Securities and Futures Act - Capital markets, securities offerings (2026-06-14 batch 2: src/securities/)
- [x] Property Law - Land Titles Act, conveyancing, leases (2026-06-14 batch 2: src/property/)

### Advanced Features
- [ ] BizFile+ API integration (ACRA electronic filing) — DEFERRED: requires live external ACRA service + credentials; out of scope for pure-Rust offline library
- [ ] CPF online portal integration — DEFERRED: requires live external CPF Board service; out of scope for pure-Rust offline library
- [ ] PDPC case law database integration — DEFERRED: requires external PDPC/LawNet data source + credentials
- [ ] Real-time statute amendment tracking — DEFERRED: requires live amendment feed/external service
- [ ] Multi-language support (English, Chinese, Malay, Tamil) — DEFERRED: needs translation datasets (error messages are already quadrilingual)
- [ ] Case law citation and precedent linking — DEFERRED: requires external case-law dataset
- [ ] Legal opinion generation — DEFERRED: depends on case-law DB + LLM integration
- [ ] Compliance dashboard and reporting — DEFERRED: UI layer, out of scope for this crate

### Testing Enhancements
- [ ] Property-based testing (quickcheck/proptest) — DEFERRED: external fuzzing/property harness explicitly out of scope for this pass
- [ ] Fuzzing for validation logic — DEFERRED: external fuzzing harness (cargo-fuzz) out of scope for this pass
- [ ] Benchmark suite for performance monitoring — DEFERRED: separate criterion harness
- [ ] Integration tests with real ACRA/MOM data (anonymized) — DEFERRED: requires real ACRA/MOM data access

### Developer Experience
- [ ] Builder derive macros for complex types — DEFERRED: hand-written builders provided (ConsentRecordBuilder, DataBreachBuilder); proc-macro crate out of scope
- [ ] Custom lints for statute reference formatting — DEFERRED: requires custom lint tooling
- [ ] CLI tool for validation and compliance checking — DEFERRED: separate binary crate
- [ ] IDE plugins for legal citation autocomplete — DEFERRED: IDE plugin, out of scope for this crate
- [ ] VS Code extension for Singapore law syntax highlighting — DEFERRED: VS Code extension, out of scope for this crate

## Known Limitations

1. **CPF Rates**: Hardcoded for 2024 rates, need annual updates
2. **Statutes**: Based on versions as of 2024, need tracking system for amendments
3. **Case Law**: No integration with eLitigation/LawNet yet
4. **Regulatory Changes**: MOM/PDPC guidelines may change, requires monitoring
5. **Language Support**: English only currently, need Chinese/Malay/Tamil for full coverage

## Notes

### Implementation Policy
- **IMPLEMENT ALL**: No simplification, full implementation of all features
- **No Warnings**: Zero compiler/clippy warnings policy
- **Latest Crates**: Always use latest versions from crates.io
- **<2000 Lines**: Single file should be <2000 lines (refactor if exceeded)

### Statute Version Tracking
All implementations based on:
- Companies Act (Cap. 50): As of 2024 revision
- Employment Act (Cap. 91): As of 2024 revision
- PDPA: Personal Data Protection Act 2012 (as amended 2020)
- Sale of Goods Act (Cap. 393): As of 1994 revision
- Consumer Protection (Fair Trading) Act (Cap. 52A): As of 2009 revision

### Testing Standards
- Unit tests for all validation functions
- Integration tests for cross-domain scenarios
- Example code as documentation tests
- Edge cases and boundary conditions covered
- Real-world scenarios from ACRA/MOM/PDPC guidance

## 2026-06-14: Companies Act + Employment Act reconcile & gap-fill

**Reconciled (already implemented, marked above with notes — no code change):**
- Companies: resident director (s.145), par/no-par share capital (s.62A),
  AGM (s.175), annual return (s.197), base UEN validation, company secretary
  type/warning (s.171), base director disqualification, `acra_company_registration` example.
- Employment: working hours (s.38), overtime (s.38(4)), CPF rates by age + wage
  ceiling, annual leave (s.43), sick leave (s.89), maternity leave, termination
  notice (s.10/11).

**Newly implemented (gap-fill):**
- Companies `acra.rs`: `UenFormat` enum + `classify_uen` (ACRA's 3 documented
  UEN formats); `validate_uen`/`is_valid_uen` now delegate to it.
- Companies `types.rs`: corrected disqualification section attribution
  (s.148 bankruptcy / s.149-155 court order / s.154 conviction);
  `DisqualificationStatus::is_active`/`statute_section`, `Director::is_eligible_as_of`.
- Companies `validator.rs`: `validate_company_secretary_requirement` (s.171),
  `validate_director_disqualification` (expiry-aware), `SECRETARY_APPOINTMENT_DEADLINE_MONTHS`.
- Companies `governance.rs`: current FYE-based `calculate_agm_deadline_from_fye`,
  `calculate_annual_return_deadline_from_fye`, `is_agm_overdue_from_fye`,
  `agm_deadline_months` + month constants (s.175(1)/s.197(1)).
- Companies `error.rs`: `CompanySecretaryVacancyExceeded` variant (s.171(1)).
- Employment `types.rs`: `EmployeeCategory`, `EaCoverage`, Part IV salary-ceiling
  constants (workman SGD 4,500 / non-workman SGD 2,600).
- Employment `validator.rs`: `determine_ea_coverage`, `is_covered_by_part_iv` (Part IV s.35).
- New examples: `director_compliance_check`, `annual_compliance_checklist`, `share_issuance`.
- New integration tests: `companies_validation_tests`, `companies_governance_tests`,
  `employment_contract_tests`, `employment_cpf_tests`.

**Gates:** `cargo nextest run -p legalis-sg` → 300 passed / 0 skipped (was 237);
doctests 53 passed / 11 ignored; `cargo clippy -p legalis-sg --all-targets -- -D warnings` → clean.

## Progress Tracking

**Total Files Planned**: 56 files
**Core Modules Completed**: 5 domains (Foundation, Companies Act, Employment Act, PDPA, Consumer Protection) ✅
**Files Implemented**: 34 files (core modules + examples + integration tests) ✅
**Tests Passing**: 300/300 nextest + 53 doctests ✅ (was 99/99)
**Warnings**: 0 ✅ (Zero warnings policy enforced, `clippy --all-targets -D warnings`)
**Examples Working**: incl. 3 new Companies Act examples (director compliance, annual checklist, share issuance) ✅
**Lines of Code**: ~10,000 lines across implemented modules
**Language Support**: Trilingual errors (English/中文/Malay) ✅

### Estimated LOC
- **Phase 1 (Foundation)**: ~500 lines ✅
- **Phase 2 (Companies Act)**: ~4,000 lines ✅
- **Phase 3 (Employment Act)**: ~2,500 lines ✅
- **Phase 4 (PDPA)**: ~800 lines ✅
- **Phase 5 (Consumer Protection)**: ~1,300 lines ✅
- **Phase 6 (Integration)**: ~500-1,000 lines (⏳ in progress)

**Total Estimated**: ~15,000-20,000 LOC (similar to legalis-jp)
