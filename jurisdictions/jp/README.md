# legalis-jp

日本法サポートライブラリ for Legalis-RS

## 概要

`legalis-jp`は、Legalis-RSフレームワークにおける日本の法体系サポートを提供するクレートです。日本特有の法制度、和暦、e-Gov XMLパーサー、主要法令の実装を含みます。

## 機能

### 和暦（元号）サポート

日本の元号システムに対応した日付処理機能を提供します。

- 明治、大正、昭和、平成、令和の各元号に対応
- 西暦と和暦の相互変換
- 法令における正式な日付表記のサポート

```rust
use legalis_jp::{Era, JapaneseDate};

let date = JapaneseDate::new(Era::Reiwa, 5, 1, 1)?;
assert_eq!(date.to_gregorian()?, (2023, 1, 1));
```

### e-Gov XML法令パーサー

e-Gov法令検索のXML形式に対応したパーサーを提供します。

- e-Gov法令データの読み込み
- 条文構造の解析
- バイリンガル（日本語/英語）対応

```rust
use legalis_jp::EGovLawParser;

let parser = EGovLawParser::new();
let law = parser.parse_xml(xml_content)?;
```

### 日本国憲法

日本国憲法の条文を構造化されたデータとして提供します。

- 全103条の憲法条文
- 章・条・項の階層構造
- バイリンガル表記

```rust
use legalis_jp::Constitution;

let constitution = Constitution::new();
let article_9 = constitution.get_article(9)?;
```

### 民法（Minpo）

民法の主要条文、特に不法行為に関する条文を実装しています。

- 第709条（一般不法行為）
- 第710条（財産以外の損害賠償）
- 第715条（使用者責任）

```rust
use legalis_jp::minpo::{article_709, article_710, article_715_1};

let tort_liability = article_709();
assert_eq!(tort_liability.number, "709");
```

## 法体系の特徴

日本法は**大陸法系（Civil Law）**に属し、以下の特徴を持ちます：

- 成文法主義：法典（民法、刑法など）が主要な法源
- 演繹的推論：法典の条文から個別事案へ適用
- 制定法優位：判例は参考だが拘束力なし

### 英米法（Common Law）との比較

| 特徴 | 大陸法（日本） | 英米法（米国） |
|------|--------------|--------------|
| 主要法源 | 法典・制定法 | 判例・先例 |
| 裁判所の役割 | 法の適用 | 法の創造 |
| 推論方法 | 演繹的（法典→事案） | 類推的（判例→判例） |
| 拘束力 | 制定法の文言 | 先例（stare decisis） |

## 依存関係

- `legalis-core` - コア型とトレイト
- `legalis-i18n` - 国際化サポート
- `chrono` - 日時処理
- `quick-xml` - XMLパーサー

## ライセンス

MIT OR Apache-2.0

## 関連リンク

- [e-Gov法令検索](https://elaws.e-gov.go.jp/)
- [日本法令外国語訳データベース](https://www.japaneselawtranslation.go.jp/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
