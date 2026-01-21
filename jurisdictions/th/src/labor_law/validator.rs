//! LPA Validation Functions

use super::error::{LpaError, LpaResult};
use super::types::*;
use serde::{Deserialize, Serialize};

/// Validate working hours (Section 23)
pub fn validate_working_hours(hours: &WorkingHours) -> LpaResult<()> {
    if hours.daily_hours > 8 {
        return Err(LpaError::WorkingHoursViolation {
            description: format!("ทำงานวันละ {} ชั่วโมง (สูงสุด 8 ชั่วโมง)", hours.daily_hours),
        });
    }

    if hours.weekly_hours > 48 {
        return Err(LpaError::WorkingHoursViolation {
            description: format!("ทำงานสัปดาห์ละ {} ชั่วโมง (สูงสุด 48 ชั่วโมง)", hours.weekly_hours),
        });
    }

    if hours.overtime_hours > 36 {
        return Err(LpaError::OvertimeViolation {
            description: format!(
                "ล่วงเวลาสัปดาห์ละ {} ชั่วโมง (สูงสุด 36 ชั่วโมง)",
                hours.overtime_hours
            ),
        });
    }

    Ok(())
}

/// Validate minimum wage (Section 90)
pub fn validate_minimum_wage(wage: u32, minimum_wage: u32) -> LpaResult<()> {
    if wage < minimum_wage {
        return Err(LpaError::MinimumWageViolation {
            wage,
            minimum: minimum_wage,
        });
    }
    Ok(())
}

/// Validate worker age (Sections 44-52)
pub fn validate_worker_age(age: u32, is_hazardous_work: bool) -> LpaResult<()> {
    if age < 15 {
        return Err(LpaError::ChildLaborViolation {
            description: "ห้ามจ้างเด็กอายุต่ำกว่า 15 ปี".to_string(),
        });
    }

    if age < 18 && is_hazardous_work {
        return Err(LpaError::ChildLaborViolation {
            description: "ห้ามจ้างเด็กอายุต่ำกว่า 18 ปีในงานอันตราย".to_string(),
        });
    }

    Ok(())
}

/// Validate rest period (Section 27)
pub fn validate_rest_period(continuous_work_hours: u32, rest_minutes: u32) -> LpaResult<()> {
    if continuous_work_hours >= 5 && rest_minutes < 60 {
        return Err(LpaError::RestPeriodViolation {
            description: format!(
                "ทำงานต่อเนื่อง {} ชั่วโมง พักเพียง {} นาที (ต้องพักไม่น้อยกว่า 60 นาที)",
                continuous_work_hours, rest_minutes
            ),
        });
    }
    Ok(())
}

/// Calculate overtime pay (Section 61)
///
/// - Weekday overtime: 1.5x base rate
/// - Holiday overtime: 3x base rate
pub fn calculate_overtime(hourly_wage: u32, overtime_hours: u32, is_holiday: bool) -> u32 {
    let multiplier = if is_holiday { 3 } else { 2 }; // 3x for holiday, 2x includes 1x base + 1x OT
    hourly_wage * overtime_hours * multiplier
}

/// Calculate severance pay based on tenure (Section 118)
pub fn calculate_severance(years_of_service: u32, monthly_wage: u32) -> Severance {
    Severance::calculate(years_of_service, monthly_wage)
}

/// Labor compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborCompliance {
    /// Overall compliance status
    pub compliant: bool,

    /// Whether working hours are compliant
    pub working_hours_compliant: bool,

    /// Whether minimum wage is met
    pub minimum_wage_compliant: bool,

    /// Whether leave entitlements are provided
    pub leave_compliant: bool,

    /// Whether SSF contributions are made
    pub ssf_compliant: bool,

    /// Whether workplace safety is adequate
    pub safety_compliant: bool,

    /// List of compliance issues
    pub issues: Vec<String>,

    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Comprehensive labor compliance check
