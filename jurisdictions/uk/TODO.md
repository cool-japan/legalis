# UK Jurisdiction Implementation TODO

## Reconciliation Status (2026-06-14)

A full code audit was performed cross-checking **every** item below against the actual `src/`
tree (~72,000 LOC across 15 modules). The original list was a stale granular build-plan: nearly
all listed types, enums and functions already exist — frequently **consolidated into each module's
`types.rs` / `validator.rs`** rather than the separate files the plan named (`redundancy.rs`,
`dpo.rs`, etc.). An item is marked `[x]` only where the named struct/enum/function was verified
to exist in code; honest notes indicate the actual file.

**Already implemented (reconciled this pass):** Phases 1-6 in full (Employment, Data Protection,
Consumer Rights, Contract, Company), plus 9 further jurisdiction modules that were never tracked
here at all: `intellectual_property`, `financial_services`, `trusts`, `family`, `tort`,
`criminal`, `public_law`, `land_law`, `reasoning`.

**Newly added this pass (employment module, ERA 1996):**
- **Legal-accuracy fix** — the statutory redundancy calculation now uses the correct per-year
  **age-banded reckoning counted backwards** (ERA 1996 s.162(1)-(3)). The previous code applied a
  single multiplier (set by the age at the dismissal date) to *all* years, over-stating the
  entitlement of any employee who crossed an age band. Example: age 45 / 10 yrs / £700 cap was
  £10,500, correct is £8,400 (4 yrs @1.5 + 6 yrs @1.0 = 12 weeks).
- `ServiceReckoning` (s.162(2) age-band breakdown) + `statutory_weeks_due()` helper.
- `BasicAward` (unfair-dismissal basic award, ERA 1996 s.119) + `statutory_maximum()` = £21,000.
- `CompensatoryAward` (ERA 1996 ss.123-124), capped at the lower of 52 weeks' gross pay or the
  £115,115 statutory maximum (April 2024).
- `UnfairDismissalAward` (combined basic + compensatory).
- Named documented constants: `WEEKLY_PAY_CAP_GBP` (£700, s.227), `MAX_RECKONABLE_YEARS` (20),
  `CompensatoryAward::{STATUTORY_MAXIMUM_GBP, WEEKS_LIMIT}`.
- New example `unfair-dismissal-award.rs`; `redundancy-calculation.rs` rewritten to the banded method.
- 10 new `#[test]`s (647 → 657, all green); `clippy -D warnings` clean.

**Genuinely outstanding (honest, deferred):** *None remaining* — both previously deferred gaps
were closed on 2026-06-14 (see the COMPLETED section below):
- ~~Data Protection: a dedicated DPO designation module~~ → **DONE** (`src/data_protection/dpo.rs`).
- ~~The remaining dedicated example *files*~~ → **DONE** (10 new files in `examples/`).

---

## Phase 1: Project Foundation ✅
- [x] Create directory structure
- [x] Create Cargo.toml with dependencies
- [x] Create README.md
- [x] Create TODO.md (this file)
- [x] Initialize lib.rs with module declarations — `src/lib.rs` declares all 15 modules

## Phase 2: Employment Law Module ✅ (implemented in `src/employment/`)
- [x] types.rs — `src/employment/types.rs`
  - [x] EmploymentContract struct with builder (`EmploymentContractBuilder`)
  - [x] ContractType enum (Permanent, FixedTerm, ZeroHours, PartTime)
  - [x] Employee / Employer structs
  - [x] Dismissal struct with DismissalReason enum (+ AutomaticallyUnfairReason)
  - [x] RedundancyPayment struct
  - [x] WorkingHours struct
  - [x] MinimumWageAssessment struct

- [x] error.rs — `src/employment/error.rs`
  - [x] EmploymentError enum with ERA/WTR/NMWA references
  - [x] Error variants for all validation failures

- [x] validator.rs — `src/employment/validator.rs`
  - [x] validate_employment_contract()
  - [x] validate_notice_period()
  - [x] validate_dismissal()
  - [x] validate_working_hours()
  - [x] validate_minimum_wage()

