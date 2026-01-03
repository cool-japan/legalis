# Legalis-RS: Kiến trúc của Luật học Sáng tạo

## Tách biệt Luật pháp và Tường thuật: Bản thiết kế cho "Quản trị dưới dạng Mã"

---

**Tác giả**: Nhóm Phát triển Legalis-RS
**Phiên bản**: 0.2.0
**Ngôn ngữ**: Rust (Edition 2024)
**Giấy phép**: MIT / Apache 2.0

---

## Tóm tắt

Bài viết này trình bày **Legalis-RS**, một framework Rust để tách biệt và cấu trúc nghiêm ngặt các tài liệu pháp lý ngôn ngữ tự nhiên thành **logic xác định (Code)** và **quyền tự quyết của tòa án (Narrative)**.

Hệ thống pháp luật hiện đại chứa hỗn hợp các lĩnh vực có thể tự động hóa bằng máy tính (yêu cầu về độ tuổi, ngưỡng thu nhập, tính toán thời hạn) và các lĩnh vực đòi hỏi sự diễn giải và phán đoán của con người ("lý do chính đáng", "đạo đức công cộng"). Các phương pháp tiếp cận trước đây đã để ranh giới này mơ hồ hoặc cố gắng tự động hóa quá mức mọi thứ.

Legalis-RS giới thiệu kiểu logic ba giá trị `LegalResult<T>` tận dụng hệ thống kiểu của Rust để làm rõ ranh giới này ở cấp độ kiểu. Điều này cho phép một mô hình mới cho việc gỡ lỗi pháp lý, mô phỏng và chuyển đổi quốc tế trong khi ngăn chặn "chế độ độc tài thuật toán" trong kỷ nguyên AI.

**Đóng góp kỹ thuật chính**:
1. Ngôn ngữ Đặc thù Miền Pháp lý (DSL) và triển khai bộ phân tích cú pháp
2. Xác minh hình thức với Z3 SMT solver
3. Engine mô phỏng kiểu ECS để dự đoán tác động xã hội
4. Tạo hợp đồng thông minh cho hơn 25 nền tảng blockchain
5. Tích hợp Linked Open Data (RDF/TTL) cho web ngữ nghĩa
6. Triển khai hệ thống pháp luật cho 4 quốc gia với điều chỉnh tham số văn hóa (Soft ODA)

**Triết lý cốt lõi**: *"Không phải mọi thứ đều nên có thể tính toán được."*

---

## 1. Giới thiệu

### 1.1 Bối cảnh: Mối quan hệ giữa Luật pháp và Tính toán

Luận điểm nổi tiếng của Lawrence Lessig "Code is Law" chỉ ra rằng kiến trúc (mã) trong không gian mạng có quyền lực điều tiết tương đương với luật pháp. Tuy nhiên, Legalis-RS đảo ngược điều này, áp dụng phương pháp "**Luật trở thành Mã**".

Mã hóa luật mang lại những lợi ích sau:

- **Khả năng xác minh**: Phát hiện mâu thuẫn logic tại thời điểm biên dịch
- **Mô phỏng**: Dự đoán tác động xã hội trước khi thực thi
- **Khả năng tương tác**: Chuyển đổi và so sánh giữa các hệ thống pháp luật khác nhau
- **Tính minh bạch**: Dấu vết kiểm toán đầy đủ của quá trình quyết định pháp lý

Tuy nhiên, việc làm cho tất cả luật có thể tính toán được là nguy hiểm cả về mặt triết học và thực tiễn. Luật pháp vốn dĩ chứa các lĩnh vực đòi hỏi "phán đoán của con người", và tự động hóa bỏ qua điều này có thể dẫn đến "chế độ độc tài AI".

### 1.2 Phát biểu vấn đề: Thách thức của Xử lý Pháp lý trong Kỷ nguyên AI

Công nghệ pháp lý hiện đại (LegalTech) đối mặt với một số thách thức cơ bản:

