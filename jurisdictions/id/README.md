# legalis-id

Republik Indonesia - Dukungan Sistem Hukum untuk Legalis-RS

**Versi 0.1.3** - Hukum Perdata, PDP, Ketenagakerjaan, Investasi, Omnibus Law

## Ikhtisar (Overview)

`legalis-id` menyediakan dukungan komprehensif untuk sistem hukum Indonesia dalam kerangka Legalis-RS. Indonesia memiliki sistem hukum campuran yang menggabungkan hukum perdata (warisan Belanda), hukum Islam, dan hukum adat.

## Sistem Hukum Indonesia (Indonesian Legal System)

Sistem hukum Indonesia ditandai dengan:
- **Hukum Perdata** - Berdasarkan Burgerlijk Wetboek (KUHPerdata) warisan kolonial Belanda
- **Hukum Islam** - Berlaku untuk umat Muslim dalam hukum keluarga dan waris
- **Hukum Adat** - Hukum kebiasaan masyarakat lokal yang diakui
- **UUD 1945** - Konstitusi negara dengan Pancasila sebagai dasar negara
- **Omnibus Law** - UU Cipta Kerja 2020 (reformasi perizinan dan ketenagakerjaan)

### Perbandingan dengan Sistem Hukum Lain

| Fitur | Indonesia | Belanda | Malaysia | Jepang |
|-------|-----------|---------|----------|--------|
| Keluarga Hukum | Campuran (Perdata/Islam/Adat) | Hukum Perdata | Campuran (Common/Islam) | Hukum Perdata |
| Sumber Utama | UU & Peraturan | Kitab UU | Case Law & Statutes | Kitab UU |
| Konstitusi | UUD 1945 | 1815 | 1957 | 1946 |
| Sistem Pengadilan | 4 tingkat | 3 tingkat | 3 tingkat | 4 tingkat |
| Pengadilan Agama | Ya (Peradilan Agama) | Tidak | Ya (Syariah Courts) | Tidak |

## Fitur yang Diimplementasikan (Implemented Features)

### ✅ Hukum Perdata (Civil Code)

Kitab Undang-Undang Hukum Perdata (KUHPerdata/BW)
- ✅ Hukum Orang (Buku I)
- ✅ Hukum Benda (Buku II)
- ✅ Hukum Perikatan (Buku III)
- ✅ Syarat sah perjanjian (Pasal 1320)

```rust
use legalis_id::civil_code::{Contract, validate_contract_formation};

let contract = Contract::new()
    .parties(vec!["Pihak Pertama", "Pihak Kedua"])
    .subject("Jual beli tanah")
    .price(500_000_000) // Rupiah
    .build()?;

// Validasi syarat sah perjanjian (Pasal 1320 KUHPerdata)
// 1. Sepakat, 2. Cakap, 3. Hal tertentu, 4. Sebab yang halal
assert!(validate_contract_formation(&contract).is_ok());
```

### ✅ Perlindungan Data Pribadi (PDP Law)

Undang-Undang No. 27 Tahun 2022 tentang Pelindungan Data Pribadi
- ✅ Hak subjek data pribadi
- ✅ Kewajiban pengendali data pribadi
- ✅ Prosesor data pribadi
- ✅ Transfer data pribadi lintas batas
- ✅ Data pribadi spesifik (sensitif)
- ✅ Sanksi administratif dan pidana

```rust
use legalis_id::data_protection::{DataProcessing, LawfulBasis, validate_processing};

let processing = DataProcessing::new()
    .controller("PT Data Indonesia")
    .purpose("Layanan pelanggan")
    .lawful_basis(LawfulBasis::Consent) // Persetujuan
    .data_categories(vec!["nama", "NIK", "alamat"])
    .retention_period_days(365 * 5) // 5 tahun
    .build()?;

assert!(validate_processing(&processing).is_ok());
```

### ✅ Ketenagakerjaan (Labor Law)

UU No. 13 Tahun 2003 jo. UU No. 11 Tahun 2020 (Cipta Kerja)
- ✅ Perjanjian kerja (PKWT dan PKWTT)
- ✅ Waktu kerja (40 jam/minggu)
- ✅ Upah minimum (UMP/UMK)
- ✅ Pesangon dan uang penghargaan masa kerja
- ✅ PHK dan penyelesaian perselisihan
- ✅ BPJS Ketenagakerjaan dan Kesehatan

```rust
use legalis_id::labor_law::{EmploymentContract, ContractType, SeveranceCalculator};

let contract = EmploymentContract::new()
    .employee_name("Budi Santoso")
    .contract_type(ContractType::PKWTT) // Perjanjian Kerja Waktu Tidak Tertentu
    .monthly_salary(8_000_000) // Rupiah
    .start_date("2020-01-01")
    .build()?;

// Hitung pesangon (UU Cipta Kerja)
let severance = SeveranceCalculator::calculate(&contract, 5 /* tahun kerja */)?;
```

### ✅ Investasi (Investment Law)

UU No. 25 Tahun 2007 jo. UU No. 11 Tahun 2020 (Cipta Kerja)
- ✅ Penanaman Modal Asing (PMA)
- ✅ Penanaman Modal Dalam Negeri (PMDN)
- ✅ Daftar Negatif Investasi (DNI) → Daftar Prioritas
- ✅ Perizinan Berusaha Berbasis Risiko (OSS-RBA)
- ✅ Fasilitas dan insentif investasi

```rust
use legalis_id::investment::{Investment, InvestmentType, validate_investment};

let investment = Investment::new()
    .investor_name("PT Investasi Global")
    .investment_type(InvestmentType::ForeignDirect) // PMA
    .sector("Teknologi Informasi")
    .capital(10_000_000_000) // Rupiah (10 miliar)
    .foreign_ownership_percentage(100) // Sektor terbuka 100%
    .build()?;

assert!(validate_investment(&investment).is_ok());
```

## 📊 Status Implementasi Saat Ini

**Statistik Versi 0.1.3:**
- ✅ **Hukum Perdata**: KUHPerdata dasar
- ✅ **PDP**: UU No. 27/2022 lengkap
- ✅ **Ketenagakerjaan**: UU 13/2003 jo. UU 11/2020
- ✅ **Investasi**: Kerangka PMA/PMDN
- ✅ **Modul**: 7 modul (civil_code, data_protection, labor_law, investment, common, citation)

## 🚧 Fitur yang Direncanakan

- 📋 Hukum Perusahaan (UU PT)
- 📋 Hukum Pertanahan (UUPA)
- 📋 Hukum Lingkungan (UUPPLH)
- 📋 Hukum Persaingan Usaha
- 📋 Hukum Kepailitan

## Dependensi (Dependencies)

- `chrono` - Penanganan tanggal/waktu
- `serde` - Serialisasi
- `thiserror` - Penanganan error

## Lisensi (License)

MIT OR Apache-2.0

## Tautan Terkait (Related Links)

- [JDIH Kemenkumham](https://peraturan.go.id/)
- [Mahkamah Agung RI](https://www.mahkamahagung.go.id/)
- [BKPM (Investasi)](https://www.bkpm.go.id/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
