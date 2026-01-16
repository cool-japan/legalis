# Legalis-RS: Arsitektur Yurisprudensi Generatif

## Memisahkan Hukum dan Narasi: Cetak Biru untuk "Tata Kelola sebagai Kode"

---

**Penulis**: Tim Pengembangan Legalis-RS
**Versi**: 0.2.0
**Bahasa**: Rust (Edition 2024)
**Lisensi**: MIT / Apache 2.0

---

## Abstrak

Makalah ini menyajikan **Legalis-RS**, sebuah framework Rust untuk memisahkan dan menyusun secara ketat dokumen hukum bahasa alami menjadi **logika deterministik (Code)** dan **diskresi yudisial (Narrative)**.

Sistem hukum modern mengandung campuran domain yang dapat diotomatisasi komputer (persyaratan usia, ambang batas pendapatan, perhitungan tenggat waktu) dan domain yang memerlukan interpretasi dan penilaian manusia ("alasan yang adil", "moralitas publik"). Pendekatan sebelumnya membiarkan batas ini kabur atau mencoba otomatisasi berlebihan yang berusaha membuat semuanya dapat dihitung.

Legalis-RS memperkenalkan tipe logika tiga-nilai `LegalResult<T>` memanfaatkan sistem tipe Rust untuk membuat batas ini eksplisit pada level tipe. Ini memungkinkan paradigma baru untuk debugging hukum, simulasi, dan porting internasional sambil mencegah "otokrasi algoritmik" di era AI.

**Kontribusi Teknis Utama**:
1. Domain Specific Language (DSL) Hukum dan implementasi parser
2. Verifikasi formal dengan OxiZ SMT solver (Pure Rust)
3. Engine simulasi bergaya ECS untuk prediksi dampak sosial
4. Pembuatan smart contract untuk 25+ platform blockchain
5. Integrasi Linked Open Data (RDF/TTL) untuk web semantik
6. Implementasi sistem hukum untuk 4 negara dengan adaptasi parameter budaya (Soft ODA)

**Filosofi Inti**: *"Tidak semua hal harus dapat dihitung."*

---

## 1. Pendahuluan

### 1.1 Latar Belakang: Hubungan Antara Hukum dan Komputasi

Tesis terkenal Lawrence Lessig "Code is Law" menunjukkan bahwa arsitektur (kode) di dunia maya memiliki kekuatan regulasi setara dengan hukum. Namun, Legalis-RS membalikkan ini, mengadopsi pendekatan "**Hukum menjadi Kode**".

Mengkodifikasi hukum menawarkan manfaat berikut:

- **Kemampuan Verifikasi**: Mendeteksi kontradiksi logis pada waktu kompilasi
- **Simulasi**: Memprediksi dampak sosial sebelum penegakan
- **Interoperabilitas**: Mengkonversi dan membandingkan antara sistem hukum yang berbeda
- **Transparansi**: Jejak audit lengkap dari proses keputusan hukum

Namun, membuat semua hukum dapat dihitung berbahaya baik secara filosofis maupun praktis. Hukum secara inheren mengandung domain yang memerlukan "penilaian manusia", dan otomatisasi yang mengabaikan ini dapat mengarah pada "otokrasi AI".

### 1.2 Pernyataan Masalah: Tantangan Pemrosesan Hukum di Era AI

Teknologi hukum modern (LegalTech) menghadapi beberapa tantangan mendasar:

1. **Penanganan Ambiguitas**: Banyak istilah hukum sengaja kabur, mengasumsikan interpretasi kasus per kasus
2. **Ketergantungan Konteks**: Ketentuan yang sama dapat diinterpretasikan berbeda tergantung konteks sosial dan budaya
3. **Perubahan Temporal**: Hukum diamendemen dan dicabut, memerlukan manajemen konsistensi lintas waktu
4. **Perbedaan Internasional**: Sistem hukum setiap negara berbeda dari fondasi filosofisnya

DSL hukum yang ada (Catala, L4, Stipula) telah mengatasi beberapa tantangan ini, tetapi tidak ada yang mengambil pendekatan yang membuat "batas antara kemampuan komputasi dan penilaian manusia" eksplisit dalam sistem tipe.

### 1.3 Proposal: Pemisahan Kemampuan Komputasi dan Diskresi Yudisial

Inti dari Legalis-RS adalah pengenalan logika tiga-nilai melalui tipe `LegalResult<T>`:

