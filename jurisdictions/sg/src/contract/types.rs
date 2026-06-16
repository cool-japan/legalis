//! Contract Law - Type Definitions
//!
//! Type-safe models of the Singapore common law of contract. Singapore inherited
//! English contract law and continues to develop it through decisions of the
//! Singapore High Court and Court of Appeal. The doctrines modelled here are:
//!
//! 1. **Formation** — offer, acceptance, consideration, and intention to create
//!    legal relations (*Gay Choon Ing v Loh Sze Ti Terence Peter* \[2009\] SGCA 3
//!    sets out the four requirements).
//! 2. **Terms** — classification into conditions, warranties and innominate
//!    terms (*Hongkong Fir Shipping v Kawasaki Kisen Kaisha* \[1962\] 2 QB 26;
//!    *RDC Concrete v Sato Kogyo* \[2007\] SGCA 1).
//! 3. **Vitiating factors** — misrepresentation, mistake, duress and undue
//!    influence.
//! 4. **Discharge** — performance, breach and frustration (*Davis Contractors v
//!    Fareham UDC* \[1956\] AC 696; Frustrated Contracts Act 1959).
//!
//! Monetary values are stored in **SGD cents** (`i64`) to avoid floating-point
//! error, mirroring the rest of `legalis-sg`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ===========================================================================
// Formation
// ===========================================================================

/// How an offer may be terminated, controlling whether it is still open for
/// acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfferStatus {
    /// The offer remains open for acceptance.
    Open,
    /// Revoked by the offeror before acceptance (effective on communication —
    /// *Byrne v Van Tienhoven* (1880) 5 CPD 344).
    Revoked,
    /// Rejected by the offeree (a rejection ends the offer).
    Rejected,
    /// Lapsed through passage of time or non-occurrence of a condition
    /// (*Ramsgate Victoria Hotel v Montefiore* (1866) LR 1 Ex 109).
    Lapsed,
    /// Met by a counter-offer, which destroys it (*Hyde v Wrench* (1840) 3 Beav 334).
    CounteredOff,
}

/// An offer: an expression of willingness to contract on stated terms, made
/// with the intention that it become binding on acceptance.
///
/// Distinguished from an invitation to treat (e.g. goods on a shop shelf —
/// *Pharmaceutical Society of Great Britain v Boots* \[1953\] 1 QB 401).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    /// Identifier for the offer.
    pub id: String,
    /// Party making the offer.
    pub offeror: String,
    /// Party (or class of persons, for a unilateral offer) to whom it is made.
    pub offeree: String,
    /// Subject matter / terms summary.
    pub terms: String,
    /// Whether the offer is a unilateral offer (accepted by performance —
    /// *Carlill v Carbolic Smoke Ball Co* \[1893\] 1 QB 256).
    pub unilateral: bool,
    /// When the offer was made.
    pub made_at: DateTime<Utc>,
    /// Current status of the offer.
    pub status: OfferStatus,
}

impl Offer {
    /// Creates a new bilateral offer that is open for acceptance.
    pub fn new(
        id: impl Into<String>,
        offeror: impl Into<String>,
        offeree: impl Into<String>,
        terms: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            offeror: offeror.into(),
            offeree: offeree.into(),
            terms: terms.into(),
            unilateral: false,
            made_at: Utc::now(),
            status: OfferStatus::Open,
        }
    }

    /// Marks this offer as a unilateral offer (accepted by performance of the
    /// stipulated act).
    pub fn unilateral(mut self) -> Self {
        self.unilateral = true;
        self
    }

    /// Returns whether the offer is currently open for acceptance.
    pub fn is_open(&self) -> bool {
        matches!(self.status, OfferStatus::Open)
    }
}

/// The mode by which acceptance is communicated, controlling when it takes
/// effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcceptanceMode {
    /// Instantaneous communication (in person, telephone, telex): effective on
    /// receipt (*Entores v Miles Far East Corp* \[1955\] 2 QB 327).
    Instantaneous,
    /// Postal acceptance: effective on posting (*Adams v Lindsell* (1818) 1 B &
    /// Ald 681), unless displaced by the offer's terms.
    Postal,
    /// Electronic (email / online): generally treated as effective on receipt;
    /// see the Electronic Transactions Act 2010 ss. 11–13 for time/place of
    /// dispatch and receipt.
    Electronic,
    /// Acceptance by conduct / performance (typical of unilateral offers).
    Conduct,
}

