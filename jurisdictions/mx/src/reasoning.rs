//! Legal reasoning and interpretation for Mexican law

use serde::{Deserialize, Serialize};

/// Legal interpretation methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpretationMethod {
    /// Literal interpretation (Interpretación literal)
    Literal,
    /// Systematic interpretation (Interpretación sistemática)
    Systematic,
    /// Historical interpretation (Interpretación histórica)
    Historical,
    /// Teleological interpretation (Interpretación teleológica)
    Teleological,
}

/// Legal reasoning structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalReasoning {
    /// Facts of the case
    pub hechos: Vec<String>,
    /// Applicable legal norms
    pub normas_aplicables: Vec<String>,
    /// Legal interpretation method used
    pub metodo_interpretacion: InterpretationMethod,
    /// Reasoning steps
    pub razonamiento: Vec<String>,
    /// Conclusion
    pub conclusion: String,
}

/// Legal principle (Principio jurídico)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalPrinciple {
    /// Principle name
    pub nombre: String,
    /// Principle description
    pub descripcion: String,
    /// Source (constitution, law, etc.)
    pub fuente: String,
}

/// Common legal principles in Mexican law
pub mod principles {
    use super::*;

    /// Principle of legality (Principio de legalidad)
    pub fn legality() -> LegalPrinciple {
        LegalPrinciple {
            nombre: "Principio de legalidad".to_string(),
            descripcion: "No puede haber delito sin ley previa".to_string(),
            fuente: "Constitución Política, Artículo 14".to_string(),
        }
    }

    /// Principle of legal certainty (Principio de seguridad jurídica)
    pub fn legal_certainty() -> LegalPrinciple {
        LegalPrinciple {
            nombre: "Principio de seguridad jurídica".to_string(),
            descripcion: "Garantía de conocer las leyes aplicables".to_string(),
            fuente: "Constitución Política, Artículo 16".to_string(),
        }
    }

    /// Principle of due process (Principio de debido proceso)
    pub fn due_process() -> LegalPrinciple {
        LegalPrinciple {
            nombre: "Principio de debido proceso".to_string(),
            descripcion: "Garantías procesales en procedimientos judiciales".to_string(),
            fuente: "Constitución Política, Artículo 14".to_string(),
        }
    }
}

impl LegalReasoning {
    /// Create new legal reasoning
    pub fn new(
        hechos: Vec<String>,
        normas_aplicables: Vec<String>,
        metodo_interpretacion: InterpretationMethod,
    ) -> Self {
        Self {
            hechos,
            normas_aplicables,
            metodo_interpretacion,
            razonamiento: Vec::new(),
            conclusion: String::new(),
        }
    }

    /// Add reasoning step
    pub fn add_step(&mut self, step: String) {
        self.razonamiento.push(step);
    }

    /// Set conclusion
    pub fn set_conclusion(&mut self, conclusion: String) {
        self.conclusion = conclusion;
    }

    /// Check if reasoning is complete
    pub fn is_complete(&self) -> bool {
        !self.hechos.is_empty()
            && !self.normas_aplicables.is_empty()
            && !self.razonamiento.is_empty()
            && !self.conclusion.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_reasoning() {
        let mut reasoning = LegalReasoning::new(
            vec!["Hecho 1".to_string(), "Hecho 2".to_string()],
            vec!["Artículo 1792 CCF".to_string()],
            InterpretationMethod::Literal,
        );

        reasoning.add_step("Análisis del contrato".to_string());
        reasoning.set_conclusion("El contrato es válido".to_string());

        assert!(reasoning.is_complete());
    }

    #[test]
    fn test_legal_principles() {
        let legality = principles::legality();
        assert_eq!(legality.nombre, "Principio de legalidad");

        let due_process = principles::due_process();
        assert_eq!(due_process.nombre, "Principio de debido proceso");
    }
}
