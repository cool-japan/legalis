# Patterns d'API et Bonnes Pratiques

Apprenez à utiliser efficacement l'API legalis-fr avec des patterns éprouvés et des bonnes pratiques.

## Table des Matières

1. [Pattern Builder](#1-pattern-builder)
2. [Gestion des Erreurs](#2-gestion-des-erreurs)
3. [Patterns de Validation](#3-patterns-de-validation)
4. [Sérialisation](#4-sérialisation)
5. [Validateurs Personnalisés](#5-validateurs-personnalisés)
6. [Patterns d'Intégration](#6-patterns-dintégration)
7. [Optimisation des Performances](#7-optimisation-des-performances)
8. [Patterns de Test](#8-patterns-de-test)

---

## 1. Pattern Builder

Tous les types majeurs dans legalis-fr utilisent le pattern builder pour une construction sûre et ergonomique.

### 1.1 Utilisation Basique du Builder

```rust
use legalis_fr::contract::{Contract, ContractType};
use chrono::NaiveDate;

// API fluide et chaînable
let contrat = Contract::builder()
    .contract_type(ContractType::Sale)
    .parties(vec!["Alice".to_string(), "Bob".to_string()])
    .object("Appartement")
    .price(450_000)
    .formation_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .build()?;  // Retourne Result<Contract, ContractLawError>
```

### 1.2 Champs Optionnels

Les builders gèrent les champs optionnels avec élégance :

```rust
use legalis_fr::labor::Employment;

// Avec période d'essai optionnelle
let emploi1 = Employment::builder()
    .employee_name("Marie")
    .probation_period_months(Some(3))
    .build()?;

// Sans période d'essai
let emploi2 = Employment::builder()
    .employee_name("Pierre")
    .probation_period_months(None)
    .build()?;

// Ou simplement omettre les champs optionnels
let emploi3 = Employment::builder()
    .employee_name("Sophie")
    .build()?;
```

### 1.3 Validation dans les Builders

Les builders effectuent automatiquement la validation :

```rust
use legalis_fr::family::Marriage;

// Cela échouera au moment de build
let resultat = Marriage::builder()
    .spouse1("Jean", 15)  // Trop jeune (âge minimum : 18 ans)
    .spouse2("Sophie", 25)
    .build();

match resultat {
    Err(e) => {
        println!("Erreur de validation : {}", e.message_fr());
        // Erreur : "Époux1 doit avoir au moins 18 ans (Article 144)"
    }
    _ => {}
}
```

### 1.4 Avantages du Pattern Builder

1. **Sécurité de Type** : Impossible de créer des objets invalides
2. **Ergonomie** : API fluide et lisible
3. **Validation** : Vérifications intégrées au moment de la construction
4. **Rétrocompatibilité** : Facile d'ajouter de nouveaux champs optionnels
5. **Auto-documenté** : Clair quels champs sont requis

---

## 2. Gestion des Erreurs

Legalis-FR fournit des types d'erreur riches et bilingues pour tous les domaines juridiques.

### 2.1 Types d'Erreur

Chaque module a son propre type d'erreur :

```rust
use legalis_fr::contract::ContractLawError;
use legalis_fr::labor::LaborLawError;
use legalis_fr::family::FamilyLawError;
use legalis_fr::inheritance::InheritanceLawError;
// ... etc
```

### 2.2 Messages d'Erreur Bilingues

Toutes les erreurs supportent le français et l'anglais :

```rust
use legalis_fr::labor::{validate_minimum_wage, LaborLawError};

match validate_minimum_wage(1200, 35.0) {
    Err(LaborLawError::MinimumWageViolation { actual, minimum, article }) => {
        // Anglais (par défaut)
        println!("{}", LaborLawError::MinimumWageViolation {
            actual,
            minimum,
            article: article.clone()
        });

        // Français
        println!("{}", LaborLawError::MinimumWageViolation {
            actual,
            minimum,
            article
        }.message_fr());
    }
    _ => {}
}
```

### 2.3 Gestion Structurée des Erreurs

Utilisez le pattern matching pour une gestion précise des erreurs :

```rust
use legalis_fr::contract::{validate_contract, ContractLawError};

fn traiter_contrat(contrat: Contract) -> Result<(), Box<dyn std::error::Error>> {
    match validate_contract(&contrat) {
        Ok(_) => {
            println!("✅ Contrat valide");
            Ok(())
        }
        Err(ContractLawError::MissingEssentialElement { element }) => {
            eprintln!("❌ Élément manquant : {}", element);
            // Gérer spécifiquement l'élément manquant
            Err("Contrat incomplet".into())
        }
        Err(ContractLawError::InvalidConsent { reason }) => {
            eprintln!("❌ Consentement invalide : {}", reason);
            // Gérer spécifiquement les problèmes de consentement
            Err("Problème de consentement".into())
        }
        Err(e) => {
            eprintln!("❌ Autre erreur : {}", e.message_fr());
            Err(e.into())
        }
    }
}
```

### 2.4 Conversion d'Erreurs

Convertir entre types d'erreur quand nécessaire :

```rust
use std::error::Error;

fn valider_contrat_travail(
    emploi: Employment,
) -> Result<(), Box<dyn Error>> {
    validate_employment(&emploi)?;  // LaborLawError converti en Box<dyn Error>
    Ok(())
}
```

### 2.5 Journalisation des Erreurs

Intégration avec les frameworks de logging :

```rust
use log::{error, warn, info};

match validate_contract(&contrat) {
    Ok(_) => {
        info!("Contrat validé avec succès : {}", contrat.object);
    }
    Err(e) => {
        error!("Échec de validation du contrat : {}", e);
        error!("Français : {}", e.message_fr());
        warn!("Contrat : {:?}", contrat);
    }
}
```

---

## 3. Patterns de Validation

### 3.1 Validation Immédiate

Valider immédiatement après construction :

```rust
// Pattern 1 : Valider séparément
let contrat = Contract::builder().build()?;
validate_contract(&contrat)?;

// Pattern 2 : Valider dans une seule expression
let contrat = {
    let c = Contract::builder().build()?;
    validate_contract(&c)?;
    c
};
```

### 3.2 Validation Paresseuse

Différer la validation jusqu'au besoin :

```rust
struct BrouillonContrat {
    contrat: Contract,
    valide: bool,
}

impl BrouillonContrat {
    fn valider(&mut self) -> Result<(), ContractLawError> {
        if !self.valide {
            validate_contract(&self.contrat)?;
            self.valide = true;
        }
        Ok(())
    }

    fn finaliser(mut self) -> Result<Contract, ContractLawError> {
        self.valider()?;
        Ok(self.contrat)
    }
}
```

### 3.3 Validation par Lot

Valider plusieurs éléments et collecter les erreurs :

```rust
fn valider_tous_contrats(
    contrats: Vec<Contract>
) -> (Vec<Contract>, Vec<(usize, ContractLawError)>) {
    let mut valides = Vec::new();
    let mut erreurs = Vec::new();

    for (i, contrat) in contrats.into_iter().enumerate() {
        match validate_contract(&contrat) {
            Ok(_) => valides.push(contrat),
            Err(e) => erreurs.push((i, e)),
        }
    }

    (valides, erreurs)
}
```

### 3.4 Validation Conditionnelle

Appliquer différentes règles de validation selon le contexte :

```rust
fn valider_contrat_contexte(
    contrat: &Contract,
    en_ligne: bool,
) -> Result<(), ContractLawError> {
    // Toujours valider les essentiels
    validate_contract(contrat)?;

    // Validation additionnelle pour les contrats en ligne
    if en_ligne {
        if contrat.formation_date > chrono::Utc::now().naive_utc().date() {
            return Err(ContractLawError::InvalidFormationDate {
                date: contrat.formation_date,
            });
        }
    }

    Ok(())
}
```

---

## 4. Sérialisation

### 4.1 Sérialisation JSON

Ajouter le support `serde` à vos types :

```rust
use serde::{Serialize, Deserialize};
use legalis_fr::contract::Contract;

#[derive(Serialize, Deserialize)]
struct DonneesContrat {
    #[serde(flatten)]
    contrat: Contract,
    metadonnees: Metadonnees,
}

#[derive(Serialize, Deserialize)]
struct Metadonnees {
    cree_le: String,
    cree_par: String,
}

// Sérialiser en JSON
let json = serde_json::to_string(&donnees_contrat)?;

// Désérialiser depuis JSON
let charge: DonneesContrat = serde_json::from_str(&json)?;
```

### 4.2 Intégration Base de Données

Stocker des données juridiques dans des bases de données :

```rust
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
struct LigneContrat {
    id: i32,
    type_contrat: String,
    parties: Vec<String>,
    objet: String,
    prix: i64,
    date_formation: chrono::NaiveDate,
}

async fn sauvegarder_contrat(
    pool: &PgPool,
    contrat: &Contract,
) -> Result<i32, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO contrats (type_contrat, parties, objet, prix, date_formation)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        format!("{:?}", contrat.contract_type),
        &contrat.parties,
        contrat.object,
        contrat.price as i64,
        contrat.formation_date,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
```

### 4.3 Stockage Fichier

Sauvegarder/charger des données juridiques depuis des fichiers :

```rust
use std::fs;

fn sauvegarder_fichier(contrat: &Contract, chemin: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(contrat)?;
    fs::write(chemin, json)?;
    Ok(())
}

fn charger_fichier(chemin: &str) -> Result<Contract, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(chemin)?;
    let contrat = serde_json::from_str(&json)?;
    Ok(contrat)
}
```

---

## 5. Validateurs Personnalisés

### 5.1 Étendre la Validation

Ajouter une logique de validation personnalisée :

```rust
use legalis_fr::contract::{Contract, validate_contract, ContractLawError};

fn valider_contrat_etendu(
    contrat: &Contract,
) -> Result<(), ContractLawError> {
    // Validation standard
    validate_contract(contrat)?;

    // Règles métier personnalisées
    if contrat.price > 1_000_000 {
        // Exiger un notaire pour les contrats de grande valeur
        if !contrat.notarized {
            return Err(ContractLawError::CustomValidation {
                reason: "Les contrats de plus de 1M€ requièrent un notaire".to_string(),
            });
        }
    }

    Ok(())
}
```

### 5.2 Composition de Validateurs

Combiner plusieurs validateurs :

```rust
fn valider_tout(contrat: &Contract) -> Result<(), ContractLawError> {
    validate_contract(contrat)?;
    valider_contrat_etendu(contrat)?;
    valider_parties(contrat)?;
    valider_juridiction(contrat)?;
    Ok(())
}

fn valider_parties(contrat: &Contract) -> Result<(), ContractLawError> {
    if contrat.parties.len() < 2 {
        return Err(ContractLawError::InvalidParties {
            reason: "Au moins 2 parties requises".to_string(),
        });
    }
    Ok(())
}
```

### 5.3 Pipeline de Validation

Créer un pipeline de validation :

```rust
type FonctionValidation = fn(&Contract) -> Result<(), ContractLawError>;

struct PipelineValidation {
    validateurs: Vec<FonctionValidation>,
}

impl PipelineValidation {
    fn nouveau() -> Self {
        Self { validateurs: Vec::new() }
    }

    fn ajouter(mut self, validateur: FonctionValidation) -> Self {
        self.validateurs.push(validateur);
        self
    }

    fn valider(&self, contrat: &Contract) -> Result<(), ContractLawError> {
        for validateur in &self.validateurs {
            validateur(contrat)?;
        }
        Ok(())
    }
}

// Utilisation
let pipeline = PipelineValidation::nouveau()
    .ajouter(validate_contract)
    .ajouter(valider_contrat_etendu)
    .ajouter(valider_parties);

pipeline.valider(&contrat)?;
```

---

## 6. Patterns d'Intégration

### 6.1 Intégration API Web

```rust
use actix_web::{web, HttpResponse, Result};
use legalis_fr::contract::{Contract, validate_contract};

async fn endpoint_validation_contrat(
    contrat: web::Json<Contract>,
) -> Result<HttpResponse> {
    match validate_contract(&contrat) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "valide": true,
            "message": "Contrat valide selon le droit français"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "valide": false,
            "erreur": e.to_string(),
            "erreur_fr": e.message_fr(),
        }))),
    }
}
```

### 6.2 Intégration gRPC

```rust
use tonic::{Request, Response, Status};

pub struct ServiceJuridique;

#[tonic::async_trait]
impl legal_service_server::LegalService for ServiceJuridique {
    async fn valider_contrat(
        &self,
        request: Request<ValidateContractRequest>,
    ) -> Result<Response<ValidateContractResponse>, Status> {
        let contrat = request.into_inner().contract
            .ok_or_else(|| Status::invalid_argument("Contrat requis"))?;

        match validate_contract(&contrat) {
            Ok(_) => Ok(Response::new(ValidateContractResponse {
                valide: true,
                erreurs: vec![],
            })),
            Err(e) => Ok(Response::new(ValidateContractResponse {
                valide: false,
                erreurs: vec![e.message_fr()],
            })),
        }
    }
}
```

### 6.3 Architecture Événementielle

```rust
use tokio::sync::mpsc;

enum EvenementJuridique {
    ContratCree(Contract),
    ContratValide(Contract),
    ContratRejete(Contract, ContractLawError),
}

async fn traiter_contrats(mut rx: mpsc::Receiver<Contract>) {
    while let Some(contrat) = rx.recv().await {
        match validate_contract(&contrat) {
            Ok(_) => {
                // Publier événement validé
                publier_evenement(EvenementJuridique::ContratValide(contrat)).await;
            }
            Err(e) => {
                // Publier événement de rejet
                publier_evenement(EvenementJuridique::ContratRejete(contrat, e)).await;
            }
        }
    }
}
```

---

## 7. Optimisation des Performances

### 7.1 Évaluation Paresseuse

Différer les opérations coûteuses :

```rust
struct ContratParesseux {
    contrat: Contract,
    valide: Option<Result<(), ContractLawError>>,
}

impl ContratParesseux {
    fn nouveau(contrat: Contract) -> Self {
        Self {
            contrat,
            valide: None,
        }
    }

    fn est_valide(&mut self) -> bool {
        if self.valide.is_none() {
            self.valide = Some(validate_contract(&self.contrat));
        }
        self.valide.as_ref().unwrap().is_ok()
    }
}
```

### 7.2 Mise en Cache

Mettre en cache les résultats de validation :

```rust
use std::collections::HashMap;

struct CacheValidation {
    cache: HashMap<u64, Result<(), ContractLawError>>,
}

impl CacheValidation {
    fn valider(&mut self, contrat: &Contract) -> Result<(), ContractLawError> {
        let cle = calculer_hash(contrat);

        if let Some(cache) = self.cache.get(&cle) {
            return cache.clone();
        }

        let resultat = validate_contract(contrat);
        self.cache.insert(cle, resultat.clone());
        resultat
    }
}
```

### 7.3 Validation Parallèle

Valider plusieurs éléments en parallèle :

```rust
use rayon::prelude::*;

fn valider_contrats_parallele(
    contrats: Vec<Contract>
) -> Vec<Result<(), ContractLawError>> {
    contrats
        .par_iter()
        .map(|c| validate_contract(c))
        .collect()
}
```

---

## 8. Patterns de Test

### 8.1 Tests Unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use legalis_fr::contract::{Contract, ContractType};

    #[test]
    fn test_contrat_valide() {
        let contrat = Contract::builder()
            .contract_type(ContractType::Sale)
            .parties(vec!["Alice".to_string(), "Bob".to_string()])
            .object("Objet de test")
            .price(1000)
            .build()
            .unwrap();

        assert!(validate_contract(&contrat).is_ok());
    }

    #[test]
    fn test_contrat_invalide_parties_manquantes() {
        let contrat = Contract::builder()
            .contract_type(ContractType::Sale)
            .parties(vec![])
            .build();

        assert!(contrat.is_err());
    }
}
```

### 8.2 Tests d'Intégration

```rust
#[cfg(test)]
mod tests_integration {
    use legalis_fr::contract::{Contract, validate_contract};
    use legalis_fr::labor::{Employment, validate_employment};

    #[test]
    fn test_workflow_complet() {
        // Créer un contrat
        let contrat = Contract::builder()
            .build()
            .unwrap();

        // Valider
        validate_contract(&contrat).unwrap();

        // Créer un emploi depuis le contrat
        let emploi = Employment::from_contract(&contrat)
            .unwrap();

        // Valider l'emploi
        validate_employment(&emploi).unwrap();
    }
}
```

---

## Résumé des Bonnes Pratiques

1. **Utilisez toujours les builders** pour la construction de types
2. **Gérez les erreurs explicitement** avec pattern matching
3. **Validez tôt** pour détecter rapidement les problèmes
4. **Utilisez les erreurs bilingues** pour les applications internationales
5. **Exploitez la sécurité de type** pour prévenir les états invalides
6. **Testez minutieusement** avec des tests unitaires, d'intégration et basés sur les propriétés
7. **Mettez en cache les résultats** pour les validations coûteuses
8. **Documentez les validateurs personnalisés** pour la maintenabilité

---

**Suivant** : [Aperçu des Domaines Juridiques](./legal-domains.md)

**Précédent** : [Guide de l'Utilisateur](./user-guide.md)

---

## 🌐 English Version / Version Anglaise

**Read this in English:** [API Patterns (English)](./api-patterns.en.md)
