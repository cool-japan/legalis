# legalis-th

ราชอาณาจักรไทย (Thailand) - ระบบกฎหมายสำหรับ Legalis-RS

**เวอร์ชัน 0.1.7** - กฎหมายแรงงาน, พ.ร.บ.คุ้มครองข้อมูลส่วนบุคคล, พ.ร.บ.ส่งเสริมการลงทุน, พ.ร.บ.การประกอบธุรกิจของคนต่างด้าว

## ภาพรวม (Overview)

`legalis-th` ให้การสนับสนุนอย่างครอบคลุมสำหรับระบบกฎหมายไทยในกรอบ Legalis-RS ประเทศไทยใช้ระบบกฎหมายแบบประมวลกฎหมาย (Civil Law) โดยได้รับอิทธิพลจากระบบกฎหมายของยุโรปภาคพื้นทวีป โดยเฉพาะกฎหมายเยอรมันและญี่ปุ่น

## ระบบกฎหมายไทย (Thai Legal System)

ระบบกฎหมายไทยมีลักษณะเด่น:
- **ระบบประมวลกฎหมาย (Civil Law)** - ได้รับอิทธิพลจากกฎหมายเยอรมันและญี่ปุ่น
- **รัฐธรรมนูญ** - รัฐธรรมนูญแห่งราชอาณาจักรไทย พ.ศ. 2560
- **พระราชบัญญัติ** - กฎหมายที่ตราโดยรัฐสภา
- **ระบบศาล** - ศาลยุติธรรม ศาลปกครอง ศาลรัฐธรรมนูญ
- **ปีพุทธศักราช** - ใช้ปฏิทินพุทธศักราช (พ.ศ. = ค.ศ. + 543)

### เปรียบเทียบกับระบบกฎหมายอื่น

| ลักษณะ | ไทย | ญี่ปุ่น | สิงคโปร์ | สหรัฐอเมริกา |
|--------|-----|--------|---------|-------------|
| ตระกูลกฎหมาย | Civil Law | Civil Law | Common Law | Common Law |
| แหล่งที่มาหลัก | ประมวลกฎหมาย | ประมวลกฎหมาย | คำพิพากษา | คำพิพากษา |
| รัฐธรรมนูญ | 2560 (2017) | 1946 | 1965 | 1787 |
| ระบบศาล | 3 ระบบ | 4 ชั้น | 3 ชั้น | สหพันธรัฐและรัฐ |
| ภาษาราชการ | ไทย | ญี่ปุ่น | อังกฤษ | อังกฤษ |

## คุณสมบัติที่พัฒนาแล้ว (Implemented Features)

### ✅ ปฏิทินพุทธศักราช (Buddhist Era Calendar)

การแปลงวันที่ระหว่างปีพุทธศักราช (พ.ศ.) และคริสต์ศักราช (ค.ศ.)
- ✅ แปลง พ.ศ. เป็น ค.ศ.
- ✅ แปลง ค.ศ. เป็น พ.ศ.
- ✅ รูปแบบวันที่ราชการไทย

```rust
use legalis_th::calendar::{BuddhistEra, to_buddhist_era, to_gregorian};

// แปลงจาก ค.ศ. เป็น พ.ศ.
let be_year = to_buddhist_era(2024);
assert_eq!(be_year, 2567);

// แปลงจาก พ.ศ. เป็น ค.ศ.
let ce_year = to_gregorian(2567);
assert_eq!(ce_year, 2024);
```

### ✅ กฎหมายคุ้มครองข้อมูลส่วนบุคคล (PDPA)

พระราชบัญญัติคุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562
- ✅ สิทธิของเจ้าของข้อมูลส่วนบุคคล
- ✅ หน้าที่ของผู้ควบคุมข้อมูลส่วนบุคคล
- ✅ ฐานทางกฎหมายในการประมวลผลข้อมูล
- ✅ การส่งหรือโอนข้อมูลส่วนบุคคลไปต่างประเทศ
- ✅ ข้อมูลส่วนบุคคลที่มีความอ่อนไหว
- ✅ การแจ้งเหตุละเมิดข้อมูลส่วนบุคคล

```rust
use legalis_th::data_protection::{DataProcessing, LawfulBasis, validate_processing};

let processing = DataProcessing::new()
    .controller("บริษัท ข้อมูล จำกัด")
    .purpose("การให้บริการลูกค้า")
    .lawful_basis(LawfulBasis::Consent) // ความยินยอม
    .data_categories(vec!["ชื่อ", "เลขบัตรประชาชน", "ที่อยู่"])
    .cross_border_transfer(false)
    .build()?;

assert!(validate_processing(&processing).is_ok());
```

### ✅ กฎหมายแรงงาน (Labor Law)

