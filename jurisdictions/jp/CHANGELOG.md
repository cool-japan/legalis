# Changelog

All notable changes to `legalis-jp` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-09

### Added

#### e-Gov Electronic Filing System (Phase 0)
- **XML/JSON Format Support**: Full support for legacy XML and modern JSON filing formats
- **Application Status Management**: State machine for Draft/Submitted/UnderReview/Approved workflow
- **Attachment Validation**: MIME type checking, 10MB size limit enforcement
- **Validation Framework**: Pre-submission validation with detailed error reporting
- 5 new modules (`egov/`): error, types, validator, xml_parser, json_format
- 49 comprehensive tests

#### Administrative Procedure Act + Electronic Signatures (Phase 1)
- **Administrative Procedure Act** (行政手続法 Act No. 88 of 1993)
  - Procedure types: Application (申請), Notification (届出), Administrative Guidance (行政指導), Disposition (処分), Hearing (聴聞)
  - Article 5: Reason statement requirements (理由の提示)
  - Article 7: Standard processing period validation (標準処理期間)
- **Electronic Signatures Act** (電子署名法 Act No. 102 of 2000)
  - Signature algorithms: RSA-2048/4096, ECDSA-P256/P384
  - Certificate validation (validity period, expiration checking)
  - Electronic signature builder pattern
- 5 new modules (`administrative_procedure/`): error, types, validator, builder, filing
- 33 comprehensive tests

#### Construction Business Act + Real Estate Transactions Act (Phase 2)
- **Construction Business Act** (建設業法 Act No. 100 of 1949)
  - Article 3: General/Special construction license types (一般建設業・特定建設業)
  - Article 7: Capital requirements (¥5M for general, ¥20M for special)
  - Article 8: Qualified manager requirements (技術者要件)
  - 5-year license validity period
- **Real Estate Transactions Act** (宅地建物取引業法 Act No. 176 of 1952)
  - Article 35: Important matters explanation obligation (重要事項説明)
  - Article 46: Commission limits (3-5% tiered calculation)
  - Licensed agent validation (宅地建物取引士)
- 4 new modules (`construction_real_estate/`): error, types, validator, mod
- 18 comprehensive tests

#### Environmental Law (Phase 3)
- **Air Pollution Control Act** (大気汚染防止法 Act No. 97 of 1968)
  - Article 3: Emission standards for SOx, NOx, particulates
  - Article 6: Prior notification (60 days before factory installation)
  - Article 16: Monitoring obligations
- **Water Pollution Prevention Act** (水質汚濁防止法 Act No. 138 of 1970)
  - BOD/COD standards validation
- **Waste Management Act** (廃棄物処理法 Act No. 137 of 1970)
  - Article 7: Collection/transport business permit (5-year validity)
  - Article 8: Technical facility standards
  - Article 12-3: Manifest system for industrial waste (マニフェスト制度)
  - Article 14: Disposal business permit (7-year validity)
- 3 new modules (`environmental_law/`): error, types, validator
- 24 comprehensive tests

#### Personal Information Protection Act (Phase 4)
- **APPI** (個人情報の保護に関する法律 Act No. 57 of 2003, amended 2020/2022)
  - Article 15: Purpose specification at collection (利用目的の特定)
  - Article 17: Consent for sensitive data (要配慮個人情報の同意)
  - Article 20: Security management measures (安全管理措置)
  - Article 23: Third-party provision restrictions (第三者提供の制限)
  - Article 24: Cross-border transfer restrictions (越境移転)
  - Article 25: Record keeping (記録の作成等)
  - Articles 28-30: Data subject rights (開示・訂正・利用停止)
  - Article 35-2: Pseudonymous processing (仮名加工情報 - 2020 amendment)
  - Article 36: Anonymous processing (匿名加工情報)
- **AI Risk Assessment**: Data volume, automated decision-making, profiling evaluation
- **Business Type Classification**: Small/Standard/Large-scale business handling
- 3 new modules (`personal_info_protection/`): error, types, validator
- 28 comprehensive tests

#### Consumer Protection Law Enhancement (Phase 5)
- **E-Commerce Features**
  - Platform types: Direct store, Marketplace, Social commerce, Auction
  - Digital content types: Software, E-book, Music, Video, Game, Subscription
  - Payment methods: Credit card, Bank transfer, Mobile payment, etc.
- **Article 11 Validation**: Legal disclosure requirements (特定商取引法に基づく表記)
  - Business name, address, contact information
  - Return policy, delivery timeframe, payment methods disclosure
- **Return Policy Validation**: Consumer-friendly checks (7+ day period, shipping cost)
- **Subscription Services**: Billing cycles, cancellation notice, auto-renewal validation
- 2 new modules (`consumer_protection/`): ecommerce, ecommerce_validator
- 15 new tests (30 total in consumer_protection module)

### Changed
- Updated `README.md` to reflect all new v0.2.0 features
- Updated `TODO.md` with comprehensive v0.2.0 status tracking
- Enhanced module exports in `lib.rs` for all new law domains

### Statistics
- **Total Tests**: 398 (increased from 176 in v0.1.1)
- **Total Production Code**: ~27,600 lines (increased from ~13,400 lines)
- **Total Modules**: 22 modules across 10 law domains
- **Code Quality**: Zero clippy warnings, zero compiler warnings

---

## [0.1.1] - 2024

### Added
- Commercial Law (商法・会社法) - Companies Act and Commercial Code
- Labor Law (労働法) - Labor Standards Act and Labor Contract Act
- Intellectual Property Law (知的財産法) - Patent, Copyright, Trademark, Design Acts
- Consumer Protection Law (消費者保護法) - Consumer Contract Act and SCTA
- Case Law Database (判例データベース) - Court decision search and citation
- Contract Templates (契約テンプレート) - Automated contract generation
- Risk Analysis System (リスク分析システム) - Contract risk detection

### Statistics
- 176 tests
- ~13,400 lines of production code
- 10 working examples

---

## [0.1.0] - Initial Release

### Added
- Japanese Era (和暦) support - Meiji through Reiwa
- e-Gov XML law parser (法令XML解析)
- Civil Code (民法) - Articles 709, 710, 715 (Tort Law)
- Civil Code (民法) - Article 415 (Contract Law)
- Japanese Constitution (憲法) basic support

### Statistics
- Initial implementation
- Basic legal framework established

---

[0.2.0]: https://github.com/your-repo/legalis/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/your-repo/legalis/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/your-repo/legalis/releases/tag/v0.1.0