1. **Xử lý sự mơ hồ**: Nhiều thuật ngữ pháp lý cố tình mơ hồ, giả định sự diễn giải theo từng trường hợp
2. **Phụ thuộc ngữ cảnh**: Cùng một điều khoản có thể được diễn giải khác nhau tùy thuộc vào bối cảnh xã hội và văn hóa
3. **Thay đổi theo thời gian**: Luật được sửa đổi và bãi bỏ, đòi hỏi quản lý tính nhất quán qua thời gian
4. **Khác biệt quốc tế**: Hệ thống pháp luật của mỗi quốc gia khác nhau từ nền tảng triết học

Các DSL pháp lý hiện có (Catala, L4, Stipula) đã giải quyết một số thách thức này, nhưng không ai áp dụng phương pháp làm rõ "ranh giới giữa khả năng tính toán và phán đoán của con người" trong hệ thống kiểu.

### 1.3 Đề xuất: Tách biệt Khả năng Tính toán và Quyền Tự quyết của Tòa án

Cốt lõi của Legalis-RS là giới thiệu logic ba giá trị thông qua kiểu `LegalResult<T>`:

```rust
pub enum LegalResult<T> {
    /// [Miền Xác định] Kết quả pháp lý có thể xử lý tự động
    Deterministic(T),

    /// [Miền Tự quyết] Miền đòi hỏi phán đoán của con người
    JudicialDiscretion {
        issue: String,           // Vấn đề đang xét
        context_id: Uuid,        // Dữ liệu ngữ cảnh
        narrative_hint: Option<String>, // Ý kiến tham khảo bởi LLM
    },

    /// [Sự cố Logic] Lỗi trong chính luật
    Void { reason: String },
}
```

Kiểu này đảm bảo kết quả của xử lý pháp lý luôn được phân loại vào một trong ba danh mục. Hệ thống dừng xử lý khi đạt đến `JudicialDiscretion` và ủy quyền phán đoán cho con người. Đây trở thành "thành trì cấp kiểu" chống lại chế độ độc tài AI.

### 1.4 Cấu trúc Bài viết

Phần còn lại của bài viết được tổ chức như sau:

- **Phần 2**: Công trình Liên quan (Lịch sử Luật Tính toán và các DSL hiện có)
- **Phần 3**: Triết lý và Nguyên tắc Thiết kế
- **Phần 4**: Kiến trúc Hệ thống (cấu trúc 7 lớp)
- **Phần 5**: Công nghệ Cốt lõi (DSL, xác minh, mô phỏng)
- **Phần 6**: Triển khai theo Quyền tài phán (tập trung vào luật Nhật Bản)
- **Phần 7**: Nghiên cứu Tình huống
- **Phần 8**: Đặc tả API và Chi tiết Kỹ thuật
- **Phần 9**: Đánh giá
- **Phần 10**: Công việc Tương lai
- **Phần 11**: Kết luận

---

## 2. Công trình Liên quan

### 2.1 Lịch sử Luật Tính toán

Mối quan hệ giữa luật và máy tính bắt nguồn từ dự án LARC (Legal Analysis and Research Computer) vào những năm 1950. Kể từ đó, nó đã phát triển qua hệ thống chuyên gia, hệ thống dựa trên quy tắc, và các phương pháp học máy hiện đại.

Các mốc quan trọng:

| Thời kỳ | Công nghệ | Đặc điểm |
|---------|-----------|----------|
| 1950s | LARC | Hệ thống truy xuất thông tin pháp lý đầu tiên |
| 1970s | Hệ thống chuyên gia kiểu MYCIN | Suy luận dựa trên quy tắc |
| 1980s | HYPO | Suy luận dựa trên trường hợp |
| 1990s | Chuẩn hóa XML/SGML | Cấu trúc hóa tài liệu pháp lý |
| 2000s | Semantic Web | Biểu diễn tri thức pháp lý dựa trên ontology |
| 2010s | Machine Learning | Mô hình dự đoán pháp lý |
| 2020s | LLM + Xác minh Hình thức | Phương pháp lai |