พระราชบัญญัติคุ้มครองแรงงาน พ.ศ. 2541 (แก้ไขเพิ่มเติม)
- ✅ สัญญาจ้างแรงงาน
- ✅ เวลาทำงาน (8 ชั่วโมง/วัน, 48 ชั่วโมง/สัปดาห์)
- ✅ ค่าจ้างขั้นต่ำ (ตามจังหวัด)
- ✅ ค่าชดเชย (ตามอายุงาน)
- ✅ วันหยุดและวันลา (พักผ่อนประจำปี, ลาป่วย, ลาคลอด)
- ✅ การเลิกจ้างและการบอกกล่าวล่วงหน้า

```rust
use legalis_th::labor_law::{EmploymentContract, SeveranceCalculator};

let contract = EmploymentContract::new()
    .employee_name("สมชาย ใจดี")
    .employer("บริษัท เทคโนโลยี จำกัด")
    .monthly_salary(35_000) // บาท
    .start_date("2562-01-01") // พ.ศ.
    .province("กรุงเทพมหานคร")
    .build()?;

// คำนวณค่าชดเชย (มาตรา 118)
let severance = SeveranceCalculator::calculate(&contract, 5 /* ปี */)?;
// 1-3 ปี = 90 วัน, 3-6 ปี = 180 วัน, 6-10 ปี = 240 วัน, >10 ปี = 300 วัน
```

### ✅ พ.ร.บ.การประกอบธุรกิจของคนต่างด้าว (Foreign Business Act)

พระราชบัญญัติการประกอบธุรกิจของคนต่างด้าว พ.ศ. 2542
- ✅ บัญชีธุรกิจ (บัญชี 1, 2, 3)
- ✅ การขอใบอนุญาตประกอบธุรกิจ
- ✅ ข้อยกเว้นสำหรับสนธิสัญญา
- ✅ สัดส่วนการถือหุ้นของคนต่างด้าว

```rust
use legalis_th::foreign_business::{Business, BusinessList, validate_foreign_business};

let business = Business::new()
    .name("Thai-Japan Technology Co., Ltd.")
    .business_type("Software Development")
    .foreign_ownership_percentage(49) // ไม่เกิน 50% สำหรับบัญชี 3
    .business_list(BusinessList::Three)
    .build()?;

assert!(validate_foreign_business(&business).is_ok());
```

### ✅ พ.ร.บ.ส่งเสริมการลงทุน (BOI)

พระราชบัญญัติส่งเสริมการลงทุน พ.ศ. 2520 (แก้ไขเพิ่มเติม)
- ✅ ประเภทกิจการที่ได้รับการส่งเสริม
- ✅ สิทธิประโยชน์ทางภาษี
- ✅ สิทธิประโยชน์ที่มิใช่ภาษี
- ✅ การถือครองที่ดินของคนต่างด้าว

## 📊 สถานะการพัฒนาปัจจุบัน

**สถิติเวอร์ชัน 0.1.7:**
- ✅ **ปฏิทินพุทธศักราช**: การแปลงวันที่ครบถ้วน
- ✅ **PDPA**: พ.ร.บ.คุ้มครองข้อมูลส่วนบุคคล
- ✅ **กฎหมายแรงงาน**: พ.ร.บ.คุ้มครองแรงงาน
- ✅ **FBA**: พ.ร.บ.การประกอบธุรกิจของคนต่างด้าว
- ✅ **โมดูล**: 19 โมดูล (calendar, citation, civil_commercial_code, company_act, competition_law, foreign_business, investment_promotion, land_code, labor_law, consumer_protection, data_protection, tax_law, arbitration, bankruptcy, criminal_code, immigration, intellectual_property, securities_law, statutes)
- ✅ **การทดสอบ**: 174 การทดสอบผ่าน, 0 คำเตือน

## 🚧 คุณสมบัติที่วางแผนไว้

- 📋 ประมวลกฎหมายแพ่งและพาณิชย์
- 📋 พ.ร.บ.บริษัทมหาชนจำกัด
- 📋 ประมวลรัษฎากร
- 📋 กฎหมายทรัพย์สินทางปัญญา

## การพึ่งพา (Dependencies)

- `chrono` - การจัดการวันที่/เวลา
- `serde` - การ Serialize
- `thiserror` - การจัดการข้อผิดพลาด

## สัญญาอนุญาต (License)

Apache-2.0

## ลิงก์ที่เกี่ยวข้อง (Related Links)

- [ราชกิจจานุเบกษา](https://ratchakitcha.soc.go.th/)
- [สำนักงานคณะกรรมการกฤษฎีกา](https://www.krisdika.go.th/)
- [ศาลยุติธรรม](https://www.coj.go.th/)
- [BOI](https://www.boi.go.th/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
