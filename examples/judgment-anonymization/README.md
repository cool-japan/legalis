# Judgment Anonymization - Proof of Concept

**Status**: Experimental Research Prototype

This example demonstrates automated anonymization of Japanese court judgments by combining:

1. **[MeCrab](https://crates.io/crates/mecrab)**: High-performance Pure Rust morphological analyzer for named entity detection
2. **Legalis-RS**: Legal document structure parsing capabilities
3. **APPI Article 35-2**: Pseudonymization logic based on Personal Information Protection Act (仮名加工情報)

## Technical Approach

### Architecture

```
Court Judgment Text
       ↓
[MeCrab Morphological Analysis]
       ↓
Named Entity Detection (人名・組織名・地名)
       ↓
[Pseudonymization Logic]
       ↓
Anonymized Text (Person1, Company1, etc.)
```

### Named Entity Recognition

MeCrab identifies entities using part-of-speech tagging:

- **Person Names** (名詞-固有名詞-人名): `田中太郎` → `Person1`
- **Organizations** (名詞-固有名詞-組織): `株式会社ABC` → `Company1`
- **Places** (名詞-固有名詞-地域): `東京都渋谷区` → `Place1`

### Legal Compliance

The pseudonymization approach follows APPI Article 35-2 (Act on the Protection of Personal Information, 2020 amendment) which defines pseudonymous processed information (仮名加工情報) as personal information that:

1. Cannot identify specific individuals without additional information
2. Can be restored if combined with the anonymization key
3. Requires secure storage of the mapping table

## Usage

### Prerequisites

**IMPORTANT**: This example requires MeCab dictionary to be installed on your system. It is **not** included in automated tests due to this external dependency.

Install MeCab dictionary:

```bash
# macOS (Homebrew)
brew install mecab mecab-ipadic

# Ubuntu/Debian
sudo apt-get install mecab mecab-ipadic-utf8

# Fedora/RHEL
sudo yum install mecab mecab-ipadic

# Arch Linux
sudo pacman -S mecab mecab-ipadic

# Windows
# Download installer from: https://taku910.github.io/mecab/
```

**Dictionary Auto-detection**: The program automatically searches for dictionaries in:
- `/opt/homebrew/lib/mecab/dic/ipadic` (macOS ARM)
- `/usr/local/lib/mecab/dic/ipadic` (macOS Intel)
- `/var/lib/mecab/dic/ipadic` (Debian/Ubuntu)
- `/usr/share/mecab/dic/ipadic` (Other Linux)
- `C:\Program Files\MeCab\dic\ipadic` (Windows)

**Manual Override**: Set `MECAB_DICDIR` environment variable:
```bash
export MECAB_DICDIR=/path/to/mecab/dic/ipadic
cargo run --example judgment-anonymization
```

### Running

```bash
# From workspace root
cargo run --example judgment-anonymization

# Or from example directory
cd examples/judgment-anonymization
cargo run

# With custom dictionary path
MECAB_DICDIR=/custom/path cargo run
```

**Note**: This example is excluded from `cargo test --workspace` due to external MeCab dependency.

### Sample Output

**Original:**
```
原告田中太郎（以下「原告」という）は、被告山田花子（以下「被告」という）に対し、
令和5年3月15日、東京都渋谷区において、金500万円を貸し付けた。
```

**Anonymized:**
```
原告Person1（以下「原告」という）は、被告Person2（以下「被告」という）に対し、
令和5年3月15日、Place1において、金500万円を貸し付けた。
```

**Mapping:**
```
田中太郎 → Person1
山田花子 → Person2
東京都渋谷区 → Place1
```

## Implementation Details

### Key Features

1. **Consistent Mapping**: Same name always maps to same pseudonym
2. **Sort-by-Length**: Longer names processed first to avoid partial replacements
3. **Entity Type Awareness**: Different counters for persons/companies/places
4. **Embedded Fallback**: Works without external judgment file

### Code Structure

```
judgment-anonymization/
├── Cargo.toml                    # Dependencies: mecrab, legalis-core
├── src/
│   └── main.rs                   # Core anonymization logic
├── sample_judgments/
│   └── civil_case_01.txt         # Sample judgment (民事貸金返還請求)
└── README.md                     # This file
```

## Limitations (PoC Level)

This is a research prototype demonstrating technical feasibility. Production use requires addressing:

### 1. Named Entity Recognition Accuracy

- **Dictionary Dependency**: Unregistered names may be missed
- **Context Ambiguity**: "田中" could be a surname or place name
- **Compound Names**: Multi-token names (e.g., "最高裁判所裁判長") may be partially detected

### 2. Legal Document Structure

- **Format Variations**: Civil, criminal, and administrative judgments have different structures
- **Section Detection**: Current implementation does not parse judgment sections (主文/理由/当事者目録)
- **Pre-anonymized Text**: Cannot distinguish already pseudonymized entities ("甲", "A社")

### 3. Privacy Considerations

- **Mapping Security**: The anonymization mapping must be securely stored separately
- **Re-identification Risk**: Small datasets or unique case characteristics may allow re-identification
- **Legal Review Required**: Human review is necessary to ensure compliance

## Future Enhancements

To achieve production-level quality:

1. **Legalis-RS Integration**: Use legal document structure parser to identify sections
2. **Context-Aware NER**: Improve accuracy using sentence context and legal patterns
3. **Multi-format Support**: Handle PDF, DOCX, and various judgment formats
4. **Bidirectional Mapping**: Support both anonymization and de-anonymization workflows
5. **Batch Processing**: Process multiple judgments maintaining consistent mappings
6. **Legal Validation**: Add validation rules based on court disclosure guidelines

## Use Cases

### 1. Privacy-Preserving Legal Databases

Create publicly accessible judgment databases while protecting privacy:
- Academic research corpora
- Legal AI training datasets
- Case law search systems

### 2. Automated Redaction for Public Disclosure

Support court systems in preparing judgments for public disclosure under transparency laws.

### 3. Legal Tech Applications

- Contract analysis platforms
- Due diligence systems
- Litigation prediction models

## Technical Specifications

- **Language**: Rust 2024
- **Dependencies**:
  - `mecrab = "0.1"` - Morphological analysis
  - `legalis-core` - Core legal types
  - `legalis-jp` - Japanese jurisdiction support
- **Lines of Code**: ~200 LoC
- **Test Coverage**: Basic unit tests for mapping consistency

## References

### Legal Framework

- [Act on the Protection of Personal Information](https://www.ppc.go.jp/en/legal/) (個人情報の保護に関する法律)
  - Article 35-2: Pseudonymous Processed Information (仮名加工情報)
  - Article 36: Anonymous Processed Information (匿名加工情報)

### Technical Background

- [MeCrab Documentation](https://docs.rs/mecrab)
- [Legalis-RS Documentation](../../README.md)
- [Japanese Morphological Analysis](https://taku910.github.io/mecab/)

## License

MIT OR Apache-2.0

Copyright 2026 COOLJAPAN OU (Team Kitasan)

---

**Disclaimer**: This is an experimental research prototype. It demonstrates technical feasibility but is NOT intended for production use without significant enhancements, testing, and legal review.
