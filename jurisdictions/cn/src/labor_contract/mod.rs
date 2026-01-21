//! Labor Contract Law Module (劳动合同法)
//!
//! # 中华人民共和国劳动合同法 / Labor Contract Law of the PRC
//!
//! Implements the Labor Contract Law effective January 1, 2008.
//!
//! ## Key Concepts
//!
//! - **劳动合同 (Labor Contract)**: Employment agreement between employer and employee
//! - **试用期 (Probation Period)**: Initial evaluation period with restrictions
//! - **经济补偿 (Severance)**: Payment upon certain terminations
//! - **五险一金 (Social Insurance + Housing Fund)**: Mandatory benefits
//! - **劳务派遣 (Labor Dispatch)**: Temporary staffing arrangements
//!
//! ## Contract Types
//!
//! 1. **固定期限合同 (Fixed-term)**: Specific end date
//! 2. **无固定期限合同 (Open-ended)**: No end date
//! 3. **以完成工作任务为期限 (Task-based)**: Until task completion
//!
//! ## Probation Period Limits (Article 19)
//!
//! | Contract Duration | Max Probation |
//! |-------------------|---------------|
//! | < 3 months | Not allowed |
//! | 3 months - 1 year | 1 month |
//! | 1 - 3 years | 2 months |
//! | 3+ years or open-ended | 6 months |
//!
//! ## Severance Rules (Article 47)
//!
//! - 1 month salary per year of service
//! - 0.5 month for less than 6 months service
//! - Capped at 12 months if salary > 3x local average
//! - Salary capped at 3x local average wage
//!
//! ## Termination Categories
//!
//! ### Employee-Initiated (Article 37-38)
//! - 30 days written notice (3 days during probation)
//! - Immediate if employer violates law
//!
//! ### Employer-Initiated with Notice (Article 40)
//! - Medical inability after treatment period
//! - Incompetence after training/adjustment
//! - Major change in objective circumstances
//!
//! ### Employer-Initiated without Notice (Article 39)
//! - Failed probation requirements
//! - Serious rule violations
//! - Criminal liability
//!
//! ## Protected Employees (Article 42)
//!
//! Cannot be terminated under Article 40-41:
//! - Occupational disease exposure workers
//! - Pregnant/nursing employees
//! - Workers on medical leave
//! - Near-retirement employees (15+ years, <5 years to retirement)
//!
//! ## Social Insurance (五险)
//!
//! Mandatory employer contributions:
//! - 养老保险 (Pension): ~16-20%
//! - 医疗保险 (Medical): ~8-10%
//! - 失业保险 (Unemployment): ~0.5-1%
//! - 工伤保险 (Work Injury): ~0.2-1.9%
//! - 生育保险 (Maternity): ~0.8-1%
//!
//! ## Housing Fund (住房公积金)
//!
//! Both employer and employee contribute 5-12% of salary.
//!
//! ## Labor Dispatch (Article 66)
//!
//! - Limited to temporary, auxiliary, or substitute positions
//! - Max 10% of host company workforce
//! - Equal pay for equal work with regular employees
//!
//! ## Overtime Pay (Article 44)
//!
//! | Type | Rate |
//! |------|------|
//! | Extended hours (weekday) | 150% |
//! | Rest day | 200% |
//! | Statutory holiday | 300% |
//!
//! ## Annual Leave (职工带薪年休假条例)
//!
//! | Cumulative Work Years | Days |
//! |----------------------|------|
//! | 1-10 years | 5 days |
//! | 10-20 years | 10 days |
//! | 20+ years | 15 days |

#![allow(missing_docs)]

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
