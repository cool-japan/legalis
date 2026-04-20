# jp-constitution-3d

This example generates a three-dimensional hierarchical visualisation of the Japanese Constitution using Legalis-RS and `legalis-jp`. The "3D" refers to three structural layers: chapters (章), articles (条), and paragraphs/items (項/号). Key articles are instantiated — Article 1 (the Emperor as symbol), Article 9 (renunciation of war), Articles 11/13/14/19/21/25 (fundamental rights), and Article 97 (eternal human rights) — then rendered as an ASCII tree, a Mermaid diagram, and exported to JSON for downstream use.

## Usage

```sh
cargo run -p jp-constitution-3d --all-features
```

## What It Demonstrates

- Bilingual text representation (Japanese and English) for constitutional articles
- Three-layer hierarchical structure: chapter → article → paragraph
- Multiple export formats: JSON, Mermaid diagram, ASCII tree
- Using `legalis-jp` jurisdiction-specific constitution models
- Decision-tree visualisation via `legalis-viz`

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