impl AcceptanceMode {
    /// Returns whether the postal acceptance rule applies to this mode.
    pub fn uses_postal_rule(&self) -> bool {
        matches!(self, AcceptanceMode::Postal)
    }
}

/// An acceptance of an offer. To be effective it must be an unqualified assent
/// to the exact terms of the offer (the mirror-image rule) and be communicated
/// to the offeror.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acceptance {
    /// Identifier of the offer being accepted.
    pub offer_id: String,
    /// Party accepting.
    pub acceptor: String,
    /// Whether the assent is unqualified (a variation makes it a counter-offer).
    pub unqualified: bool,
    /// Mode of communication.
    pub mode: AcceptanceMode,
    /// When the acceptance was sent / performed.
    pub sent_at: DateTime<Utc>,
    /// When the acceptance was received by the offeror (if known).
    pub received_at: Option<DateTime<Utc>>,
}

impl Acceptance {
    /// Creates a new, unqualified acceptance.
    pub fn new(
        offer_id: impl Into<String>,
        acceptor: impl Into<String>,
        mode: AcceptanceMode,
    ) -> Self {
        Self {
            offer_id: offer_id.into(),
            acceptor: acceptor.into(),
            unqualified: true,
            mode,
            sent_at: Utc::now(),
            received_at: None,
        }
    }

    /// Marks this acceptance as qualified (introducing new or varied terms),
    /// which in law operates as a counter-offer.
    pub fn qualified(mut self) -> Self {
        self.unqualified = false;
        self
    }

    /// Records the time the acceptance was received by the offeror.
    pub fn received(mut self, at: DateTime<Utc>) -> Self {
        self.received_at = Some(at);
        self
    }

    /// Returns the time at which the acceptance takes legal effect, given the
    /// mode of communication.
    ///
    /// Under the postal rule acceptance is complete on posting; otherwise it is
    /// complete on receipt (falling back to the send time when receipt is not
    /// recorded).
    pub fn effective_at(&self) -> DateTime<Utc> {
        if self.mode.uses_postal_rule() {
            self.sent_at
        } else {
            self.received_at.unwrap_or(self.sent_at)
        }
    }
}

/// The category of consideration supplied by a party.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsiderationKind {
    /// A present exchange (an act or forbearance given now for the promise).
    Executed,
    /// A promise of future performance.
    Executory,
    /// Past consideration — generally not good consideration (*Roscorla v
    /// Thomas* (1842) 3 QB 234), save for the *Pao On* exception.
    Past,
    /// Performance of (or a promise to perform) a pre-existing duty.
    ExistingDuty,
}

/// Consideration moving from the promisee.
///
/// Consideration must be sufficient (have some value in the eye of the law) but
/// need not be adequate (*Chappell v Nestlé* \[1960\] AC 87). It must move from
/// the promisee but need not move to the promisor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consideration {
    /// Party providing the consideration.
    pub provider: String,
    /// Description of what is given.
    pub description: String,
    /// Kind of consideration.
    pub kind: ConsiderationKind,
    /// Whether it moves from the promisee (a requirement).
    pub moves_from_promisee: bool,
    /// For an existing-duty case: whether a practical benefit was conferred on
    /// the promisor (*Williams v Roffey Bros* \[1991\] 1 QB 1).
    pub confers_practical_benefit: bool,
}

impl Consideration {
    /// Creates executory consideration (a promise) moving from the promisee.
    pub fn promise(provider: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            description: description.into(),
            kind: ConsiderationKind::Executory,
            moves_from_promisee: true,
            confers_practical_benefit: false,
        }
    }

    /// Creates executed consideration (an act done) moving from the promisee.
    pub fn act(provider: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            kind: ConsiderationKind::Executed,
            ..Self::promise(provider, description)
        }
    }

    /// Sets the kind of consideration.
    pub fn with_kind(mut self, kind: ConsiderationKind) -> Self {
        self.kind = kind;
        self
    }

    /// Records that an existing-duty promise nonetheless conferred a practical
    /// benefit on the promisor.
    pub fn with_practical_benefit(mut self) -> Self {
        self.confers_practical_benefit = true;
        self
    }

    /// Returns whether the consideration is good in law.
    ///
    /// Past consideration is bad; existing-duty consideration is bad unless a
    /// practical benefit was conferred; consideration must move from the
    /// promisee.
    pub fn is_good(&self) -> bool {
        if !self.moves_from_promisee {
            return false;
        }
        match self.kind {
            ConsiderationKind::Past => false,
            ConsiderationKind::ExistingDuty => self.confers_practical_benefit,
            ConsiderationKind::Executed | ConsiderationKind::Executory => true,
        }
    }
}

