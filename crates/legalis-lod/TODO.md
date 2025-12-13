# legalis-lod TODO

## Completed

- [x] RDF serialization with multiple formats (Turtle, N-Triples, RDF/XML, JSON-LD)
- [x] ELI vocabulary support
- [x] Dublin Core metadata
- [x] Custom Legalis ontology
- [x] Namespace management

## Formats

- [ ] Add SPARQL query generation for exported data
- [ ] Implement streaming serialization for large datasets
- [ ] Add RDFa output for HTML embedding
- [ ] Support TriG format for named graphs

## Ontologies

### Standard Vocabularies
- [ ] Add FRBR-aligned Bibliographic Ontology (FaBiO)
- [ ] Implement LKIF-Core (Legal Knowledge Interchange Format)
- [ ] Add LegalRuleML ontology mapping
- [ ] Support Akoma Ntoso ontology
- [ ] Add SKOS for concept hierarchies

### Custom Extensions
- [ ] Create Legalis ontology specification document
- [ ] Add condition/effect relationship properties
- [ ] Define discretion zone modeling
- [ ] Create simulation result vocabulary

## Linked Data

- [ ] Add URI dereferencing support
- [ ] Implement content negotiation
- [ ] Create VOID dataset descriptions
- [ ] Add provenance tracking (PROV-O)
- [ ] Implement license metadata (CC, etc.)

## Integration

### SPARQL Endpoints
- [ ] Add SPARQL endpoint integration
- [ ] Implement federated query support
- [ ] Create SPARQL CONSTRUCT templates
- [ ] Add graph store protocol support

### External Services
- [ ] Integration with EUR-Lex
- [ ] Integration with legislation.gov.uk
- [ ] Support for Wikidata linking
- [ ] Add DBpedia concept mapping

## Validation

- [ ] Add SHACL shape generation
- [ ] Implement ShEx validation
- [ ] Create RDF validation reports
- [ ] Add ontology consistency checking

## Performance

- [ ] Optimize large statute serialization
- [ ] Add batch export support
- [ ] Implement incremental graph updates
- [ ] Create export caching

## Testing

- [ ] Add RDF validation tests
- [ ] Create round-trip conversion tests
- [ ] Test all output formats
- [ ] Benchmark serialization performance