```rust
pub enum LegalResult<T> {
    /// [Domain Deterministik] Hasil hukum yang dapat diproses otomatis
    Deterministic(T),

    /// [Domain Diskresi] Domain yang memerlukan penilaian manusia
    JudicialDiscretion {
        issue: String,           // Masalah yang dipertanyakan
        context_id: Uuid,        // Data kontekstual
        narrative_hint: Option<String>, // Opini referensi oleh LLM
    },

    /// [Kerusakan Logis] Bug dalam hukum itu sendiri
    Void { reason: String },
}
```

Tipe ini menjamin bahwa hasil pemrosesan hukum selalu diklasifikasikan ke dalam salah satu dari tiga kategori. Sistem berhenti memproses saat mencapai `JudicialDiscretion` dan mendelegasikan penilaian kepada manusia. Ini menjadi "benteng level-tipe" terhadap otokrasi AI.

### 1.4 Organisasi Makalah

Sisa makalah ini diorganisasikan sebagai berikut:

- **Bagian 2**: Karya Terkait
- **Bagian 3**: Filosofi dan Prinsip Desain
- **Bagian 4**: Arsitektur Sistem (struktur 7 lapis)
- **Bagian 5**: Teknologi Inti
- **Bagian 6**: Implementasi Yurisdiksi
- **Bagian 7**: Studi Kasus
- **Bagian 8**: Spesifikasi API dan Detail Teknis
- **Bagian 9**: Evaluasi
- **Bagian 10**: Pekerjaan Masa Depan
- **Bagian 11**: Kesimpulan

---

## 2. Karya Terkait

### 2.1 Sejarah Hukum Komputasional

Hubungan antara hukum dan komputer berasal dari proyek LARC (Legal Analysis and Research Computer) pada 1950-an. Sejak itu telah berkembang melalui sistem pakar, sistem berbasis aturan, dan pendekatan machine learning modern.

| Era | Teknologi | Karakteristik |
|-----|-----------|---------------|
| 1950-an | LARC | Sistem pengambilan informasi hukum pertama |
| 1970-an | Sistem pakar tipe MYCIN | Penalaran berbasis aturan |
| 1980-an | HYPO | Penalaran berbasis kasus |
| 1990-an | Standarisasi XML/SGML | Penataan dokumen hukum |
| 2000-an | Semantic Web | Representasi pengetahuan hukum berbasis ontologi |
| 2010-an | Machine Learning | Model prediksi hukum |
| 2020-an | LLM + Verifikasi Formal | Pendekatan hybrid |

### 2.2 DSL Hukum yang Ada

#### Catala (Inria, Prancis)
- **Fitur**: Pemrograman literate, berbasis scope, pengetikan kuat
- **Keterbatasan**: Tidak ada penandaan eksplisit domain diskresi

#### L4 (Singapura)
- **Fitur**: Logika deontik (MUST/MAY/SHANT), penalaran berbasis aturan
- **Keterbatasan**: Tidak ada fungsi simulasi

#### Stipula (Universitas Bologna, Italia)
- **Fitur**: Berorientasi smart contract, mesin keadaan, model pihak/aset
- **Keterbatasan**: Tidak ada verifikasi formal

### 2.3 Posisi Proyek Ini

Legalis-RS memperluas penelitian yang ada dengan cara berikut:

1. **Penandaan diskresi level-tipe**: Logika tiga-nilai via `LegalResult<T>`
2. **Arsitektur terintegrasi**: Pipeline Parse→Verify→Simulate→Output
3. **Interoperabilitas multi-format**: Konversi dengan Catala/L4/Stipula/Akoma Ntoso
4. **Desain internasionalisasi**: Adaptasi parameter budaya (Soft ODA)
5. **Integrasi blockchain**: Pembuatan smart contract untuk 25+ platform

---

## 3. Filosofi & Prinsip Desain

### 3.1 "Tata Kelola sebagai Kode, Keadilan sebagai Narasi"

Slogan Legalis-RS mencerminkan perbedaan esensial antara tata kelola dan keadilan:

- **Tata Kelola**: Penerapan aturan, kepatuhan prosedural, penentuan kelayakan → **Dapat dikodifikasi**
- **Keadilan**: Realisasi kesetaraan, interpretasi kontekstual, penilaian nilai → **Diceritakan sebagai narasi**

Perbedaan ini sesuai dengan perbedaan antara "aturan" dan "prinsip" (Dworkin) dalam filsafat hukum, atau antara "keadilan formal" dan "keadilan substansial".