### 2.2 Các DSL Pháp lý Hiện có

#### Catala (Inria, Pháp)
```
declaration scope AdultRights:
  context age content integer
  context has_rights content boolean

scope AdultRights:
  definition has_rights equals age >= 18
```
- **Tính năng**: Lập trình literate, dựa trên phạm vi, định kiểu mạnh
- **Hạn chế**: Không đánh dấu rõ ràng miền tự quyết

#### L4 (Singapore)
```
RULE adult_voting
  PARTY citizen
  MUST vote
  IF age >= 18
```
- **Tính năng**: Logic nghĩa vụ (MUST/MAY/SHANT), suy luận dựa trên quy tắc
- **Hạn chế**: Không có chức năng mô phỏng

#### Stipula (Đại học Bologna, Ý)
- **Tính năng**: Hướng hợp đồng thông minh, máy trạng thái, mô hình bên/tài sản
- **Hạn chế**: Không có xác minh hình thức

### 2.3 Định vị của Dự án này

Legalis-RS mở rộng nghiên cứu hiện có theo các cách sau:

1. **Đánh dấu tự quyết cấp kiểu**: Logic ba giá trị qua `LegalResult<T>`
2. **Kiến trúc tích hợp**: Pipeline Parse→Verify→Simulate→Output
3. **Khả năng tương tác đa định dạng**: Chuyển đổi với Catala/L4/Stipula/Akoma Ntoso
4. **Thiết kế quốc tế hóa**: Điều chỉnh tham số văn hóa (Soft ODA)
5. **Tích hợp blockchain**: Tạo hợp đồng thông minh cho hơn 25 nền tảng

---

## 3. Triết lý & Nguyên tắc Thiết kế

### 3.1 "Quản trị dưới dạng Mã, Công lý dưới dạng Tường thuật"

Slogan của Legalis-RS phản ánh sự khác biệt cơ bản giữa quản trị và công lý:

- **Quản trị**: Áp dụng quy tắc, tuân thủ thủ tục, xác định tư cách → **Có thể mã hóa**
- **Công lý**: Thực hiện công bằng, diễn giải theo ngữ cảnh, phán đoán giá trị → **Kể như tường thuật**

Sự phân biệt này tương ứng với sự phân biệt giữa "quy tắc" và "nguyên tắc" (Dworkin) trong triết học pháp luật, hoặc giữa "công lý hình thức" và "công lý thực chất".

### 3.2 Thiết kế Logic Ba Giá trị

Ba giá trị của `LegalResult<T>` tương ứng với các khái niệm triết học pháp luật sau:

| Kiểu | Khái niệm Triết học Pháp luật | Tác nhân Xử lý |
|------|------------------------------|----------------|
| `Deterministic(T)` | Quy tắc áp dụng cơ học | Máy tính |
| `JudicialDiscretion` | Nguyên tắc đòi hỏi diễn giải | Con người |
| `Void` | Lỗ hổng pháp lý/mâu thuẫn | Nhà lập pháp (cần sửa) |

### 3.3 "Không phải mọi thứ đều nên có thể tính toán được"

Đối với sự cám dỗ làm cho mọi thứ có thể tính toán, Legalis-RS nói rõ ràng "Không". Các miền sau được thiết kế cố ý là không thể tính toán:

1. **Lý do chính đáng**
2. **Trật tự công cộng và đạo đức**
3. **Thiện chí**
4. **Tính hợp lý**

### 3.4 Ngăn chặn Chế độ Độc tài AI

Legalis-RS ngăn chặn chế độ độc tài AI thông qua các cơ chế sau:

