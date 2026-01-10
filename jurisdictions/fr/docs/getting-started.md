# Guide de Démarrage avec Legalis-FR

Ce guide vous aidera à ajouter legalis-fr à votre projet Rust et à écrire votre premier code de validation juridique.

## 📦 Installation

### Ajouter à Cargo.toml

Ajoutez legalis-fr à vos dépendances `Cargo.toml` :

```toml
[dependencies]
legalis-fr = "0.2.0"
legalis-core = "0.2.0"  # Types et traits de base
chrono = "0.4"          # Gestion des dates (requis)
```

### Dépendances Optionnelles

Pour des cas d'usage spécifiques, vous pouvez ajouter :

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }  # Sérialisation
serde_json = "1.0"                                   # Support JSON
```

### Vérifier l'Installation

Créez un fichier de test simple pour vérifier l'installation :

```rust
// src/main.rs ou tests/test_install.rs
use legalis_fr::contract::{Contract, ContractType};

fn main() {
    println!("Legalis-FR installé avec succès !");

    let contrat = Contract::builder()
        .contract_type(ContractType::Sale)
        .build();

    println!("Contrat créé : {:?}", contrat);
}
```

Exécutez avec :
```bash
cargo run
# ou
cargo test test_install
```

## 🎯 Votre Première Validation Juridique

Créons un programme simple qui valide un contrat de travail français.

### Exemple : Validation de Contrat de Travail

```rust
use legalis_fr::labor::{Employment, validate_employment};
use chrono::NaiveDate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Créer un contrat de travail
    let emploi = Employment::builder()
        .employee_name("Marie Dupont")
        .employer_name("TechCorp SARL")
        .start_date(NaiveDate::from_ymd_opt(2023, 1, 15).unwrap())
        .end_date(None)  // Contrat à durée indéterminée (CDI)
        .position("Ingénieure logiciel")
        .monthly_salary(3500)
        .weekly_hours(35.0)  // Durée légale de 35 heures
        .probation_period_months(Some(3))
        .build()?;

    // Valider l'emploi selon le droit du travail français
    match validate_employment(&emploi) {
        Ok(_) => {
            println!("✅ Contrat de travail valide selon le droit français");
            println!("   Employé(e) : {}", emploi.employee_name);
            println!("   Salaire : {}€/mois", emploi.monthly_salary);
            println!("   Heures hebdomadaires : {}", emploi.weekly_hours);
        }
        Err(e) => {
            println!("❌ Contrat de travail invalide : {}", e.message_fr());
        }
    }

    Ok(())
}
```

### Ce Que Cela Fait

1. **Crée un contrat de travail** en utilisant le pattern builder
2. **Le valide** selon le droit du travail français (Code du travail)
3. **Vérifie** :
   - Conformité au salaire minimum (Article L3231-2 - SMIC)
   - Durée maximale du travail (Article L3121-27 : 35h/semaine)
   - Limites de période d'essai (Article L1221-19)
   - Éléments contractuels requis (Article L1221-1)

## 🔨 Patterns Courants

### Pattern 1 : Builder Pattern

Tous les types principaux utilisent des builders pour une construction sûre :

```rust
use legalis_fr::family::{Marriage, MarriageRegime};
use chrono::NaiveDate;

let mariage = Marriage::builder()
    .spouse1("Jean Martin", 28)
    .spouse2("Sophie Dubois", 26)
    .marriage_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .regime(MarriageRegime::CommunityOfProperty)
    .build()?;  // Retourne Result<Marriage, FamilyLawError>
```

### Pattern 2 : Fonctions de Validation

Chaque module fournit des fonctions de validation :

```rust
use legalis_fr::contract::{Contract, validate_contract};

let contrat = Contract::builder()
    .contract_type(ContractType::Sale)
    .parties(vec!["Vendeur".to_string(), "Acheteur".to_string()])
    .object("Appartement à Paris")
    .price(450_000)
    .build()?;

// Valider selon Article 1128 (éléments essentiels)
match validate_contract(&contrat) {
    Ok(_) => println!("Contrat valide"),
    Err(e) => println!("Invalide : {}", e.message_fr()),
}
```

### Pattern 3 : Gestion des Erreurs

Toutes les erreurs sont bilingues et descriptives :

```rust
use legalis_fr::labor::LaborLawError;

match validate_employment(&emploi) {
    Err(LaborLawError::MinimumWageViolation {
        actual,
        minimum,
        article
    }) => {
        println!("Le salaire {}€ est inférieur au minimum {}€", actual, minimum);
        println!("Viole : {}", article);
        println!("Français : {}",
            LaborLawError::MinimumWageViolation {
                actual,
                minimum,
                article: article.clone()
            }.message_fr()
        );
    }
    _ => {}
}
```

## 📚 Prochaines Étapes

### Explorer les Domaines Juridiques

Choisissez le domaine pertinent pour votre cas d'usage :

```rust
// Droit des Contrats
use legalis_fr::contract::{Contract, validate_contract};

