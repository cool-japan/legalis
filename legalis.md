# **Project Legalis-RS: The Architecture of Generative Jurisprudence**

## **～法と物語の分離、そして「統治のコード化」への青写真～**

Version: 0.2.0 (Standalone Edition)  
Language: Rust (Edition 2024\)  
License: MIT / Apache 2.0 (Open Source)  
Core Philosophy: "Governance as Code, Justice as Narrative"

## **1\. エグゼクティブ・サマリー**

**Legalis-RS**は、自然言語で記述された法律・契約・ルールを、\*\*「決定論的な論理（Code）」**と**「裁量が必要な物語（Narrative）」\*\*に厳密に分離・構造化するためのRust製フレームワークである。

本プロジェクトは、外部の特定ツールに依存せず、Rust標準のエコシステムを活用して**自己完結**する形で以下の実現を目指す。

1. **デバッグ法学:** 法案の論理的矛盾（バグ）をコンパイルエラーとして検出する。  
2. **シミュレーション法学:** 法施行前の社会影響をデジタルツイン上で高速に検証する。  
3. **ソフトODAの自動化:** 日本法のカーネルを他国の文化・言語に合わせて「移植（Porting）」する。

## **2\. システムアーキテクチャ (Workspace構成)**

RustのWorkspace機能を活用し、モノレポ構成で開発する。外部サービス（LLM等）への接続はアダプターパターンで抽象化し、特定のベンダーに依存しない設計とする。

\[workspace\]  
members \= \[  
    "crates/legalis-core",      \# 型定義・状態管理・共通トレイト  
    "crates/legalis-dsl",       \# 法記述言語 (Parser / AST / Macro)  
    "crates/legalis-llm",       \# LLM連携 (抽象化レイヤー)  
    "crates/legalis-sim",       \# シミュレーション (ECSエンジン)  
    "crates/legalis-verifier",  \# 形式検証 (SMT Solver連携)  
    "crates/legalis-chain",     \# Smart Contract Export (WASM/Solidity)  
\]

\[workspace.dependencies\]  
serde \= { version \= "1.0", features \= \["derive"\] }  
serde\_json \= "1.0"  
tokio \= { version \= "1.0", features \= \["full"\] }  
anyhow \= "1.0"  
uuid \= { version \= "1.0", features \= \["v4"\] }  
reqwest \= { version \= "0.11", features \= \["json"\] } \# LLM APIコール用  
async-trait \= "0.1"

## **3\. Core Module: 「曖昧さ」の型定義**

本プロジェクトの哲学的核となる部分。  
\*\*「すべてを計算可能にしない」\*\*ことで、AI神政（ディストピア）を防ぐ。人間の判断が必要な領域をJudicialDiscretion型として明示的に残す。

### **crates/legalis-core/src/lib.rs**

use serde::{Deserialize, Serialize};  
use uuid::Uuid;

/// 法的判断の結果を表す代数的データ型 (ADT)  
\#\[derive(Debug, Clone, Serialize, Deserialize)\]  
pub enum LegalResult\<T\> {  
    /// 【決定論的領域】  
    /// 計算により自動的に導出された結果。  
    /// (例: 年齢要件、所得制限、期限の計算)  
    Deterministic(T),

    /// 【裁量領域】  
    /// 論理だけでは決定できず、人間の「物語（解釈）」が必要な領域。  
    /// ここに「AI神政」への防波堤がある。  
    /// システムはここで停止し、人間にボールを投げる。  
    JudicialDiscretion {  
        /// 争点 (例: "正当な事由の有無", "公共の福祉への反則")  
        issue: String,  
        /// 参照すべき文脈データ  
        context\_id: Uuid,  
        /// 推奨される判断材料（LLMが生成するが、決定はしない）  
        narrative\_hint: Option\<String\>,  
    },

    /// 【論理破綻】  
    /// 法律自体のバグ。  
    Void { reason: String },  
}

/// 法的主体 (自然人、法人、あるいはAIエージェント)  
pub trait LegalEntity {  
    fn id(\&self) \-\> Uuid;  
    fn get\_attribute(\&self, key: \&str) \-\> Option\<String\>;  
}

/// 条文 (Statute) の定義  
\#\[derive(Debug, Clone, Serialize, Deserialize)\]  
pub struct Statute {  
    pub id: String,  
    pub title: String,  
    /// 適用要件 (If)  
    pub preconditions: Vec\<Condition\>,  
    /// 法的効果 (Then)  
    pub effect: Effect,  
    /// 裁量ロジック (Else If Maybe)  
    pub discretion\_logic: Option\<String\>,   
}

