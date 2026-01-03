# Legalis-RS : L'Architecture de la Jurisprudence Générative

## Séparation du Droit et du Récit : Un Plan pour "Governance as Code"

---

**Auteurs** : Équipe de Développement Legalis-RS
**Version** : 0.2.0
**Langage** : Rust (Édition 2024)
**Licence** : MIT / Apache 2.0

---

## Résumé

Cet article présente **Legalis-RS**, un framework Rust pour séparer et structurer rigoureusement les documents juridiques en langage naturel en **logique déterministe (Code)** et **discrétion judiciaire (Récit)**.

Les systèmes juridiques modernes contiennent un mélange de domaines susceptibles d'automatisation informatique (exigences d'âge, seuils de revenus, calculs de délais) et de domaines nécessitant une interprétation et un jugement humains ("juste cause", "bonnes mœurs"). Les approches précédentes ont soit laissé cette frontière ambiguë, soit tenté une automatisation excessive cherchant à tout rendre calculable.

Legalis-RS introduit un type de logique à trois valeurs `LegalResult<T>` exploitant le système de types de Rust pour rendre cette frontière explicite au niveau du type. Cela permet un nouveau paradigme pour le débogage juridique, la simulation et le portage international tout en empêchant l'"autocratie algorithmique" à l'ère de l'IA.

**Contributions Techniques Clés** :
1. Langage Spécifique au Domaine Juridique (DSL) et implémentation de parseur
2. Vérification formelle avec le solveur SMT Z3
3. Moteur de simulation de style ECS pour la prédiction d'impact social
4. Génération de contrats intelligents pour plus de 25 plateformes blockchain
5. Intégration Linked Open Data (RDF/TTL) pour le web sémantique
6. Implémentations de systèmes juridiques pour 4 pays avec adaptation des paramètres culturels (Soft ODA)

**Philosophie Centrale** : *"Tout ne devrait pas être calculable."*

---

## 1. Introduction

### 1.1 Contexte : La Relation Entre Droit et Computation

La célèbre thèse de Lawrence Lessig "Code is Law" soulignait que l'architecture (code) dans le cyberespace a un pouvoir réglementaire équivalent au droit. Cependant, Legalis-RS inverse cela, adoptant une approche de "**Law becomes Code**" (Le Droit devient Code).

La codification du droit offre les avantages suivants :

- **Vérifiabilité** : Détecter les contradictions logiques à la compilation
- **Simulation** : Prédire les impacts sociaux avant l'application
- **Interopérabilité** : Convertir et comparer entre différents systèmes juridiques
- **Transparence** : Pistes d'audit complètes des processus de décision juridique

Cependant, rendre toutes les lois calculables est dangereux tant philosophiquement que pratiquement. Le droit contient intrinsèquement des domaines nécessitant le "jugement humain", et l'automatisation qui ignore cela peut mener à l'"autocratie de l'IA".

### 1.2 Énoncé du Problème : Défis du Traitement Juridique à l'Ère de l'IA

La technologie juridique moderne (LegalTech) fait face à plusieurs défis fondamentaux :

1. **Gestion de l'Ambiguïté** : De nombreux termes juridiques sont intentionnellement vagues, présupposant une interprétation au cas par cas
2. **Dépendance au Contexte** : La même disposition peut être interprétée différemment selon le contexte social et culturel
3. **Changement Temporel** : Les lois sont amendées et abrogées, nécessitant une gestion de cohérence dans le temps
4. **Différences Internationales** : Les systèmes juridiques de chaque pays diffèrent de leurs fondements philosophiques

### 1.3 Proposition : Séparation de la Calculabilité et de la Discrétion Judiciaire

Le cœur de Legalis-RS est l'introduction de la logique à trois valeurs via le type `LegalResult<T>` :

```rust
pub enum LegalResult<T> {
    /// [Domaine Déterministe] Résultats juridiques traitables automatiquement
    Deterministic(T),

    /// [Domaine Discrétionnaire] Domaine nécessitant le jugement humain
    JudicialDiscretion {
        issue: String,           // La question en jeu
        context_id: Uuid,        // Données contextuelles
        narrative_hint: Option<String>, // Opinion de référence par LLM
    },

    /// [Rupture Logique] Bug dans la loi elle-même
    Void { reason: String },
}
```