1. **Dừng bắt buộc theo kiểu**: Tự động dừng khi đạt đến `JudicialDiscretion`
2. **Dấu vết kiểm toán bắt buộc**: Ghi lại tất cả quá trình quyết định
3. **Khả năng giải thích**: Đầu ra có cấu trúc của lý do quyết định
4. **Đảm bảo vòng lặp con người**: Con người luôn đưa ra quyết định cuối cùng trong miền tự quyết

---

## 4. Kiến trúc Hệ thống

### 4.1 Tổng quan Kiến trúc 7 Lớp

Legalis-RS bao gồm 7 lớp sau:

```
┌─────────────────────────────────────────────────────────┐
│                   Lớp Cơ sở Hạ tầng                      │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                      Lớp Đầu ra                          │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│                 Lớp Khả năng Tương tác                   │
│                    (legalis-interop)                     │
├─────────────────────────────────────────────────────────┤
│                  Lớp Quốc tế hóa                         │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│              Lớp Mô phỏng & Phân tích                    │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                  Lớp Trí tuệ                             │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                      Lớp Lõi                             │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Lớp Lõi

#### legalis-core
Crate triển khai lõi triết học của dự án.

**Định nghĩa Kiểu Chính**:
- `LegalResult<T>`: Kiểu logic ba giá trị
- `Statute`: Biểu diễn cơ bản của luật
- `Condition`: Biểu thức điều kiện (AND/OR/NOT, tuổi, thu nhập, v.v.)
- `Effect`: Hiệu lực pháp lý (Grant/Revoke/Obligation/Prohibition)
- `EvaluationContext`: Trait để đánh giá điều kiện

#### legalis-dsl
Bộ phân tích cú pháp cho ngôn ngữ đặc thù miền pháp lý.

**Ví dụ Cú pháp DSL**:
```
STATUTE adult-voting: "Quyền Bầu cử Người lớn" {
    JURISDICTION "VN"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Quyền bầu cử"

    EXCEPTION WHEN HAS disqualified
    DISCRETION "Xác định năng lực tinh thần cần chẩn đoán của bác sĩ"
}
```

### 4.3 Lớp Trí tuệ

#### legalis-llm
Lớp trừu tượng nhà cung cấp LLM.

**Nhà cung cấp Được hỗ trợ**:
- OpenAI (GPT-4, GPT-4o)
- Anthropic (Claude)
- Google (Gemini)
- LLM Cục bộ

#### legalis-verifier
Engine xác minh hình thức.

**Mục tiêu Xác minh**:
- Phát hiện tham chiếu vòng
- Phát hiện luật không thể đạt được (Dead Statute)
- Phát hiện mâu thuẫn logic
- Kiểm tra xung đột hiến pháp
- Phân tích sự mơ hồ

### 4.4 Lớp Mô phỏng

#### legalis-sim
Engine mô phỏng kiểu ECS.

**Tính năng**:
- Mô phỏng dựa trên dân số (hỗ trợ hàng triệu agent)
- Mô phỏng Monte Carlo
- Phân tích độ nhạy
- Kiểm thử A/B
- Tăng tốc GPU (CUDA/OpenCL/WebGPU)

### 4.5 Lớp Quốc tế hóa

#### legalis-i18n
Hỗ trợ đa ngôn ngữ và đa quyền tài phán.

**Quyền tài phán Được hỗ trợ**: JP, US, GB, DE, FR, ES, IT, CN, TW, KR, CA, AU, IN, BR, RU, SA, NL, CH, MX, SG, VN

#### legalis-porting
Chuyển đổi giữa các hệ thống pháp luật (Soft ODA).

### 4.6 Lớp Khả năng Tương tác

#### legalis-interop
Chuyển đổi giữa nhiều định dạng DSL pháp lý.

**Định dạng Được hỗ trợ**:

| Định dạng | Nguồn gốc | Tính năng |
|-----------|-----------|-----------|
| Catala | Inria, Pháp | Lập trình literate |
| Stipula | Đại học Bologna, Ý | Hợp đồng thông minh |
| L4 | Singapore | Logic nghĩa vụ |
| Akoma Ntoso | OASIS | Tài liệu lập pháp XML |

### 4.7 Lớp Đầu ra

#### legalis-viz
Engine trực quan hóa.

**Định dạng Đầu ra**:
- Cây quyết định
- Lưu đồ
- Đồ thị phụ thuộc
- SVG / PNG / JSON tương thích D3.js

#### legalis-chain
Tạo hợp đồng thông minh.

**Nền tảng Được hỗ trợ (25+)**:
- EVM: Solidity, Vyper
- Substrate: Ink!
- Move: Aptos, Sui
- StarkNet: Cairo
- Cosmos: CosmWasm
- Khác: TON FunC, Algorand Teal, Fuel Sway, Clarity, Noir, Leo, Circom

**Ràng buộc**: Chỉ `Deterministic` mới có thể chuyển đổi (`JudicialDiscretion` không thể chuyển đổi)

#### legalis-lod
Đầu ra Linked Open Data.

**Ontology Được hỗ trợ**:
- ELI (European Legislation Identifier)
- FaBiO
- LKIF-Core
- Akoma Ntoso
- Dublin Core
- SKOS

---

## 5. Công nghệ Cốt lõi

### 5.1 DSL Pháp lý (Cú pháp, Ngữ nghĩa, Triển khai Bộ phân tích)

**Cấu trúc Cơ bản**:
```
STATUTE <id>: "<title>" {
    [JURISDICTION "<jurisdiction>"]
    [VERSION <number>]
    [EFFECTIVE_DATE <date>]
    [EXPIRY_DATE <date>]

    WHEN <condition>
    THEN <effect>

    [EXCEPTION WHEN <condition>]
    [DISCRETION "<description>"]

    [AMENDMENT <statute-id>]
    [SUPERSEDES <statute-id>]
}
```

### 5.2 Kiểu LegalResult<T> và Giá trị Chân lý Bán phần

```rust
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion {
        issue: String,
        context_id: Uuid,
        narrative_hint: Option<String>,
    },
    Void { reason: String },
}

