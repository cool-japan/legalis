# Guide de l'Utilisateur Legalis-FR

Guide complet avec exemples pratiques pour les 11 domaines juridiques de legalis-fr.

## Table des Matières

1. [Droit des Contrats](#1-droit-des-contrats)
2. [Droit du Travail](#2-droit-du-travail)
3. [Droit de la Famille](#3-droit-de-la-famille)
4. [Droit des Successions](#4-droit-des-successions)
5. [Droit des Biens](#5-droit-des-biens)
6. [Propriété Intellectuelle](#6-propriété-intellectuelle)
7. [Droit de la Preuve](#7-droit-de-la-preuve)
8. [Droit des Sociétés](#8-droit-des-sociétés)
9. [Droit Constitutionnel](#9-droit-constitutionnel)
10. [Droit Administratif](#10-droit-administratif)
11. [Responsabilité Délictuelle](#11-responsabilité-délictuelle)
12. [Moteur de Raisonnement Juridique](#12-moteur-de-raisonnement-juridique)

---

## 1. Droit des Contrats

**Base juridique** : Code civil, Livre III, Articles 1101-1231

### 1.1 Création et Validation de Contrats

```rust
use legalis_fr::contract::{Contract, ContractType, validate_contract};
use chrono::NaiveDate;

// Exemple : Contrat de vente immobilière
let contrat_vente = Contract::builder()
    .contract_type(ContractType::Sale)
    .parties(vec!["Jean Martin".to_string(), "Sophie Dubois".to_string()])
    .object("Appartement au 45 Rue de Rivoli, Paris 75001")
    .price(450_000)  // 450 000€
    .formation_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .build()?;

// Valider selon Article 1128 (éléments essentiels : consentement, capacité, contenu)
match validate_contract(&contrat_vente) {
    Ok(_) => println!("✅ Contrat valide"),
    Err(e) => println!("❌ Contrat invalide : {}", e.message_fr()),
}
```

### 1.2 Exécution du Contrat (Article 1217)

```rust
use legalis_fr::contract::{assess_contract_performance, PerformanceStatus};

// Implémente les remèdes de l'Article 1217 :
// - Exception d'inexécution
// - Réduction du prix
// - Résolution du contrat
// - Exécution forcée en nature
// - Dommages et intérêts

let execution = assess_contract_performance(
    &contrat_vente,
    PerformanceStatus::Performed,
)?;
```

### 1.3 Cas d'Usage : Plateforme E-Commerce

```rust
fn valider_achat_en_ligne(
    acheteur_id: &str,
    vendeur_id: &str,
    produit: &str,
    prix: u64,
) -> Result<Contract, Box<dyn std::error::Error>> {
    let contrat = Contract::builder()
        .contract_type(ContractType::Sale)
        .parties(vec![acheteur_id.to_string(), vendeur_id.to_string()])
        .object(produit)
        .price(prix)
        .formation_date(chrono::Utc::now().naive_utc().date())
        .build()?;

    validate_contract(&contrat)?;
    Ok(contrat)
}
```

---

## 2. Droit du Travail

**Base juridique** : Code du travail, Articles L1221-1 à L5422-3

### 2.1 Contrats de Travail (CDI/CDD)

```rust
use legalis_fr::labor::{Employment, EmploymentType, validate_employment};
use chrono::NaiveDate;

// Exemple 1 : Contrat à durée indéterminée (CDI)
let cdi = Employment::builder()
    .employee_name("Marie Dupont")
    .employer_name("TechCorp SARL")
    .employment_type(EmploymentType::Indefinite)
    .start_date(NaiveDate::from_ymd_opt(2023, 1, 15).unwrap())
    .position("Ingénieure logiciel")
    .monthly_salary(3_500)
    .weekly_hours(35.0)  // Durée légale de 35 heures
    .probation_period_months(Some(3))
    .build()?;

validate_employment(&cdi)?;

// Exemple 2 : Contrat à durée déterminée (CDD)
let cdd = Employment::builder()
    .employee_name("Pierre Martin")
    .employer_name("SeasonalCo SAS")
    .employment_type(EmploymentType::FixedTerm {
        reason: "Accroissement temporaire d'activité".to_string(),
    })
    .start_date(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap())
    .end_date(Some(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()))
    .position("Vendeur")
    .monthly_salary(2_000)
    .weekly_hours(35.0)
    .build()?;

validate_employment(&cdd)?;
```

### 2.2 Validation du SMIC (Article L3231-2)

```rust
use legalis_fr::labor::{validate_minimum_wage, SMIC_2024};

let salaire = 1_800;  // 1 800€/mois
let heures = 35.0;

match validate_minimum_wage(salaire, heures) {
    Ok(_) => println!("✅ Salaire conforme au SMIC ({}€)", SMIC_2024),
    Err(e) => println!("❌ Inférieur au SMIC : {}", e.message_fr()),
}
```

### 2.3 Durée du Travail (Article L3121-27)

```rust
use legalis_fr::labor::validate_working_hours;

let heures_hebdo = 40.0;

match validate_working_hours(heures_hebdo) {
    Ok(_) => println!("✅ Durée de travail valide"),
    Err(e) => println!("❌ Dépasse la limite légale : {}", e.message_fr()),
}
```

### 2.4 Rupture du Contrat (Article L1234-1)

```rust
use legalis_fr::labor::{TerminationReason, validate_termination};

let rupture = validate_termination(
    &cdi,
    TerminationReason::EconomicDismissal,
    NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
)?;

println!("Préavis : {} mois", rupture.notice_period_months);
println!("Indemnité de licenciement : {}€", rupture.severance_pay);
```

---

## 3. Droit de la Famille

**Base juridique** : Code civil, Livre I, Articles 143-515-13

### 3.1 Mariage (Articles 143-227)

```rust
use legalis_fr::family::{Marriage, MarriageRegime, validate_marriage};
use chrono::NaiveDate;

let mariage = Marriage::builder()
    .spouse1("Jean Martin", 28)  // Âge minimum : 18 ans
    .spouse2("Sophie Dubois", 26)
    .marriage_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .regime(MarriageRegime::CommunityOfProperty)  // Communauté de biens
    .build()?;

validate_marriage(&mariage)?;
```

### 3.2 Divorce (Articles 229-232)

```rust
use legalis_fr::family::{Divorce, DivorceType, validate_divorce};

// Quatre types de divorce :
// 1. Consentement mutuel
// 2. Divorce accepté
// 3. Faute
// 4. Altération définitive du lien conjugal

let divorce = Divorce::builder()
    .marriage(mariage.clone())
    .divorce_type(DivorceType::MutualConsent)
    .filing_date(NaiveDate::from_ymd_opt(2025, 3, 10).unwrap())
    .build()?;

validate_divorce(&divorce)?;
```

### 3.3 Autorité Parentale (Article 371-1)

```rust
use legalis_fr::family::{ParentalAuthority, assess_parental_authority};

let autorite = ParentalAuthority::builder()
    .child_name("Emma Martin")
    .child_birthdate(NaiveDate::from_ymd_opt(2015, 4, 20).unwrap())
    .parents(vec!["Jean Martin".to_string(), "Sophie Dubois".to_string()])
    .joint_authority(true)  // Autorité parentale conjointe
    .build()?;

assess_parental_authority(&autorite)?;
```

### 3.4 PACS (Articles 515-1 à 515-7)

```rust
use legalis_fr::family::{PACS, validate_pacs};

let pacs = PACS::builder()
    .partner1("Alice Moreau", 30)
    .partner2("Bob Lefebvre", 32)
    .registration_date(NaiveDate::from_ymd_opt(2023, 9, 1).unwrap())
    .build()?;

validate_pacs(&pacs)?;
```

---

## 4. Droit des Successions

**Base juridique** : Code civil, Livre III, Titre I, Articles 720-892

### 4.1 Ouverture de la Succession (Article 720)

```rust
use legalis_fr::inheritance::{Succession, Heir, Relationship, calculate_succession};
use chrono::NaiveDate;

let succession = Succession::builder()
    .deceased("Jean Martin")
    .death_date(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap())
    .heirs(vec![
        Heir::new("Marie Martin", Relationship::Child, None),
        Heir::new("Pierre Martin", Relationship::Child, None),
        Heir::new("Sophie Martin", Relationship::Spouse, None),
    ])
    .estate_value(500_000)  // 500 000€
    .build()?;

let resultat = calculate_succession(&succession)?;
println!("Répartition : {:?}", resultat.distribution);
```

### 4.2 Réserve Héréditaire (Articles 912-913)

```rust
use legalis_fr::inheritance::calculate_reserved_portions;

// Deux enfants : réserve héréditaire = 2/3, quotité disponible = 1/3
let portions = calculate_reserved_portions(2)?;

println!("Avec 2 enfants :");
println!("  Réserve héréditaire : {:.2}%", portions.reserved_portion * 100.0);
println!("  Quotité disponible : {:.2}%", portions.available_portion * 100.0);
```

### 4.3 Testaments (Articles 774-792)

```rust
use legalis_fr::inheritance::{Will, WillType, validate_will};

// Trois types de testaments :
// 1. Olographe : manuscrit, daté, signé
// 2. Authentique : notarié
// 3. Mystique : scellé, présenté au notaire

let testament = Will::builder()
    .testator("Jean Martin")
    .will_type(WillType::Holographic {
        handwritten: true,
        dated: true,
        signed: true,
    })
    .date(NaiveDate::from_ymd_opt(2023, 1, 10).unwrap())
    .dispositions(vec![
        "Lègue l'appartement à Marie Martin".to_string(),
        "Lègue la voiture à Pierre Martin".to_string(),
    ])
    .build()?;

validate_will(&testament)?;
```

---

## 5. Droit des Biens

**Base juridique** : Code civil, Livres II-III, Articles 490-734

### 5.1 Droit de Propriété (Article 544)

```rust
use legalis_fr::property::{Property, PropertyType, validate_ownership};

let propriete = Property::builder()
    .property_type(PropertyType::Immovable {
        land_area: 500.0,  // 500 m²
        building_area: Some(150.0),  // 150 m²
    })
    .owner("Marie Dupont")
    .location("12 Rue de la Paix, Paris 75002")
    .value(750_000)  // 750 000€
    .build()?;

// Article 544 : Droit d'user, de jouir et de disposer (usus, fructus, abusus)
validate_ownership(&propriete)?;
```

### 5.2 Servitudes (Articles 637-710)

```rust
use legalis_fr::property::{Easement, EasementType, validate_easement};

// Servitude de passage
let servitude = Easement::builder()
    .easement_type(EasementType::RightOfWay)
    .dominant_estate(Some("Parcelle A"))  // Fonds dominant
    .servient_estate("Parcelle B")        // Fonds servant
    .description("Chemin de 3 mètres pour accès véhicules")
    .build()?;

validate_easement(&servitude)?;
```

### 5.3 Droits d'Eau (Article 555)

```rust
// Servitude obligatoire pour l'accès à l'eau (bétail, irrigation)
let servitude_eau = Easement::builder()
    .easement_type(EasementType::WaterRights)
    .dominant_estate(Some("Exploitation agricole"))
    .servient_estate("Propriété avec cours d'eau")
    .description("Droits d'abreuvement du bétail")
    .build()?;
```

---

## 6. Propriété Intellectuelle

**Base juridique** : Code de la propriété intellectuelle (CPI)

### 6.1 Brevets (Articles L611-10, L611-11)

```rust
use legalis_fr::intellectual_property::{Patent, validate_patent};
use chrono::NaiveDate;

let brevet = Patent::builder()
    .title("Nouveau Design de Panneau Solaire")
    .inventor("Dr. Marie Curie")
    .filing_date(NaiveDate::from_ymd_opt(2023, 3, 15).unwrap())
    .novelty(true)              // Nouveauté
    .inventive_step(true)       // Activité inventive
    .industrial_applicability(true)  // Application industrielle
    .build()?;

validate_patent(&brevet)?;
// Protection : 20 ans à compter du dépôt
```

### 6.2 Droits d'Auteur (Articles L122-1, L123-1)

```rust
use legalis_fr::intellectual_property::{Copyright, WorkType};

let droit_auteur = Copyright::builder()
    .work_title("Le Petit Prince")
    .author("Antoine de Saint-Exupéry")
    .creation_date(NaiveDate::from_ymd_opt(1943, 4, 6).unwrap())
    .work_type(WorkType::Literary)
    .build()?;

// Protection : vie de l'auteur + 70 ans
println!("Protégé jusqu'au : {}", droit_auteur.expiry_date());
```

### 6.3 Marques (Articles L711-1, L712-1)

```rust
use legalis_fr::intellectual_property::{Trademark, validate_trademark};

let marque = Trademark::builder()
    .mark("LEGALIS™")
    .owner("Legalis SAS")
    .registration_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
    .distinctiveness(true)  // Caractère distinctif
    .build()?;

validate_trademark(&marque)?;
// Protection : 10 ans, renouvelable indéfiniment
```

---

## 7. Droit de la Preuve

**Base juridique** : Code civil, Livre III, Titre XX, Articles 1353-1378

### 7.1 Charge de la Preuve (Article 1353)

```rust
use legalis_fr::evidence::{BurdenOfProof, assess_burden_of_proof};

let charge_preuve = BurdenOfProof::builder()
    .claimant_must_prove(vec![
        "Le contrat a été signé".to_string(),
        "Le paiement a été effectué".to_string(),
    ])
    .defendant_must_prove(vec![
        "Les marchandises ont été livrées".to_string(),
    ])
    .build()?;

assess_burden_of_proof(&charge_preuve)?;
```

### 7.2 Preuve Électronique (Articles 1366-1378)

```rust
use legalis_fr::evidence::{Evidence, EvidenceType};

// La preuve électronique a la même force que la preuve écrite
let preuve = Evidence::builder()
    .evidence_type(EvidenceType::WrittenDocument {
        electronic: true,
        signed: true,  // Signature électronique
    })
    .description("Contrat signé électroniquement")
    .authenticity_verified(true)
    .build()?;
```

### 7.3 Présomptions (Article 1354)

```rust
use legalis_fr::evidence::{PresumptionType, assess_presumption};

// Trois types :
// 1. Simple (réfragable)
// 2. Mixte
// 3. Irréfragable (absolue)

let simple = assess_presumption(PresumptionType::Simple)?;
assert!(simple.rebuttable);

let irrefragable = assess_presumption(PresumptionType::Irrebuttable)?;
assert!(!irrefragable.rebuttable);
```

---

## 8. Droit des Sociétés

**Base juridique** : Code de commerce, Articles L210-1 à L247-1

### 8.1 Constitution de Société (Article L210-2)

```rust
use legalis_fr::company::{Company, CompanyType, validate_company_formation};

// SARL : capital minimum 1€
let sarl = Company::builder()
    .name("TechCorp SARL")
    .company_type(CompanyType::SARL)
    .capital(10_000)
    .shareholders(vec![
        "Jean Martin (60%)".to_string(),
        "Sophie Dubois (40%)".to_string(),
    ])
    .registered_office("45 Rue de Rivoli, Paris 75001")
    .build()?;

validate_company_formation(&sarl)?;

// SAS : capital minimum 1€, plus flexible
let sas = Company::builder()
    .name("InnovateSAS")
    .company_type(CompanyType::SAS)
    .capital(50_000)
    .build()?;
```

---

## 9. Droit Constitutionnel

**Base juridique** : Constitution de la Ve République (1958)

### 9.1 Contrôle de Constitutionnalité (Article 61)

```rust
use legalis_fr::constitution::{assess_constitutionality, ConstitutionalIssue};

let question = ConstitutionalIssue::builder()
    .law_text("Nouvelle loi sur la conservation des données")
    .challenged_provisions(vec![
        "Conservation obligatoire des données pendant 5 ans".to_string(),
    ])
    .constitutional_rights_at_stake(vec![
        "Droit à la vie privée (Article 2)".to_string(),
        "Liberté de communication (Article 11)".to_string(),
    ])
    .build()?;

let evaluation = assess_constitutionality(&question)?;
println!("Constitutionnel : {}", evaluation.is_constitutional);
```

---

## 10. Droit Administratif

**Base juridique** : Code de justice administrative

### 10.1 Actes Administratifs

```rust
use legalis_fr::administrative::{AdministrativeAct, validate_act};

let acte = AdministrativeAct::builder()
    .authority("Maire de Paris")
    .subject("Permis de construire pour nouvelle construction")
    .date(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap())
    .legal_basis("Article L421-1 du Code de l'urbanisme")
    .build()?;

validate_act(&acte)?;
```

---

## 11. Responsabilité Délictuelle

**Base juridique** : Code civil, Articles 1240-1244

### 11.1 Responsabilité Civile (Article 1240)

```rust
use legalis_fr::code_civil::{assess_tort_liability, TortClaim};

let reclamation = TortClaim::builder()
    .wrongful_act("Conduite négligente causant une collision")
    .damage("Dommages au véhicule : 5 000€ ; Frais médicaux : 2 000€")
    .causation("Lien de causalité direct et certain")
    .build()?;

let responsabilite = assess_tort_liability(&reclamation)?;
println!("Responsable : {}", responsabilite.is_liable);
println!("Dommages accordés : {}€", responsabilite.damages_awarded);
```

---

## 12. Moteur de Raisonnement Juridique

Analyse juridique avancée et évaluation de cas.

### 12.1 Analyse de Cas

```rust
use legalis_fr::reasoning::{LegalCase, apply_legal_reasoning, Domain};

let affaire = LegalCase::builder()
    .facts(vec![
        "Contrat signé le 15 janvier 2023".to_string(),
        "Le vendeur n'a pas livré les marchandises à la date convenue".to_string(),
        "L'acheteur a subi une perte financière de 10 000€".to_string(),
    ])
    .legal_question("L'acheteur peut-il réclamer des dommages-intérêts pour inexécution ?")
    .relevant_domain(Domain::ContractLaw)
    .build()?;

let resultat = apply_legal_reasoning(affaire)?;

println!("Articles applicables : {:?}", resultat.applicable_articles);
println!("Conclusion : {}", resultat.conclusion);
println!("Confiance : {:.0}%", resultat.confidence * 100.0);
```

---

## Bonnes Pratiques

### 1. Gestion des Erreurs

Gérez toujours les erreurs avec élégance :

```rust
match validate_employment(&emploi) {
    Ok(_) => {
        // Chemin de succès
    }
    Err(e) => {
        // Logger l'erreur
        eprintln!("Validation échouée : {}", e);
        // Afficher message en français
        println!("Français : {}", e.message_fr());
        // Retourner l'erreur à l'appelant
        return Err(e.into());
    }
}
```

### 2. Gestion des Dates

Utilisez `chrono` pour toutes les opérations sur les dates :

```rust
use chrono::NaiveDate;

// Correct
let date = NaiveDate::from_ymd_opt(2023, 6, 15)
    .ok_or("Date invalide")?;

// Arithmétique sur les dates
let futur = date + chrono::Duration::days(30);
```

### 3. Pattern Builder

Utilisez toujours les builders pour la construction de types :

```rust
// Bon
let emploi = Employment::builder()
    .employee_name("Marie")
    .build()?;

// Éviter la construction directe
```

---

## Prochaines Étapes

- **[Patterns d'API](./api-patterns.md)** - Apprenez les patterns avancés et bonnes pratiques
- **[Domaines Juridiques](./legal-domains.md)** - Plongée profonde dans chaque domaine juridique

**Besoin d'aide ?** Consultez la [section dépannage](./getting-started.md#dépannage) ou ouvrez une issue sur GitHub.

---

## 🌐 English Version / Version Anglaise

**Read this in English:** [User Guide (English)](./user-guide.en.md)
