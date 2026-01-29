//! Vietnamese Law on Consumer Rights Protection (Luật Bảo vệ quyền lợi người tiêu dùng)
//!
//! Law No. 59/2010/QH12, effective from July 1, 2011.
//! Amended by Laws 06/2023.
//!
//! ## Consumer Rights (Article 8)
//!
//! 1. Right to safety (Quyền được bảo đảm an toàn)
//! 2. Right to be informed (Quyền được thông tin)
//! 3. Right to choose (Quyền được lựa chọn)
//! 4. Right to be heard (Quyền được bày tỏ ý kiến)
//! 5. Right to compensation (Quyền được bồi thường)
//! 6. Right to education (Quyền được giáo dục)
//! 7. Right to representation (Quyền được đại diện)
//! 8. Right to privacy (Quyền riêng tư)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Consumer rights (Quyền của người tiêu dùng) - Article 8
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsumerRight {
    /// Right to safety (Quyền được bảo đảm an toàn)
    Safety,
    /// Right to be informed (Quyền được thông tin đầy đủ, chính xác)
    Information,
    /// Right to choose (Quyền được lựa chọn hàng hóa, dịch vụ)
    Choice,
    /// Right to be heard (Quyền được bày tỏ ý kiến)
    Voice,
    /// Right to compensation (Quyền được bồi thường thiệt hại)
    Compensation,
    /// Right to education (Quyền được giáo dục về tiêu dùng)
    Education,
    /// Right to representation (Quyền được các tổ chức bảo vệ đại diện)
    Representation,
    /// Right to privacy (Quyền riêng tư, bảo vệ dữ liệu cá nhân)
    Privacy,
}

impl ConsumerRight {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> &'static str {
        match self {
            Self::Safety => "Quyền được bảo đảm an toàn về tính mạng, sức khỏe, tài sản",
            Self::Information => "Quyền được cung cấp thông tin đầy đủ, chính xác",
            Self::Choice => "Quyền tự do lựa chọn hàng hóa, dịch vụ",
            Self::Voice => "Quyền được bày tỏ ý kiến về hàng hóa, dịch vụ",
            Self::Compensation => "Quyền được bồi thường thiệt hại",
            Self::Education => "Quyền được giáo dục về tiêu dùng",
            Self::Representation => "Quyền được đại diện, bảo vệ quyền lợi",
            Self::Privacy => "Quyền riêng tư, bảo vệ thông tin cá nhân",
        }
    }

    /// Get English description
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Safety => "Right to safety and health protection",
            Self::Information => "Right to full and accurate information",
            Self::Choice => "Right to freedom of choice",
            Self::Voice => "Right to be heard and express opinions",
            Self::Compensation => "Right to compensation for damages",
            Self::Education => "Right to consumer education",
            Self::Representation => "Right to representation and protection",
            Self::Privacy => "Right to privacy and data protection",
        }
    }

    /// Get article reference
    pub fn article(&self) -> &'static str {
        "Điều 8"
    }
}

/// Product warranty requirements (Bảo hành sản phẩm) - Article 13-16
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductWarranty {
    /// Product name
    pub product_name: String,
    /// Warranty period in months
    pub warranty_months: u16,
    /// Warranty coverage description
    pub coverage: String,
    /// Warranty conditions
    pub conditions: Vec<String>,
}

impl ProductWarranty {
    /// Minimum warranty period for durable goods (months)
    pub const MIN_DURABLE_GOODS: u16 = 12; // 12 months

    /// Check if warranty period meets minimum requirements
    pub fn meets_minimum_requirement(&self, is_durable_good: bool) -> bool {
        if is_durable_good {
            self.warranty_months >= Self::MIN_DURABLE_GOODS
        } else {
            true // No minimum for non-durable goods
        }
    }

    /// Get warranty expiry date (months from purchase)
    pub fn expiry_months(&self) -> u16 {
        self.warranty_months
    }
}

/// Product recall (Thu hồi sản phẩm) - Article 18-20
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRecall {
    /// Product name and details
    pub product_name: String,
    /// Reason for recall
    pub recall_reason: RecallReason,
    /// Number of units affected
    pub affected_units: u64,
    /// Recall announced publicly
    pub publicly_announced: bool,
    /// Compensation offered
    pub compensation_offered: bool,
}

