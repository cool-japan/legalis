# Aperçu des Domaines Juridiques

Référence complète pour les 11 domaines juridiques de legalis-fr, organisés par codes juridiques français.

## Vue d'Ensemble

Legalis-FR implémente **11 domaines juridiques majeurs** couvrant le droit civil, commercial, du travail et constitutionnel français :

| Domaine | Code | Articles | Tests | Lignes |
|---------|------|----------|-------|--------|
| **Droit des Contrats** | Code civil III | 4 | 33 | 1 816 |
| **Droit du Travail** | Code du travail | 15 | 80 | 2 946 |
| **Droit de la Famille** | Code civil I | 19 | 71 | 3 350 |
| **Droit des Successions** | Code civil III | 12 | 63 | 1 711 |
| **Droit des Biens** | Code civil II-III | 13 | 77 | 1 967 |
| **Propriété Intellectuelle** | CPI | 8 | 56 | 1 897 |
| **Droit de la Preuve** | Code civil III | 8 | 42 | 1 132 |
| **Droit des Sociétés** | Code de commerce | 3 | 19 | 1 557 |
| **Droit Constitutionnel** | Constitution 1958 | 1 | 6 | 755 |
| **Droit Administratif** | CJA | 1 | 4 | 391 |
| **Responsabilité Délictuelle** | Code civil | 3 | 9 | 391 |

**Total** : 87 articles, 573 tests, 17 539 lignes

---

## 1. Droit des Contrats

**Base juridique** : Code civil, Livre III, Articles 1101-1231

### Portée

Implémentation de la réforme 2016 du droit des contrats français, couvrant :
- Formation des contrats (Articles 1101-1171)
- Exécution et inexécution (Articles 1217-1231)
- Résolution et dommages-intérêts

### Articles Clés Implémentés

#### Article 1128 : Éléments Essentiels

```rust
use legalis_fr::contract::{Contract, ContractType, validate_contract};

// Valide trois éléments essentiels :
// 1. Consentement des parties
// 2. Capacité de contracter
// 3. Contenu licite et certain

let contrat = Contract::builder()
    .contract_type(ContractType::Sale)
    .parties(vec!["Vendeur".to_string(), "Acheteur".to_string()])
    .object("Objet clairement défini")
    .price(100_000)
    .build()?;

validate_contract(&contrat)?;  // Valide l'Article 1128
```

#### Article 1217 : Remèdes à l'Inexécution

```rust
use legalis_fr::contract::{assess_contract_performance, PerformanceStatus};

// Implémente les remèdes de l'Article 1217 :
// - Exception d'inexécution
// - Réduction du prix
// - Résolution du contrat
// - Exécution forcée en nature
// - Dommages et intérêts

let remedes = assess_contract_performance(
    &contrat,
    PerformanceStatus::PartialFailure,
)?;
```

### Cas d'Usage

- **Plateformes e-commerce** : Formation et validation de contrats
- **Immobilier** : Contrats de vente et de bail
- **Plateformes B2B** : Contrats de service et SLA
- **LegalTech** : Analyse de contrats et évaluation des risques

---

## 2. Droit du Travail

**Base juridique** : Code du travail, Articles L1221-1 à L5422-3

### Portée

Implémentation complète du droit du travail français :
- Contrats de travail (CDI, CDD, temps partiel)
- Durée du travail (35 heures hebdomadaires, heures supplémentaires)
- Salaire minimum (SMIC)
- Procédures de licenciement
- Négociation collective

### Articles Clés Implémentés

#### Article L1221-1 : Contrat de Travail

```rust
use legalis_fr::labor::{Employment, EmploymentType, validate_employment};

let emploi = Employment::builder()
    .employee_name("Marie Dupont")
    .employer_name("TechCorp SARL")
    .employment_type(EmploymentType::Indefinite)  // CDI
    .start_date(NaiveDate::from_ymd_opt(2023, 1, 15).unwrap())
    .position("Ingénieure logiciel")
    .monthly_salary(3_500)
    .weekly_hours(35.0)
    .build()?;

validate_employment(&emploi)?;
```

#### Article L3121-27 : Durée Maximale du Travail

```rust
use legalis_fr::labor::validate_working_hours;

// Maximum 35 heures/semaine (durée légale)
// Les conventions collectives peuvent permettre jusqu'à 48 heures
validate_working_hours(35.0)?;  // OK
validate_working_hours(50.0)?;  // Erreur : dépasse le maximum légal
```

