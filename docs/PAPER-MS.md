# Legalis-RS: Seni Bina Jurisprudens Generatif

## Memisahkan Undang-undang dan Naratif: Pelan Tindakan untuk "Tadbir Urus sebagai Kod"

---

**Pengarang**: Pasukan Pembangunan Legalis-RS
**Versi**: 0.2.0
**Bahasa**: Rust (Edition 2024)
**Lesen**: MIT / Apache 2.0

---

## Abstrak

Kertas kerja ini membentangkan **Legalis-RS**, sebuah rangka kerja Rust untuk memisahkan dan menyusun dokumen undang-undang bahasa semula jadi secara ketat kepada **logik deterministik (Code)** dan **budi bicara kehakiman (Narrative)**.

Sistem undang-undang moden mengandungi campuran domain yang boleh diautomasikan komputer (keperluan umur, ambang pendapatan, pengiraan tarikh akhir) dan domain yang memerlukan tafsiran dan pertimbangan manusia ("sebab yang adil", "moral awam"). Pendekatan sebelumnya sama ada membiarkan sempadan ini kabur atau cuba automasi berlebihan yang berusaha menjadikan segala-galanya boleh dikira.

Legalis-RS memperkenalkan jenis logik tiga-nilai `LegalResult<T>` yang memanfaatkan sistem jenis Rust untuk menjadikan sempadan ini eksplisit pada peringkat jenis. Ini membolehkan paradigma baharu untuk nyahpepijat undang-undang, simulasi, dan pemindahan antarabangsa sambil mencegah "autokrasi algoritma" dalam era AI.

**Sumbangan Teknikal Utama**:
1. Bahasa Khusus Domain Undang-undang (DSL) dan pelaksanaan penghurai
2. Pengesahan formal dengan penyelesai Z3 SMT
3. Enjin simulasi gaya ECS untuk ramalan impak sosial
4. Penjanaan kontrak pintar untuk 25+ platform blockchain
5. Integrasi Linked Open Data (RDF/TTL) untuk web semantik
6. Pelaksanaan sistem undang-undang untuk 4 negara dengan penyesuaian parameter budaya (Soft ODA)

**Falsafah Teras**: *"Tidak semua perkara harus boleh dikira."*

---

## 1. Pengenalan

### 1.1 Latar Belakang: Hubungan Antara Undang-undang dan Pengkomputeran

Tesis terkenal Lawrence Lessig "Code is Law" menunjukkan bahawa seni bina (kod) dalam ruang siber mempunyai kuasa pengawalseliaan yang setara dengan undang-undang. Walau bagaimanapun, Legalis-RS membalikkan ini, mengamalkan pendekatan "**Undang-undang menjadi Kod**".

Mengkodifikasikan undang-undang menawarkan faedah berikut:

- **Kebolehan pengesahan**: Mengesan percanggahan logik pada masa kompilasi
- **Simulasi**: Meramalkan impak sosial sebelum penguatkuasaan
- **Kebolehkendalian antara**: Menukar dan membandingkan antara sistem undang-undang yang berbeza
- **Ketelusan**: Jejak audit lengkap proses keputusan undang-undang

Walau bagaimanapun, menjadikan semua undang-undang boleh dikira adalah berbahaya dari segi falsafah dan praktikal. Undang-undang secara semula jadi mengandungi domain yang memerlukan "pertimbangan manusia", dan automasi yang mengabaikan ini boleh membawa kepada "autokrasi AI".

### 1.2 Pernyataan Masalah: Cabaran Pemprosesan Undang-undang dalam Era AI

Teknologi undang-undang moden (LegalTech) menghadapi beberapa cabaran asas:

1. **Pengendalian kekaburan**: Banyak istilah undang-undang sengaja kabur, mengandaikan tafsiran kes demi kes
2. **Kebergantungan konteks**: Peruntukan yang sama boleh ditafsirkan secara berbeza bergantung pada konteks sosial dan budaya
3. **Perubahan temporal**: Undang-undang dipinda dan dimansuhkan, memerlukan pengurusan konsistensi merentasi masa
4. **Perbezaan antarabangsa**: Sistem undang-undang setiap negara berbeza dari asas falsafahnya