pub fn validate_labor_compliance(
    working_hours_compliant: bool,
    minimum_wage_compliant: bool,
    leave_compliant: bool,
    ssf_compliant: bool,
    safety_compliant: bool,
) -> LaborCompliance {
    let mut compliance = LaborCompliance {
        compliant: true,
        working_hours_compliant,
        minimum_wage_compliant,
        leave_compliant,
        ssf_compliant,
        safety_compliant,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    if !working_hours_compliant {
        compliance.compliant = false;
        compliance.issues.push("ชั่วโมงทำงานเกินกำหนด".to_string());
        compliance
            .recommendations
            .push("ปรับชั่วโมงทำงานให้ไม่เกิน 48 ชั่วโมงต่อสัปดาห์".to_string());
    }

    if !minimum_wage_compliant {
        compliance.compliant = false;
        compliance.issues.push("ค่าจ้างต่ำกว่าอัตราขั้นต่ำ".to_string());
        compliance
            .recommendations
            .push("ปรับค่าจ้างให้ไม่ต่ำกว่าอัตราค่าจ้างขั้นต่ำตามประกาศ".to_string());
    }

    if !leave_compliant {
        compliance.compliant = false;
        compliance.issues.push("วันลาไม่ครบตามกฎหมาย".to_string());
        compliance
            .recommendations
            .push("จัดให้มีวันหยุดตามประเพณี 13 วัน และวันลาพักร้อน 6 วัน".to_string());
    }

    if !ssf_compliant {
        compliance.compliant = false;
        compliance.issues.push("ไม่ได้ส่งเงินสมทบประกันสังคม".to_string());
        compliance
            .recommendations
            .push("ส่งเงินสมทบประกันสังคมตามกฎหมาย".to_string());
    }

    if !safety_compliant {
        compliance.compliant = false;
        compliance
            .issues
            .push("มาตรการความปลอดภัยไม่เพียงพอ".to_string());
        compliance
            .recommendations
            .push("ปรับปรุงมาตรการความปลอดภัยในสถานที่ทำงาน".to_string());
    }

    compliance
}

/// Get labor rights checklist
pub fn get_labor_rights_checklist() -> Vec<(&'static str, &'static str, u32)> {
    vec![
        ("ชั่วโมงทำงานไม่เกิน 8 ชม./วัน", "8h/day max", 23),
        ("ชั่วโมงทำงานไม่เกิน 48 ชม./สัปดาห์", "48h/week max", 23),
        ("ค่าล่วงเวลา 1.5x-3x", "Overtime 1.5x-3x", 61),
        ("เวลาพักไม่น้อยกว่า 1 ชม.", "1h rest break", 27),
        ("วันหยุดประจำสัปดาห์ 1 วัน", "1 weekly rest day", 28),
        ("วันหยุดตามประเพณี 13 วัน", "13 public holidays", 29),
        ("วันลาพักร้อน 6 วัน/ปี", "6 days annual leave", 30),
        ("ลาป่วย 30 วัน (จ่ายค่าจ้าง)", "30 days sick leave", 32),
        ("ลาคลอด 98 วัน", "98 days maternity", 41),
        ("ค่าชดเชย 30-400 วัน", "Severance 30-400 days", 118),
    ]
}

/// Thailand minimum wage rates by province group (2024)
pub fn get_minimum_wage_2024(province: &str) -> u32 {
    // Simplified - actual rates vary by province
    match province.to_lowercase().as_str() {
        "bangkok" | "กรุงเทพ" | "กรุงเทพมหานคร" => 363,
        "phuket" | "ภูเก็ต" => 370,
        "chonburi" | "ชลบุรี" => 361,
        "rayong" | "ระยอง" => 361,
        "nonthaburi" | "นนทบุรี" => 363,
        "samutprakan" | "สมุทรปราการ" => 363,
        "pathumthani" | "ปทุมธานี" => 363,
        _ => 354, // Base rate for most provinces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_working_hours_ok() {
        let hours = WorkingHours::standard();
        assert!(validate_working_hours(&hours).is_ok());
    }

    #[test]
    fn test_validate_working_hours_exceeded() {
        let hours = WorkingHours {
            daily_hours: 10,
            weekly_hours: 60,
            overtime_hours: 0,
            schedule_type: ScheduleType::Standard,
        };
        assert!(validate_working_hours(&hours).is_err());
    }

    #[test]
    fn test_validate_minimum_wage() {
        assert!(validate_minimum_wage(400, 363).is_ok());
        assert!(validate_minimum_wage(300, 363).is_err());
    }

    #[test]
    fn test_validate_worker_age() {
        assert!(validate_worker_age(20, true).is_ok());
        assert!(validate_worker_age(16, false).is_ok());
        assert!(validate_worker_age(16, true).is_err()); // Hazardous
        assert!(validate_worker_age(14, false).is_err()); // Too young
    }

    #[test]
    fn test_validate_rest_period() {
        assert!(validate_rest_period(5, 60).is_ok());
        assert!(validate_rest_period(5, 30).is_err());
        assert!(validate_rest_period(4, 30).is_ok()); // Less than 5 hours
    }

    #[test]
    fn test_calculate_severance() {
        let sev = calculate_severance(5, 30_000);
        assert_eq!(sev.severance_days, 180);
        assert_eq!(sev.total_amount, 180_000);
    }

    #[test]
    fn test_labor_compliance_check() {
        let compliance = validate_labor_compliance(true, true, true, true, true);
        assert!(compliance.compliant);
        assert!(compliance.issues.is_empty());
    }

    #[test]
    fn test_labor_compliance_check_missing() {
        let compliance = validate_labor_compliance(false, true, true, true, true);
        assert!(!compliance.compliant);
        assert!(!compliance.issues.is_empty());
    }

    #[test]
    fn test_minimum_wage_lookup() {
        assert_eq!(get_minimum_wage_2024("bangkok"), 363);
        assert_eq!(get_minimum_wage_2024("phuket"), 370);
        assert_eq!(get_minimum_wage_2024("unknown"), 354);
    }

    #[test]
    fn test_labor_rights_checklist() {
        let checklist = get_labor_rights_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(th, _, _)| th.contains("ค่าล่วงเวลา")));
    }
}