/// The context of an agreement, controlling the presumption as to intention to
/// create legal relations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgreementContext {
    /// Commercial / business dealing: presumed intended to be legally binding.
    Commercial,
    /// Social or domestic arrangement: presumed not intended to bind
    /// (*Balfour v Balfour* \[1919\] 2 KB 571; cf *Merritt v Merritt* \[1970\]
    /// 1 WLR 1211 where the presumption was rebutted).
    SocialDomestic,
}

impl AgreementContext {
    /// Returns the default (rebuttable) presumption that the parties intended
    /// legal relations.
    pub fn presumes_intention(&self) -> bool {
        matches!(self, AgreementContext::Commercial)
    }
}

// ===========================================================================
// Terms
// ===========================================================================

/// The classification of a contractual term, which determines the remedy for
/// its breach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermClassification {
    /// A condition: a term so important that any breach entitles the innocent
    /// party to terminate and claim damages (*Poussard v Spiers* (1876) 1 QBD
    /// 410).
    Condition,
    /// A warranty: a minor term; breach sounds in damages only (*Bettini v Gye*
    /// (1876) 1 QBD 183).
    Warranty,
    /// An innominate (intermediate) term: the remedy depends on the gravity of
    /// the breach and whether it deprives the innocent party of substantially
    /// the whole benefit of the contract (*Hongkong Fir* \[1962\] 2 QB 26).
    Innominate,
}

impl TermClassification {
    /// Returns the leading authority for this classification.
    pub fn authority(&self) -> &'static str {
        match self {
            TermClassification::Condition => "Poussard v Spiers (1876) 1 QBD 410",
            TermClassification::Warranty => "Bettini v Gye (1876) 1 QBD 183",
            TermClassification::Innominate => {
                "Hongkong Fir Shipping v Kawasaki Kisen Kaisha [1962] 2 QB 26"
            }
        }
    }
}

/// How a term came to form part of the contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermSource {
    /// Expressly agreed by the parties.
    Express,
    /// Implied in fact (business efficacy / officious bystander — *The Moorcock*
    /// (1889) 14 PD 64; and in Singapore the three-step test in *Sembcorp
    /// Marine v PPL Holdings* \[2013\] SGCA 60).
    ImpliedInFact,
    /// Implied in law (by statute or as a legal incident of a class of
    /// contract).
    ImpliedInLaw,
}

/// A contractual term.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractTerm {
    /// Identifier of the term.
    pub id: String,
    /// Text / substance of the term.
    pub text: String,
    /// Classification (condition / warranty / innominate).
    pub classification: TermClassification,
    /// How the term entered the contract.
    pub source: TermSource,
}

impl ContractTerm {
    /// Creates an express term with the given classification.
    pub fn new(
        id: impl Into<String>,
        text: impl Into<String>,
        classification: TermClassification,
    ) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            classification,
            source: TermSource::Express,
        }
    }

    /// Sets the source of the term.
    pub fn with_source(mut self, source: TermSource) -> Self {
        self.source = source;
        self
    }
}

// ===========================================================================
// Vitiating factors
// ===========================================================================

/// The category of an actionable misrepresentation, determining the available
/// remedy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MisrepresentationCategory {
    /// Fraudulent: a false statement made knowingly, without belief in its
    /// truth, or recklessly (*Derry v Peek* (1889) 14 App Cas 337). Remedy:
    /// rescission + damages in the tort of deceit.
    Fraudulent,
    /// Negligent: under s. 2(1) of the Misrepresentation Act 1967 the
    /// representor is liable unless it proves reasonable grounds for belief.
    Negligent,
    /// Innocent: made with reasonable grounds for belief. Remedy: rescission,
    /// or damages in lieu under s. 2(2).
    Innocent,
}

