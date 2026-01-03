# Legalis-RS：生成式法学的架构

## 法律与叙事的分离：「代码即治理」的设计蓝图

---

**作者**：Legalis-RS 开发团队
**版本**：0.2.0
**语言**：Rust（2024 版）
**许可证**：MIT / Apache 2.0

---

## 摘要

本文介绍 **Legalis-RS**，一个用于将自然语言法律文档严格分离和结构化为**确定性逻辑（代码）**和**司法裁量权（叙事）**的 Rust 框架。

现代法律系统包含可进行计算机自动化处理的领域（年龄要求、收入门槛、期限计算）和需要人类解释与判断的领域（"正当理由"、"公序良俗"）的混合体。以往的方法要么使这一边界模糊不清，要么试图通过过度自动化使一切都变得可计算。

Legalis-RS 引入了三值逻辑类型 `LegalResult<T>`，利用 Rust 的类型系统在类型层面明确这一边界。这为法律调试、模拟和国际移植提供了新的范式，同时防止了人工智能时代的"算法独裁"。

**主要技术贡献**：
1. 法律领域特定语言（DSL）和解析器实现
2. 使用 Z3 SMT 求解器进行形式验证
3. 用于预测社会影响的 ECS 风格模拟引擎
4. 为 25+ 区块链平台生成智能合约
5. 用于语义网的关联开放数据（RDF/TTL）集成
6. 4 个国家的法律系统实现，具有文化参数适配（软性 ODA）

**核心哲学**：*「并非一切都应该是可计算的。」*

---

## 1. 引言

### 1.1 背景：法律与计算的关系

Lawrence Lessig 著名的论点「代码即法律」指出，网络空间中的架构（代码）具有与法律同等的规制力量。然而，Legalis-RS 将其反转，采用「**法律成为代码**」的方法。

法律的代码化提供以下优势：

- **可验证性**：在编译时检测逻辑矛盾
- **模拟性**：在执行前预测社会影响
- **互操作性**：在不同法律系统之间转换和比较
- **透明性**：法律决策过程的完整审计追踪

然而，将所有法律都变得可计算，无论从哲学还是实践角度来看都是危险的。法律本质上包含需要「人类判断」的领域，忽视这一点的自动化可能导致「人工智能独裁」。

### 1.2 问题陈述：人工智能时代法律处理的挑战

现代法律科技（LegalTech）面临几个根本性挑战：

1. **模糊性处理**：许多法律术语故意模糊，预设逐案解释
2. **语境依赖性**：同一条款可能因社会和文化语境不同而有不同解释
3. **时间变化**：法律被修订和废除，需要跨时间的一致性管理
4. **国际差异**：各国法律系统从其哲学基础就存在差异

### 1.3 提案：可计算性与司法裁量权的分离

Legalis-RS 的核心是通过 `LegalResult<T>` 类型引入三值逻辑：

```rust
pub enum LegalResult<T> {
    /// 【确定性领域】可自动处理的法律结果
    Deterministic(T),

    /// 【裁量领域】需要人类判断的领域
    JudicialDiscretion {
        issue: String,           // 争议问题
        context_id: Uuid,        // 上下文数据
        narrative_hint: Option<String>, // LLM 提供的参考意见
    },

    /// 【逻辑崩溃】法律本身的错误
    Void { reason: String },
}
```

该类型保证法律处理的结果始终被分类为三个类别之一。系统在达到 `JudicialDiscretion` 时停止处理并将判断权委托给人类。这成为对抗人工智能独裁的「类型级堡垒」。

---

## 2. 相关工作

### 2.1 计算法学的历史

法律与计算机的关系可追溯到 1950 年代的 LARC（法律分析与研究计算机）项目。此后，它经历了专家系统、基于规则的系统和现代机器学习方法的演变。

| 年代 | 技术 | 特征 |
|------|------|------|
| 1950 年代 | LARC | 首个法律信息检索系统 |
| 1970 年代 | MYCIN 型专家系统 | 基于规则的推理 |
| 1980 年代 | HYPO | 基于案例的推理 |
| 1990 年代 | XML/SGML 标准化 | 法律文档结构化 |
| 2000 年代 | 语义网 | 基于本体的法律知识表示 |
| 2010 年代 | 机器学习 | 法律预测模型 |
| 2020 年代 | LLM + 形式验证 | 混合方法 |

### 2.2 现有法律 DSL

#### Catala（法国 Inria）
- **特点**：文学化编程、基于作用域、强类型
- **局限性**：无裁量领域的显式标记