- [x] Statutory redundancy payment calc — `RedundancyPayment::calculate_statutory_payment` (types.rs)
  - [x] Age-based multipliers (0.5x, 1.0x, 1.5x) — **corrected to per-year banded reckoning, s.162**
  - [x] (NEW) `ServiceReckoning` age-band breakdown + `statutory_weeks_due()`

- [x] Working time — `validate_working_hours`, `WorkingHours`, `AnnualLeaveEntitlement`, `RestEntitlement`
  - [x] 48-hour week validation (WTR 1998 Reg 4)
  - [x] Rest break entitlements (WTR 1998 Reg 12)
  - [x] Annual leave calculation (5.6 weeks / 28 days, WTR 1998 Reg 13)

- [x] Minimum wage — `MinimumWageAssessment` (types.rs) + `validate_minimum_wage`
  - [x] Age-based rates (£6.40-£11.44, April 2024 constants)
  - [x] National Living Wage (21+)
  - [x] Apprentice rate

- [x] Builders — `EmploymentContractBuilder` (types.rs)
- [x] mod.rs — module docs + re-exports

- [x] (NEW) Unfair dismissal awards — `src/employment/types.rs`
  - [x] BasicAward (ERA 1996 s.119) + `statutory_maximum()`
  - [x] CompensatoryAward (ERA 1996 ss.123-124) with statutory cap
  - [x] UnfairDismissalAward (combined)

## Phase 3: Data Protection Module ✅ (implemented in `src/data_protection/`)
- [x] mod.rs with re-exports from EU GDPR — extensive `pub use legalis_eu::gdpr::{...}`
  - [x] Re-export DataProcessing, LawfulBasis, rights, accountability, design, etc.

- [x] enforcement.rs — `IcoEnforcement`, `IcoEnforcementType`, `Dpa2018Offence`, `Article83Tier`
  - [x] ICO action types (information notice, enforcement notice, assessment, penalty, prosecution)

- [x] adequacy.rs — `UkAdequacyDecision`, `is_adequate_country_uk`
  - [x] Post-Brexit adequacy landscape

- [x] exemptions.rs — `Dpa2018Exemption` (+ Defense/Crime/Legal/Health/ArmedForces purposes)
  - [x] National security (s.26), journalism (Sch 2 Pt 5), research exemptions

- [x] Cross-border transfers — `src/data_protection/adequacy.rs` (`TransferMechanism`)
  - [x] UK IDTA (`TransferMechanism::UkIdta`), EU SCCs with Addendum, Article 49 derogations

- [x] dpo.rs — **IMPLEMENTED** (`src/data_protection/dpo.rs`): UK DPO designation rules under
      UK GDPR Art 37-39 / DPA 2018 s.69. `DpoAssessment` + `DpoAssessmentOutcome` (typed
      "is designation required?" assessment over Art 37(1)(a) public authority, (b) large-scale
      regular & systematic monitoring, (c) large-scale special-category/criminal data, plus
      DPA 2018 s.69 competent authorities, with the court-acting-judicially exemption and an
      Unclear/borderline "recommended" tier). `DpoTask` (Art 39(1)(a)-(e) tasks),
      `DpoPosition`/`DpoPositionFailure` (Art 38 position & independence — involvement, resources,
      operational independence, protection from dismissal, reporting line, conflict of interests),
      `DpoContactDetails`/`DpoNotificationFailure` (Art 37(7) publication + ICO notification of
      contact details). Wired into `data_protection/mod.rs`; 19 new `#[test]`s + 1 runnable doctest.

- [x] error.rs — `UkDataProtectionError` (in `src/data_protection/mod.rs`)

## Phase 4: Consumer Rights Module ✅ (implemented in `src/consumer_rights/`)
- [x] types.rs — `src/consumer_rights/types.rs`
  - [x] GoodsContract, ServicesContract, DigitalContentContract (+ `ConsumerContract`)
  - [x] GoodsStatutoryRight enum (CRA ss.9-11)
  - [x] ServicesStatutoryRight enum (CRA ss.49-52)
  - [x] DigitalContentStatutoryRight enum (CRA ss.34-47)
  - [x] Remedy types (`ConsumerRemedy`, `RemedyType`, `RemedyStage`)
  - [x] UnfairTerm / UnfairTermAssessment