impl ProductRecall {
    /// Check if recall is compliant with law
    pub fn is_compliant(&self) -> bool {
        // Must publicly announce recall
        if !self.publicly_announced {
            return false;
        }

        // Must offer compensation if dangerous
        if matches!(self.recall_reason, RecallReason::SafetyHazard) && !self.compensation_offered {
            return false;
        }

        true
    }
}

/// Recall reasons (Lý do thu hồi)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecallReason {
    /// Safety hazard (Nguy hiểm đến an toàn)
    SafetyHazard,
    /// Quality defect (Lỗi chất lượng)
    QualityDefect,
    /// False advertising (Quảng cáo gian dối)
    FalseAdvertising,
    /// Expired product (Hết hạn sử dụng)
    Expired,
    /// Other
    Other(String),
}

impl RecallReason {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::SafetyHazard => "Nguy hiểm đến sức khỏe, tính mạng".to_string(),
            Self::QualityDefect => "Lỗi nghiêm trọng về chất lượng".to_string(),
            Self::FalseAdvertising => "Quảng cáo gian dối, lừa đảo".to_string(),
            Self::Expired => "Hết hạn sử dụng, không an toàn".to_string(),
            Self::Other(reason) => reason.clone(),
        }
    }
}

/// Consumer complaint (Khiếu nại của người tiêu dùng) - Article 37-41
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerComplaint {
    /// Consumer name
    pub consumer_name: String,
    /// Business/seller name
    pub business_name: String,
    /// Product or service
    pub subject: String,
    /// Complaint details
    pub complaint_details: String,
    /// Requested remedy
    pub requested_remedy: RemedyType,
    /// Evidence provided
    pub has_evidence: bool,
}

impl ConsumerComplaint {
    /// Check if complaint is valid (Article 38)
    pub fn is_valid(&self) -> bool {
        // Must have complete information
        !self.consumer_name.is_empty()
            && !self.business_name.is_empty()
            && !self.subject.is_empty()
            && !self.complaint_details.is_empty()
    }

    /// Get resolution deadline in days (Article 40)
    pub fn resolution_deadline_days(&self) -> u8 {
        match self.requested_remedy {
            RemedyType::Refund | RemedyType::Exchange => 15, // 15 days for refund/exchange
            RemedyType::Repair => 30,                        // 30 days for repair
            RemedyType::Compensation => 30,                  // 30 days for compensation
            RemedyType::Other => 30,
        }
    }
}

/// Remedy types (Biện pháp khắc phục) - Article 31
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RemedyType {
    /// Refund (Hoàn tiền)
    Refund,
    /// Exchange (Đổi hàng)
    Exchange,
    /// Repair (Sửa chữa)
    Repair,
    /// Compensation (Bồi thường thiệt hại)
    Compensation,
    /// Other remedy
    Other,
}

impl RemedyType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::Refund => "Trả lại hàng và hoàn tiền",
            Self::Exchange => "Đổi hàng hóa",
            Self::Repair => "Sửa chữa, bảo hành",
            Self::Compensation => "Bồi thường thiệt hại",
            Self::Other => "Biện pháp khác",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Refund => "Refund",
            Self::Exchange => "Exchange",
            Self::Repair => "Repair",
            Self::Compensation => "Compensation",
            Self::Other => "Other remedy",
        }
    }
}

/// Unfair commercial practices (Hành vi thương mại không công bằng) - Article 4-7
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnfairPractice {
    /// False or misleading advertising (Quảng cáo gian dối)
    FalseAdvertising,
    /// Forced sales (Ép buộc tiêu dùng)
    ForcedSales,
    /// Price manipulation (Thao túng giá)
    PriceManipulation,
    /// Discriminatory practices (Phân biệt đối xử)
    Discrimination,
    /// Misleading pricing (Niêm yết giá gian dối)
    MisleadingPricing,
    /// Hidden charges (Phí ẩn)
    HiddenCharges,
    /// Bait and switch (Mồi nhử)
    BaitAndSwitch,
    /// Other unfair practice
    Other(String),
}

