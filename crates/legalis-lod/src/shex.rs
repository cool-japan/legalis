//! ShEx (Shape Expressions) schema generation.
//!
//! This module provides utilities to generate ShEx schemas for validating
//! legal statute RDF data. ShEx is an alternative to SHACL for describing
//! RDF graph structures.

use crate::Namespaces;

/// ShEx schema generator.
#[derive(Debug)]
pub struct ShexSchemaGenerator {
    namespaces: Namespaces,
}

impl Default for ShexSchemaGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ShexSchemaGenerator {
    /// Creates a new ShEx schema generator.
    pub fn new() -> Self {
        Self {
            namespaces: Namespaces::default(),
        }
    }

    /// Creates a new ShEx schema generator with custom namespaces.
    pub fn with_namespaces(namespaces: Namespaces) -> Self {
        Self { namespaces }
    }

    /// Generates ShEx schema for Legalis statute validation.
    pub fn generate_statute_schema(&self) -> String {
        let mut schema = String::new();

        // Prefixes
        for (prefix, uri) in Namespaces::standard_prefixes() {
            schema.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        schema.push_str(&format!("BASE <{}>\n\n", self.namespaces.base));

        // Statute shape
        schema.push_str(&self.create_statute_shape());
        schema.push('\n');

        // Effect shape
        schema.push_str(&self.create_effect_shape());
        schema.push('\n');

        // Condition shapes
        schema.push_str(&self.create_condition_shapes());

        schema
    }

    /// Creates the Statute shape in ShEx.
    fn create_statute_shape(&self) -> String {
        let mut shape = String::new();

        shape.push_str("<StatuteShape> {\n");
        shape.push_str("  a [ eli:LegalResource legalis:Statute ] ;\n");
        shape.push_str("  eli:title xsd:string ;\n");
        shape.push_str("  dcterms:title xsd:string ? ;\n");
        shape.push_str("  dcterms:identifier xsd:string ;\n");
        shape.push_str("  legalis:hasEffect @<EffectShape> ;\n");
        shape.push_str("  legalis:hasPrecondition @<ConditionShape> * ;\n");
        shape.push_str("  eli:jurisdiction xsd:string ? ;\n");
        shape.push_str("  eli:version xsd:integer ? ;\n");
        shape.push_str("  eli:date_document xsd:date ? ;\n");
        shape.push_str("  legalis:expiryDate xsd:date ? ;\n");
        shape.push_str("  legalis:hasDiscretion xsd:boolean ? ;\n");
        shape.push_str("  prov:wasAttributedTo IRI ? ;\n");
        shape.push_str("  prov:wasGeneratedBy IRI ? ;\n");
        shape.push_str("  prov:generatedAtTime xsd:dateTime ? ;\n");
        shape.push_str("  dcterms:license IRI ? ;\n");
        shape.push_str("  dcterms:subject IRI *\n");
        shape.push_str("}\n");

        shape
    }

    /// Creates the Effect shape in ShEx.
    fn create_effect_shape(&self) -> String {
        let mut shape = String::new();

        shape.push_str("<EffectShape> {\n");
        shape.push_str("  a [ legalis:Effect ] ;\n");
        shape.push_str("  legalis:effectType [\n");
        shape.push_str("    legalis:GrantEffect\n");
        shape.push_str("    legalis:RevokeEffect\n");
        shape.push_str("    legalis:MonetaryTransferEffect\n");
        shape.push_str("    legalis:ObligationEffect\n");
        shape.push_str("    legalis:ProhibitionEffect\n");
        shape.push_str("    legalis:StatusChangeEffect\n");
        shape.push_str("    legalis:CustomEffect\n");
        shape.push_str("  ] ;\n");
        shape.push_str("  rdfs:label xsd:string\n");
        shape.push_str("}\n");

        shape
    }

    /// Creates the Condition shapes in ShEx.
    fn create_condition_shapes(&self) -> String {
        let mut shapes = String::new();

        // Generic Condition shape
        shapes.push_str("<ConditionShape> @<AgeConditionShape>\n");
        shapes.push_str("  OR @<IncomeConditionShape>\n");
        shapes.push_str("  OR @<AttributeConditionShape>\n");
        shapes.push_str("  OR @<AndConditionShape>\n");
        shapes.push_str("  OR @<OrConditionShape>\n");
        shapes.push_str("  OR @<NotConditionShape>\n\n");

        // Age Condition
        shapes.push_str("<AgeConditionShape> {\n");
        shapes.push_str("  a [ legalis:Condition legalis:AgeCondition ] ;\n");
        shapes.push_str("  legalis:operator [\n");
        shapes.push_str("    legalis:Equal\n");
        shapes.push_str("    legalis:NotEqual\n");
        shapes.push_str("    legalis:GreaterThan\n");
        shapes.push_str("    legalis:GreaterOrEqual\n");
        shapes.push_str("    legalis:LessThan\n");
        shapes.push_str("    legalis:LessOrEqual\n");
        shapes.push_str("  ] ;\n");
        shapes.push_str("  legalis:value xsd:integer MinInclusive 0\n");
        shapes.push_str("}\n\n");

        // Income Condition
        shapes.push_str("<IncomeConditionShape> {\n");
        shapes.push_str("  a [ legalis:Condition legalis:IncomeCondition ] ;\n");
        shapes.push_str("  legalis:operator [\n");
        shapes.push_str("    legalis:Equal\n");
        shapes.push_str("    legalis:NotEqual\n");
        shapes.push_str("    legalis:GreaterThan\n");
        shapes.push_str("    legalis:GreaterOrEqual\n");
        shapes.push_str("    legalis:LessThan\n");
        shapes.push_str("    legalis:LessOrEqual\n");
        shapes.push_str("  ] ;\n");
        shapes.push_str("  legalis:value xsd:integer MinInclusive 0\n");
        shapes.push_str("}\n\n");

        // Attribute Condition
        shapes.push_str("<AttributeConditionShape> {\n");
        shapes.push_str("  a [ legalis:Condition legalis:AttributeCondition ] ;\n");
        shapes.push_str("  legalis:attributeKey xsd:string\n");
        shapes.push_str("}\n\n");

        // And Condition
        shapes.push_str("<AndConditionShape> {\n");
        shapes.push_str("  a [ legalis:Condition legalis:AndCondition ] ;\n");
        shapes.push_str("  legalis:leftOperand @<ConditionShape> ;\n");
        shapes.push_str("  legalis:rightOperand @<ConditionShape>\n");
        shapes.push_str("}\n\n");

        // Or Condition
        shapes.push_str("<OrConditionShape> {\n");
        shapes.push_str("  a [ legalis:Condition legalis:OrCondition ] ;\n");
        shapes.push_str("  legalis:leftOperand @<ConditionShape> ;\n");
        shapes.push_str("  legalis:rightOperand @<ConditionShape>\n");
        shapes.push_str("}\n\n");

        // Not Condition
        shapes.push_str("<NotConditionShape> {\n");
        shapes.push_str("  a [ legalis:Condition legalis:NotCondition ] ;\n");
        shapes.push_str("  legalis:operand @<ConditionShape>\n");
        shapes.push_str("}\n\n");

        shapes
    }

    /// Generates ShEx schema for SKOS concepts.
    pub fn generate_skos_schema(&self) -> String {
        let mut schema = String::new();

        schema.push_str("<ConceptSchemeShape> {\n");
        schema.push_str("  a [ skos:ConceptScheme ] ;\n");
        schema.push_str("  skos:prefLabel rdf:langString + ;\n");
        schema.push_str("  dcterms:title rdf:langString + ;\n");
        schema.push_str("  skos:hasTopConcept @<ConceptShape> *\n");
        schema.push_str("}\n\n");

        schema.push_str("<ConceptShape> {\n");
        schema.push_str("  a [ skos:Concept ] ;\n");
        schema.push_str("  skos:prefLabel rdf:langString + ;\n");
        schema.push_str("  skos:altLabel rdf:langString * ;\n");
        schema.push_str("  skos:definition rdf:langString ? ;\n");
        schema.push_str("  skos:inScheme IRI ;\n");
        schema.push_str("  skos:broader @<ConceptShape> * ;\n");
        schema.push_str("  skos:narrower @<ConceptShape> * ;\n");
        schema.push_str("  skos:related @<ConceptShape> *\n");
        schema.push_str("}\n");

        schema
    }

    /// Exports the complete ShEx schema with both statute and SKOS schemas.
    pub fn export_complete_schema(&self) -> String {
        let mut schema = String::new();

        // Prefixes
        for (prefix, uri) in Namespaces::standard_prefixes() {
            schema.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        schema.push_str(&format!("BASE <{}>\n\n", self.namespaces.base));

        // Statute schema
        schema.push_str("# Statute Shapes\n");
        schema.push_str(&self.create_statute_shape());
        schema.push('\n');
        schema.push_str(&self.create_effect_shape());
        schema.push('\n');
        schema.push_str(&self.create_condition_shapes());

        // SKOS schema
        schema.push_str("\n# SKOS Shapes\n");
        schema.push_str(&self.generate_skos_schema());

        // Provenance schema
        schema.push_str("\n# Provenance Shapes\n");
        schema.push_str(&self.create_provenance_shapes());

        // License schema
        schema.push_str("\n# License Shapes\n");
        schema.push_str(&self.create_license_shapes());

        schema
    }

    /// Creates provenance validation shapes.
    fn create_provenance_shapes(&self) -> String {
        let mut shapes = String::new();

        shapes.push_str("<ProvenanceShape> {\n");
        shapes.push_str("  prov:wasGeneratedBy IRI ? ;\n");
        shapes.push_str("  prov:wasAttributedTo IRI ? ;\n");
        shapes.push_str("  prov:generatedAtTime xsd:dateTime ? ;\n");
        shapes.push_str("  prov:wasDerivedFrom IRI ? ;\n");
        shapes.push_str("  dcterms:creator xsd:string ?\n");
        shapes.push_str("}\n\n");

        shapes.push_str("<ActivityShape> {\n");
        shapes.push_str("  a [ prov:Activity ] ;\n");
        shapes.push_str("  rdfs:label xsd:string ;\n");
        shapes.push_str("  prov:startedAtTime xsd:dateTime ? ;\n");
        shapes.push_str("  prov:endedAtTime xsd:dateTime ?\n");
        shapes.push_str("}\n\n");

        shapes.push_str("<AgentShape> {\n");
        shapes.push_str("  a [ prov:Agent prov:Person prov:Organization ] ;\n");
        shapes.push_str("  rdfs:label xsd:string\n");
        shapes.push_str("}\n");

        shapes
    }

    /// Creates license validation shapes.
    fn create_license_shapes(&self) -> String {
        let mut shapes = String::new();

        shapes.push_str("<LicenseShape> {\n");
        shapes.push_str("  a [ cc:License dcterms:LicenseDocument ] ;\n");
        shapes.push_str("  rdfs:label xsd:string ? ;\n");
        shapes.push_str("  dcterms:title rdf:langString *\n");
        shapes.push_str("}\n\n");

        shapes.push_str("<LicensedResourceShape> {\n");
        shapes.push_str("  dcterms:license IRI ;\n");
        shapes.push_str("  cc:license IRI ? ;\n");
        shapes.push_str("  dcterms:rightsHolder xsd:string ?\n");
        shapes.push_str("}\n");

        shapes
    }

    /// Generates temporal validation shape.
    pub fn generate_temporal_shape(&self) -> String {
        let mut shape = String::new();

        shape.push_str("<TemporalValidityShape> {\n");
        shape.push_str("  eli:date_document xsd:date ;\n");
        shape.push_str("  legalis:expiryDate xsd:date ? ;\n");
        shape.push_str("  eli:in_force xsd:boolean ?\n");
        shape.push_str("}\n");

        shape
    }

    /// Generates jurisdiction validation shape.
    pub fn generate_jurisdiction_shape(&self) -> String {
        let mut shape = String::new();

        shape.push_str("<JurisdictionShape> {\n");
        shape.push_str("  eli:jurisdiction xsd:string ;\n");
        shape.push_str("  eli:jurisdiction_level [\n");
        shape.push_str("    \"national\" \"regional\" \"local\" \"international\"\n");
        shape.push_str("  ] ?\n");
        shape.push_str("}\n");

        shape
    }

    /// Generates versioning validation shape.
    pub fn generate_versioning_shape(&self) -> String {
        let mut shape = String::new();

        shape.push_str("<VersionShape> {\n");
        shape.push_str("  eli:version xsd:integer MinInclusive 0 ;\n");
        shape.push_str("  dcterms:hasVersion IRI * ;\n");
        shape.push_str("  dcterms:isVersionOf IRI ? ;\n");
        shape.push_str("  prov:wasRevisionOf IRI ?\n");
        shape.push_str("}\n");

        shape
    }

    /// Generates shape for linked data relationships.
    pub fn generate_linked_data_shape(&self) -> String {
        let mut shape = String::new();

        shape.push_str("<LinkedDataShape> {\n");
        shape.push_str("  owl:sameAs IRI * ;\n");
        shape.push_str("  rdfs:seeAlso IRI * ;\n");
        shape.push_str("  dcterms:relation IRI * ;\n");
        shape.push_str("  dcterms:references IRI * ;\n");
        shape.push_str("  dcterms:isReferencedBy IRI *\n");
        shape.push_str("}\n");

        shape
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_statute_schema() {
        let generator = ShexSchemaGenerator::new();
        let schema = generator.generate_statute_schema();

        assert!(schema.contains("PREFIX eli:"));
        assert!(schema.contains("<StatuteShape>"));
        assert!(schema.contains("<EffectShape>"));
    }

    #[test]
    fn test_statute_shape() {
        let generator = ShexSchemaGenerator::new();
        let shape = generator.create_statute_shape();

        assert!(shape.contains("eli:title"));
        assert!(shape.contains("dcterms:identifier"));
        assert!(shape.contains("legalis:hasEffect"));
    }

    #[test]
    fn test_effect_shape() {
        let generator = ShexSchemaGenerator::new();
        let shape = generator.create_effect_shape();

        assert!(shape.contains("<EffectShape>"));
        assert!(shape.contains("legalis:effectType"));
        assert!(shape.contains("legalis:GrantEffect"));
    }

    #[test]
    fn test_condition_shapes() {
        let generator = ShexSchemaGenerator::new();
        let shapes = generator.create_condition_shapes();

        assert!(shapes.contains("<AgeConditionShape>"));
        assert!(shapes.contains("<IncomeConditionShape>"));
        assert!(shapes.contains("<AndConditionShape>"));
        assert!(shapes.contains("MinInclusive 0"));
    }

    #[test]
    fn test_skos_schema() {
        let generator = ShexSchemaGenerator::new();
        let schema = generator.generate_skos_schema();

        assert!(schema.contains("<ConceptSchemeShape>"));
        assert!(schema.contains("<ConceptShape>"));
        assert!(schema.contains("skos:prefLabel"));
    }

    #[test]
    fn test_complete_schema() {
        let generator = ShexSchemaGenerator::new();
        let schema = generator.export_complete_schema();

        assert!(schema.contains("PREFIX"));
        assert!(schema.contains("<StatuteShape>"));
        assert!(schema.contains("<ConceptShape>"));
        assert!(schema.contains("<ProvenanceShape>"));
        assert!(schema.contains("<LicenseShape>"));
    }

    #[test]
    fn test_provenance_shapes() {
        let generator = ShexSchemaGenerator::new();
        let shapes = generator.create_provenance_shapes();

        assert!(shapes.contains("<ProvenanceShape>"));
        assert!(shapes.contains("<ActivityShape>"));
        assert!(shapes.contains("<AgentShape>"));
        assert!(shapes.contains("prov:wasGeneratedBy"));
        assert!(shapes.contains("prov:wasAttributedTo"));
    }

    #[test]
    fn test_license_shapes() {
        let generator = ShexSchemaGenerator::new();
        let shapes = generator.create_license_shapes();

        assert!(shapes.contains("<LicenseShape>"));
        assert!(shapes.contains("<LicensedResourceShape>"));
        assert!(shapes.contains("dcterms:license"));
        assert!(shapes.contains("cc:license"));
    }

    #[test]
    fn test_temporal_shape() {
        let generator = ShexSchemaGenerator::new();
        let shape = generator.generate_temporal_shape();

        assert!(shape.contains("<TemporalValidityShape>"));
        assert!(shape.contains("eli:date_document"));
        assert!(shape.contains("legalis:expiryDate"));
        assert!(shape.contains("eli:in_force"));
    }

    #[test]
    fn test_jurisdiction_shape() {
        let generator = ShexSchemaGenerator::new();
        let shape = generator.generate_jurisdiction_shape();

        assert!(shape.contains("<JurisdictionShape>"));
        assert!(shape.contains("eli:jurisdiction"));
        assert!(shape.contains("national"));
        assert!(shape.contains("regional"));
    }

    #[test]
    fn test_versioning_shape() {
        let generator = ShexSchemaGenerator::new();
        let shape = generator.generate_versioning_shape();

        assert!(shape.contains("<VersionShape>"));
        assert!(shape.contains("eli:version"));
        assert!(shape.contains("dcterms:hasVersion"));
        assert!(shape.contains("prov:wasRevisionOf"));
        assert!(shape.contains("MinInclusive 0"));
    }

    #[test]
    fn test_linked_data_shape() {
        let generator = ShexSchemaGenerator::new();
        let shape = generator.generate_linked_data_shape();

        assert!(shape.contains("<LinkedDataShape>"));
        assert!(shape.contains("owl:sameAs"));
        assert!(shape.contains("rdfs:seeAlso"));
        assert!(shape.contains("dcterms:relation"));
    }
}