// 共通型定義のプレースホルダー  
\#\[derive(Debug, Clone, Serialize, Deserialize)\]  
pub struct Condition { /\* ... \*/ }  
\#\[derive(Debug, Clone, Serialize, Deserialize)\]  
pub struct Effect { /\* ... \*/ }

## **4\. LLM Module: 汎用的な「知性のアダプター」**

特定のLLMオーケストレーターに依存せず、LLMProvider トレイトを通じて任意のAIモデル（OpenAI, Anthropic, Gemini, Local LLM）をプラグイン可能な設計にする。

### **crates/legalis-llm/src/lib.rs**

use async\_trait::async\_trait;  
use anyhow::Result;

/// LLMプロバイダーの抽象化トレイト  
\#\[async\_trait\]  
pub trait LLMProvider {  
    /// プロンプトを受け取り、テキスト応答を返す  
    async fn generate\_text(\&self, prompt: \&str) \-\> Result\<String\>;  
      
    /// 構造化データ（JSON等）への変換を強制する  
    async fn generate\_structured\<T: serde::de::DeserializeOwned\>(\&self, prompt: \&str) \-\> Result\<T\>;  
}

/// 具体的な実装例: OpenAI (または互換API)  
pub struct OpenAiClient {  
    api\_key: String,  
    model: String,  
    client: reqwest::Client,  
}

impl OpenAiClient {  
    pub fn new(api\_key: String, model: String) \-\> Self {  
        Self {  
            api\_key,  
            model,  
            client: reqwest::Client::new(),  
        }  
    }  
}

\#\[async\_trait\]  
impl LLMProvider for OpenAiClient {  
    async fn generate\_text(\&self, prompt: \&str) \-\> Result\<String\> {  
        // ここに実際のリクエストロジックを実装  
        // reqwestを使ってAPIエンドポイントを叩く  
        Ok("Mock response for now".to\_string())  
    }

    async fn generate\_structured\<T: serde::de::DeserializeOwned\>(\&self, prompt: \&str) \-\> Result\<T\> {  
        let text \= self.generate\_text(prompt).await?;  
        // JSONパース処理  
        let data \= serde\_json::from\_str(\&text)?;  
        Ok(data)  
    }  
}

### **crates/legalis-llm/src/compiler.rs**

use crate::LLMProvider;  
use legalis\_core::Statute;  
use anyhow::Result;

pub struct LawCompiler\<P: LLMProvider\> {  
    provider: P,  
}

impl\<P: LLMProvider\> LawCompiler\<P\> {  
    pub fn new(provider: P) \-\> Self {  
        Self { provider }  
    }

    /// 自然言語の条文をRustコード（AST）に変換  
    pub async fn compile(\&self, raw\_text: \&str) \-\> Result\<Statute\> {  
        let system\_prompt \= "あなたは『法務コンパイラ』です。自然言語の条文をRustの構造体に変換します。解釈が分かれる部分は必ず'JudicialDiscretion'としてマークしてください。";  
        let prompt \= format\!("{}\\n\\n以下の条文を解析せよ: {}", system\_prompt, raw\_text);

        // プロバイダー経由で構造化データを取得  
        self.provider.generate\_structured(\&prompt).await  
    }  
}

## **5\. Sim Module: インメモリ並列シミュレーション**

外部のタスクキューシステムを使わず、Rust標準の非同期ランタイム（Tokio）とチャネルを用いた、シンプルかつ高速なECS（Entity Component System）ライクなシミュレーターを構築する。

### **crates/legalis-sim/src/engine.rs**

use tokio::sync::mpsc;  
use legalis\_core::{Statute, LegalEntity, LegalResult, Effect};  
use std::sync::Arc;

/// シミュレーションエンジン  
pub struct SimEngine {  
    /// シミュレーション対象の法案  
    statutes: Vec\<Statute\>,  
    /// 市民エージェント（並行処理のためArcで共有）  
    population: Vec\<Arc\<dyn LegalEntity \+ Send \+ Sync\>\>,  
}

impl SimEngine {  
    pub fn new(statutes: Vec\<Statute\>, population: Vec\<Box\<dyn LegalEntity \+ Send \+ Sync\>\>) \-\> Self {  
        Self {  
            statutes,  
            population: population.into\_iter().map(Arc::from).collect(),  
        }  
    }