pub enum PartialBool {
    True,
    False,
    Unknown,      // Thông tin không đủ
    Contradiction, // Mâu thuẫn
}
```

### 5.3 Xác minh Hình thức với Z3 SMT Solver

**Mục tiêu Xác minh**:
1. Tham chiếu vòng
2. Luật không thể đạt được
3. Mâu thuẫn logic
4. Xung đột hiến pháp

### 5.4 Engine Mô phỏng Kiểu ECS

Engine mô phỏng áp dụng mẫu Entity-Component-System (ECS):
- **Entity**: Agent công dân
- **Component**: Thuộc tính (tuổi, thu nhập, nơi ở, v.v.)
- **System**: Logic áp dụng luật

### 5.5 Tạo Hợp đồng Thông minh

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AdultVotingRights {
    struct Citizen {
        uint256 age;
        bool hasCitizenship;
    }

    function isEligible(Citizen memory citizen)
        public pure returns (bool)
    {
        return citizen.age >= 18 && citizen.hasCitizenship;
    }
}
```

---

## 6. Triển khai theo Quyền tài phán

### 6.1 Hệ thống Pháp luật Nhật Bản

Crate legalis-jp cung cấp biểu diễn có cấu trúc của Hiến pháp Nhật Bản.

### 6.2 Đức, Pháp, Hoa Kỳ (Đã lên kế hoạch)

| Quyền tài phán | Trạng thái | Lĩnh vực Tập trung |
|----------------|------------|---------------------|
| Đức (DE) | Đang phát triển | BGB, GG |
| Pháp (FR) | Đang phát triển | Code civil, Hiến pháp |
| Hoa Kỳ (US) | Đang phát triển | UCC, Hiến pháp, Luật án lệ |

### 6.3 Điều chỉnh Tham số Văn hóa (Soft ODA)