// Droit du Travail
use legalis_fr::labor::{Employment, validate_employment};

// Droit de la Famille
use legalis_fr::family::{Marriage, validate_marriage};

// Droit des Successions
use legalis_fr::inheritance::{Succession, calculate_reserved_portions};

// Droit des Biens
use legalis_fr::property::{Property, validate_easement};

// Propriété Intellectuelle
use legalis_fr::intellectual_property::{Patent, Copyright, Trademark};

// Droit de la Preuve
use legalis_fr::evidence::{Evidence, assess_burden_of_proof};

// Droit des Sociétés
use legalis_fr::company::{Company, validate_company_formation};

// Droit Constitutionnel
use legalis_fr::constitution::{assess_constitutionality};

// Droit Administratif
use legalis_fr::administrative::{AdministrativeAct, validate_act};

// Responsabilité Délictuelle (Articles 1240-1244 du Code civil)
use legalis_fr::code_civil::{assess_tort_liability};
```

### Fonctionnalités Avancées

Une fois à l'aise avec les bases, explorez :

1. **[Moteur de Raisonnement Juridique](./user-guide.md#moteur-de-raisonnement-juridique)** - Analyse de cas avancée
2. **[Validateurs Personnalisés](./api-patterns.md#validateurs-personnalisés)** - Étendre la logique de validation
3. **[Sérialisation](./api-patterns.md#sérialisation)** - Sauvegarder/charger des données juridiques
4. **[Droit Comparé](./user-guide.md#droit-comparé)** - Comparer avec le droit allemand/japonais

## 🐛 Dépannage

### Problème : "Cannot find module legalis_fr"

**Solution** : Assurez-vous d'avoir ajouté la dépendance correctement :
```toml
[dependencies]
legalis-fr = "0.2.0"  # Notez le trait d'union, pas le tiret bas
```

Importez avec un tiret bas :
```rust
use legalis_fr::contract::Contract;  // Tiret bas dans le code
```

### Problème : "Le pattern builder retourne une erreur"

**Solution** : Utilisez l'opérateur `?` ou `match` pour gérer `Result` :

```rust
// Option 1 : Utiliser l'opérateur ?
let emploi = Employment::builder()
    .employee_name("Marie Dupont")
    .build()?;  // Propage l'erreur

// Option 2 : Matcher sur Result
match Employment::builder().build() {
    Ok(emp) => println!("Succès : {:?}", emp),
    Err(e) => println!("Erreur : {}", e.message_fr()),
}
```

### Problème : "Champs requis manquants"

**Solution** : Vérifiez les exigences du builder. La plupart des types requièrent :
- Noms/identifiants
- Dates (utilisez `chrono::NaiveDate`)
- Valeurs numériques (montants, âges, etc.)

Utilisez la documentation du type pour voir tous les champs requis :
```bash
cargo doc --open -p legalis-fr
```

### Problème : "L'analyse de date échoue"

**Solution** : Utilisez `NaiveDate::from_ymd_opt()` et gérez `Option` :

```rust
use chrono::NaiveDate;

// Correct
let date = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();

// Ou gérer gracieusement
let date = NaiveDate::from_ymd_opt(2023, 6, 15)
    .ok_or("Date invalide")?;
```

## 💡 Conseils

1. **Commencez simplement** : Débutez avec un domaine juridique pertinent pour votre cas d'usage
2. **Lisez la documentation** : Chaque module a une documentation extensive avec exemples
3. **Vérifiez les tests** : Le répertoire `tests/` contient des exemples réels
4. **Utilisez l'inférence de type** : Laissez Rust inférer les types quand possible
5. **Activez clippy** : `cargo clippy` aide à détecter les erreurs courantes

## 📖 Ressources d'Apprentissage

- **[Guide de l'Utilisateur](./user-guide.md)** - Exemples complets pour tous les domaines
- **[Patterns d'API](./api-patterns.md)** - Bonnes pratiques et patterns de conception
- **[Domaines Juridiques](./legal-domains.md)** - Aperçu détaillé de chaque domaine
- **[Documentation API](https://docs.rs/legalis-fr)** - Référence API complète

## ✅ Liste de Vérification

Avant de passer au Guide de l'Utilisateur, assurez-vous de pouvoir :

- [ ] Ajouter legalis-fr à votre `Cargo.toml`
- [ ] Importer des modules (ex : `use legalis_fr::contract::Contract`)
- [ ] Créer un type en utilisant le pattern builder
- [ ] Appeler une fonction de validation
- [ ] Gérer les erreurs en utilisant `Result` et `?`
- [ ] Travailler avec `chrono::NaiveDate` pour les dates

---

**Prêt pour plus ?** → Continuez vers le **[Guide de l'Utilisateur](./user-guide.md)** pour des exemples complets de tous les domaines juridiques.

---

## 🌐 English Version / Version Anglaise

**Read this in English:** [Getting Started (English)](./getting-started.en.md)