#### Article L3231-2 : Salaire Minimum (SMIC)

```rust
use legalis_fr::labor::{validate_minimum_wage, SMIC_2024};

// SMIC 2024 : 1 766,92€/mois pour 35h/semaine
validate_minimum_wage(1_800, 35.0)?;  // OK
validate_minimum_wage(1_500, 35.0)?;  // Erreur : inférieur au SMIC
```

### Cas d'Usage

- **Systèmes RH** : Gestion des contrats de travail
- **Systèmes de paie** : Conformité au SMIC, calcul des heures supplémentaires
- **Gestion de la main-d'œuvre** : Suivi de la durée du travail
- **Conformité légale** : Validation des procédures de licenciement

---

## 3. Droit de la Famille

**Base juridique** : Code civil, Livre I, Articles 143-515-13

### Portée

Implémentation complète du droit de la famille :
- Mariage et PACS
- Divorce
- Filiation et adoption
- Autorité parentale
- Changements de nom

### Articles Clés Implémentés

#### Articles 143-144 : Conditions du Mariage

```rust
use legalis_fr::family::{Marriage, MarriageRegime, validate_marriage};

let mariage = Marriage::builder()
    .spouse1("Jean Martin", 28)  // Âge minimum : 18 ans
    .spouse2("Sophie Dubois", 26)
    .marriage_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .regime(MarriageRegime::CommunityOfProperty)
    .build()?;

validate_marriage(&mariage)?;
```

#### Articles 229-232 : Types de Divorce

```rust
use legalis_fr::family::{Divorce, DivorceType};

// Quatre types de divorce :
// 1. Consentement mutuel
// 2. Divorce accepté
// 3. Faute
// 4. Altération définitive du lien conjugal

let divorce = Divorce::builder()
    .marriage(mariage)
    .divorce_type(DivorceType::MutualConsent)
    .filing_date(NaiveDate::from_ymd_opt(2025, 3, 10).unwrap())
    .build()?;
```

#### Article 371-1 : Autorité Parentale

```rust
use legalis_fr::family::{ParentalAuthority, assess_parental_authority};

let autorite = ParentalAuthority::builder()
    .child_name("Emma Martin")
    .child_birthdate(NaiveDate::from_ymd_opt(2015, 4, 20).unwrap())
    .parents(vec!["Jean Martin".to_string(), "Sophie Dubois".to_string()])
    .joint_authority(true)  // L'autorité conjointe est la règle
    .build()?;

assess_parental_authority(&autorite)?;
```

### Fonctionnalités Spéciales

- **Loi 2013 sur le mariage pour tous** : Implémentation complète
- **Régimes matrimoniaux** : Communauté, séparation, participation
- **Terminologie bilingue** : Termes juridiques français avec traductions anglaises

---

## 4. Droit des Successions

**Base juridique** : Code civil, Livre III, Titre I, Articles 720-892

### Portée

Cadre complet de succession et de testaments :
- Ouverture et dévolution de la succession
- Réserve héréditaire
- Quotité disponible
- Testaments
- Partage successoral

### Articles Clés Implémentés

#### Article 720 : Ouverture de la Succession

```rust
use legalis_fr::inheritance::{Succession, Heir, Relationship};

let succession = Succession::builder()
    .deceased("Jean Martin")
    .death_date(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap())
    .heirs(vec![
        Heir::new("Marie Martin", Relationship::Child, None),
        Heir::new("Pierre Martin", Relationship::Child, None),
        Heir::new("Sophie Martin", Relationship::Spouse, None),
    ])
    .estate_value(500_000)
    .build()?;
```

#### Articles 912-913 : Réserve Héréditaire

```rust
use legalis_fr::inheritance::calculate_reserved_portions;

// La réserve héréditaire protège les enfants et le conjoint :
// - 1 enfant : réserve = 1/2 (disponible = 1/2)
// - 2 enfants : réserve = 2/3 (disponible = 1/3)
// - 3+ enfants : réserve = 3/4 (disponible = 1/4)

let portions = calculate_reserved_portions(2)?;
println!("Réserve : {:.2}%", portions.reserved_portion * 100.0);    // 66,67%
println!("Disponible : {:.2}%", portions.available_portion * 100.0); // 33,33%
```

#### Articles 774-792 : Testaments

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
        "Lègue l'appartement à Marie".to_string(),
        "Lègue la voiture à Pierre".to_string(),
    ])
    .build()?;