#### L4（新加坡）
- **特点**：义务逻辑（MUST/MAY/SHANT）、基于规则的推理
- **局限性**：无模拟功能

#### Stipula（意大利博洛尼亚大学）
- **特点**：面向智能合约、状态机、当事人/资产模型
- **局限性**：无形式验证

---

## 3. 哲学与设计原则

### 3.1 「代码即治理，叙事即正义」

Legalis-RS 的口号反映了治理与正义之间的本质区别：

- **治理**：规则应用、程序合规、资格判定 → **可代码化**
- **正义**：公平实现、语境解释、价值判断 → **以叙事方式讲述**

### 3.2 三值逻辑的设计

`LegalResult<T>` 的三个值对应以下法哲学概念：

| 类型 | 法哲学概念 | 处理主体 |
|------|-----------|---------|
| `Deterministic(T)` | 可机械适用的规则 | 计算机 |
| `JudicialDiscretion` | 需要解释的原则 | 人类 |
| `Void` | 法律漏洞/矛盾 | 立法者（需要修正） |

### 3.3 「并非一切都应该是可计算的」

针对使一切都可计算的诱惑，Legalis-RS 明确说「不」。以下领域被有意设计为不可计算：

1. **正当理由**
2. **公序良俗**
3. **诚实信用**
4. **合理性**

---

## 4. 系统架构

### 4.1 七层架构概览

```
┌─────────────────────────────────────────────────────────┐
│                      基础设施层                          │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                        输出层                            │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│                      互操作层                            │
│                    (legalis-interop)                     │
├─────────────────────────────────────────────────────────┤
│                      国际化层                            │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│                   模拟与分析层                           │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                       智能层                             │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                       核心层                             │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 核心层

#### legalis-core
实现项目哲学核心的 crate。

**主要类型定义**：
- `LegalResult<T>`：三值逻辑类型
- `Statute`：法律的基本表示
- `Condition`：条件表达式（AND/OR/NOT、年龄、收入等）
- `Effect`：法律效果（Grant/Revoke/Obligation/Prohibition）

#### legalis-dsl
法律领域特定语言的解析器。

**DSL 语法示例**：
```
STATUTE adult-voting: "成年人投票权" {
    JURISDICTION "CN"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "投票权"

    EXCEPTION WHEN HAS disqualified
    DISCRETION "精神能力的判定需要医生诊断"
}
```

### 4.3 智能层

#### legalis-llm
LLM 提供者抽象层。

**支持的提供者**：OpenAI、Anthropic、Google Gemini、本地 LLM

#### legalis-verifier
集成 Z3 SMT 求解器的形式验证引擎。

**验证目标**：
- 循环引用检测
- 不可达法律检测
- 逻辑矛盾检测
- 宪法冲突检查

### 4.4 模拟层

#### legalis-sim
ECS 风格模拟引擎。

**功能**：
- 基于人口的模拟（支持数百万代理）
- 蒙特卡洛模拟
- 敏感性分析
- A/B 测试
- GPU 加速（CUDA/OpenCL/WebGPU）

### 4.5 输出层

#### legalis-chain
智能合约生成。

**支持的平台（25+）**：
- EVM：Solidity、Vyper
- Substrate：Ink!
- Move：Aptos、Sui
- StarkNet：Cairo
- Cosmos：CosmWasm

**约束**：只有 `Deterministic` 可以转换（`JudicialDiscretion` 无法转换）

#### legalis-lod
关联开放数据输出。

**支持的本体**：ELI、FaBiO、LKIF-Core、Akoma Ntoso、Dublin Core、SKOS

**RDF 格式**：Turtle、N-Triples、RDF/XML、JSON-LD、TriG

---

## 5. 核心技术

### 5.1 法律 DSL

**基本结构**：
```
STATUTE <id>: "<标题>" {
    [JURISDICTION "<管辖区>"]
    [VERSION <数字>]
    [EFFECTIVE_DATE <日期>]

    WHEN <条件>
    THEN <效果>

    [EXCEPTION WHEN <条件>]
    [DISCRETION "<描述>"]
}
```

### 5.2 LegalResult<T> 类型与部分真值

条件评估使用四值逻辑 `PartialBool`：

```rust
pub enum PartialBool {
    True,
    False,
    Unknown,      // 信息不足
    Contradiction, // 矛盾
}
```

### 5.3 使用 Z3 SMT 求解器的形式验证

法律条件表达式被转换为 SMT-LIB 格式：

```smt2
(declare-const age Int)
(declare-const income Int)
(declare-const has_citizen Bool)