### 3.2 Desain Logika Tiga-Nilai

Tiga nilai `LegalResult<T>` sesuai dengan konsep filsafat hukum berikut:

| Tipe | Konsep Filsafat Hukum | Agen Pemrosesan |
|------|------------------------|-----------------|
| `Deterministic(T)` | Aturan yang dapat diterapkan secara mekanis | Komputer |
| `JudicialDiscretion` | Prinsip yang memerlukan interpretasi | Manusia |
| `Void` | Kesenjangan hukum/kontradiksi | Legislator (perlu koreksi) |

### 3.3 "Tidak semua hal harus dapat dihitung"

Terhadap godaan untuk membuat semuanya dapat dihitung, Legalis-RS dengan jelas mengatakan "Tidak". Domain berikut sengaja dirancang sebagai tidak dapat dihitung:

1. **Alasan yang adil**
2. **Ketertiban umum dan moralitas**
3. **Itikad baik**
4. **Kewajaran**

### 3.4 Mencegah Otokrasi AI

Legalis-RS mencegah otokrasi AI melalui mekanisme berikut:

1. **Penghentian paksa oleh tipe**: Penghentian otomatis saat mencapai `JudicialDiscretion`
2. **Jejak audit wajib**: Pencatatan semua proses keputusan
3. **Kemampuan penjelasan**: Output terstruktur dari alasan keputusan
4. **Jaminan loop manusia**: Manusia selalu membuat keputusan akhir di domain diskresi

---

## 4. Arsitektur Sistem

### 4.1 Gambaran Arsitektur 7 Lapis

Legalis-RS terdiri dari 7 lapis berikut:

```
┌─────────────────────────────────────────────────────────┐
│                   Lapis Infrastruktur                    │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                      Lapis Output                        │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│                 Lapis Interoperabilitas                  │
│                    (legalis-interop)                     │
├─────────────────────────────────────────────────────────┤
│                  Lapis Internasionalisasi                │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│              Lapis Simulasi & Analisis                   │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                   Lapis Intelijen                        │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                       Lapis Inti                         │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Lapis Inti

#### legalis-core
Crate yang mengimplementasikan inti filosofis proyek.

**Definisi Tipe Utama**:
- `LegalResult<T>`: Tipe logika tiga-nilai
- `Statute`: Representasi dasar undang-undang
- `Condition`: Ekspresi kondisi (AND/OR/NOT, usia, pendapatan, dll.)
- `Effect`: Efek hukum (Grant/Revoke/Obligation/Prohibition)

#### legalis-dsl
Parser untuk domain specific language hukum.

**Contoh Sintaks DSL**:
```
STATUTE adult-voting: "Hak Suara Dewasa" {
    JURISDICTION "ID"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 17 AND HAS citizen
    THEN GRANT "Hak memilih"

    EXCEPTION WHEN HAS disqualified
    DISCRETION "Penentuan kapasitas mental memerlukan diagnosis dokter"
}
```

### 4.3 Lapis Intelijen

#### legalis-llm
Lapisan abstraksi penyedia LLM.

**Penyedia yang Didukung**:
- OpenAI (GPT-4, GPT-4o)
- Anthropic (Claude)
- Google (Gemini)
- LLM Lokal

#### legalis-verifier
Engine verifikasi formal.

**Target Verifikasi**:
- Deteksi referensi sirkuler
- Deteksi undang-undang yang tidak terjangkau (Dead Statute)
- Deteksi kontradiksi logis
- Pemeriksaan konflik konstitusional
- Analisis ambiguitas

### 4.4 Lapis Simulasi

#### legalis-sim
Engine simulasi bergaya ECS.

**Fitur**:
- Simulasi berbasis populasi (mendukung jutaan agen)
- Simulasi Monte Carlo
- Analisis sensitivitas
- Pengujian A/B
- Akselerasi GPU (CUDA/OpenCL/WebGPU)

### 4.5 Lapis Internasionalisasi

#### legalis-i18n
Dukungan multi-bahasa dan multi-yurisdiksi.

**Yurisdiksi yang Didukung**: JP, US, GB, DE, FR, ES, IT, CN, TW, KR, CA, AU, IN, BR, RU, SA, NL, CH, MX, SG, ID

#### legalis-porting
Porting antar sistem hukum (Soft ODA).

### 4.6 Lapis Interoperabilitas

#### legalis-interop
Interkonversi dengan berbagai format DSL hukum.

**Format yang Didukung**:

| Format | Asal | Fitur |
|--------|------|-------|
| Catala | Inria, Prancis | Pemrograman literate |
| Stipula | Universitas Bologna, Italia | Smart contract |
| L4 | Singapura | Logika deontik |
| Akoma Ntoso | OASIS | Dokumen legislatif XML |

### 4.7 Lapis Output

#### legalis-viz
Engine visualisasi.

**Format Output**:
- Pohon keputusan
- Flowchart
- Graf dependensi
- SVG / PNG / JSON yang kompatibel dengan D3.js

#### legalis-chain
Pembuatan smart contract.

**Platform yang Didukung (25+)**:
- EVM: Solidity, Vyper
- Substrate: Ink!
- Move: Aptos, Sui
- StarkNet: Cairo
- Cosmos: CosmWasm
- Lainnya: TON FunC, Algorand Teal, Fuel Sway, Clarity, Noir, Leo, Circom

**Batasan**: Hanya `Deterministic` yang dapat dikonversi (`JudicialDiscretion` tidak dapat dikonversi)

#### legalis-lod
Output Linked Open Data.

**Ontologi yang Didukung**:
- ELI (European Legislation Identifier)
- FaBiO
- LKIF-Core
- Akoma Ntoso
- Dublin Core
- SKOS

---

## 5. Teknologi Inti

### 5.1 DSL Hukum

**Struktur Dasar**:
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

### 5.2 Tipe LegalResult<T> dan Nilai Kebenaran Parsial

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
    Unknown,      // Informasi tidak cukup
    Contradiction, // Kontradiksi
}
```

