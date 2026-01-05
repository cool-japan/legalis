# legalis-lod

Linked Open Data (RDF/TTL/JSON-LD) export for Legalis-RS.

## Overview

`legalis-lod` enables exporting legal statutes to semantic web formats, making them compatible with Linked Open Data ecosystems and enabling machine-readable legal knowledge graphs.

## Features

- **Multiple RDF Formats**: Turtle, N-Triples, RDF/XML, JSON-LD
- **Standard Vocabularies**: ELI, Dublin Core, FRBR
- **Semantic Web Compatible**: Interoperable with legal ontologies
- **Configurable Namespaces**: Custom URI patterns

## Supported Formats

| Format | Extension | Use Case |
|--------|-----------|----------|
| Turtle | `.ttl` | Human-readable RDF |
| N-Triples | `.nt` | Line-based processing |
| RDF/XML | `.rdf` | XML-based exchange |
| JSON-LD | `.jsonld` | Web APIs, JavaScript |

## Usage

```rust
use legalis_lod::{LodExporter, RdfFormat, ExportConfig};
use legalis_core::Statute;

let statute = /* ... */;

// Create exporter with custom base URI
let config = ExportConfig::new()
    .with_base_uri("https://example.org/laws/")
    .with_format(RdfFormat::Turtle);

let exporter = LodExporter::new(config);
let turtle = exporter.export(&statute)?;

// Export as JSON-LD for web APIs
let jsonld = exporter.export_jsonld(&statute)?;
```

## Output Example (Turtle)

```turtle
@prefix eli: <http://data.europa.eu/eli/ontology#> .
@prefix dct: <http://purl.org/dc/terms/> .

<https://example.org/laws/adult-rights>
    a eli:LegalResource ;
    eli:id_local "adult-rights" ;
    dct:title "Adult Rights" ;
    eli:jurisdiction "US" ;
    eli:version "1" .
```

## Vocabularies

| Prefix | Namespace | Description |
|--------|-----------|-------------|
| `eli` | European Legislation Identifier | Legal resource metadata |
| `dct` | Dublin Core Terms | General metadata |
| `frbr` | FRBR | Work/Expression/Manifestation |
| `skos` | SKOS | Concept schemes |

## CLI Integration

```bash
# Export to Turtle
legalis lod input.legalis --format turtle

# Export to JSON-LD with custom base URI
legalis lod input.legalis --format jsonld --base-uri https://example.org/

# Export multiple statutes
legalis lod *.legalis --format ntriples > laws.nt
```

## License

MIT OR Apache-2.0