DSL undang-undang sedia ada (Catala, L4, Stipula) telah menangani beberapa cabaran ini, tetapi tiada yang mengambil pendekatan yang menjadikan "sempadan antara kebolehan pengkomputeran dan pertimbangan manusia" eksplisit dalam sistem jenis.

### 1.3 Cadangan: Pemisahan Kebolehan Pengkomputeran dan Budi Bicara Kehakiman

Teras Legalis-RS adalah pengenalan logik tiga-nilai melalui jenis `LegalResult<T>`:

```rust
pub enum LegalResult<T> {
    /// [Domain Deterministik] Hasil undang-undang yang boleh diproses secara automatik
    Deterministic(T),

    /// [Domain Budi Bicara] Domain yang memerlukan pertimbangan manusia
    JudicialDiscretion {
        issue: String,           // Isu yang dipersoalkan
        context_id: Uuid,        // Data kontekstual
        narrative_hint: Option<String>, // Pendapat rujukan oleh LLM
    },

    /// [Kerosakan Logik] Pepijat dalam undang-undang itu sendiri
    Void { reason: String },
}
```

Jenis ini menjamin bahawa hasil pemprosesan undang-undang sentiasa diklasifikasikan ke dalam salah satu daripada tiga kategori. Sistem berhenti memproses apabila mencapai `JudicialDiscretion` dan mewakilkan pertimbangan kepada manusia. Ini menjadi "kubu peringkat jenis" terhadap autokrasi AI.

### 1.4 Organisasi Kertas Kerja

Baki kertas kerja ini disusun seperti berikut:

- **Bahagian 2**: Kerja Berkaitan
- **Bahagian 3**: Falsafah dan Prinsip Reka Bentuk
- **Bahagian 4**: Seni Bina Sistem (struktur 7 lapisan)
- **Bahagian 5**: Teknologi Teras
- **Bahagian 6**: Pelaksanaan Bidang Kuasa
- **Bahagian 7**: Kajian Kes
- **Bahagian 8**: Spesifikasi API dan Butiran Teknikal
- **Bahagian 9**: Penilaian
- **Bahagian 10**: Kerja Masa Depan
- **Bahagian 11**: Kesimpulan

---

## 2. Kerja Berkaitan

### 2.1 Sejarah Undang-undang Pengkomputeran

Hubungan antara undang-undang dan komputer bermula dari projek LARC (Legal Analysis and Research Computer) pada tahun 1950-an.

| Era | Teknologi | Ciri-ciri |
|-----|-----------|-----------|
| 1950-an | LARC | Sistem mendapatkan maklumat undang-undang pertama |
| 1970-an | Sistem pakar jenis MYCIN | Penaakulan berasaskan peraturan |
| 1980-an | HYPO | Penaakulan berasaskan kes |
| 1990-an | Penstandardan XML/SGML | Penstrukturan dokumen undang-undang |
| 2000-an | Web Semantik | Perwakilan pengetahuan undang-undang berasaskan ontologi |
| 2010-an | Pembelajaran Mesin | Model ramalan undang-undang |
| 2020-an | LLM + Pengesahan Formal | Pendekatan hibrid |

### 2.2 Kedudukan Projek Ini

Legalis-RS memperluaskan penyelidikan sedia ada dengan cara berikut:

1. **Penandaan budi bicara peringkat jenis**: Logik tiga-nilai melalui `LegalResult<T>`
2. **Seni bina bersepadu**: Paip Parse→Verify→Simulate→Output
3. **Kebolehkendalian pelbagai format**: Penukaran dengan Catala/L4/Stipula/Akoma Ntoso
4. **Reka bentuk pengantarabangsaan**: Penyesuaian parameter budaya (Soft ODA)
5. **Integrasi blockchain**: Penjanaan kontrak pintar untuk 25+ platform

---

## 3. Falsafah & Prinsip Reka Bentuk

### 3.1 "Tadbir Urus sebagai Kod, Keadilan sebagai Naratif"

Slogan Legalis-RS mencerminkan perbezaan penting antara tadbir urus dan keadilan:

- **Tadbir Urus**: Penerapan peraturan, pematuhan prosedur, penentuan kelayakan → **Boleh dikodifikasi**
- **Keadilan**: Realisasi kesaksamaan, tafsiran kontekstual, pertimbangan nilai → **Diceritakan sebagai naratif**