Các tham số văn hóa sau được xem xét trong việc chuyển đổi hệ thống pháp luật quốc tế:

1. **Hệ thống pháp luật**: Civil law vs Common law vs Religious law
2. **Cấu trúc ngôn ngữ**: Khả năng dịch thuật ngữ pháp lý
3. **Chuẩn mực xã hội**: Điều cấm kỵ, phong tục, ràng buộc tôn giáo
4. **Cấu trúc hành chính**: Tập trung vs Liên bang
5. **Hệ thống tư pháp**: Bồi thẩm đoàn vs Thẩm phán chuyên nghiệp

---

## 7. Nghiên cứu Tình huống

### 7.1 Hệ thống Xác định Tư cách Phúc lợi

**Kết quả**:
- **Quyết định xác định**: 85% trường hợp
- **JudicialDiscretion**: 15% trường hợp (phán đoán về "tính cấp bách", "nhu cầu thực sự", v.v.)

### 7.2 Mô phỏng Điều 709 Bộ luật Dân sự (Bồi thường thiệt hại)

**Kịch bản Kiểm thử**:
1. Cố ý gây hại rõ ràng → `Deterministic(Liable)`
2. Vi phạm do sơ suất → `Deterministic(Liable)`
3. Trường hợp ranh giới → `JudicialDiscretion`
4. Không có vi phạm → `Deterministic(NotLiable)`
5. Không có quan hệ nhân quả → `Deterministic(NotLiable)`

### 7.3 Phân tích So sánh Luật Bồi thường 4 Quốc gia

| Quốc gia | Bộ luật | Đặc điểm |
|----------|---------|----------|
| Nhật Bản | BLDS Điều 709 | Điều khoản chung (quyền tự quyết rộng) |
| Đức | BGB §823/§826 | Lợi ích được bảo vệ liệt kê |
| Pháp | Code civil Art. 1240 | Trừu tượng hóa tối đa |
| Hoa Kỳ | Luật án lệ | Phân loại (Battery, v.v.) |

---

## 8. Đặc tả API & Chi tiết Kỹ thuật

### 8.1 Các Kiểu và Trait Chính

```rust
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion { issue: String, context_id: Uuid, narrative_hint: Option<String> },
    Void { reason: String },
}

pub trait LegalEntity: Send + Sync {
    fn id(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn attributes(&self) -> &[String];
}

pub struct Statute {
    pub id: String,
    pub title: String,
    pub primary_effect: Effect,
    pub preconditions: Vec<Condition>,
    pub jurisdiction: String,
    pub temporal_validity: TemporalValidity,
}
```

### 8.2 Hệ thống Lệnh CLI

```bash
# Phân tích
legalis parse <file.dsl> [--format json|yaml]

# Xác minh
legalis verify <file.dsl> [--strict]

# Mô phỏng
legalis simulate <file.dsl> --population 1000

# Trực quan hóa
legalis visualize <file.dsl> --output tree.svg

# Xuất
legalis export <file.dsl> --format solidity|catala|l4|rdf
```

---

## 9. Đánh giá

### 9.1 Benchmark Hiệu năng

| Thao tác | Mục tiêu | Thời gian |
|----------|----------|-----------|
| Phân tích DSL | 100 luật | 15ms |
| Xác minh | 100 luật | 250ms |
| Mô phỏng | 10,000 agent | 1.2s |
| Mô phỏng | 100,000 agent | 8.5s |
| Tạo hợp đồng thông minh | 1 luật | 45ms |
| Xuất RDF | 100 luật | 120ms |

### 9.2 Chất lượng Mã

- **Độ bao phủ kiểm thử**: Kiểm thử tích hợp, kiểm thử thuộc tính, kiểm thử snapshot
- **Phân tích tĩnh**: Clippy (chính sách không cảnh báo)
- **Tài liệu**: rustdoc cho tất cả API công khai

---