Ce type garantit que le résultat du traitement juridique est toujours classé dans l'une des trois catégories. Le système arrête le traitement lorsqu'il atteint `JudicialDiscretion` et délègue le jugement aux humains. Cela devient un "rempart au niveau du type" contre l'autocratie de l'IA.

---

## 2. Travaux Connexes

### 2.1 Histoire du Droit Computationnel

La relation entre droit et ordinateurs remonte au projet LARC (Legal Analysis and Research Computer) dans les années 1950. Elle a depuis évolué à travers les systèmes experts, les systèmes à base de règles et les approches modernes d'apprentissage automatique.

| Ère | Technologie | Caractéristiques |
|-----|-------------|------------------|
| 1950s | LARC | Premier système de recherche d'information juridique |
| 1970s | Systèmes experts type MYCIN | Raisonnement à base de règles |
| 1980s | HYPO | Raisonnement à base de cas |
| 1990s | Standardisation XML/SGML | Structuration des documents juridiques |
| 2000s | Web Sémantique | Représentation des connaissances juridiques basée sur l'ontologie |
| 2010s | Apprentissage Automatique | Modèles de prédiction juridique |
| 2020s | LLM + Vérification Formelle | Approche hybride |

### 2.2 DSL Juridiques Existants

#### Catala (Inria, France)
- **Caractéristiques** : Programmation littéraire, basée sur les portées, typage fort
- **Limitations** : Pas de marquage explicite des domaines discrétionnaires

#### L4 (Singapour)
- **Caractéristiques** : Logique déontique (MUST/MAY/SHANT), raisonnement à base de règles
- **Limitations** : Pas de fonctionnalité de simulation

#### Stipula (Université de Bologne, Italie)
- **Caractéristiques** : Orienté contrats intelligents, machines à états, modèle partie/actif
- **Limitations** : Pas de vérification formelle

---

## 3. Philosophie et Principes de Conception

### 3.1 "Governance as Code, Justice as Narrative"

Le slogan de Legalis-RS reflète la différence essentielle entre gouvernance et justice :

- **Gouvernance** : Application des règles, conformité procédurale, détermination d'éligibilité → **Codifiable**
- **Justice** : Réalisation de l'équité, interprétation contextuelle, jugement de valeur → **Racontée comme récit**

### 3.2 Conception de la Logique à Trois Valeurs

Les trois valeurs de `LegalResult<T>` correspondent aux concepts philosophiques juridiques suivants :

| Type | Concept Philosophique Juridique | Agent de Traitement |
|------|--------------------------------|---------------------|
| `Deterministic(T)` | Règles mécaniquement applicables | Ordinateur |
| `JudicialDiscretion` | Principes nécessitant interprétation | Humain |
| `Void` | Lacunes/contradictions juridiques | Législateur (correction nécessaire) |

### 3.3 "Tout ne devrait pas être calculable"

Contre la tentation de tout rendre calculable, Legalis-RS dit clairement "Non". Les domaines suivants sont intentionnellement conçus comme non-calculables :

1. **Juste cause**
2. **Ordre public et bonnes mœurs**
3. **Bonne foi**
4. **Caractère raisonnable**

---

## 4. Architecture du Système

### 4.1 Vue d'Ensemble de l'Architecture à 7 Couches