(assert (and (>= age 18) has_citizen))
(check-sat)
```

---

## 6. 管辖区实现

### 6.1 日本法律系统

#### 日本宪法
legalis-jp crate 提供日本宪法的结构化表示。

#### 民法第 709 条（侵权）
```
STATUTE minpo-709: "侵权损害赔偿" {
    JURISDICTION "JP"

    WHEN HAS intentional_act OR HAS negligence
    AND HAS violation_of_rights
    AND HAS causation
    AND HAS damages

    THEN OBLIGATION "损害赔偿"

    DISCRETION "过失的认定和损害的计算由法院裁量决定"
}
```

### 6.2 计划中的管辖区

| 管辖区 | 状态 | 重点领域 |
|--------|------|---------|
| 德国（DE） | 开发中 | BGB、GG |
| 法国（FR） | 开发中 | 民法典、宪法 |
| 美国（US） | 开发中 | UCC、宪法、判例法 |

---

## 7. 案例研究

### 7.1 社会福利资格判定系统

6 个福利项目的自动资格判定：
1. 基本福利援助
2. 老年养老金补贴
3. 儿童抚养津贴
4. 残疾人援助
5. 紧急住房援助
6. 医疗保健补贴

**结果**：
- 确定性决策：85% 的案例
- JudicialDiscretion：15% 的案例

### 7.2 民法第 709 条（侵权）模拟

模拟的 5 个场景：
1. 明确的故意侵权 → `Deterministic(Liable)`
2. 过失侵权 → `Deterministic(Liable)`
3. 边界案例 → `JudicialDiscretion`
4. 无侵权 → `Deterministic(NotLiable)`
5. 无因果关系 → `Deterministic(NotLiable)`

### 7.3 四国侵权法比较分析

| 国家 | 法律 | 特征 |
|------|------|------|
| 日本 | 民法第 709 条 | 一般条款（广泛裁量） |
| 德国 | BGB §823/§826 | 列举保护利益 |
| 法国 | 民法典第 1240 条 | 最大抽象性 |
| 美国 | 判例法 | 类型化（Battery 等） |

---

## 8. API 参考与技术细节

### 8.1 主要类型与 Trait

```rust
// 三值逻辑类型
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion { issue: String, context_id: Uuid, narrative_hint: Option<String> },
    Void { reason: String },
}

// 法律实体 Trait
pub trait LegalEntity: Send + Sync {
    fn id(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn attributes(&self) -> &[String];
}
```

### 8.2 REST API / GraphQL 端点

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | /api/v1/statutes | 获取法律列表 |
| POST | /api/v1/verify | 执行验证 |
| POST | /api/v1/simulate | 执行模拟 |

### 8.3 CLI 命令系统

```bash
legalis parse <file.dsl> [--format json|yaml]
legalis verify <file.dsl> [--strict]
legalis simulate <file.dsl> --population 1000
legalis visualize <file.dsl> --output tree.svg
legalis export <file.dsl> --format solidity|catala|l4|rdf
```

---

## 9. 评估

### 9.1 性能基准测试

| 操作 | 目标 | 时间 |
|------|------|------|
| DSL 解析 | 100 条法律 | 15ms |
| 验证 | 100 条法律 | 250ms |
| 模拟 | 10,000 代理 | 1.2s |
| 模拟 | 100,000 代理 | 8.5s |

### 9.2 代码质量

- **测试覆盖率**：集成测试、属性测试、快照测试
- **静态分析**：Clippy（零警告策略）
- **文档**：所有公共 API 的 rustdoc

---

## 10. 未来工作

- Web UI 前端（React）
- VS Code 扩展
- Jupyter Notebook 集成
- 更多管辖区（欧盟法、国际法）

---

## 11. 结论

Legalis-RS 通过在类型系统中明确「可计算性与人类判断之间的边界」，提出了一种新的法律代码化方法。

**主要成就**：
1. **哲学基础**：「代码即治理，叙事即正义」
2. **类型系统**：通过 `LegalResult<T>` 实现的三值逻辑
3. **集成架构**：包含 7 层和 16 个 crate 的综合设计
4. **实现**：约 450,000 行 Rust 代码
5. **验证**：Z3 SMT 求解器集成
6. **模拟**：ECS 风格引擎（支持 GPU 加速）
7. **输出**：25+ 区块链、RDF/TTL、多种格式

**核心哲学**：*「并非一切都应该是可计算的。」*

---

## 参考文献

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

*人们说「代码即法律」，但我们采取「法律成为代码」的方法。然而，我们在这段代码中嵌入了一个名为「人性」的类型。*

---

**Legalis-RS 开发团队**
版本 0.2.0 | 2024