impl MisrepresentationCategory {
    /// Returns the controlling authority / statutory provision.
    pub fn authority(&self) -> &'static str {
        match self {
            MisrepresentationCategory::Fraudulent => "Derry v Peek (1889) 14 App Cas 337",
            MisrepresentationCategory::Negligent => "Misrepresentation Act 1967 s. 2(1)",
            MisrepresentationCategory::Innocent => "Misrepresentation Act 1967 s. 2(2)",
        }
    }

    /// Returns whether damages are recoverable in addition to (or in lieu of)
    /// rescission.
    pub fn damages_available(&self) -> bool {
        // Fraudulent: deceit damages; negligent: s.2(1) damages; innocent:
        // damages in lieu under s.2(2) at the court's discretion.
        true
    }
}

/// A statement alleged to be a misrepresentation: a false statement of fact (or
/// of law) that induced the representee to enter the contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Misrepresentation {
    /// The statement made.
    pub statement: String,
    /// Whether it was a statement of existing fact or law (mere puff or future
    /// intention is generally not actionable).
    pub statement_of_fact: bool,
    /// Whether the statement was false.
    pub false_statement: bool,
    /// Whether it induced the representee to contract.
    pub induced_contract: bool,
    /// Category (fraudulent / negligent / innocent).
    pub category: MisrepresentationCategory,
}

impl Misrepresentation {
    /// Creates a misrepresentation record.
    pub fn new(statement: impl Into<String>, category: MisrepresentationCategory) -> Self {
        Self {
            statement: statement.into(),
            statement_of_fact: true,
            false_statement: true,
            induced_contract: true,
            category,
        }
    }

    /// Returns whether the misrepresentation is actionable: a false statement of
    /// fact that induced the contract.
    pub fn is_actionable(&self) -> bool {
        self.statement_of_fact && self.false_statement && self.induced_contract
    }
}

/// The kind of operative mistake.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MistakeKind {
    /// Common mistake: both parties share the same mistaken assumption. Void at
    /// common law only if it renders performance essentially different
    /// (*Great Peace Shipping v Tsavliris* \[2002\] EWCA Civ 1407, adopted in
    /// *Olivine Capital v Chia Chin Yan* \[2014\] SGCA 19).
    Common,
    /// Mutual mistake: the parties are at cross-purposes (*Raffles v
    /// Wichelhaus* (1864) 2 H & C 906); may negative agreement.
    Mutual,
    /// Unilateral mistake: one party is mistaken and the other knows (or, in
    /// equity, ought to know) of the mistake (*Chwee Kin Keong v
    /// Digilandmall.com* \[2005\] SGCA 2).
    Unilateral,
}

impl MistakeKind {
    /// Returns the leading authority.
    pub fn authority(&self) -> &'static str {
        match self {
            MistakeKind::Common => "Great Peace Shipping v Tsavliris [2002] EWCA Civ 1407",
            MistakeKind::Mutual => "Raffles v Wichelhaus (1864) 2 H & C 906",
            MistakeKind::Unilateral => "Chwee Kin Keong v Digilandmall.com [2005] SGCA 2",
        }
    }
}

/// An operative mistake affecting the contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperativeMistake {
    /// Kind of mistake.
    pub kind: MistakeKind,
    /// Description of the mistaken matter.
    pub detail: String,
    /// Whether the mistake is fundamental (goes to the root of the contract /
    /// the subject matter is essentially different).
    pub fundamental: bool,
    /// For unilateral mistake: whether the non-mistaken party had actual
    /// knowledge of the mistake (governing whether the contract is void at
    /// common law or only voidable in equity).
    pub other_party_knew: bool,
}

impl OperativeMistake {
    /// Creates a mistake record.
    pub fn new(kind: MistakeKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
            fundamental: true,
            other_party_knew: false,
        }
    }

    /// Records that the non-mistaken party knew of a unilateral mistake.
    pub fn with_actual_knowledge(mut self) -> Self {
        self.other_party_knew = true;
        self
    }

    /// Sets whether the mistake is fundamental.
    pub fn fundamental(mut self, value: bool) -> Self {
        self.fundamental = value;
        self
    }
}

