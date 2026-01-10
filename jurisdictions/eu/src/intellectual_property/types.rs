//! Core types for EU Intellectual Property law

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// EU Trademark type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarkType {
    /// Word mark (text only)
    WordMark,
    /// Figurative mark (logo/image)
    FigurativeMark,
    /// Combined mark (word + logo)
    CombinedMark,
    /// 3D mark (shape)
    ThreeDimensionalMark,
    /// Color mark
    ColorMark,
    /// Sound mark
    SoundMark,
    /// Motion mark
    MotionMark,
    /// Multimedia mark
    MultimediaMark,
    /// Position mark
    PositionMark,
    /// Pattern mark
    PatternMark,
}

/// Nice Classification classes (1-45)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NiceClass(u8);

impl NiceClass {
    /// Create a Nice class (1-45)
    pub fn new(class: u8) -> Result<Self, crate::intellectual_property::IpError> {
        if (1..=45).contains(&class) {
            Ok(Self(class))
        } else {
            Err(crate::intellectual_property::IpError::InvalidNiceClass { class })
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    /// Check if class is for goods (1-34) or services (35-45)
    pub fn is_goods(&self) -> bool {
        self.0 <= 34
    }

    pub fn is_services(&self) -> bool {
        self.0 >= 35
    }
}

/// Copyright work types under EU law
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WorkType {
    /// Literary works (novels, articles, etc.)
    Literary,
    /// Musical works
    Musical,
    /// Artistic works (paintings, sculptures)
    Artistic,
    /// Photographic works
    Photographic,
    /// Cinematographic/audiovisual works
    Audiovisual,
    /// Software (protected as literary works under Software Directive)
    Software,
    /// Database (sui generis protection under Database Directive)
    Database,
    /// Architectural works
    Architectural,
    /// Choreographic works
    Choreographic,
    /// Applied art
    AppliedArt,
}

/// Community Design type
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DesignType {
    /// Registered Community Design (RCD)
    Registered,
    /// Unregistered Community Design (UCD) - 3 years protection
    Unregistered,
}

/// Design appearance features
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DesignAppearance {
    /// Lines, contours, colors, shape, texture
    pub features: Vec<String>,
    /// Product to which design is applied
    pub product_indication: String,
}

/// Trade secret characteristics under EU Directive 2016/943
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TradeSecretCharacteristics {
    /// Information is secret (not generally known)
    pub is_secret: bool,
    /// Has commercial value because it's secret
    pub has_commercial_value: bool,
    /// Subject to reasonable steps to keep it secret
    pub reasonable_steps_taken: bool,
}

/// Copyright exception under InfoSoc Directive
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CopyrightException {
    /// Private copying (Article 5(2)(b) InfoSoc Directive)
    PrivateCopying,
    /// Quotation (Article 5(3)(d))
    Quotation,
    /// Parody (Article 5(3)(k))
    Parody,
    /// Education (Article 5(3)(a))
    EducationalUse,
    /// News reporting (Article 5(3)(c))
    NewsReporting,
    /// Text and data mining (DSM Directive Article 3-4)
    TextDataMining,
    /// Accessibility (DSM Directive Article 6)
    AccessibilityForDisabled,
}

/// Trademark status
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TrademarkStatus {
    /// Application filed
    ApplicationFiled,
    /// Under examination
    UnderExamination,
    /// Published for opposition
    PublishedForOpposition,
    /// Registered
    Registered {
        registration_date: DateTime<Utc>,
        renewal_due: DateTime<Utc>,
    },
    /// Expired
    Expired,
    /// Cancelled
    Cancelled,
    /// Refused
    Refused,
}

/// Design protection duration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DesignProtectionPeriod {
    /// Registration date
    pub registration_date: DateTime<Utc>,
    /// Current renewal period (0-4, each 5 years)
    pub renewal_periods: u8,
}

impl DesignProtectionPeriod {
    /// Calculate expiry date (max 25 years)
    pub fn expiry_date(&self) -> DateTime<Utc> {
        let years = ((self.renewal_periods + 1) * 5).min(25) as i64;
        self.registration_date + chrono::Duration::days(years * 365)
    }

    /// Check if protection is still active
    pub fn is_active(&self) -> bool {
        Utc::now() < self.expiry_date()
    }
}
