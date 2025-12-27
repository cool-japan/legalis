# legalis-lod TODO

## Completed

- [x] RDF serialization with multiple formats (Turtle, N-Triples, RDF/XML, JSON-LD)
- [x] ELI vocabulary support
- [x] Dublin Core metadata
- [x] Custom Legalis ontology
- [x] Namespace management

## Formats

- [x] Add SPARQL query generation for exported data
- [x] Implement streaming serialization for large datasets
- [x] Add RDFa output for HTML embedding
- [x] Support TriG format for named graphs

## Ontologies

### Standard Vocabularies
- [x] Add FRBR-aligned Bibliographic Ontology (FaBiO)
- [x] Implement LKIF-Core (Legal Knowledge Interchange Format)
- [x] Add LegalRuleML ontology mapping
- [x] Support Akoma Ntoso ontology
- [x] Add SKOS for concept hierarchies

### Custom Extensions
- [ ] Create Legalis ontology specification document
- [ ] Add condition/effect relationship properties
- [ ] Define discretion zone modeling
- [ ] Create simulation result vocabulary

## Linked Data

- [x] Add URI dereferencing support
- [x] Implement content negotiation
- [x] Create VOID dataset descriptions
- [x] Add provenance tracking (PROV-O)
- [x] Implement license metadata (CC, etc.)
- [x] Add Cool URIs for legal resources
- [x] Add owl:sameAs linking for entity resolution
- [x] Add rdfs:seeAlso for related resources

## Integration

### SPARQL Endpoints
- [x] Add SPARQL endpoint framework
- [x] Implement federated query support
- [x] Create SPARQL CONSTRUCT templates
- [x] Add graph store protocol support
- [x] Add named graph management

### External Services
- [ ] Integration with EUR-Lex
- [ ] Integration with legislation.gov.uk
- [ ] Support for Wikidata linking
- [ ] Add DBpedia concept mapping

## Validation

- [x] Add SHACL shape generation
- [x] Implement ShEx validation
- [x] Create RDF validation reports
- [x] Add ontology consistency checking

## Performance

- [x] Optimize large statute serialization
- [x] Add batch export support
- [x] Implement incremental graph updates
- [x] Create export caching

## Testing

- [x] Add RDF validation tests
- [x] Create round-trip conversion tests
- [x] Test all output formats
- [x] Benchmark serialization performance

## Roadmap for 0.1.0 Series

### Ontology Extensions (v0.1.1)
- [x] Add FRBR-aligned Bibliographic Ontology (FaBiO)
- [x] Add LKIF-Core (Legal Knowledge Interchange Format)
- [x] Add LegalRuleML ontology mapping
- [x] Add Akoma Ntoso ontology
- [ ] Add custom ontology definition DSL

### Linked Data Features (v0.1.2)
- [x] Add URI dereferencing support
- [x] Add Cool URIs for legal resources
- [x] Add owl:sameAs linking for entity resolution
- [x] Add rdfs:seeAlso for related resources
- [x] Add link validation for dead references (stub implementation)

### SPARQL Enhancements (v0.1.3)
- [x] Add SPARQL endpoint framework
- [x] Add federated query support
- [x] Add graph store protocol (SPARQL 1.1 Update)
- [x] Add named graph management
- [x] Expand SPARQL query templates library

### External Integrations (v0.1.4)
- [ ] Add EUR-Lex integration
- [ ] Add legislation.gov.uk integration
- [ ] Add Wikidata linking
- [ ] Add DBpedia concept mapping
- [ ] Add GovTrack.us integration (US legislation)

### Knowledge Graph (v0.1.5)
- [ ] Add statute knowledge graph construction
- [ ] Add entity relationship extraction
- [ ] Add temporal knowledge graph support
- [ ] Add graph reasoning (inference)
- [ ] Add graph visualization export

### Validation & Quality (v0.1.6)
- [ ] Add comprehensive SHACL shape library
- [ ] Add ShEx validation rules
- [ ] Add data quality metrics
- [ ] Add completeness analysis
- [ ] Add consistency checking

### Performance (v0.1.7)
- [ ] Add in-memory RDF store
- [ ] Add persistent triple store integration
- [ ] Add query result caching
- [ ] Add incremental graph updates
- [ ] Add graph partitioning for large datasets

### Reasoning (v0.1.8)
- [ ] Add OWL 2 RL reasoning
- [ ] Add rule-based inference
- [ ] Add temporal reasoning
- [ ] Add legal-specific inference rules
- [ ] Add explanation generation for inferences

### Export & Publishing (v0.1.9)
- [ ] Add dataset publishing workflow
- [ ] Add DCAT catalog generation
- [ ] Add data.gov integration
- [ ] Add dataset versioning
- [ ] Add change notification (Linked Data Notifications)
