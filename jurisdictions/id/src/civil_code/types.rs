//! Types for Indonesian Civil Code (KUHPerdata)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Legal capacity of a person - Pasal 1329-1331
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalCapacity {
    /// Full capacity (dewasa dan cakap)
    Full,
    /// Minor under parental authority (belum dewasa)
    Minor { age: u32, has_guardian: bool },
    /// Under guardianship (di bawah pengampuan)
    UnderGuardianship { reason: GuardianshipReason },
    /// Married woman (historically restricted, now equal) - for historical reference
    MarriedWoman,
}

impl LegalCapacity {
    /// Check if person has capacity to contract - Pasal 1330
    pub fn can_contract(&self) -> bool {
        matches!(self, Self::Full)
    }

    /// Get minimum age for legal capacity (21 years or married)
    pub fn minimum_age() -> u32 {
        21
    }
}

/// Reason for guardianship - Pasal 433
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GuardianshipReason {
    /// Mental illness (sakit ingatan)
    MentalIllness,
    /// Prodigality (boros)
    Prodigality,
    /// Weak-mindedness (lemah akal)
    WeakMindedness,
    /// Physical disability requiring assistance
    PhysicalDisability,
}

/// Contract validity requirements - Pasal 1320
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractValidity {
    /// 1. Sepakat (Agreement/Consent)
    pub has_agreement: bool,
    /// Free from defects: fraud (penipuan), duress (paksaan), mistake (kekhilafan)
    pub agreement_free_from_defects: bool,

    /// 2. Cakap (Capacity)
    pub parties_have_capacity: bool,

    /// 3. Suatu hal tertentu (Specific object)
    pub has_specific_object: bool,
    /// Object must be determinable
    pub object_is_determinable: bool,

    /// 4. Sebab yang halal (Lawful cause)
    pub has_lawful_cause: bool,
    /// Not contrary to law, morality, or public order
    pub not_contrary_to_law: bool,
}

impl ContractValidity {
    /// Check if contract is valid under Pasal 1320
    pub fn is_valid(&self) -> bool {
        self.has_agreement
            && self.agreement_free_from_defects
            && self.parties_have_capacity
            && self.has_specific_object
            && self.object_is_determinable
            && self.has_lawful_cause
            && self.not_contrary_to_law
    }

    /// Check if contract is voidable (dapat dibatalkan) vs void (batal demi hukum)
    pub fn validity_status(&self) -> ContractValidityStatus {
        // Requirements 1 & 2 (subjective): voidable
        // Requirements 3 & 4 (objective): void ab initio
        if !self.has_specific_object || !self.object_is_determinable {
            return ContractValidityStatus::VoidAbInitio(
                "Tidak ada hal tertentu (Pasal 1320 ayat 3)".to_string(),
            );
        }

        if !self.has_lawful_cause || !self.not_contrary_to_law {
            return ContractValidityStatus::VoidAbInitio(
                "Sebab tidak halal (Pasal 1320 ayat 4)".to_string(),
            );
        }

        if !self.has_agreement || !self.agreement_free_from_defects {
            return ContractValidityStatus::Voidable(
                "Cacat kesepakatan (Pasal 1320 ayat 1)".to_string(),
            );
        }

        if !self.parties_have_capacity {
            return ContractValidityStatus::Voidable(
                "Ketidakcakapan pihak (Pasal 1320 ayat 2)".to_string(),
            );
        }

        ContractValidityStatus::Valid
    }
}

/// Contract validity status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractValidityStatus {
    /// Valid and binding
    Valid,
    /// Voidable at option of affected party (dapat dibatalkan)
    Voidable(String),
    /// Void ab initio (batal demi hukum)
    VoidAbInitio(String),
}