```
┌─────────────────────────────────────────────────────────┐
│                  Couche Infrastructure                   │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                     Couche Sortie                        │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│                 Couche Interopérabilité                  │
│                    (legalis-interop)                     │
├─────────────────────────────────────────────────────────┤
│              Couche Internationalisation                 │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│              Couche Simulation & Analyse                 │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                  Couche Intelligence                     │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                      Couche Core                         │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Couche Core

#### legalis-core
Le crate implémentant le cœur philosophique du projet.

**Définitions de Types Clés** :
- `LegalResult<T>` : Type de logique à trois valeurs
- `Statute` : Représentation de base des lois
- `Condition` : Expressions de condition (AND/OR/NOT, âge, revenu, etc.)
- `Effect` : Effets juridiques (Grant/Revoke/Obligation/Prohibition)

#### legalis-dsl
Parseur pour le langage spécifique au domaine juridique.

**Exemple de Syntaxe DSL** :
```
STATUTE adult-voting: "Droits de Vote des Adultes" {
    JURISDICTION "FR"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Droit de vote"

    EXCEPTION WHEN HAS disqualified
    DISCRETION "La capacité mentale nécessite un diagnostic médical"
}
```

### 4.3 Couche Intelligence

#### legalis-llm
Couche d'abstraction des fournisseurs LLM.

**Fournisseurs Supportés** : OpenAI, Anthropic, Google Gemini, LLM Local

#### legalis-verifier
Moteur de vérification formelle avec intégration du solveur SMT Z3.

**Cibles de Vérification** :
- Détection de références circulaires
- Détection de lois inatteignables
- Détection de contradictions logiques
- Vérification de conflits constitutionnels

### 4.4 Couche Simulation

#### legalis-sim
Moteur de simulation de style ECS.

**Caractéristiques** :
- Simulation basée sur la population (supporte des millions d'agents)
- Simulation Monte Carlo
- Analyse de sensibilité
- Tests A/B
- Accélération GPU (CUDA/OpenCL/WebGPU)

### 4.5 Couche Sortie

#### legalis-chain
Génération de contrats intelligents.

**Plateformes Supportées (25+)** :
- EVM : Solidity, Vyper
- Substrate : Ink!
- Move : Aptos, Sui
- StarkNet : Cairo
- Cosmos : CosmWasm

**Contrainte** : Seul `Deterministic` peut être converti (`JudicialDiscretion` ne peut pas être converti)

#### legalis-lod
Sortie Linked Open Data.

**Ontologies Supportées** : ELI, FaBiO, LKIF-Core, Akoma Ntoso, Dublin Core, SKOS

**Formats RDF** : Turtle, N-Triples, RDF/XML, JSON-LD, TriG

---

## 5. Technologies Core

### 5.1 DSL Juridique

**Structure de Base** :
```
STATUTE <id>: "<titre>" {
    [JURISDICTION "<juridiction>"]
    [VERSION <nombre>]
    [EFFECTIVE_DATE <date>]

    WHEN <condition>
    THEN <effet>

    [EXCEPTION WHEN <condition>]
    [DISCRETION "<description>"]
}
```

### 5.2 Type LegalResult<T> et Valeurs de Vérité Partielles

L'évaluation des conditions utilise la logique à 4 valeurs `PartialBool` :

```rust
pub enum PartialBool {
    True,
    False,
    Unknown,      // Information insuffisante
    Contradiction, // Contradiction
}
```

### 5.3 Vérification Formelle avec le Solveur SMT Z3

Les expressions de condition juridique sont converties au format SMT-LIB :

```smt2
(declare-const age Int)
(declare-const income Int)
(declare-const has_citizen Bool)

