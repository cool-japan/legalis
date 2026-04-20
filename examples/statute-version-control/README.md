# statute-version-control

This example demonstrates full statute lifecycle management using `legalis-registry`. Three versions of an "Adult Age Definition Act" are registered — Version 1 (age 20, pre-2022), Version 2 (age 18, reflecting Japan's 2022 Civil Code amendment), and Version 3 (age 18 with nationality clarification) — to show version tracking and rollback via point-in-time snapshots. The example also covers full and incremental backups, tag-based and jurisdiction-based search, and paginated retrieval of large statute collections using `SearchQuery` and `Pagination`.

## Usage

```sh
cargo run -p statute-version-control --all-features
```

## What It Demonstrates

- `StatuteRegistry` with versioned `StatuteEntry` records and `StatuteStatus` lifecycle
- Point-in-time snapshot creation and rollback
- Full and incremental backup support
- Tag-based, jurisdiction-based, and full-text statute search
- Paginated retrieval via `Pagination` for large collections
- Real-world amendment modelling: Japan's 2022 age-of-majority change

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