Perbezaan ini sepadan dengan perbezaan antara "peraturan" dan "prinsip" (Dworkin) dalam falsafah undang-undang, atau antara "keadilan formal" dan "keadilan substantif".

### 3.2 Reka Bentuk Logik Tiga-Nilai

Tiga nilai `LegalResult<T>` sepadan dengan konsep falsafah undang-undang berikut:

| Jenis | Konsep Falsafah Undang-undang | Ejen Pemprosesan |
|-------|------------------------------|------------------|
| `Deterministic(T)` | Peraturan yang boleh digunakan secara mekanikal | Komputer |
| `JudicialDiscretion` | Prinsip yang memerlukan tafsiran | Manusia |
| `Void` | Jurang undang-undang/percanggahan | Penggubal undang-undang (perlu pembetulan) |

### 3.3 "Tidak semua perkara harus boleh dikira"

Terhadap godaan untuk menjadikan segala-galanya boleh dikira, Legalis-RS dengan jelas berkata "Tidak". Domain berikut sengaja direka bentuk sebagai tidak boleh dikira:

1. **Sebab yang adil**
2. **Ketenteraman awam dan moral**
3. **Niat baik**
4. **Kemunasabahan**

### 3.4 Mencegah Autokrasi AI

Legalis-RS mencegah autokrasi AI melalui mekanisme berikut:

1. **Pemberhentian paksa oleh jenis**: Pemberhentian automatik apabila mencapai `JudicialDiscretion`
2. **Jejak audit wajib**: Merekodkan semua proses keputusan
3. **Kebolehjelasan**: Output berstruktur sebab keputusan
4. **Gelung manusia terjamin**: Manusia sentiasa membuat keputusan muktamad dalam domain budi bicara

---

## 4. Seni Bina Sistem

### 4.1 Gambaran Keseluruhan Seni Bina 7 Lapisan

Legalis-RS terdiri daripada 7 lapisan berikut:

```
┌─────────────────────────────────────────────────────────┐
│                   Lapisan Infrastruktur                  │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                      Lapisan Output                      │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│               Lapisan Kebolehkendalian                   │
│                    (legalis-interop)                     │
├─────────────────────────────────────────────────────────┤
│               Lapisan Pengantarabangsaan                 │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│            Lapisan Simulasi & Analisis                   │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                   Lapisan Kecerdasan                     │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                      Lapisan Teras                       │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Lapisan Teras

#### legalis-core
Peti yang melaksanakan teras falsafah projek.

**Definisi Jenis Utama**:
- `LegalResult<T>`: Jenis logik tiga-nilai
- `Statute`: Perwakilan asas undang-undang
- `Condition`: Ungkapan syarat (AND/OR/NOT, umur, pendapatan, dll.)
- `Effect`: Kesan undang-undang (Grant/Revoke/Obligation/Prohibition)

#### legalis-dsl
Penghurai untuk bahasa khusus domain undang-undang.

**Contoh Sintaks DSL**:
```
STATUTE adult-voting: "Hak Mengundi Dewasa" {
    JURISDICTION "MY"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Hak mengundi"

    EXCEPTION WHEN HAS disqualified
    DISCRETION "Penentuan keupayaan mental memerlukan diagnosis doktor"
}
```

### 4.3 Lapisan Kecerdasan

#### legalis-llm
Lapisan abstraksi penyedia LLM.

**Penyedia yang Disokong**:
- OpenAI (GPT-4, GPT-4o)
- Anthropic (Claude)
- Google (Gemini)
- LLM Tempatan

#### legalis-verifier
Enjin pengesahan formal.

**Sasaran Pengesahan**:
- Pengesanan rujukan bulatan
- Pengesanan undang-undang tidak boleh dicapai (Dead Statute)
- Pengesanan percanggahan logik
- Pemeriksaan konflik perlembagaan
- Analisis kekaburan

### 4.4 Lapisan Simulasi

#### legalis-sim
Enjin simulasi gaya ECS.

**Ciri-ciri**:
- Simulasi berasaskan populasi (menyokong berjuta-juta ejen)
- Simulasi Monte Carlo
- Analisis sensitiviti
- Ujian A/B
- Pecutan GPU (CUDA/OpenCL/WebGPU)

### 4.5 Lapisan Pengantarabangsaan

#### legalis-i18n
Sokongan pelbagai bahasa dan pelbagai bidang kuasa.

**Bidang Kuasa yang Disokong**: JP, US, GB, DE, FR, ES, IT, CN, TW, KR, CA, AU, IN, BR, RU, SA, NL, CH, MX, SG, MY

### 4.6 Lapisan Output

#### legalis-chain
Penjanaan kontrak pintar.

**Platform yang Disokong (25+)**:
- EVM: Solidity, Vyper
- Substrate: Ink!
- Move: Aptos, Sui
- StarkNet: Cairo
- Cosmos: CosmWasm
- Lain-lain: TON FunC, Algorand Teal, Fuel Sway, Clarity, Noir, Leo, Circom

**Kekangan**: Hanya `Deterministic` yang boleh ditukar (`JudicialDiscretion` tidak boleh ditukar)

#### legalis-lod
Output Linked Open Data.

**Ontologi yang Disokong**:
- ELI (European Legislation Identifier)
- FaBiO
- LKIF-Core
- Akoma Ntoso
- Dublin Core
- SKOS

---

## 5. Teknologi Teras

### 5.1 DSL Undang-undang

**Struktur Asas**:
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

### 5.2 Jenis LegalResult<T> dan Nilai Kebenaran Separa

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
    Unknown,      // Maklumat tidak mencukupi
    Contradiction, // Percanggahan
}
```