- [x] error.rs — `ConsumerRightsError` with CRA references
- [x] goods — CRA Part 1 goods (s.9-11): satisfactory quality, fit for purpose, as described
- [x] services — CRA Part 1 services (s.49): reasonable care and skill
- [x] digital — CRA Part 1 digital content (s.34)
- [x] remedies — tiered state machine (`RemedyStage`): short-term reject (30 days) → repair/replace → price reduction/final reject
- [x] unfair_terms — CRA Part 2 test + grey list (`GreyListItem`, Schedule 2)
- [x] validator.rs — cross-cutting validation (`src/consumer_rights/validator.rs`)
- [x] mod.rs

## Phase 5: Contract Law Module ✅ (implemented in `src/contract/`)
- [x] types.rs — `src/contract/types.rs`
  - [x] ContractFormation struct
  - [x] Offer, Acceptance, Consideration
  - [x] IntentionToCreateLegalRelations enum (+ AgreementContext, IntentionPresumption)
  - [x] Capacity (`ContractualCapacity`, `IncapacityType`)
  - [x] TermClassification enum (Condition / Warranty / Innominate)
  - [x] Breach (`ContractBreach`, `BreachType`)
  - [x] ContractRemedy enum (+ DamagesType, RemotenessTest, HadleyLimb)

- [x] error.rs — `ContractError` with case-law citations
- [x] formation — rules in `validator.rs` (mirror image rule; postal rule — Adams v Lindsell)
- [x] terms.rs — Condition, Warranty, Innominate term (`src/contract/terms.rs`)
- [x] breach — `src/contract/breach_contract.rs`
- [x] consideration — validation in `validator.rs` (past consideration — Re McArdle; move from promisee — Tweddle v Atkinson)
- [x] capacity — `ContractualCapacity` / `IncapacityType` (types.rs)
- [x] remedies.rs — Damages (Hadley v Baxendale), specific performance, injunction (`src/contract/remedies.rs`)
- [x] validator.rs — formation validation
- [x] vitiating.rs — bonus: misrep/mistake/duress/undue influence (`src/contract/vitiating.rs`)
- [x] mod.rs

## Phase 6: Company Law Module ✅ (implemented in `src/company/`)
- [x] types.rs — `src/company/types.rs`
  - [x] CompanyType enum
  - [x] CompanyFormation struct
  - [x] Director struct (+ DirectorType)
  - [x] DirectorDutiesCompliance struct (7 duties, ss.171-177, with per-duty sub-structs)
  - [x] ShareCapital struct (+ ShareClass, ShareRights)
  - [x] CompanyNameValidation struct
  - [x] AnnualAccountsRequirement, MeetingType, ResolutionType

- [x] error.rs — `CompanyLawError` with CA 2006 references (ss.53-81, 171-177, 282-283, 307, 336, 441, 475)
- [x] formation — process in types.rs/validator.rs (CA 2006 Part 2)
- [x] directors — seven statutory duties (ss.171-177) via `DirectorDutiesCompliance` + errors
- [x] shares.rs — share capital structure, classes and rights (`src/company/shares.rs`)
- [x] names — restrictions (ss.53-81), sensitive words (s.55) via `CompanyNameValidation`
- [x] accounts — annual accounts (Part 15, s.441/s.475) via `AnnualAccountsRequirement`
- [x] meetings.rs — general meetings, resolutions (`src/company/meetings.rs`)
- [x] validator.rs — multi-stage validation
- [x] insolvency.rs + restructuring.rs — bonus modules
- [x] mod.rs

## Phase 7: Examples
### Employment Law Examples
- [x] uk-employment-contract-validation.rs (was: employment-contract-validation.rs)
- [x] unfair-dismissal-award.rs **(NEW — basic + compensatory award, ss.119/124)** (was: unfair-dismissal-claim.rs)
- [x] redundancy-calculation.rs (rewritten to banded s.162 method)
- [x] working-time-compliance.rs — **ADDED** (`examples/working-time-compliance.rs`): WTR 1998
      Reg 4 48h average limit + opt-out via `validate_working_hours`/`WorkingHours`, Regs 10-12
      rest breaks via `RestEntitlement`, Reg 13 5.6-weeks annual leave via `AnnualLeaveEntitlement`.