/// The kind of duress alleged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuressKind {
    /// Duress to the person (threats of violence).
    ToThePerson,
    /// Duress to goods (unlawful detention of property).
    ToGoods,
    /// Economic duress: illegitimate commercial pressure that is a significant
    /// cause of entry into the contract, the victim having no realistic
    /// practical alternative (*The Universe Sentinel* \[1983\] 1 AC 366;
    /// *E C Investment Holding v Ridout Residence* \[2011\] SGHC 231).
    Economic,
}

/// A claim that the contract was procured by duress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuressClaim {
    /// Kind of duress.
    pub kind: DuressKind,
    /// Description of the pressure applied.
    pub detail: String,
    /// Whether the pressure was illegitimate.
    pub illegitimate_pressure: bool,
    /// Whether the pressure was a significant cause of entry into the contract.
    pub significant_cause: bool,
    /// Whether the victim had a realistic practical alternative (its presence
    /// negatives economic duress).
    pub practical_alternative: bool,
}

impl DuressClaim {
    /// Creates a duress claim.
    pub fn new(kind: DuressKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
            illegitimate_pressure: true,
            significant_cause: true,
            practical_alternative: false,
        }
    }

    /// Returns whether the duress is made out.
    ///
    /// Requires illegitimate pressure that was a significant cause; for economic
    /// duress the absence of a realistic practical alternative is also required.
    pub fn is_established(&self) -> bool {
        if !self.illegitimate_pressure || !self.significant_cause {
            return false;
        }
        match self.kind {
            DuressKind::Economic => !self.practical_alternative,
            DuressKind::ToThePerson | DuressKind::ToGoods => true,
        }
    }
}

/// The class of undue influence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UndueInfluenceClass {
    /// Class 1 — actual undue influence: proved as a fact.
    Actual,
    /// Class 2A — presumed from a recognised relationship of trust and
    /// confidence (e.g. solicitor/client, doctor/patient).
    PresumedRecognised,
    /// Class 2B — presumed where a relationship of trust and confidence is
    /// proved on the facts (*RBS v Etridge (No 2)* \[2001\] UKHL 44; *BOM v BOK*
    /// \[2018\] SGCA 83).
    PresumedProved,
}

/// A claim of undue influence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndueInfluenceClaim {
    /// Class of undue influence.
    pub class: UndueInfluenceClass,
    /// Description of the relationship / circumstances.
    pub detail: String,
    /// Whether the transaction calls for explanation (is not readily
    /// explicable by the relationship — required to raise the Class 2
    /// presumption).
    pub transaction_calls_for_explanation: bool,
    /// Whether the presumption (if raised) has been rebutted, e.g. by proof of
    /// independent legal advice.
    pub rebutted: bool,
}

impl UndueInfluenceClaim {
    /// Creates an undue-influence claim.
    pub fn new(class: UndueInfluenceClass, detail: impl Into<String>) -> Self {
        Self {
            class,
            detail: detail.into(),
            transaction_calls_for_explanation: true,
            rebutted: false,
        }
    }

    /// Records that the presumption has been rebutted (e.g. independent advice).
    pub fn rebutted(mut self) -> Self {
        self.rebutted = true;
        self
    }

    /// Returns whether undue influence is established.
    ///
    /// Actual undue influence does not require the transaction to call for
    /// explanation; the presumed classes require it and that the presumption is
    /// not rebutted.
    pub fn is_established(&self) -> bool {
        match self.class {
            UndueInfluenceClass::Actual => !self.rebutted,
            UndueInfluenceClass::PresumedRecognised | UndueInfluenceClass::PresumedProved => {
                self.transaction_calls_for_explanation && !self.rebutted
            }
        }
    }
}

// ===========================================================================
// Discharge
// ===========================================================================

/// The manner in which a contract was, or is alleged to have been, discharged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DischargeMode {
    /// By complete and precise performance (*Cutter v Powell* (1795) 6 TR 320;
    /// cf the doctrine of substantial performance — *Hoenig v Isaacs* \[1952\]
    /// 2 All ER 176).
    Performance,
    /// By agreement (accord and satisfaction, or a release).
    Agreement,
    /// By breach: a repudiatory breach accepted by the innocent party.
    Breach,
    /// By frustration: a supervening event without fault makes performance
    /// impossible or radically different.
    Frustration,
}