(assert (and (>= age 18) has_citizen))
(check-sat)
```

---

## 6. Implémentations Juridictionnelles

### 6.1 Système Juridique Japonais

#### Constitution du Japon
Le crate legalis-jp fournit une représentation structurée de la Constitution du Japon.

#### Code Civil Article 709 (Délit)
```
STATUTE minpo-709: "Dommages pour Délit" {
    JURISDICTION "JP"

    WHEN HAS intentional_act OR HAS negligence
    AND HAS violation_of_rights
    AND HAS causation
    AND HAS damages

    THEN OBLIGATION "Indemnisation des dommages"

    DISCRETION "La détermination de la négligence et le calcul des dommages
                sont à la discrétion du tribunal"
}
```

### 6.2 Juridictions Planifiées

| Juridiction | Statut | Domaines Prioritaires |
|-------------|--------|----------------------|
| Allemagne (DE) | En développement | BGB, GG |
| France (FR) | En développement | Code civil, Constitution |
| États-Unis (US) | En développement | UCC, Constitution, Jurisprudence |

---

## 7. Études de Cas

### 7.1 Système de Détermination d'Éligibilité aux Prestations Sociales

Détermination automatique d'éligibilité pour 6 programmes de prestations :
1. Aide sociale de base
2. Supplément de pension pour seniors
3. Allocation de soutien à l'enfance
4. Aide aux personnes handicapées
5. Aide au logement d'urgence
6. Subvention santé

**Résultats** :
- Décisions déterministes : 85% des cas
- JudicialDiscretion : 15% des cas

### 7.2 Simulation de l'Article 709 du Code Civil (Délit)

5 scénarios simulés :
1. Délit intentionnel clair → `Deterministic(Liable)`
2. Délit par négligence → `Deterministic(Liable)`
3. Cas limite → `JudicialDiscretion`
4. Pas de délit → `Deterministic(NotLiable)`
5. Pas de causalité → `Deterministic(NotLiable)`

### 7.3 Analyse Comparative du Droit des Délits dans 4 Pays

| Pays | Code | Caractéristiques |
|------|------|------------------|
| Japon | Code Civil Art. 709 | Clause générale (large discrétion) |
| Allemagne | BGB §823/§826 | Intérêts protégés énumérés |
| France | Code civil Art. 1240 | Abstraction maximale |
| États-Unis | Jurisprudence | Typifié (Battery, etc.) |

---

## 8. Référence API et Détails Techniques

### 8.1 Types et Traits Clés

```rust
// Type de logique à trois valeurs
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion { issue: String, context_id: Uuid, narrative_hint: Option<String> },
    Void { reason: String },
}

// Trait d'entité juridique
pub trait LegalEntity: Send + Sync {
    fn id(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn attributes(&self) -> &[String];
}
```

### 8.2 Points de Terminaison REST API / GraphQL

| Méthode | Point de Terminaison | Description |
|---------|---------------------|-------------|
| GET | /api/v1/statutes | Obtenir la liste des lois |
| POST | /api/v1/verify | Exécuter la vérification |
| POST | /api/v1/simulate | Exécuter la simulation |

### 8.3 Système de Commandes CLI

```bash
legalis parse <fichier.dsl> [--format json|yaml]
legalis verify <fichier.dsl> [--strict]
legalis simulate <fichier.dsl> --population 1000
legalis visualize <fichier.dsl> --output tree.svg
legalis export <fichier.dsl> --format solidity|catala|l4|rdf
```

---

## 9. Évaluation

### 9.1 Benchmarks de Performance

| Opération | Cible | Temps |
|-----------|-------|-------|
| Parsing DSL | 100 lois | 15ms |
| Vérification | 100 lois | 250ms |
| Simulation | 10 000 agents | 1.2s |
| Simulation | 100 000 agents | 8.5s |

### 9.2 Qualité du Code

- **Couverture de tests** : Tests d'intégration, tests de propriétés, tests de snapshot
- **Analyse statique** : Clippy (politique zéro avertissement)
- **Documentation** : rustdoc pour toutes les API publiques

---

## 10. Travaux Futurs

- Interface Web UI (React)
- Extension VS Code
- Intégration Jupyter Notebook
- Juridictions supplémentaires (Droit UE, Droit international)

---

## 11. Conclusion

Legalis-RS présente une nouvelle approche de codification du droit en rendant la "frontière entre calculabilité et jugement humain" explicite dans le système de types.

**Réalisations Clés** :
1. **Fondement philosophique** : "Governance as Code, Justice as Narrative"
2. **Système de types** : Logique à trois valeurs via `LegalResult<T>`
3. **Architecture intégrée** : Conception complète avec 7 couches et 16 crates
4. **Implémentation** : Environ 450 000 lignes de code Rust
5. **Vérification** : Intégration du solveur SMT Z3
6. **Simulation** : Moteur de style ECS (support accélération GPU)
7. **Sortie** : 25+ blockchains, RDF/TTL, formats multiples

**Philosophie Centrale** : *"Tout ne devrait pas être calculable."*

---

## Références

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

*"Code is Law", dit-on, mais nous adoptons l'approche "Law becomes Code". Cependant, nous intégrons dans ce code un type appelé 'Humanité'.*

---

**Équipe de Développement Legalis-RS**
Version 0.2.0 | 2024