validate_will(&testament)?;
```

### Cas d'Usage

- **Outils de planification successorale** : Création et validation de testaments
- **Systèmes notariaux** : Gestion des successions
- **Gestion de patrimoine** : Calcul des droits de succession
- **Conseil juridique** : Conformité à la réserve héréditaire

---

## 5. Droit des Biens

**Base juridique** : Code civil, Livres II-III, Articles 490-734

### Portée

Biens immobiliers et servitudes :
- Droit de propriété
- Servitudes
- Droits d'eau
- Droit de passage
- Transactions immobilières

### Articles Clés Implémentés

#### Article 544 : Propriété Absolue

```rust
use legalis_fr::property::{Property, PropertyType, validate_ownership};

let propriete = Property::builder()
    .property_type(PropertyType::Immovable {
        land_area: 500.0,
        building_area: Some(150.0),
    })
    .owner("Marie Dupont")
    .location("12 Rue de la Paix, Paris")
    .value(750_000)
    .build()?;

// Article 544 : Droit d'user, de jouir et de disposer (usus, fructus, abusus)
validate_ownership(&propriete)?;
```

#### Articles 637-710 : Servitudes

```rust
use legalis_fr::property::{Easement, EasementType, validate_easement};

let servitude = Easement::builder()
    .easement_type(EasementType::RightOfWay)
    .dominant_estate(Some("Parcelle A"))
    .servient_estate("Parcelle B")
    .description("Chemin de 3 mètres pour accès véhicules")
    .build()?;

validate_easement(&servitude)?;
```

### Cas d'Usage

- **Plateformes immobilières** : Transactions immobilières
- **Systèmes de cadastre** : Suivi des servitudes
- **Technologie agricole** : Gestion des droits d'eau
- **Urbanisme** : Conformité aux servitudes

---

## 6. Propriété Intellectuelle

**Base juridique** : Code de la propriété intellectuelle (CPI)

### Portée

Propriété industrielle et littéraire :
- Brevets
- Droits d'auteur
- Marques
- Dessins et modèles

### Articles Clés Implémentés

#### Articles L611-10, L611-11 : Brevets

```rust
use legalis_fr::intellectual_property::{Patent, validate_patent};

let brevet = Patent::builder()
    .title("Nouveau Design de Panneau Solaire")
    .inventor("Dr. Marie Curie")
    .filing_date(NaiveDate::from_ymd_opt(2023, 3, 15).unwrap())
    .novelty(true)
    .inventive_step(true)
    .industrial_applicability(true)
    .build()?;

validate_patent(&brevet)?;
// Protection : 20 ans à compter du dépôt
```

#### Articles L122-1, L123-1 : Droits d'Auteur

```rust
use legalis_fr::intellectual_property::{Copyright, WorkType};

let droit_auteur = Copyright::builder()
    .work_title("Les Misérables")
    .author("Victor Hugo")
    .creation_date(NaiveDate::from_ymd_opt(1862, 4, 3).unwrap())
    .work_type(WorkType::Literary)
    .build()?;

// Protection : vie de l'auteur + 70 ans
println!("Expiration : {}", droit_auteur.expiry_date());
```

#### Articles L711-1, L712-1 : Marques

```rust
use legalis_fr::intellectual_property::{Trademark, validate_trademark};

let marque = Trademark::builder()
    .mark("LEGALIS™")
    .owner("Legalis SAS")
    .registration_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
    .distinctiveness(true)
    .build()?;

// Protection : 10 ans, renouvelable indéfiniment
```

### Fonctionnalités Spéciales

- **Calcul de durée** : Calcul automatique de la date d'expiration
- **Triple exigence** : Nouveauté, activité inventive, application industrielle
- **Suivi du renouvellement** : Dates de renouvellement des marques

---

## 7. Droit de la Preuve

**Base juridique** : Code civil, Livre III, Titre XX, Articles 1353-1378

### Portée

Preuve et éléments de preuve dans les procédures civiles :
- Charge de la preuve
- Modes de preuve
- Preuve électronique
- Présomptions

### Articles Clés Implémentés

#### Article 1353 : Charge de la Preuve

```rust
use legalis_fr::evidence::{BurdenOfProof, assess_burden_of_proof};