/// A supervening event said to frustrate the contract.
///
/// Frustration occurs where, without fault of either party, a contractual
/// obligation has become incapable of being performed because the circumstances
/// in which performance is called for would render it a thing radically
/// different from that which was undertaken (*Davis Contractors v Fareham UDC*
/// \[1956\] AC 696). Consequences are governed by the Frustrated Contracts Act
/// 1959 (money paid is recoverable; expenses may be retained; valuable benefits
/// may be paid for).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrustratingEvent {
    /// Description of the supervening event.
    pub description: String,
    /// Whether performance has become impossible or radically different.
    pub radically_different: bool,
    /// Whether the event was self-induced (which precludes frustration —
    /// *Maritime National Fish v Ocean Trawlers* \[1935\] AC 524).
    pub self_induced: bool,
    /// Whether the risk of the event was allocated by an express term (e.g. a
    /// force majeure clause), which ousts frustration.
    pub risk_allocated_by_term: bool,
    /// Whether the event was foreseeable / foreseen (which generally precludes
    /// frustration).
    pub foreseeable: bool,
}

impl FrustratingEvent {
    /// Creates a frustrating-event record.
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            radically_different: true,
            self_induced: false,
            risk_allocated_by_term: false,
            foreseeable: false,
        }
    }

    /// Records that the event was self-induced.
    pub fn self_induced(mut self) -> Self {
        self.self_induced = true;
        self
    }

    /// Records that an express term allocated the risk of the event.
    pub fn risk_allocated(mut self) -> Self {
        self.risk_allocated_by_term = true;
        self
    }

    /// Records that the event was foreseeable.
    pub fn foreseeable(mut self) -> Self {
        self.foreseeable = true;
        self
    }

    /// Returns whether frustration is established on these facts.
    pub fn frustrates(&self) -> bool {
        self.radically_different
            && !self.self_induced
            && !self.risk_allocated_by_term
            && !self.foreseeable
    }
}

// ===========================================================================
// Aggregate contract
// ===========================================================================

/// An aggregate model of a Singapore-law contract assembled from its formation
/// elements and terms. Used by the validators to test formation, classify
/// breaches and assess remedies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    /// Identifier of the contract.
    pub id: String,
    /// The offer.
    pub offer: Offer,
    /// The acceptance, if one has been made.
    pub acceptance: Option<Acceptance>,
    /// Consideration moving from each party (a bilateral contract has at least
    /// two items, one from each side).
    pub considerations: Vec<Consideration>,
    /// The context controlling the presumption of intention.
    pub context: AgreementContext,
    /// Whether the presumption of intention has been rebutted on the facts.
    pub intention_rebutted: bool,
    /// The contractual terms.
    pub terms: Vec<ContractTerm>,
}

impl Contract {
    /// Begins assembling a contract around an offer.
    pub fn new(id: impl Into<String>, offer: Offer, context: AgreementContext) -> Self {
        Self {
            id: id.into(),
            offer,
            acceptance: None,
            considerations: Vec::new(),
            context,
            intention_rebutted: false,
            terms: Vec::new(),
        }
    }

    /// Records the acceptance of the offer.
    pub fn with_acceptance(mut self, acceptance: Acceptance) -> Self {
        self.acceptance = Some(acceptance);
        self
    }

    /// Adds an item of consideration.
    pub fn add_consideration(&mut self, consideration: Consideration) {
        self.considerations.push(consideration);
    }

    /// Adds a contractual term.
    pub fn add_term(&mut self, term: ContractTerm) {
        self.terms.push(term);
    }

    /// Records that the (default) presumption of intention is rebutted, e.g. an
    /// "honour clause" in a commercial deal, or evidence of intention to bind
    /// in a domestic one.
    pub fn with_intention_rebutted(mut self) -> Self {
        self.intention_rebutted = true;
        self
    }

