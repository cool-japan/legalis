# legalis-vn

Cộng hòa Xã hội Chủ nghĩa Việt Nam - Hỗ trợ Hệ thống Pháp luật cho Legalis-RS

**Phiên bản 0.1.6** - Bộ luật Lao động, Luật Doanh nghiệp, Luật Đầu tư

## Tổng quan (Overview)

`legalis-vn` cung cấp hỗ trợ toàn diện cho hệ thống pháp luật Việt Nam trong khung Legalis-RS. Việt Nam có hệ thống pháp luật dân sự (civil law) với đặc trưng xã hội chủ nghĩa, đang trong quá trình chuyển đổi sang nền kinh tế thị trường định hướng xã hội chủ nghĩa kể từ Đổi Mới (1986).

## Hệ thống Pháp luật Việt Nam (Vietnamese Legal System)

Hệ thống pháp luật Việt Nam có các đặc điểm:
- **Hệ thống pháp luật xã hội chủ nghĩa** - Dựa trên học thuyết Mác-Lênin
- **Truyền thống dân luật** - Ảnh hưởng từ pháp luật Pháp (thời thuộc địa)
- **Hiến pháp** - Hiến pháp năm 2013
- **Đổi Mới** - Cải cách kinh tế từ năm 1986
- **Hội nhập quốc tế** - WTO, CPTPP, EVFTA, RCEP

### So sánh với các hệ thống pháp luật khác

| Đặc điểm | Việt Nam | Trung Quốc | Pháp | Nhật Bản |
|----------|----------|------------|------|----------|
| Họ pháp luật | Dân luật (XHCN) | Dân luật (XHCN) | Dân luật | Dân luật |
| Nguồn chính | Luật & Nghị định | Luật & Quy định | Bộ luật | Bộ luật |
| Hiến pháp | 2013 | 1982 | 1958 | 1946 |
| Hệ thống tòa án | 4 cấp | 4 cấp | 3 cấp | 4 cấp |
| Tòa án tối cao | TAND Tối cao | Tòa án ND TC | Cour de cassation | Tối cao Pháp viện |

## Tính năng đã triển khai (Implemented Features)

### ✅ Bộ luật Lao động (Labor Code)

Bộ luật Lao động 2019 (Luật số 45/2019/QH14)
- ✅ Hợp đồng lao động (xác định thời hạn, không xác định thời hạn)
- ✅ Thời giờ làm việc (8 giờ/ngày, 48 giờ/tuần)
- ✅ Tiền lương tối thiểu vùng
- ✅ Trợ cấp thôi việc
- ✅ Nghỉ phép năm, nghỉ ốm, nghỉ thai sản
- ✅ Bảo hiểm xã hội, bảo hiểm y tế

```rust
use legalis_vn::labor_code::{LaborContract, ContractType, SeveranceCalculator};

let contract = LaborContract::new()
    .employee_name("Nguyễn Văn A")
    .contract_type(ContractType::IndefiniteTerm) // Không xác định thời hạn
    .monthly_salary(15_000_000) // VNĐ
    .start_date("2020-01-01")
    .region(Region::One) // Vùng I (Hà Nội, TP.HCM)
    .build()?;

// Tính trợ cấp thôi việc (Điều 46)
let severance = SeveranceCalculator::calculate(&contract, 5 /* năm */)?;
// Nửa tháng lương cho mỗi năm làm việc
```

### ✅ Luật Doanh nghiệp (Enterprise Law)

Luật Doanh nghiệp 2020 (Luật số 59/2020/QH14)
- ✅ Loại hình doanh nghiệp (TNHH, Cổ phần, Hợp danh, DNTN)
- ✅ Thành lập doanh nghiệp
- ✅ Quản trị công ty
- ✅ Quyền và nghĩa vụ của thành viên/cổ đông
- ✅ Giải thể và phá sản

```rust
use legalis_vn::enterprise::{Enterprise, EnterpriseType, validate_establishment};

let company = Enterprise::new()
    .name("Công ty TNHH Công nghệ ABC")
    .enterprise_type(EnterpriseType::LimitedLiabilityCompany)
    .charter_capital(1_000_000_000) // 1 tỷ VNĐ
    .members(vec!["Thành viên A", "Thành viên B"])
    .legal_representative("Nguyễn Văn B")
    .build()?;

assert!(validate_establishment(&company).is_ok());
```

### ✅ Luật Đầu tư (Investment Law)

Luật Đầu tư 2020 (Luật số 61/2020/QH14)
- ✅ Hình thức đầu tư (trực tiếp, gián tiếp, PPP)
- ✅ Ngành nghề đầu tư có điều kiện
- ✅ Ngành nghề cấm đầu tư kinh doanh
- ✅ Ưu đãi và hỗ trợ đầu tư
- ✅ Khu công nghiệp, khu kinh tế

```rust
use legalis_vn::investment::{Investment, InvestmentForm, validate_investment};

let investment = Investment::new()
    .investor_name("Foreign Investor Corp")
    .investment_form(InvestmentForm::ForeignDirectInvestment)
    .sector("Công nghệ thông tin")
    .capital(10_000_000) // USD
    .location("Khu công nghệ cao Hòa Lạc")
    .build()?;

assert!(validate_investment(&investment).is_ok());
```

## 📊 Trạng thái triển khai hiện tại

**Thống kê phiên bản 0.1.6:**
- ✅ **Bộ luật Lao động**: Luật số 45/2019/QH14
- ✅ **Luật Doanh nghiệp**: Luật số 59/2020/QH14
- ✅ **Luật Đầu tư**: Luật số 61/2020/QH14
- ✅ **Modules**: 16 modules (banking_law, citation, civil_code, common, competition_law, construction_law, consumer_protection, criminal_code, cybersecurity_law, enterprise, intellectual_property, investment, labor_code, land_law, statutes, tax_law)
- ✅ **Tests**: 154 tests passing, 0 warnings

## 🚧 Tính năng dự kiến

- 📋 Bộ luật Dân sự 2015
- 📋 Luật Thương mại 2005
- 📋 Luật Sở hữu trí tuệ
- 📋 Luật Bảo vệ môi trường
- 📋 Luật An ninh mạng

## Phụ thuộc (Dependencies)

- `chrono` - Xử lý ngày/giờ
- `serde` - Serialization
- `thiserror` - Xử lý lỗi

## Giấy phép (License)

Apache-2.0

## Liên kết liên quan (Related Links)

- [Cổng thông tin điện tử Chính phủ](https://vanban.chinhphu.vn/)
- [Tòa án nhân dân tối cao](https://www.toaan.gov.vn/)
- [Bộ Tư pháp](https://www.moj.gov.vn/)
- [Bộ Kế hoạch và Đầu tư](https://www.mpi.gov.vn/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