- [x] zero-hours-contract.rs — **ADDED** (`examples/zero-hours-contract.rs`): builds a
      `ContractType::ZeroHours` `EmploymentContract` and shows the s.27A exclusivity-clause ban via
      `validate_employment_contract` / `validate_contract_type`.

### Data Protection Examples
- [x] uk-gdpr-consent-validation.rs — **ADDED** (`examples/uk-gdpr-consent-validation.rs`): the five
      Article 4(11)/Article 7 consent conditions via `ConsentQuality::is_valid()`.
- [x] ico-enforcement-actions.rs (was: uk-ico-enforcement.rs)
- [x] gdpr-adequacy-transfers.rs (was: uk-international-transfers.rs)
- [x] uk-dpa2018-exemptions.rs — **ADDED** (`examples/uk-dpa2018-exemptions.rs`): `Dpa2018Exemption`
      statutory bases + `validate_journalism_exemption` / `validate_academic_research_exemption`.
- [x] uk-dpo-registration.rs — **ADDED** (`examples/uk-dpo-registration.rs`): uses the new DPO
      module — `DpoAssessment` designation, `DpoTask`, `DpoPosition`, `DpoContactDetails`
      (Art 37(7) ICO notification).

### Consumer Rights Examples
- [x] consumer-rights-remedies.rs (was: consumer-goods-remedy.rs)
- [x] consumer-digital-content.rs — **ADDED** (`examples/consumer-digital-content.rs`): CRA 2015
      ss.33-47 digital-content rights via `DigitalContentContract` /
      `validate_digital_content_contract` + s.34 `validate_satisfactory_quality`.
- [x] consumer-unfair-terms.rs — **ADDED** (`examples/consumer-unfair-terms.rs`): CRA 2015 Part 2
      (ss.62-76) fairness test via `UnfairTermAssessment::is_unfair()` / `validate_unfair_term`
      with the Schedule 2 `GreyListItem` grey list.

### Contract Law Examples
- [x] uk-contract-formation.rs (was: contract-formation.rs)
- [x] contract-consideration.rs — **ADDED** (`examples/contract-consideration.rs`): the
      consideration doctrine via `Consideration::is_valid()` / `validate_consideration`
      (Chappell v Nestlé sufficiency, Re McArdle past consideration, Williams v Roffey practical benefit).
- [x] contract-breach-damages.rs — **ADDED** (`examples/contract-breach-damages.rs`): term
      classification via `validate_breach` and the measure of damages via `DamagesCalculation` with
      Hadley v Baxendale remoteness (`RemotenessAnalysis`) and mitigation (`MitigationAnalysis`).

### Company Law Examples
- [x] company-formation.rs
- [x] director-duties.rs (was: company-director-duties.rs)
- [x] company-name-validation.rs — **ADDED** (`examples/company-name-validation.rs`): CA 2006
      name rules via `validate_company_name` (required ss.58-59 ending, s.55 sensitive words,
      s.57 prohibited characters) assembling a `CompanyNameValidation` record.

### Bonus Examples (beyond original plan)
- [x] fca-authorization-principles.rs (financial_services)
- [x] fca-suitability-assessment.rs (financial_services)

## Phase 8: Testing & Integration
- [x] Unit tests for all validators — 657 inline `#[test]`s across all modules
- [x] Integration across modules — `reasoning` engine + `statute_adapter` + doctests
- [x] Run cargo nextest — 657 passed, 0 failed (this pass)
- [x] Fix all warnings (no warnings policy) — `cargo clippy -p legalis-uk --all-targets -- -D warnings` clean
- [x] Verify examples compile/run — all 11 examples build; redundancy + unfair-dismissal run-verified
- [x] Documentation review — doctests pass (7 run)

## Critical Success Factors
- [x] No warnings policy enforced
- [x] EU GDPR dependency working correctly (re-exports compile and are used)
- [x] Case law properly referenced in contract module (Hadley v Baxendale, Adams v Lindsell, Tweddle v Atkinson, Re McArdle, …)
- [x] Tiered remedies state machine correct (`RemedyStage`)
- [x] Seven director duties fully implemented (ss.171-177)