impl UnfairPractice {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::FalseAdvertising => "Quảng cáo sai sự thật, gây nhầm lẫn".to_string(),
            Self::ForcedSales => "Ép buộc người tiêu dùng mua hàng".to_string(),
            Self::PriceManipulation => "Thao túng giá cả bất hợp lý".to_string(),
            Self::Discrimination => "Phân biệt đối xử người tiêu dùng".to_string(),
            Self::MisleadingPricing => "Niêm yết giá không đúng thực tế".to_string(),
            Self::HiddenCharges => "Tính phí ẩn, không thông báo trước".to_string(),
            Self::BaitAndSwitch => "Quảng cáo hàng này nhưng bán hàng khác".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }

    /// Get article reference
    pub fn article(&self) -> u32 {
        match self {
            Self::FalseAdvertising => 4,
            Self::ForcedSales => 5,
            Self::PriceManipulation => 6,
            Self::Discrimination => 7,
            _ => 4,
        }
    }
}

/// Result type for consumer protection operations
pub type ConsumerProtectionResult<T> = Result<T, ConsumerProtectionError>;

/// Errors related to Consumer Protection Law
#[derive(Debug, Error)]
pub enum ConsumerProtectionError {
    /// Consumer right violation
    #[error("Vi phạm quyền người tiêu dùng ({article}): {right}")]
    RightViolation { article: String, right: String },

    /// Warranty violation
    #[error("Vi phạm quy định bảo hành (Điều 13): {reason}")]
    WarrantyViolation { reason: String },

    /// Unfair commercial practice
    #[error("Hành vi thương mại không công bằng (Điều {article}): {practice}")]
    UnfairPractice { article: u32, practice: String },

    /// Product recall violation
    #[error("Vi phạm quy định thu hồi sản phẩm (Điều 18): {reason}")]
    RecallViolation { reason: String },

    /// Other consumer protection violation
    #[error("Vi phạm Luật Bảo vệ quyền lợi người tiêu dùng: {reason}")]
    ConsumerViolation { reason: String },
}

/// Validate warranty meets requirements
pub fn validate_warranty(
    warranty: &ProductWarranty,
    is_durable_good: bool,
) -> ConsumerProtectionResult<()> {
    if !warranty.meets_minimum_requirement(is_durable_good) {
        Err(ConsumerProtectionError::WarrantyViolation {
            reason: format!(
                "Thời hạn bảo hành {} tháng không đủ tối thiểu {} tháng cho hàng bền",
                warranty.warranty_months,
                ProductWarranty::MIN_DURABLE_GOODS
            ),
        })
    } else {
        Ok(())
    }
}

/// Validate product recall compliance
pub fn validate_product_recall(recall: &ProductRecall) -> ConsumerProtectionResult<()> {
    if !recall.is_compliant() {
        let mut reasons = Vec::new();

        if !recall.publicly_announced {
            reasons.push("Chưa công bố thu hồi sản phẩm");
        }

        if matches!(recall.recall_reason, RecallReason::SafetyHazard)
            && !recall.compensation_offered
        {
            reasons.push("Chưa bồi thường cho sản phẩm nguy hiểm");
        }

        Err(ConsumerProtectionError::RecallViolation {
            reason: reasons.join("; "),
        })
    } else {
        Ok(())
    }
}