### 5.3 Verifikasi Formal dengan OxiZ SMT Solver (Pure Rust)

**Target Verifikasi**:
1. Referensi sirkuler
2. Undang-undang yang tidak terjangkau
3. Kontradiksi logis
4. Konflik konstitusional

### 5.4 Engine Simulasi Bergaya ECS

Engine simulasi mengadopsi pola Entity-Component-System (ECS):
- **Entity**: Agen warga negara
- **Component**: Atribut (usia, pendapatan, tempat tinggal, dll.)
- **System**: Logika penerapan hukum

### 5.5 Pembuatan Smart Contract

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
        return citizen.age >= 17 && citizen.hasCitizenship;
    }
}
```

---

## 6. Implementasi Yurisdiksi

### 6.1 Sistem Hukum Jepang

Crate legalis-jp menyediakan representasi terstruktur dari Konstitusi Jepang.

### 6.2 Jerman, Prancis, AS (Direncanakan)

| Yurisdiksi | Status | Area Fokus |
|------------|--------|------------|
| Jerman (DE) | Dalam pengembangan | BGB, GG |
| Prancis (FR) | Dalam pengembangan | Code civil, Konstitusi |
| AS (US) | Dalam pengembangan | UCC, Konstitusi, Hukum kasus |

### 6.3 Adaptasi Parameter Budaya (Soft ODA)

Parameter budaya berikut dipertimbangkan dalam porting sistem hukum internasional:

1. **Sistem hukum**: Civil law vs Common law vs Religious law
2. **Struktur bahasa**: Kemampuan terjemahan istilah hukum
3. **Norma sosial**: Tabu, adat istiadat, batasan agama
4. **Struktur administratif**: Terpusat vs Federal
5. **Sistem yudisial**: Juri vs Hakim profesional

---

## 7. Studi Kasus

### 7.1 Sistem Penentuan Kelayakan Kesejahteraan

**Hasil**:
- **Keputusan deterministik**: 85% kasus
- **JudicialDiscretion**: 15% kasus

### 7.2 Simulasi Pasal 709 KUH Perdata (Perbuatan Melawan Hukum)

**Skenario Pengujian**:
1. Perbuatan melawan hukum yang jelas disengaja → `Deterministic(Liable)`
2. Perbuatan melawan hukum karena kelalaian → `Deterministic(Liable)`
3. Kasus batas → `JudicialDiscretion`
4. Tidak ada perbuatan melawan hukum → `Deterministic(NotLiable)`
5. Tidak ada kausalitas → `Deterministic(NotLiable)`

### 7.3 Analisis Perbandingan Hukum Perbuatan Melawan Hukum 4 Negara

| Negara | Kode | Karakteristik |
|--------|------|---------------|
| Jepang | KUH Perdata Pasal 709 | Klausul umum (diskresi luas) |
| Jerman | BGB §823/§826 | Kepentingan yang dilindungi yang dienumerasi |
| Prancis | Code civil Art. 1240 | Abstraksi maksimum |
| AS | Hukum kasus | Ditipifikasi (Battery, dll.) |

---

## 8. Spesifikasi API & Detail Teknis

### 8.1 Tipe dan Trait Utama

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

### 8.2 Sistem Perintah CLI

```bash
# Parse
legalis parse <file.dsl> [--format json|yaml]