## Notes
- Files must be < 2000 lines (largest: `criminal/offences/homicide.rs` 1885; `employment/types.rs` now 1100 — all within limit)
- All errors reference relevant statutes
- April-2024 statutory figures used throughout (NMW, £700 week's-pay cap, £115,115 compensatory cap)
- Remaining genuine gaps: **none** — DPO designation module and all dedicated example files now provided
  (see COMPLETED 2026-06-14 below).

---

## COMPLETED (2026-06-14 — DPO module + example files)

This pass closed the two genuinely outstanding gaps recorded above.

### 1. DPO designation module (`src/data_protection/dpo.rs`, 814 lines)
Deep implementation of the UK GDPR Data Protection Officer regime, wired into
`src/data_protection/mod.rs` (new `pub mod dpo;` + re-exports).

- **Designation assessment (UK GDPR Art 37(1) / DPA 2018 s.69).** `DpoAssessment` (structured
  input: organisation type, court-acting-judicially flag, per-category `*_is_core` booleans and
  `MonitoringScale` for monitoring / special-category / criminal-offence data, and the
  competent-authority flag) → `DpoAssessmentOutcome` listing every applicable `DesignationGround`:
  - Art 37(1)(a) — public authority/body (courts acting judicially excluded);
  - Art 37(1)(b) — core-activity large-scale regular & systematic monitoring;
  - Art 37(1)(c) — core-activity large-scale special-category (Art 9) data;
  - Art 37(1)(c) — core-activity large-scale criminal-offence (Art 10) data;
  - DPA 2018 s.69 — competent authority processing for the law-enforcement purposes.
  `is_mandatory()` / `is_recommended()` (borderline `Unclear` scale → best-practice) /
  `is_voluntary_only()`.
- **DPO tasks (Art 39(1)(a)-(e)).** `DpoTask` enum with `all()`, `statutory_provision()`,
  `description()`.
- **Position & independence (Art 38).** `DpoPosition` (involvement, resources, operational
  independence, protection from dismissal, reporting to highest management, no conflict of
  interests) → `is_compliant()` / `compliance_failures()` returning typed `DpoPositionFailure`s.
- **ICO notification (Art 37(7)).** `DpoContactDetails` → `validate_notification()` /
  `is_compliant()` returning `DpoNotificationFailure` (no contact point / not published /
  not notified to ICO); `ICO_DPO_NOTIFICATION_URL` constant.
- **Tests:** 19 new `#[test]`s + 1 runnable doctest (657 → 676 inline tests, all green).

### 2. Dedicated example files (10 new files in `examples/`)
Each compiles under `cargo build -p legalis-uk --examples`, calls real library APIs, uses
`fn main() -> Result<…>` + `?` where fallible (no `unwrap`/`expect`/`panic!`), and demonstrates the
rule meaningfully:

- `working-time-compliance.rs` — WTR 1998 (48h limit + opt-out, rest breaks, 5.6-weeks leave).
- `zero-hours-contract.rs` — `ContractType::ZeroHours`; ERA 1996 s.27A exclusivity-clause ban.
- `uk-gdpr-consent-validation.rs` — Art 4(11)/Art 7 consent via `ConsentQuality::is_valid()`.
- `uk-dpa2018-exemptions.rs` — `Dpa2018Exemption` + journalism / research validators.
- `uk-dpo-registration.rs` — the new DPO module (designation, tasks, position, ICO notification).
- `consumer-digital-content.rs` — CRA 2015 ss.33-47 digital-content rights.
- `consumer-unfair-terms.rs` — CRA 2015 Part 2 fairness test + Schedule 2 grey list.
- `contract-consideration.rs` — consideration doctrine (Chappell, Re McArdle, Williams v Roffey).
- `contract-breach-damages.rs` — breach classification + Hadley v Baxendale damages + mitigation.
- `company-name-validation.rs` — CA 2006 name rules (suffix, sensitive words, prohibited chars).

### Verification (this pass)
- `cargo nextest run -p legalis-uk` — **676 tests run: 676 passed, 0 skipped** (was 657).
- `cargo test -p legalis-uk --doc` — doctests pass (incl. the new DPO doctest).
- `cargo build -p legalis-uk --examples` — all 21 examples build.
- `cargo clippy -p legalis-uk --all-targets -- -D warnings` — **clean** (no warnings).
- All source files remain < 2000 lines (`dpo.rs` = 814).