    /// Returns the net presumption that the parties intended legal relations,
    /// after applying any rebuttal.
    pub fn intends_legal_relations(&self) -> bool {
        self.context.presumes_intention() ^ self.intention_rebutted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offer_starts_open_and_can_be_unilateral() {
        let offer = Offer::new("o1", "A", "world", "reward for finding dog").unilateral();
        assert!(offer.is_open());
        assert!(offer.unilateral);
    }

    #[test]
    fn postal_acceptance_is_effective_on_posting() {
        let acc = Acceptance::new("o1", "B", AcceptanceMode::Postal);
        assert!(acc.mode.uses_postal_rule());
        // Effective time equals the send (posting) time even without receipt.
        assert_eq!(acc.effective_at(), acc.sent_at);
    }

    #[test]
    fn instantaneous_acceptance_is_effective_on_receipt() {
        let later = Utc::now() + chrono::Duration::seconds(30);
        let acc = Acceptance::new("o1", "B", AcceptanceMode::Instantaneous).received(later);
        assert_eq!(acc.effective_at(), later);
    }

    #[test]
    fn past_consideration_is_not_good() {
        let c = Consideration::act("B", "work already done").with_kind(ConsiderationKind::Past);
        assert!(!c.is_good());
    }

    #[test]
    fn existing_duty_is_good_only_with_practical_benefit() {
        let bare = Consideration::promise("B", "finish the carpentry already owed")
            .with_kind(ConsiderationKind::ExistingDuty);
        assert!(!bare.is_good());

        let with_benefit = bare.with_practical_benefit();
        assert!(with_benefit.is_good());
    }

    #[test]
    fn commercial_context_presumes_intention_and_can_be_rebutted() {
        let offer = Offer::new("o1", "A", "B", "supply widgets");
        let contract = Contract::new("k1", offer, AgreementContext::Commercial);
        assert!(contract.intends_legal_relations());

        let offer2 = Offer::new("o2", "A", "B", "supply widgets");
        let honour =
            Contract::new("k2", offer2, AgreementContext::Commercial).with_intention_rebutted();
        assert!(!honour.intends_legal_relations());
    }

    #[test]
    fn domestic_context_does_not_presume_intention_but_can_be_rebutted() {
        let offer = Offer::new("o1", "H", "W", "maintenance after separation");
        let separated =
            Contract::new("k1", offer, AgreementContext::SocialDomestic).with_intention_rebutted();
        // Merritt v Merritt: domestic but intended to bind.
        assert!(separated.intends_legal_relations());
    }

    #[test]
    fn economic_duress_needs_no_practical_alternative() {
        let mut claim = DuressClaim::new(DuressKind::Economic, "threat to breach supply");
        assert!(claim.is_established());
        claim.practical_alternative = true;
        assert!(!claim.is_established());
    }

    #[test]
    fn presumed_undue_influence_requires_explanation_and_no_rebuttal() {
        let established = UndueInfluenceClaim::new(
            UndueInfluenceClass::PresumedProved,
            "elderly parent guarantee",
        );
        assert!(established.is_established());
        assert!(!established.rebutted().is_established());
    }

    #[test]
    fn frustration_excluded_by_self_inducement_or_allocation() {
        let event = FrustratingEvent::new("performance venue destroyed");
        assert!(event.frustrates());
        assert!(!event.clone().self_induced().frustrates());
        assert!(!event.clone().risk_allocated().frustrates());
        assert!(!event.foreseeable().frustrates());
    }

    #[test]
    fn unilateral_mistake_records_actual_knowledge() {
        let m = OperativeMistake::new(MistakeKind::Unilateral, "obvious pricing error")
            .with_actual_knowledge();
        assert!(m.other_party_knew);
        assert_eq!(
            m.kind.authority(),
            "Chwee Kin Keong v Digilandmall.com [2005] SGCA 2"
        );
    }

    #[test]
    fn serde_roundtrip_contract() {
        let offer = Offer::new("o1", "A", "B", "sale of car");
        let mut k = Contract::new("k1", offer, AgreementContext::Commercial)
            .with_acceptance(Acceptance::new("o1", "B", AcceptanceMode::Electronic));
        k.add_consideration(Consideration::promise("A", "deliver car"));
        k.add_consideration(Consideration::promise("B", "pay SGD 30,000"));
        k.add_term(ContractTerm::new(
            "t1",
            "car to be roadworthy",
            TermClassification::Condition,
        ));
        let json = serde_json::to_string(&k).expect("serialize");
        let back: Contract = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(k, back);
    }
}