/// Get Consumer Protection Law checklist
pub fn get_consumer_protection_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Cung cấp thông tin đầy đủ",
            "Provide complete information",
            "Điều 11-12",
        ),
        ("Bảo hành sản phẩm", "Product warranty", "Điều 13-16"),
        ("Niêm yết giá rõ ràng", "Clear price display", "Điều 9"),
        (
            "Xử lý khiếu nại kịp thời",
            "Timely complaint resolution",
            "Điều 37-41",
        ),
        (
            "Thu hồi sản phẩm lỗi",
            "Recall defective products",
            "Điều 18-20",
        ),
        (
            "Bồi thường thiệt hại",
            "Compensate for damages",
            "Điều 31-36",
        ),
        (
            "Bảo vệ thông tin cá nhân",
            "Protect personal information",
            "Điều 8.8",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_rights() {
        let safety = ConsumerRight::Safety;
        assert_eq!(safety.article(), "Điều 8");
        assert!(safety.description_vi().contains("an toàn"));

        let info = ConsumerRight::Information;
        assert!(info.description_en().contains("information"));
    }

    #[test]
    fn test_warranty() {
        let warranty = ProductWarranty {
            product_name: "Laptop".to_string(),
            warranty_months: 24,
            coverage: "Full hardware warranty".to_string(),
            conditions: vec!["No water damage".to_string()],
        };

        assert!(warranty.meets_minimum_requirement(true));
        assert_eq!(warranty.expiry_months(), 24);

        let short_warranty = ProductWarranty {
            product_name: "Phone".to_string(),
            warranty_months: 6,
            coverage: "Limited".to_string(),
            conditions: vec![],
        };

        assert!(!short_warranty.meets_minimum_requirement(true));
        assert!(short_warranty.meets_minimum_requirement(false));
    }

    #[test]
    fn test_warranty_validation() {
        let valid = ProductWarranty {
            product_name: "Fridge".to_string(),
            warranty_months: 12,
            coverage: "Full".to_string(),
            conditions: vec![],
        };

        assert!(validate_warranty(&valid, true).is_ok());

        let invalid = ProductWarranty {
            product_name: "TV".to_string(),
            warranty_months: 6,
            coverage: "Limited".to_string(),
            conditions: vec![],
        };

        assert!(validate_warranty(&invalid, true).is_err());
    }

    #[test]
    fn test_product_recall() {
        let compliant_recall = ProductRecall {
            product_name: "Defective Battery".to_string(),
            recall_reason: RecallReason::SafetyHazard,
            affected_units: 10000,
            publicly_announced: true,
            compensation_offered: true,
        };

        assert!(compliant_recall.is_compliant());
        assert!(validate_product_recall(&compliant_recall).is_ok());

        let non_compliant = ProductRecall {
            product_name: "Faulty Product".to_string(),
            recall_reason: RecallReason::SafetyHazard,
            affected_units: 5000,
            publicly_announced: false,
            compensation_offered: false,
        };

        assert!(!non_compliant.is_compliant());
        assert!(validate_product_recall(&non_compliant).is_err());
    }

    #[test]
    fn test_consumer_complaint() {
        let valid_complaint = ConsumerComplaint {
            consumer_name: "Nguyen Van A".to_string(),
            business_name: "XYZ Store".to_string(),
            subject: "Defective phone".to_string(),
            complaint_details: "Phone stopped working after 1 week".to_string(),
            requested_remedy: RemedyType::Exchange,
            has_evidence: true,
        };

        assert!(valid_complaint.is_valid());
        assert_eq!(valid_complaint.resolution_deadline_days(), 15);

        let repair_complaint = ConsumerComplaint {
            consumer_name: "Tran Thi B".to_string(),
            business_name: "ABC Corp".to_string(),
            subject: "Laptop".to_string(),
            complaint_details: "Screen damaged".to_string(),
            requested_remedy: RemedyType::Repair,
            has_evidence: false,
        };

        assert_eq!(repair_complaint.resolution_deadline_days(), 30);
    }

    #[test]
    fn test_remedy_types() {
        assert_eq!(RemedyType::Refund.name_vi(), "Trả lại hàng và hoàn tiền");
        assert_eq!(RemedyType::Exchange.name_en(), "Exchange");
    }

    #[test]
    fn test_unfair_practices() {
        let false_ad = UnfairPractice::FalseAdvertising;
        assert_eq!(false_ad.article(), 4);
        assert!(false_ad.description_vi().contains("sai sự thật"));

        let forced = UnfairPractice::ForcedSales;
        assert_eq!(forced.article(), 5);
    }

    #[test]
    fn test_recall_reasons() {
        let hazard = RecallReason::SafetyHazard;
        assert!(hazard.description_vi().contains("Nguy hiểm"));

        let defect = RecallReason::QualityDefect;
        assert!(defect.description_vi().contains("chất lượng"));
    }

    #[test]
    fn test_consumer_protection_checklist() {
        let checklist = get_consumer_protection_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 7);
    }
}
