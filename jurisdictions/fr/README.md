# legalis-fr

Support de la juridiction française pour Legalis-RS

## Aperçu

`legalis-fr` fournit un support pour le système juridique français dans le framework Legalis-RS, incluant le Code civil, le Code pénal et la Constitution de 1958.

## Fonctionnalités

### Code civil

Le Code civil français (Code Napoléon de 1804) est l'une des codifications les plus influentes au monde. Ce crate fournit des implémentations structurées des articles principaux :

- **Article 1240** - Responsabilité civile délictuelle (ancien art. 1382)
- **Article 1241** - Négligence (ancien art. 1383)
- **Article 1242** - Responsabilité du fait d'autrui (ancien art. 1384)

```rust
use legalis_fr::{article_1240, article_1241, article_1242};

let art_1240 = article_1240();
assert_eq!(art_1240.number, "1240");
```

### Code pénal

Support pour le droit pénal français (en développement).

### Constitution de 1958

Constitution de la Cinquième République française (en développement).

## Caractéristiques du système juridique

Le droit français appartient au **système de droit civil (Civil Law)** et présente les caractéristiques suivantes :

- **Codification**: Les codes comme source principale du droit
- **Raisonnement déductif**: Du code au cas particulier
- **Primauté de la loi**: La loi écrite prime sur la jurisprudence

### Comparaison avec la Common Law

| Caractéristique | Droit civil (France) | Common Law (États-Unis) |
|----------------|---------------------|------------------------|
| Source principale | Codes/Lois | Jurisprudence/Précédents |
| Rôle des tribunaux | Application de la loi | Création du droit |
| Raisonnement | Déductif (loi → cas) | Analogique (cas → cas) |
| Force obligatoire | Texte de la loi | Stare decisis |
| Flexibilité | Faible (législateur doit modifier) | Élevée (tribunaux distinguent) |

## Structure du Code civil

Le Code civil est organisé en livres thématiques :

1. **Des personnes** - Droit des personnes
2. **Des biens et des différentes modifications de la propriété** - Droit des biens
3. **Des différentes manières dont on acquiert la propriété** - Obligations et contrats (art. 1240-1242)

## Réforme de 2016

Les articles 1240-1242 correspondent aux anciens articles 1382-1384 avant la réforme du droit des obligations de 2016.

## Dépendances

- `legalis-core` - Types et traits de base
- `serde` - Sérialisation
- `chrono` - Gestion des dates/heures

## Licence

MIT OR Apache-2.0

## Liens

- [Légifrance](https://www.legifrance.gouv.fr/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