## 10. Công việc Tương lai

### 10.1 Web UI Frontend
- Dashboard dựa trên React
- Trực quan hóa mô phỏng thời gian thực
- Tính năng chỉnh sửa cộng tác

### 10.2 VS Code Extension
- Đánh dấu cú pháp DSL
- Xác minh thời gian thực
- Tự động hoàn thành

### 10.3 Tích hợp Jupyter Notebook
- Python bindings qua PyO3
- Phân tích tương tác
- Widget trực quan hóa

### 10.4 Quyền tài phán Bổ sung
- Luật EU (tích hợp EURLex)
- Luật quốc tế (hiệp ước, thỏa thuận)
- Luật tôn giáo (luật Hồi giáo)

---

## 11. Kết luận

Legalis-RS trình bày một phương pháp mới để mã hóa luật bằng cách làm rõ "ranh giới giữa khả năng tính toán và phán đoán của con người" trong hệ thống kiểu.

**Thành tựu Chính**:

1. **Nền tảng triết học**: "Quản trị dưới dạng Mã, Công lý dưới dạng Tường thuật"
2. **Hệ thống kiểu**: Logic ba giá trị qua `LegalResult<T>`
3. **Kiến trúc tích hợp**: Thiết kế toàn diện với 7 lớp và 16 crate
4. **Triển khai**: Khoảng 450,000 dòng mã Rust
5. **Xác minh**: Tích hợp Z3 SMT solver
6. **Mô phỏng**: Engine kiểu ECS (hỗ trợ tăng tốc GPU)
7. **Đầu ra**: 25+ blockchain, RDF/TTL, nhiều định dạng

**Triết lý Cốt lõi**: *"Không phải mọi thứ đều nên có thể tính toán được."*

Không phải tự động hóa hoàn toàn luật, mà là tách biệt rõ ràng các miền nên được tự động hóa khỏi các miền đòi hỏi phán đoán của con người. Đây là kiến trúc của "luật học sáng tạo" mà Legalis-RS hướng tới.

---

## Tài liệu Tham khảo

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. Governatori, G., & Shams, Z. (2019). L4: Legal Language and Logic for Law. *JURIX 2019*.
5. Azzopardi, S., & Pace, G. J. (2018). Stipula: A domain-specific language for legal contracts. *JURIX 2018*.
6. Palmirani, M., & Vitali, F. (2011). Akoma-Ntoso for Legal Documents. *Legislative XML for the Semantic Web*.
7. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

## Phụ lục

### A. Đặc tả Ngữ pháp DSL

```ebnf
statute      = "STATUTE" identifier ":" string "{" body "}" ;
body         = { metadata } when_clause then_clause { exception } { discretion } ;
metadata     = jurisdiction | version | effective_date | expiry_date ;
jurisdiction = "JURISDICTION" string ;
version      = "VERSION" number ;
when_clause  = "WHEN" condition ;
then_clause  = "THEN" effect ;
exception    = "EXCEPTION" "WHEN" condition ;
discretion   = "DISCRETION" string ;
```

### B. Danh sách Định nghĩa Kiểu

Để biết định nghĩa đầy đủ của các kiểu chính, xem `crates/legalis-core/src/lib.rs`.

### C. Tùy chọn Cấu hình

```toml
[legalis]
default_jurisdiction = "VN"
enable_z3 = true
enable_gpu = false
cache_dir = "~/.legalis/cache"
log_level = "info"

[api]
port = 8080
enable_graphql = true
enable_auth = true
rate_limit = 100

[simulation]
max_agents = 1000000
parallel_workers = 8
```

---

*"Code is Law," họ nói, nhưng chúng tôi áp dụng phương pháp "Luật trở thành Mã". Tuy nhiên, chúng tôi nhúng một kiểu gọi là 'Nhân loại' vào mã đó.*

---

**Nhóm Phát triển Legalis-RS**
Phiên bản 0.2.0 | 2024