/// Contract formation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFormation {
    /// Offer date (penawaran)
    pub offer_date: DateTime<Utc>,
    /// Acceptance date (penerimaan)
    pub acceptance_date: Option<DateTime<Utc>>,
    /// Contract formation date
    pub formation_date: Option<NaiveDate>,
    /// Consideration (prestasi) description
    pub consideration: String,
    /// Counter-consideration (kontra-prestasi)
    pub counter_consideration: String,
    /// Formation method
    pub formation_method: FormationMethod,
}

/// Method of contract formation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FormationMethod {
    /// Written contract (tertulis)
    Written,
    /// Verbal/oral contract (lisan)
    Verbal,
    /// Electronic contract (elektronik)
    Electronic,
    /// Notarial deed (akta notaris)
    NotarialDeed,
    /// Under-hand deed (akta di bawah tangan)
    UnderHandDeed,
}

/// Contract record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// Contract identifier
    pub id: String,
    /// Party A (first party)
    pub party_a: ContractParty,
    /// Party B (second party)
    pub party_b: ContractParty,
    /// Contract type
    pub contract_type: ContractTypeCategory,
    /// Contract object (hal tertentu)
    pub object: String,
    /// Contract value (if monetary)
    pub value: Option<i64>,
    /// Formation details
    pub formation: ContractFormation,
    /// Validity requirements
    pub validity: ContractValidity,
    /// Governing law clause
    pub governing_law: String,
    /// Dispute resolution
    pub dispute_resolution: DisputeResolution,
}

/// Contract party
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractParty {
    /// Party name
    pub name: String,
    /// Legal capacity
    pub capacity: LegalCapacity,
    /// Party type
    pub party_type: PartyType,
    /// Address
    pub address: String,
    /// ID number (KTP/NIK for individuals, NPWP/NIB for companies)
    pub id_number: Option<String>,
}

/// Party type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartyType {
    /// Natural person (orang perseorangan)
    NaturalPerson,
    /// Legal entity - company (badan hukum - PT)
    LegalEntityCompany,
    /// Legal entity - foundation (yayasan)
    LegalEntityFoundation,
    /// Legal entity - cooperative (koperasi)
    LegalEntityCooperative,
    /// Legal entity - association (perkumpulan)
    LegalEntityAssociation,
    /// Government entity
    GovernmentEntity,
}

/// Contract type category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractTypeCategory {
    /// Sale and purchase (jual beli) - Pasal 1457-1540
    SaleAndPurchase,
    /// Exchange (tukar menukar) - Pasal 1541-1546
    Exchange,
    /// Lease/rent (sewa menyewa) - Pasal 1548-1600
    Lease,
    /// Employment (perburuhan) - now UU Ketenagakerjaan
    Employment,
    /// Service contract (pemborongan pekerjaan) - Pasal 1601b-1617
    ServiceContract,
    /// Partnership (persekutuan/maatschap) - Pasal 1618-1652
    Partnership,
    /// Gift/donation (hibah) - Pasal 1666-1693
    Gift,
    /// Loan (pinjam meminjam) - Pasal 1754-1769
    Loan,
    /// Deposit (penitipan barang) - Pasal 1694-1739
    Deposit,
    /// Guarantee/suretyship (penanggungan) - Pasal 1820-1850
    Guarantee,
    /// Settlement (perdamaian) - Pasal 1851-1864
    Settlement,
    /// Mandate/agency (pemberian kuasa) - Pasal 1792-1819
    Mandate,
    /// Other innominate contract
    Other(String),
}

/// Dispute resolution method
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisputeResolution {
    /// Court litigation (pengadilan negeri)
    Litigation,
    /// Arbitration (arbitrase) - UU 30/1999
    Arbitration { institution: String },
    /// Mediation (mediasi)
    Mediation,
    /// Negotiation (musyawarah)
    Negotiation,
    /// Combined (escalation clause)
    Combined(Vec<DisputeResolution>),
}

