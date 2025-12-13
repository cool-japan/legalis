# legalis-interop

Interoperability layer for legal DSL formats.

## Overview

`legalis-interop` enables Legalis-RS to import from and export to other legal DSL formats, making it a universal bridge between legal technology ecosystems.

## Supported Formats

### Catala (Inria, France)
- Literate programming style for legal specifications
- Scope and context model support
- Bidirectional conversion

### Stipula (University of Bologna)
- Smart legal contract language
- Party/asset model mapping
- State machine conversion

### L4 / SLL (Singapore)
- Deontic logic (MUST, MAY, SHANT)
- Rule-based reasoning model
- Decision table support

### Akoma Ntoso (OASIS Standard)
- XML-based legal document standard
- Full metadata preservation
- International legal document exchange

## Usage

```rust
use legalis_interop::{
    CatalaParser, CatalaExporter,
    StipulaParser, StipulaExporter,
    L4Parser, L4Exporter,
    AkomaNtosoParser, AkomaNtosoExporter,
    ConversionReport,
};
use legalis_core::Statute;

// Import from Catala
let catala_source = std::fs::read_to_string("law.catala_en")?;
let parser = CatalaParser::new();
let (statute, report) = parser.parse(&catala_source)?;

println!("Conversion confidence: {}%", report.confidence);

// Export to L4
let exporter = L4Exporter::new();
let l4_output = exporter.export(&statute)?;

// Direct format-to-format conversion
use legalis_interop::convert;
let l4_output = convert(&catala_source, "catala", "l4")?;
```

## CLI Integration

```bash
# Import from external format
legalis import --from catala input.catala_en

# Export to external format
legalis convert input.legalis --from legalis --to l4

# Auto-detect format
legalis import input.stipula  # Detected from extension
```

## Conversion Features

| Feature | Catala | Stipula | L4 | Akoma Ntoso |
|---------|--------|---------|----|----|
| Import | ✓ | ✓ | ✓ | ✓ |
| Export | ✓ | ✓ | ✓ | ✓ |
| Metadata | ✓ | Partial | ✓ | ✓ |
| Conditions | ✓ | ✓ | ✓ | ✓ |
| Effects | ✓ | ✓ | ✓ | ✓ |
| Temporal | Partial | ✓ | Partial | ✓ |

## Conversion Reports

Each conversion generates a report with:
- Confidence score (0-100%)
- Features preserved
- Features lost or approximated
- Warnings and suggestions

## License

MIT OR Apache-2.0