# Verifikasi
legalis verify <file.dsl> [--strict]

# Simulasi
legalis simulate <file.dsl> --population 1000

# Visualisasi
legalis visualize <file.dsl> --output tree.svg

# Ekspor
legalis export <file.dsl> --format solidity|catala|l4|rdf
```

---

## 9. Evaluasi

### 9.1 Benchmark Performa

| Operasi | Target | Waktu |
|---------|--------|-------|
| Parse DSL | 100 undang-undang | 15ms |
| Verifikasi | 100 undang-undang | 250ms |
| Simulasi | 10.000 agen | 1,2s |
| Simulasi | 100.000 agen | 8,5s |
| Pembuatan smart contract | 1 undang-undang | 45ms |
| Ekspor RDF | 100 undang-undang | 120ms |

### 9.2 Kualitas Kode

- **Cakupan pengujian**: Pengujian integrasi, pengujian properti, pengujian snapshot
- **Analisis statis**: Clippy (kebijakan tanpa peringatan)
- **Dokumentasi**: rustdoc untuk semua API publik

---

## 10. Pekerjaan Masa Depan

### 10.1 Web UI Frontend
- Dashboard berbasis React
- Visualisasi simulasi real-time
- Fitur pengeditan kolaboratif

### 10.2 Ekstensi VS Code
- Penyorotan sintaks DSL
- Verifikasi real-time
- Pelengkapan otomatis

### 10.3 Integrasi Jupyter Notebook
- Python binding via PyO3
- Analisis interaktif
- Widget visualisasi

### 10.4 Yurisdiksi Tambahan
- Hukum UE (integrasi EURLex)
- Hukum internasional (perjanjian, kesepakatan)
- Hukum agama (yurisprudensi Islam)

---

## 11. Kesimpulan

Legalis-RS menyajikan pendekatan baru untuk mengkodifikasi hukum dengan membuat "batas antara kemampuan komputasi dan penilaian manusia" eksplisit dalam sistem tipe.

**Pencapaian Utama**:

1. **Fondasi filosofis**: "Tata Kelola sebagai Kode, Keadilan sebagai Narasi"
2. **Sistem tipe**: Logika tiga-nilai via `LegalResult<T>`
3. **Arsitektur terintegrasi**: Desain komprehensif dengan 7 lapis dan 16 crate
4. **Implementasi**: Sekitar 450.000 baris kode Rust
5. **Verifikasi**: Integrasi OxiZ SMT solver (Pure Rust)
6. **Simulasi**: Engine bergaya ECS (dukungan akselerasi GPU)
7. **Output**: 25+ blockchain, RDF/TTL, berbagai format

**Filosofi Inti**: *"Tidak semua hal harus dapat dihitung."*

Bukan otomatisasi lengkap hukum, tetapi pemisahan yang jelas dari domain yang harus diotomatisasi dari domain yang memerlukan penilaian manusia. Inilah arsitektur "yurisprudensi generatif" yang dituju Legalis-RS.

---

## Referensi

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. Governatori, G., & Shams, Z. (2019). L4: Legal Language and Logic for Law. *JURIX 2019*.
5. Azzopardi, S., & Pace, G. J. (2018). Stipula: A domain-specific language for legal contracts. *JURIX 2018*.
6. Palmirani, M., & Vitali, F. (2011). Akoma-Ntoso for Legal Documents. *Legislative XML for the Semantic Web*.
7. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

## Lampiran

### A. Spesifikasi Tata Bahasa DSL

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

### B. Daftar Definisi Tipe

Untuk definisi lengkap tipe utama, lihat `crates/legalis-core/src/lib.rs`.

### C. Opsi Konfigurasi

```toml
[legalis]
default_jurisdiction = "ID"
enable_smt = true
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

*"Code is Law," kata mereka, tetapi kami mengambil pendekatan "Hukum menjadi Kode". Namun, kami menanamkan tipe yang disebut 'Kemanusiaan' ke dalam kode tersebut.*

---

**Tim Pengembangan Legalis-RS**
Versi 0.2.0 | 2024