/// Contract termination - Pasal 1381
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractTermination {
    /// Performance/fulfillment (pembayaran/pelunasan)
    Performance,
    /// Novation (pembaharuan utang) - Pasal 1413-1424
    Novation,
    /// Set-off (kompensasi) - Pasal 1425-1435
    SetOff,
    /// Confusion/merger (pencampuran utang) - Pasal 1436-1437
    Confusion,
    /// Release (pembebasan utang) - Pasal 1438-1443
    Release,
    /// Destruction of object (musnahnya barang)
    DestructionOfObject,
    /// Annulment (pembatalan) - Pasal 1446-1456
    Annulment,
    /// Expiration of time (daluwarsa) - Pasal 1946-1993
    Limitation,
    /// Rescission for breach (pemutusan)
    Rescission,
    /// Mutual agreement (kesepakatan bersama)
    MutualAgreement,
}

/// Obligation type - Pasal 1233-1456
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObligationType {
    /// Contractual obligation (perikatan yang lahir dari perjanjian)
    Contractual,
    /// Statutory obligation (perikatan yang lahir dari undang-undang)
    Statutory,
    /// Tort/delict (perbuatan melawan hukum) - Pasal 1365
    Tort,
    /// Quasi-contract/unjust enrichment - Pasal 1359-1364
    QuasiContract,
}

/// Property type - Book II
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    /// Movable property (benda bergerak)
    Movable,
    /// Immovable property (benda tidak bergerak)
    Immovable,
    /// Tangible property (benda berwujud)
    Tangible,
    /// Intangible property (benda tidak berwujud)
    Intangible,
}

/// Property rights - Book II
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyRight {
    /// Ownership (hak milik) - Pasal 570
    Ownership,
    /// Right of use (hak pakai) - Pasal 818
    UseRight,
    /// Usufruct (hak memungut hasil) - Pasal 756
    Usufruct,
    /// Servitude (hak pengabdian pekarangan) - Pasal 674
    Servitude,
    /// Mortgage (hipotik) - now Hak Tanggungan
    Mortgage,
    /// Pledge (gadai) - Pasal 1150
    Pledge,
    /// Fiduciary (fidusia) - UU 42/1999
    Fiduciary,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_capacity() {
        let full = LegalCapacity::Full;
        assert!(full.can_contract());

        let minor = LegalCapacity::Minor {
            age: 17,
            has_guardian: true,
        };
        assert!(!minor.can_contract());
    }

    #[test]
    fn test_contract_validity_valid() {
        let validity = ContractValidity {
            has_agreement: true,
            agreement_free_from_defects: true,
            parties_have_capacity: true,
            has_specific_object: true,
            object_is_determinable: true,
            has_lawful_cause: true,
            not_contrary_to_law: true,
        };

        assert!(validity.is_valid());
        assert_eq!(validity.validity_status(), ContractValidityStatus::Valid);
    }

    #[test]
    fn test_contract_validity_voidable() {
        let validity = ContractValidity {
            has_agreement: false, // Defect in subjective requirement
            agreement_free_from_defects: false,
            parties_have_capacity: true,
            has_specific_object: true,
            object_is_determinable: true,
            has_lawful_cause: true,
            not_contrary_to_law: true,
        };

        assert!(!validity.is_valid());
        assert!(matches!(
            validity.validity_status(),
            ContractValidityStatus::Voidable(_)
        ));
    }

    #[test]
    fn test_contract_validity_void() {
        let validity = ContractValidity {
            has_agreement: true,
            agreement_free_from_defects: true,
            parties_have_capacity: true,
            has_specific_object: false, // Defect in objective requirement
            object_is_determinable: false,
            has_lawful_cause: true,
            not_contrary_to_law: true,
        };

        assert!(!validity.is_valid());
        assert!(matches!(
            validity.validity_status(),
            ContractValidityStatus::VoidAbInitio(_)
        ));
    }

    #[test]
    fn test_contract_termination_types() {
        let terminations = [
            ContractTermination::Performance,
            ContractTermination::Novation,
            ContractTermination::SetOff,
            ContractTermination::Limitation,
        ];

        assert_eq!(terminations.len(), 4);
    }
}
