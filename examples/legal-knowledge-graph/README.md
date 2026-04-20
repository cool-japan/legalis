# legal-knowledge-graph

This example exports Legalis-RS statutes to Linked Open Data formats for semantic web integration using `legalis-lod`. Three sample statutes (voting rights, data protection, senior benefits) are serialised into Turtle (TTL), JSON-LD, N-Triples, RDF/XML, and TriG with named graphs. The example also demonstrates SPARQL query generation via `SparqlQueryBuilder` and `SparqlTemplates`, and attaches ELI, FaBiO, LKIF-Core, and PROV-O ontology metadata including licence and provenance information.

## Usage

```sh
cargo run -p legal-knowledge-graph --all-features
```

## What It Demonstrates

- Five RDF serialisation formats: Turtle, JSON-LD, N-Triples, RDF/XML, TriG
- ELI (European Legislation Identifier) ontology alignment
- LKIF-Core legal knowledge interchange format
- PROV-O provenance and licence metadata attachment
- SPARQL query template generation for statute retrieval
- Linked Open Data patterns for open legal data publishing

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