### 5.3 Pengesahan Formal dengan Penyelesai Z3 SMT

**Sasaran Pengesahan**:
1. Rujukan bulatan
2. Undang-undang tidak boleh dicapai
3. Percanggahan logik
4. Konflik perlembagaan

### 5.4 Enjin Simulasi Gaya ECS

Enjin simulasi mengamalkan corak Entity-Component-System (ECS):
- **Entity**: Ejen warganegara
- **Component**: Atribut (umur, pendapatan, tempat tinggal, dll.)
- **System**: Logik penerapan undang-undang

### 5.5 Penjanaan Kontrak Pintar

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

## 6. Pelaksanaan Bidang Kuasa

### 6.1 Sistem Undang-undang Jepun

Peti legalis-jp menyediakan perwakilan berstruktur Perlembagaan Jepun.

### 6.2 Jerman, Perancis, AS (Dirancang)

| Bidang Kuasa | Status | Bidang Fokus |
|--------------|--------|--------------|
| Jerman (DE) | Dalam pembangunan | BGB, GG |
| Perancis (FR) | Dalam pembangunan | Code civil, Perlembagaan |
| AS (US) | Dalam pembangunan | UCC, Perlembagaan, Undang-undang kes |

### 6.3 Penyesuaian Parameter Budaya (Soft ODA)

Parameter budaya berikut dipertimbangkan dalam pemindahan sistem undang-undang antarabangsa:

1. **Sistem undang-undang**: Civil law vs Common law vs Religious law
2. **Struktur bahasa**: Kebolehterjemahan istilah undang-undang
3. **Norma sosial**: Pantang larang, adat, kekangan agama
4. **Struktur pentadbiran**: Berpusat vs Persekutuan
5. **Sistem kehakiman**: Juri vs Hakim profesional

---

## 7. Kajian Kes

### 7.1 Sistem Penentuan Kelayakan Kebajikan

**Keputusan**:
- **Keputusan deterministik**: 85% kes
- **JudicialDiscretion**: 15% kes (pertimbangan "kesegeraan", "keperluan sebenar", dll.)

### 7.2 Simulasi Artikel 709 Kanun Sivil (Tort)

**Senario Ujian**:
1. Tort sengaja yang jelas → `Deterministic(Liable)`
2. Tort akibat kecuaian → `Deterministic(Liable)`
3. Kes sempadan → `JudicialDiscretion`
4. Tiada tort → `Deterministic(NotLiable)`
5. Tiada sebab akibat → `Deterministic(NotLiable)`

### 7.3 Analisis Perbandingan Undang-undang Tort 4 Negara

| Negara | Kod | Ciri-ciri |
|--------|-----|-----------|
| Jepun | Kanun Sivil Art. 709 | Klausa umum (budi bicara luas) |
| Jerman | BGB §823/§826 | Kepentingan dilindungi yang dienumerasi |
| Perancis | Code civil Art. 1240 | Abstraksi maksimum |
| AS | Undang-undang kes | Bertaip (Battery, dll.) |

---

## 8. Spesifikasi API & Butiran Teknikal