let charge = BurdenOfProof::builder()
    .claimant_must_prove(vec![
        "Le contrat a été signé".to_string(),
        "Le paiement a été effectué".to_string(),
    ])
    .defendant_must_prove(vec![
        "Les marchandises ont été livrées".to_string(),
    ])
    .build()?;

assess_burden_of_proof(&charge)?;
```

#### Articles 1366-1378 : Preuve Électronique

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

### Fonctionnalités Spéciales

- **Preuve électronique** : Implémentation complète de la réforme 2016
- **Types de présomptions** : Classification en trois niveaux
- **Terminologie bilingue** : Concepts juridiques français de la preuve

---

## 8. Droit des Sociétés

**Base juridique** : Code de commerce, Articles L210-1 à L247-1

### Portée

Création et gestion d'entités commerciales :
- SARL (Société à responsabilité limitée)
- SAS (Société par actions simplifiée)
- SA (Société anonyme)
- Conditions de création
- Exigences de capital

### Article Clé Implémenté

#### Article L210-2 : Constitution de Société

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
    .registered_office("45 Rue de Rivoli, Paris")
    .build()?;

validate_company_formation(&sarl)?;
```

---

## 9. Droit Constitutionnel

**Base juridique** : Constitution de la Ve République (1958)

### Portée

Contrôle constitutionnel et droits fondamentaux :
- Contrôle de constitutionnalité
- Droits fondamentaux
- Séparation des pouvoirs

### Cas d'Usage

- **Tribunaux constitutionnels** : Contrôle de constitutionnalité
- **Rédaction législative** : Pré-contrôle de conformité
- **Recherche juridique** : Analyse constitutionnelle

---

## 10. Droit Administratif

**Base juridique** : Code de justice administrative

### Portée

Actes et procédures administratifs :
- Actes administratifs
- Recours administratifs
- Obligations de service public

---

## 11. Responsabilité Délictuelle

**Base juridique** : Code civil, Articles 1240-1244

### Portée

Responsabilité civile et dommages :
- Article 1240 : Responsabilité délictuelle générale
- Article 1241 : Responsabilité pour négligence
- Article 1242 : Responsabilité du fait d'autrui

---

## Relations Entre Domaines

```
┌────────────────────────────────────────────────┐
│      Moteur de Raisonnement Juridique          │
│        (Méta-couche pour tous domaines)        │
└────────────────────────────────────────────────┘
                     ▲
                     │
        ┌────────────┼────────────┐
        │            │            │
   ┌────▼───┐   ┌───▼────┐   ┌──▼─────┐
   │Contrats│   │Travail │   │ Famille│
   └────┬───┘   └───┬────┘   └──┬─────┘
        │           │            │
        ├───────────┴────────────┤
        │                        │
   ┌────▼──────┐          ┌─────▼────┐
   │Successions│          │  Biens   │
   └───────────┘          └──────────┘
```

### Interactions Inter-Domaines

1. **Contrats + Travail** : Les contrats de travail sont des contrats spéciaux
2. **Famille + Successions** : Droits du conjoint dans la succession
3. **Biens + Successions** : Succession immobilière
4. **Preuve + Tous** : Exigences de preuve à travers tous les domaines
5. **Constitutionnel + Tous** : Protection des droits fondamentaux

---

## Fonctionnalités de Droit Comparé

### vs. Droit Allemand (legalis-de)

| Fonctionnalité | France | Allemagne |
|----------------|--------|-----------|
| **Emploi** | 35h hebdo | Pas de limite fédérale |
| **Âge mariage** | 18 ans | 18 ans |
| **Réserve héréditaire** | 1/2 à 3/4 | 1/2 |
| **Durée brevet** | 20 ans | 20 ans |

### vs. Droit Japonais (legalis-jp)

| Fonctionnalité | France | Japon |
|----------------|--------|-------|
| **Types divorce** | 4 types | 2 types |
| **Durée droits d'auteur** | Vie + 70 | Vie + 70 |
| **Types société** | SARL, SAS, SA | KK, GK |

---

## Prochaines Étapes

- **[Guide de Démarrage](./getting-started.md)** - Commencer à utiliser legalis-fr
- **[Guide de l'Utilisateur](./user-guide.md)** - Exemples pratiques pour chaque domaine
- **[Patterns d'API](./api-patterns.md)** - Bonnes pratiques et patterns

**Questions ?** Consultez le [README principal](../README.md) ou la documentation API.

---

## 🌐 English Version / Version Anglaise

**Read this in English:** [Legal Domains (English)](./legal-domains.en.md)
