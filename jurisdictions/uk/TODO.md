# UK Jurisdiction Implementation TODO

## Overview
Total estimated: ~26,140 lines across 65-70 files

## Phase 1: Project Foundation ✅
- [x] Create directory structure
- [x] Create Cargo.toml with dependencies
- [x] Create README.md
- [x] Create TODO.md (this file)
- [ ] Initialize lib.rs with module declarations

## Phase 2: Employment Law Module (~2,470 LOC)
- [ ] types.rs (~900 LOC)
  - [ ] EmploymentContract struct with builder
  - [ ] ContractType enum (Permanent, FixedTerm, ZeroHours, PartTime)
  - [ ] Employee / Employer structs
  - [ ] Dismissal struct with DismissalReason enum
  - [ ] RedundancyPayment struct
  - [ ] WorkingHours struct
  - [ ] MinimumWageAssessment struct

- [ ] error.rs (~180 LOC)
  - [ ] UkEmploymentError enum with ERA/WTR references
  - [ ] Error variants for all validation failures

- [ ] validator.rs (~550 LOC)
  - [ ] validate_employment_contract()
  - [ ] validate_notice_period()
  - [ ] validate_dismissal()
  - [ ] validate_working_hours()
  - [ ] validate_minimum_wage()

- [ ] redundancy.rs (~250 LOC)
  - [ ] Statutory redundancy payment calculation
  - [ ] Age-based multipliers (0.5x, 1.0x, 1.5x)

- [ ] working_time.rs (~220 LOC)
  - [ ] 48-hour week validation
  - [ ] Rest break entitlements
  - [ ] Annual leave calculation (5.6 weeks)

- [ ] minimum_wage.rs (~180 LOC)
  - [ ] Age-based rates (£6.40-£11.44)
  - [ ] National Living Wage (21+)
  - [ ] Apprentice rate

- [ ] builders.rs (~150 LOC)
  - [ ] Builder patterns for contracts

- [ ] mod.rs (~40 LOC)
  - [ ] Module documentation
  - [ ] Re-exports

## Phase 3: Data Protection Module (~11,390 LOC)
- [ ] mod.rs with re-exports from EU GDPR (~100 LOC)
  - [ ] Re-export article6, article9, rights, security, etc.

- [ ] enforcement.rs (~350 LOC)
  - [ ] IcoEnforcement type
  - [ ] ICO action types (information notice, enforcement notice, fine)

- [ ] adequacy.rs (~200 LOC)
  - [ ] UkAdequacyDecision enum
  - [ ] Post-Brexit adequacy landscape

- [ ] exemptions.rs (~280 LOC)
  - [ ] Dpa2018Exemption enum
  - [ ] National security, journalism, research exemptions

- [ ] cross_border_transfers.rs (~250 LOC)
  - [ ] UkCrossBorderTransfer struct
  - [ ] UK IDTA, EU SCCs with addendum

- [ ] dpo.rs (~450 LOC)
  - [ ] UK DPO designation rules (adapted from EU)
  - [ ] ICO notification requirements

- [ ] error.rs (~120 LOC)
  - [ ] UK-specific GDPR errors

## Phase 4: Consumer Rights Module (~2,660 LOC)
- [ ] types.rs (~450 LOC)
  - [ ] GoodsContract, ServicesContract, DigitalContent
  - [ ] GoodsStatutoryRight enum
  - [ ] ServicesStatutoryRight enum
  - [ ] Remedy enum
  - [ ] UnfairTermTest struct

- [ ] error.rs (~180 LOC)
  - [ ] ConsumerRightsError with CRA references

- [ ] goods.rs (~350 LOC)
  - [ ] CRA Part 1 goods contracts (s.9-11)
  - [ ] Satisfactory quality, fit for purpose, as described

- [ ] services.rs (~300 LOC)
  - [ ] CRA Part 1 services contracts (s.49-52)
  - [ ] Reasonable care and skill

- [ ] digital.rs (~300 LOC)
  - [ ] CRA Part 1 digital content (s.34-47)

- [ ] remedies.rs (~400 LOC)
  - [ ] Tiered remedy calculation
  - [ ] Short-term reject (30 days)
  - [ ] Repair/replace → price reduction/final reject

- [ ] unfair_terms.rs (~250 LOC)
  - [ ] CRA Part 2 unfair terms test
  - [ ] Grey list (Schedule 2)

- [ ] validator.rs (~350 LOC)
  - [ ] Cross-cutting validation

- [ ] mod.rs (~80 LOC)