### 8.1 Jenis dan Trait Utama

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

### 8.2 Sistem Arahan CLI

```bash
# Hurai
legalis parse <file.dsl> [--format json|yaml]

# Sahkan
legalis verify <file.dsl> [--strict]

# Simulasi
legalis simulate <file.dsl> --population 1000

# Visualisasi
legalis visualize <file.dsl> --output tree.svg

# Eksport
legalis export <file.dsl> --format solidity|catala|l4|rdf
```

---

## 9. Penilaian

### 9.1 Penanda Aras Prestasi

| Operasi | Sasaran | Masa |
|---------|---------|------|
| Penghuraian DSL | 100 undang-undang | 15ms |
| Pengesahan | 100 undang-undang | 250ms |
| Simulasi | 10,000 ejen | 1.2s |
| Simulasi | 100,000 ejen | 8.5s |
| Penjanaan kontrak pintar | 1 undang-undang | 45ms |
| Eksport RDF | 100 undang-undang | 120ms |

### 9.2 Kualiti Kod

- **Liputan ujian**: Ujian integrasi, ujian sifat, ujian snapshot
- **Analisis statik**: Clippy (dasar sifar amaran)
- **Dokumentasi**: rustdoc untuk semua API awam

---

## 10. Kerja Masa Depan

### 10.1 Frontend Web UI
- Papan pemuka berasaskan React
- Visualisasi simulasi masa nyata
- Ciri penyuntingan kolaboratif

### 10.2 Sambungan VS Code
- Penyerlahan sintaks DSL
- Pengesahan masa nyata
- Autolengkap

### 10.3 Integrasi Jupyter Notebook
- Ikatan Python melalui PyO3
- Analisis interaktif
- Widget visualisasi

### 10.4 Bidang Kuasa Tambahan
- Undang-undang EU (integrasi EURLex)
- Undang-undang antarabangsa (perjanjian, persetujuan)
- Undang-undang agama (jurisprudens Islam)

---

## 11. Kesimpulan

Legalis-RS membentangkan pendekatan baharu untuk mengkodifikasikan undang-undang dengan menjadikan "sempadan antara kebolehan pengkomputeran dan pertimbangan manusia" eksplisit dalam sistem jenis.

**Pencapaian Utama**:

1. **Asas falsafah**: "Tadbir Urus sebagai Kod, Keadilan sebagai Naratif"
2. **Sistem jenis**: Logik tiga-nilai melalui `LegalResult<T>`
3. **Seni bina bersepadu**: Reka bentuk komprehensif dengan 7 lapisan dan 16 peti
4. **Pelaksanaan**: Kira-kira 450,000 baris kod Rust
5. **Pengesahan**: Integrasi penyelesai Z3 SMT
6. **Simulasi**: Enjin gaya ECS (sokongan pecutan GPU)
7. **Output**: 25+ blockchain, RDF/TTL, pelbagai format

**Falsafah Teras**: *"Tidak semua perkara harus boleh dikira."*

Bukan automasi lengkap undang-undang, tetapi pemisahan jelas domain yang harus diautomasikan daripada domain yang memerlukan pertimbangan manusia. Inilah seni bina "jurisprudens generatif" yang disasarkan Legalis-RS.

---

## Rujukan

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. Governatori, G., & Shams, Z. (2019). L4: Legal Language and Logic for Law. *JURIX 2019*.
5. Azzopardi, S., & Pace, G. J. (2018). Stipula: A domain-specific language for legal contracts. *JURIX 2018*.
6. Palmirani, M., & Vitali, F. (2011). Akoma-Ntoso for Legal Documents. *Legislative XML for the Semantic Web*.
7. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

## Lampiran

### A. Spesifikasi Tatabahasa DSL

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

### B. Senarai Definisi Jenis

Untuk definisi lengkap jenis utama, lihat `crates/legalis-core/src/lib.rs`.

### C. Pilihan Konfigurasi

```toml
[legalis]
default_jurisdiction = "MY"
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

*"Code is Law," kata mereka, tetapi kami mengambil pendekatan "Undang-undang menjadi Kod". Walau bagaimanapun, kami membenamkan jenis yang dipanggil 'Kemanusiaan' ke dalam kod tersebut.*

---

**Pasukan Pembangunan Legalis-RS**
Versi 0.2.0 | 2024