    /// シミュレーション実行  
    pub async fn run\_simulation(\&self) {  
        let (tx, mut rx) \= mpsc::channel(1000);

        println\!("Starting simulation with {} agents...", self.population.len());

        for agent in \&self.population {  
            let agent\_ref \= agent.clone();  
            let statutes\_ref \= self.statutes.clone();  
            let tx\_clone \= tx.clone();

            // Tokioタスクとして並列実行  
            tokio::spawn(async move {  
                for statute in statutes\_ref {  
                    let result \= Self::apply\_law(agent\_ref.as\_ref(), \&statute);  
                    // 結果をレシーバーに送信  
                    let \_ \= tx\_clone.send((agent\_ref.id(), statute.id, result)).await;  
                }  
            });  
        }

        // 送信側をドロップして受信ループを終了できるようにする  
        drop(tx);

        // 結果の集計  
        while let Some((agent\_id, statute\_id, result)) \= rx.recv().await {  
            match result {  
                LegalResult::Deterministic(\_) \=\> {  
                    // 統計データに加算  
                }  
                LegalResult::JudicialDiscretion { issue, .. } \=\> {  
                    println\!("Conflict detected for Agent {}: Law {} \-\> {}", agent\_id, statute\_id, issue);  
                }  
                \_ \=\> {}  
            }  
        }  
    }

    fn apply\_law(agent: &(dyn LegalEntity \+ Send \+ Sync), law: \&Statute) \-\> LegalResult\<Effect\> {  
        // 法適用ロジック（要件判定 \-\> 効果発生）  
        // 実際には legalis-core のロジックを呼び出す  
          
        // 仮実装: 裁量が必要なケースのシミュレーション  
        if law.discretion\_logic.is\_some() {  
             return LegalResult::JudicialDiscretion {  
                issue: "Discretion required".to\_string(),  
                context\_id: agent.id(),  
                narrative\_hint: None,  
            };  
        }  
          
        // 仮実装: 決定論的  
        LegalResult::Deterministic(law.effect.clone())  
    }  
}

## **6\. Verifier Module: 形式検証 (Formal Verification)**

「バグのある法律」を世に出さないための静的解析器。  
RustからZ3 (SMT Solver) を呼び出し、論理矛盾をチェックする。

### **crates/legalis-verifier/src/lib.rs**

// 概念コード  
// z3 クレートなどを利用することを想定  
pub fn verify\_integrity(laws: &\[Statute\]) \-\> Result\<(), String\> {  
    // 1\. 循環参照チェック  
    // 条文Aの要件が条文Bに依存し、BがAに依存していないかグラフ探索

    // 2\. デッドロックチェック  
    // どのような入力（市民の属性）を与えても、決してTrueにならない「死に条文」がないか

    // 3\. 憲法適合性チェック  
    // 上位規定（Constitution）と論理的に矛盾する（AかつNot A）パスが存在しないか  
      
    // Z3 Solverへの変換ロジックをここに実装  
    Ok(())  
}

## **7\. ユースケースとロードマップ**

このプロジェクトが目指す社会実装のフェーズ。

### **Phase 1: "The Visualizer" (可視化)**

* **Target:** 地方自治体の条例。  
* **Action:** 複雑怪奇な「補助金支給要件」をLegalis-RSに入力。  
* **Output:** 「誰がもらえて、誰がもらえないか」の決定木を自動生成し、フローチャートとして可視化する。また、「解釈が曖昧なグレーゾーン」を赤色でハイライトする。

### **Phase 2: "The Debugger" (国会DX)**

* **Target:** 新規法案の作成プロセス。  
* **Action:** 官僚が書いた法案テキストをLLM経由でコンパイルする。  
* **Output:** Verifierが走り、「第X条と第Y条が論理矛盾しています（コンパイルエラー）」と警告を出す。

### **Phase 3: "The Soft ODA" (法制度輸出)**

* **Target:** 途上国の法整備支援。  
* **Action:** 日本の商法（J-Kernel）をベースに、現地の文化パラメータ（宗教的タブーなど）を注入。  
* **Output:** 現地語で記述され、かつ論理的に整合性が取れた「現地版商法」を自動生成する。

### **Phase 4: "The Hybrid Court" (半自動裁判)**

* **Target:** 少額訴訟や行政手続き。  
* **Action:** LegalResult::Deterministic の案件は即時自動処理（ブロックチェーンで執行）。  
* **Human Loop:** JudicialDiscretion が返された案件のみ、人間の裁判官・調停委員に回される。これにより、人間は「創造的な解釈（物語の紡ぎ出し）」のみに集中できる。

## **8\. 結び：エンジニアリング・チームへのメッセージ**

「コードは法律である（Code is Law）」と言うが、我々は「法律をコードにする（Law becomes Code）」アプローチをとる。  
ただし、そのコードには『人間性（Humanity）』という名の型（Type）を埋め込む。

Legalis-RS は、外部のブラックボックスに依存しない、純粋な Rust Native な法務カーネルです。  
LLMやシミュレーション基盤をプラグイン可能なモジュールとして設計することで、特定のベンダーロックインを回避し、国家レベルのインフラとして耐えうる堅牢性を目指します。  
まずは cargo new legalis-rs \--workspace から始めましょう。  
20年前のUMLの夢を、Rustで実装する時が来ました。