## Phase 5: Contract Law Module (~3,050 LOC)
- [ ] types.rs (~500 LOC)
  - [ ] ContractFormation struct
  - [ ] Offer, Acceptance, Consideration
  - [ ] IntentionToCreateLegalRelations enum
  - [ ] Capacity enum
  - [ ] TermClassification enum
  - [ ] BreachOfContract struct
  - [ ] ContractRemedy enum

- [ ] error.rs (~200 LOC)
  - [ ] ContractLawError with case law citations

- [ ] formation.rs (~400 LOC)
  - [ ] Contract formation rules
  - [ ] Mirror image rule
  - [ ] Postal rule

- [ ] terms.rs (~350 LOC)
  - [ ] Condition, Warranty, Innominate term

- [ ] breach.rs (~300 LOC)
  - [ ] Breach types and remedies

- [ ] consideration.rs (~250 LOC)
  - [ ] Consideration validation
  - [ ] Must move from promisee
  - [ ] Not past consideration

- [ ] capacity.rs (~200 LOC)
  - [ ] Contractual capacity (age, mental, corporate)

- [ ] remedies.rs (~350 LOC)
  - [ ] Damages (Hadley v Baxendale)
  - [ ] Specific performance
  - [ ] Injunction

- [ ] validator.rs (~400 LOC)
  - [ ] Formation validation

- [ ] mod.rs (~100 LOC)

## Phase 6: Company Law Module (~3,570 LOC)
- [ ] types.rs (~600 LOC)
  - [ ] CompanyType enum
  - [ ] CompanyFormation struct
  - [ ] Director struct
  - [ ] DirectorDutiesCompliance struct (7 duties)
  - [ ] ShareCapital struct
  - [ ] CompanyNameValidation struct

- [ ] error.rs (~300 LOC)
  - [ ] CompanyLawError with CA 2006 references

- [ ] formation.rs (~450 LOC)
  - [ ] Formation process (CA 2006 Part 2)

- [ ] directors.rs (~400 LOC)
  - [ ] Seven statutory duties (ss.171-177)

- [ ] shares.rs (~350 LOC)
  - [ ] Share capital structure
  - [ ] Share classes and rights

- [ ] names.rs (~300 LOC)
  - [ ] Name restrictions (ss.53-81)
  - [ ] Sensitive words

- [ ] accounts.rs (~250 LOC)
  - [ ] Annual accounts (Part 15)

- [ ] meetings.rs (~300 LOC)
  - [ ] General meetings, resolutions

- [ ] validator.rs (~500 LOC)
  - [ ] Multi-stage validation

- [ ] mod.rs (~120 LOC)

## Phase 7: Examples (~3,000 LOC)
### Employment Law Examples
- [ ] employment-contract-validation.rs (~200 LOC)
- [ ] unfair-dismissal-claim.rs (~150 LOC)
- [ ] redundancy-calculation.rs (~150 LOC)
- [ ] working-time-compliance.rs (~150 LOC)
- [ ] zero-hours-contract.rs (~120 LOC)

### Data Protection Examples
- [ ] uk-gdpr-consent-validation.rs (~150 LOC)
- [ ] uk-ico-enforcement.rs (~180 LOC)
- [ ] uk-international-transfers.rs (~200 LOC)
- [ ] uk-dpa2018-exemptions.rs (~170 LOC)
- [ ] uk-dpo-registration.rs (~150 LOC)

### Consumer Rights Examples
- [ ] consumer-goods-remedy.rs (~200 LOC)
- [ ] consumer-digital-content.rs (~150 LOC)
- [ ] consumer-unfair-terms.rs (~180 LOC)

### Contract Law Examples
- [ ] contract-formation.rs (~200 LOC)
- [ ] contract-consideration.rs (~150 LOC)
- [ ] contract-breach-damages.rs (~200 LOC)

### Company Law Examples
- [ ] company-formation.rs (~200 LOC)
- [ ] company-director-duties.rs (~180 LOC)
- [ ] company-name-validation.rs (~150 LOC)

## Phase 8: Testing & Integration
- [ ] Unit tests for all validators
- [ ] Integration tests across modules
- [ ] Run cargo nextest continuously
- [ ] Fix all warnings (no warnings policy)
- [ ] Verify all examples run correctly
- [ ] Documentation review

## Critical Success Factors
- [ ] No warnings policy enforced
- [ ] EU GDPR dependency working correctly
- [ ] Case law properly referenced in contract module
- [ ] Tiered remedies state machine correct
- [ ] Seven director duties fully implemented

## Notes
- Files must be < 2000 lines (refactor if needed)
- Use latest crate versions from crates.io
- All errors must reference relevant statutes
- Follow patterns from DE, EU, JP jurisdictions
