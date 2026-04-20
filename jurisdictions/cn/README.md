# legalis-cn

中华人民共和国 (China) 法律体系支持 - Legalis-RS

**版本 0.1.3** - 民法典、网络安全法、个人信息保护法、劳动法

## 概述 (Overview)

`legalis-cn` 为 Legalis-RS 框架提供中华人民共和国法律体系的全面支持。中国实行中国特色社会主义法律体系，以成文法为主，结合民法典传统与社会主义法制原则。

## 中国法律体系 (Chinese Legal System)

中国法律体系的特点：
- **社会主义法制** - 中国特色社会主义法律体系
- **成文法传统** - 以法典和法律法规为主要法源
- **民法典** - 2021年实施的《中华人民共和国民法典》
- **宪法至上** - 中华人民共和国宪法（1982年，2018年修正）
- **行政法规** - 国务院制定的行政法规具有重要地位

### 与其他法律体系的比较

| 特征 | 中国 | 日本 | 德国 | 美国 |
|------|------|------|------|------|
| 法系 | 社会主义民法 | 大陆法系 | 大陆法系 | 普通法系 |
| 主要法源 | 法典与法规 | 法典 | 法典 | 判例法 |
| 宪法 | 1982年 | 1946年 | 1949年 | 1787年 |
| 法院体系 | 四级两审 | 四级三审 | 三级 | 联邦与州 |
| 最高法院 | 最高人民法院 | 最高裁判所 | 联邦宪法法院 | 联邦最高法院 |

## 已实现功能 (Implemented Features)

### ✅ 劳动合同法 (Labor Contract Law)

《中华人民共和国劳动合同法》（2008年，2012年修正）
- ✅ 劳动合同类型（固定期限、无固定期限、以完成一定工作任务为期限）
- ✅ 劳动合同订立、变更、解除、终止
- ✅ 经济补偿金计算
- ✅ 试用期规定
- ✅ 社会保险义务
- ✅ 劳务派遣规定

```rust
use legalis_cn::labor_contract::{LaborContract, ContractType, SeveranceCalculator};

let contract = LaborContract::new()
    .employee_name("张三")
    .contract_type(ContractType::FixedTerm { months: 36 })
    .monthly_salary(15_000) // 人民币
    .start_date("2022-01-01")
    .probation_period_months(3) // 3年合同最多3个月试用期
    .build()?;

// 计算经济补偿金（N+1）
let severance = SeveranceCalculator::calculate(&contract, 5 /* 工作年限 */)?;
// 每满一年支付一个月工资
```

### ✅ 公司法 (Company Law)

《中华人民共和国公司法》（2023年修订）
- ✅ 公司类型（有限责任公司、股份有限公司）
- ✅ 注册资本认缴制
- ✅ 股东权利与义务
- ✅ 董事、监事、高级管理人员的义务
- ✅ 公司治理结构

```rust
use legalis_cn::company_law::{Company, CompanyType, validate_formation};

let company = Company::new()
    .name("北京科技有限公司")
    .company_type(CompanyType::LimitedLiabilityCompany)
    .registered_capital(1_000_000) // 人民币
    .shareholders(vec!["股东甲", "股东乙"])
    .legal_representative("李四")
    .build()?;

assert!(validate_formation(&company).is_ok());
```

### ✅ 个人信息保护法 (Personal Information Protection Law - PIPL)

《中华人民共和国个人信息保护法》（2021年）
- ✅ 个人信息处理规则
- ✅ 敏感个人信息的特别规定
- ✅ 个人信息跨境提供规则
- ✅ 个人信息主体权利
- ✅ 个人信息处理者的义务
- ✅ 安全影响评估

```rust
use legalis_cn::data_protection::{DataProcessing, LawfulBasis, validate_processing};

let processing = DataProcessing::new()
    .controller("数据科技有限公司")
    .purpose("客户服务")
    .lawful_basis(LawfulBasis::Consent) // 同意
    .data_categories(vec!["姓名", "电话", "地址"])
    .cross_border_transfer(false)
    .build()?;

// 验证处理合规性
assert!(validate_processing(&processing).is_ok());
```

### ✅ 网络安全法 (Cybersecurity Law)

《中华人民共和国网络安全法》（2017年）
- ✅ 网络运营者安全义务
- ✅ 关键信息基础设施保护
- ✅ 网络信息安全
- ✅ 数据本地化要求

```rust
use legalis_cn::cybersecurity::{NetworkOperator, CriticalInfrastructure};

let operator = NetworkOperator::new()
    .name("网络服务公司")
    .is_critical_infrastructure(true)
    .security_measures(vec!["实名认证", "日志留存", "安全评估"])
    .data_localization(true) // 数据境内存储
    .build()?;
```

## 📊 当前实现状态

**版本 0.1.3 统计：**
- ✅ **劳动合同法**：完整的劳动合同管理
- ✅ **公司法**：公司设立与治理
- ✅ **个人信息保护法**：PIPL合规框架
- ✅ **网络安全法**：网络运营者义务
- ✅ **模块**：7个模块（labor_contract, company_law, data_protection, cybersecurity, common, citation, i18n）

## 🚧 计划功能

- 📋 民法典（物权编、合同编、侵权责任编）
- 📋 反垄断法
- 📋 消费者权益保护法
- 📋 知识产权法（专利法、商标法、著作权法）

## 依赖 (Dependencies)

- `legalis-core` - 核心类型和特征
- `chrono` - 日期时间处理
- `serde` - 序列化
- `thiserror` - 错误处理

## 许可证 (License)

Apache-2.0

## 相关链接 (Related Links)

- [中国人大网](http://www.npc.gov.cn/)
- [国家法律法规数据库](https://flk.npc.gov.cn/)
- [最高人民法院](https://www.court.gov.cn/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
