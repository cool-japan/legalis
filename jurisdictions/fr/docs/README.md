# Documentation Legalis-FR

Bienvenue dans la documentation de **legalis-fr** - une implémentation complète en Rust du droit français pour les applications de raisonnement juridique et de conformité.

## 📚 Structure de la Documentation

- **[Guide de Démarrage](./getting-started.md)** - Guide rapide pour ajouter legalis-fr à votre projet
- **[Guide de l'Utilisateur](./user-guide.md)** - Guide complet avec exemples pour tous les domaines juridiques
- **[Patterns d'API](./api-patterns.md)** - Bonnes pratiques pour travailler avec l'API legalis-fr
- **[Domaines Juridiques](./legal-domains.md)** - Aperçu des 11 domaines juridiques disponibles

## 🎯 Qu'est-ce que Legalis-FR ?

Legalis-FR est une bibliothèque Rust prête pour la production qui fournit :

- **11 domaines juridiques** couvrant le droit civil français, le droit du travail, le droit constitutionnel, et plus
- **524 tests complets** garantissant la précision juridique
- **Documentation bilingue** (français/anglais) avec un ratio de 69,7% docs/code
- **Moteur de Raisonnement Juridique** pour l'analyse juridique avancée et l'évaluation de cas
- **API type-safe** empêchant les états juridiques invalides à la compilation

## 🚀 Exemple Rapide

```rust
use legalis_fr::labor::{Employment, TerminationReason, validate_termination};
use chrono::NaiveDate;

// Créer un contrat de travail
let emploi = Employment::builder()
    .employee_name("Marie Dupont")
    .employer_name("TechCorp SARL")
    .start_date(NaiveDate::from_ymd_opt(2020, 1, 15).unwrap())
    .position("Ingénieure logiciel")
    .monthly_salary(3500)
    .build()
    .unwrap();

// Valider une rupture selon le droit du travail français (Article L1234-1)
let rupture = validate_termination(
    &emploi,
    TerminationReason::EconomicDismissal,
    NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
);

match rupture {
    Ok(result) => println!("Rupture valide : {:?}", result),
    Err(e) => println!("Invalide : {}", e.message_fr()),
}
```

## 🌍 Support Linguistique

Toutes les documentations et messages d'erreur sont disponibles en **français et en anglais** :

```rust
use legalis_fr::contract::ContractLawError;

let erreur = ContractLawError::InvalidConsent {
    reason: "Vice du consentement détecté".to_string(),
};

// Message d'erreur en anglais (par défaut)
println!("{}", erreur);

// Message d'erreur en français
println!("{}", erreur.message_fr());
```

## 📖 Concepts Fondamentaux

### 1. Domaines Juridiques

Legalis-FR organise le droit français en 11 domaines spécialisés :

- **Droit des Contrats** (Code civil, Livre III)
- **Droit du Travail** (Code du travail)
- **Droit de la Famille** (Code civil, Livre I)
- **Droit des Successions** (Code civil, Livre III)
- **Droit des Biens** (Code civil, Livre II)
- **Droit des Sociétés** (Code de commerce)
- **Droit de la Preuve** (Code civil, Livre III, Titre XX)
- **Propriété Intellectuelle** (Code de la propriété intellectuelle)
- **Droit Constitutionnel** (Constitution de 1958)
- **Droit Administratif** (Code de justice administrative)
- **Responsabilité Délictuelle** (Code civil, Articles 1240-1244)

### 2. Moteur de Raisonnement Juridique

Le **Moteur de Raisonnement** fournit une analyse juridique avancée :

```rust
use legalis_fr::reasoning::{LegalCase, apply_legal_reasoning};

let affaire = LegalCase::builder()
    .facts(vec!["Contrat signé sous contrainte".to_string()])
    .legal_question("Le contrat est-il valide ?")
    .build()
    .unwrap();

let resultat = apply_legal_reasoning(affaire);
println!("Conclusion juridique : {}", resultat.conclusion);
```

### 3. Sécurité de Type

Legalis-FR utilise le système de types de Rust pour garantir la validité juridique :

```rust
// Ceci ne compilera pas - état invalide empêché à la compilation
let mariage_invalide = Marriage {
    spouse1_age: 15,  // Erreur : L'âge doit être au moins 18 ans
    // ...
};

// Utiliser les builders avec validation
let mariage_valide = Marriage::builder()
    .spouse1("Jean Martin", 25)
    .spouse2("Sophie Dubois", 23)
    .marriage_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .build()?;  // Retourne Result<Marriage, FamilyLawError>
```

## 🔗 Ressources Connexes

- **[README Principal](../README.md)** - Aperçu du projet et statistiques
- **[Cargo.toml](../Cargo.toml)** - Dépendances et métadonnées
- **[Code Source](../src/)** - Détails d'implémentation
- **[Tests](../tests/)** - Tests d'intégration et exemples

## 💡 Cas d'Usage

Legalis-FR est conçu pour :

- **Applications LegalTech** - Analyse de contrats, vérification de conformité
- **Systèmes RH** - Validation du droit du travail, procédures de licenciement
- **Plateformes Immobilières** - Transactions immobilières, validation de servitudes
- **Outils de Planification Successorale** - Calculs de succession, validation de testaments
- **Systèmes de Gestion PI** - Validation brevets/marques, analyse de droits d'auteur
- **Recherche Académique** - Études de droit comparé, recherche en raisonnement juridique

## 🤝 Contribution

Vous avez trouvé un problème ou souhaitez améliorer la documentation ? Les contributions sont les bienvenues !

1. Consultez le [dépôt principal](https://github.com/your-org/legalis-rs)
2. Examinez les issues et pull requests existantes
3. Suivez les directives de contribution

## 📄 Licence

Legalis-FR fait partie du framework legalis-rs. Consultez le dépôt principal pour les informations de licence.

---

**Prêt à commencer ?** → [Guide de Démarrage](./getting-started.md)

**Besoin d'exemples ?** → [Guide de l'Utilisateur](./user-guide.md)

**Comprendre l'API ?** → [Patterns d'API](./api-patterns.md)

---

## 🌐 English Documentation / Documentation en Anglais

**Read this in English:**
- **[README (English)](./README.en.md)**
- [Getting Started (English)](./getting-started.en.md)
- [User Guide (English)](./user-guide.en.md)
- [API Patterns (English)](./api-patterns.en.md)
- [Legal Domains (English)](./legal-domains.en.md)
