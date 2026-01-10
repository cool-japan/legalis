//! Constitutional titles structure (Structure des titres constitutionnels)
//!
//! Complete structure of the French Constitution of 1958 with all 16 titles.

use super::types::ConstitutionTitle;

/// Get all 16 titles of the Constitution de 1958
#[must_use]
pub fn all_titles() -> Vec<ConstitutionTitle> {
    vec![
        title_i_sovereignty(),
        title_ii_president(),
        title_iii_government(),
        title_iv_parliament(),
        title_v_relations(),
        title_vi_treaties(),
        title_vii_constitutional_council(),
        title_viii_judicial_authority(),
        title_ix_high_court(),
        title_x_criminal_responsibility(),
        title_xi_economic_social_council(),
        title_xii_territorial_collectivities(),
        title_xiii_transitional_provisions(),
        title_xiv_francophonie(),
        title_xv_european_union(),
        title_xvi_constitutional_revision(),
    ]
}

/// Title I - On Sovereignty (De la souveraineté)
///
/// Articles 1-4: Fundamental principles of the Republic
#[must_use]
pub fn title_i_sovereignty() -> ConstitutionTitle {
    ConstitutionTitle::new(1, "De la souveraineté", "On Sovereignty", (1, 4))
        .with_description_fr(
            "Principes fondamentaux : République indivisible, laïque, démocratique et sociale. \
        Égalité devant la loi, souveraineté nationale, suffrage universel.",
        )
        .with_description_en(
            "Fundamental principles: Indivisible, secular, democratic and social Republic. \
        Equality before law, national sovereignty, universal suffrage.",
        )
}

/// Title II - The President of the Republic (Le Président de la République)
///
/// Articles 5-19: Powers, election, and responsibilities of the President
#[must_use]
pub fn title_ii_president() -> ConstitutionTitle {
    ConstitutionTitle::new(
        2,
        "Le Président de la République",
        "The President of the Republic",
        (5, 19),
    )
    .with_description_fr(
        "Garant des institutions, élu au suffrage universel direct pour 5 ans (quinquennat). \
        Nomme le Premier ministre, peut dissoudre l'Assemblée, dispose de pouvoirs exceptionnels (Article 16)."
    )
    .with_description_en(
        "Guarantor of institutions, elected by direct universal suffrage for 5 years. \
        Appoints Prime Minister, can dissolve Assembly, has exceptional powers (Article 16)."
    )
}

/// Title III - The Government (Le Gouvernement)
///
/// Articles 20-23: Government composition and powers
#[must_use]
pub fn title_iii_government() -> ConstitutionTitle {
    ConstitutionTitle::new(3, "Le Gouvernement", "The Government", (20, 23))
        .with_description_fr(
            "Le Gouvernement détermine et conduit la politique de la Nation. \
        Responsable devant l'Assemblée nationale (motion de censure possible).",
        )
        .with_description_en(
            "The Government determines and conducts the policy of the Nation. \
        Responsible to the National Assembly (motion of censure possible).",
        )
}

/// Title IV - The Parliament (Le Parlement)
///
/// Articles 24-33: Parliament composition and functioning
#[must_use]
pub fn title_iv_parliament() -> ConstitutionTitle {
    ConstitutionTitle::new(
        4,
        "Le Parlement",
        "The Parliament",
        (24, 33),
    )
    .with_description_fr(
        "Parlement bicaméral : Assemblée nationale (577 députés, 5 ans) et Sénat (348 sénateurs, 6 ans). \
        Vote la loi, contrôle le Gouvernement."
    )
    .with_description_en(
        "Bicameral Parliament: National Assembly (577 deputies, 5 years) and Senate (348 senators, 6 years). \
        Votes laws, controls Government."
    )
}

/// Title V - Relations between Parliament and Government
///
/// Articles 34-51: Legislative process and government-parliament relations
#[must_use]
pub fn title_v_relations() -> ConstitutionTitle {
    ConstitutionTitle::new(
        5,
        "Des rapports entre le Parlement et le Gouvernement",
        "On Relations between Parliament and Government",
        (34, 51),
    )
    .with_description_fr(
        "Domaine de la loi (Article 34), procédure législative, 49.3 (engagement de responsabilité), \
        ordonnances (Article 38)."
    )
    .with_description_en(
        "Domain of law (Article 34), legislative procedure, 49.3 (confidence vote), \
        ordinances (Article 38)."
    )
}

/// Title VI - On Treaties and International Agreements
///
/// Articles 52-55: Treaty ratification and international law
#[must_use]
pub fn title_vi_treaties() -> ConstitutionTitle {
    ConstitutionTitle::new(
        6,
        "Des traités et accords internationaux",
        "On Treaties and International Agreements",
        (52, 55),
    )
    .with_description_fr(
        "Président négocie et ratifie les traités. Primauté du droit international (Article 55).",
    )
    .with_description_en(
        "President negotiates and ratifies treaties. Primacy of international law (Article 55).",
    )
}

/// Title VII - The Constitutional Council (Le Conseil constitutionnel)
///
/// Articles 56-63: Constitutional review
#[must_use]
pub fn title_vii_constitutional_council() -> ConstitutionTitle {
    ConstitutionTitle::new(
        7,
        "Le Conseil constitutionnel",
        "The Constitutional Council",
        (56, 63),
    )
    .with_description_fr(
        "9 membres (3 nommés par Président, 3 par Président Assemblée, 3 par Président Sénat). \
        Contrôle constitutionnalité des lois, QPC (Question Prioritaire de Constitutionnalité).",
    )
    .with_description_en(
        "9 members (3 appointed by President, 3 by Assembly President, 3 by Senate President). \
        Reviews constitutionality of laws, QPC (Priority Question of Constitutionality).",
    )
}

/// Title VIII - On the Judicial Authority (De l'autorité judiciaire)
///
/// Articles 64-66-1: Independence of judiciary
#[must_use]
pub fn title_viii_judicial_authority() -> ConstitutionTitle {
    ConstitutionTitle::new(
        8,
        "De l'autorité judiciaire",
        "On the Judicial Authority",
        (64, 66),
    )
    .with_description_fr(
        "Président garant de l'indépendance de l'autorité judiciaire. \
        Conseil supérieur de la magistrature.",
    )
    .with_description_en(
        "President guarantor of judicial independence. \
        High Council of the Judiciary.",
    )
}

/// Title IX - The High Court (La Haute Cour)
///
/// Article 67-68: Presidential impeachment
#[must_use]
pub fn title_ix_high_court() -> ConstitutionTitle {
    ConstitutionTitle::new(
        9,
        "La Haute Cour",
        "The High Court",
        (67, 68),
    )
    .with_description_fr(
        "Peut destituer le Président pour manquement à ses devoirs manifestement incompatible avec son mandat."
    )
    .with_description_en(
        "Can impeach President for breach of duties manifestly incompatible with office."
    )
}

/// Title X - On the Criminal Responsibility of Government Members
///
/// Articles 68-1 to 68-3
#[must_use]
pub fn title_x_criminal_responsibility() -> ConstitutionTitle {
    ConstitutionTitle::new(
        10,
        "De la responsabilité pénale des membres du Gouvernement",
        "On the Criminal Responsibility of Government Members",
        (68, 68), // Articles 68-1, 68-2, 68-3
    )
    .with_description_fr("Cour de justice de la République pour juger les ministres.")
    .with_description_en("Court of Justice of the Republic to judge ministers.")
}

/// Title XI - The Economic, Social and Environmental Council
///
/// Articles 69-71: Advisory council on economic/social policy
#[must_use]
pub fn title_xi_economic_social_council() -> ConstitutionTitle {
    ConstitutionTitle::new(
        11,
        "Le Conseil économique, social et environnemental",
        "The Economic, Social and Environmental Council",
        (69, 71),
    )
    .with_description_fr(
        "Assemblée consultative sur les questions économiques, sociales et environnementales.",
    )
    .with_description_en("Advisory assembly on economic, social and environmental matters.")
}

/// Title XII - On Territorial Collectivities
///
/// Articles 72-74-1: Decentralization and local government
#[must_use]
pub fn title_xii_territorial_collectivities() -> ConstitutionTitle {
    ConstitutionTitle::new(
        12,
        "Des collectivités territoriales",
        "On Territorial Collectivities",
        (72, 74),
    )
    .with_description_fr(
        "Communes, départements, régions, collectivités à statut particulier. \
        Principe de libre administration.",
    )
    .with_description_en(
        "Communes, departments, regions, collectivities with special status. \
        Principle of free administration.",
    )
}

/// Title XIII - Transitional Provisions on New Caledonia
///
/// Articles 76-77: Special provisions for New Caledonia
#[must_use]
pub fn title_xiii_transitional_provisions() -> ConstitutionTitle {
    ConstitutionTitle::new(
        13,
        "Dispositions transitoires relatives à la Nouvelle-Calédonie",
        "Transitional Provisions on New Caledonia",
        (76, 77),
    )
    .with_description_fr(
        "Statut particulier de la Nouvelle-Calédonie, processus d'autodétermination.",
    )
    .with_description_en("Special status of New Caledonia, self-determination process.")
}

/// Title XIV - On Francophonie and Association Agreements
///
/// Article 87-88: French-speaking community
#[must_use]
pub fn title_xiv_francophonie() -> ConstitutionTitle {
    ConstitutionTitle::new(
        14,
        "De la Francophonie et des accords d'association",
        "On Francophonie and Association Agreements",
        (87, 88),
    )
    .with_description_fr(
        "France participe à la Francophonie et peut conclure des accords d'association.",
    )
    .with_description_en(
        "France participates in Francophonie and may conclude association agreements.",
    )
}

/// Title XV - On the European Union
///
/// Article 88-1 to 88-7: European integration
#[must_use]
pub fn title_xv_european_union() -> ConstitutionTitle {
    ConstitutionTitle::new(
        15,
        "De l'Union européenne",
        "On the European Union",
        (88, 88), // Articles 88-1 through 88-7
    )
    .with_description_fr(
        "France participe à l'Union européenne. Transfert de compétences, subsidiarité.",
    )
    .with_description_en(
        "France participates in the European Union. Transfer of powers, subsidiarity.",
    )
}

/// Title XVI - On the Revision of the Constitution
///
/// Article 89: Constitutional amendment procedure
#[must_use]
pub fn title_xvi_constitutional_revision() -> ConstitutionTitle {
    ConstitutionTitle::new(16, "De la révision", "On the Revision", (89, 89))
        .with_description_fr(
            "Initiative du Président sur proposition du Premier ministre ou des parlementaires. \
        Adoption par referendum ou par Congrès (3/5 des suffrages).",
        )
        .with_description_en(
            "Initiative of President on proposal of Prime Minister or parliamentarians. \
        Adoption by referendum or Congress (3/5 of votes).",
        )
}

/// Get title by number
#[must_use]
pub fn get_title(number: u8) -> Option<ConstitutionTitle> {
    all_titles().into_iter().find(|t| t.number == number)
}

/// Get total article count across all titles
#[must_use]
pub fn total_article_count() -> u16 {
    all_titles()
        .iter()
        .map(|t| u16::from(t.article_count()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_titles_count() {
        let titles = all_titles();
        assert_eq!(titles.len(), 16);
    }

    #[test]
    fn test_title_numbers_sequential() {
        let titles = all_titles();
        for (i, title) in titles.iter().enumerate() {
            assert_eq!(title.number, (i + 1) as u8);
        }
    }

    #[test]
    fn test_title_i_sovereignty() {
        let title = title_i_sovereignty();
        assert_eq!(title.number, 1);
        assert_eq!(title.articles, (1, 4));
        assert_eq!(title.article_count(), 4);
        assert!(title.title_fr.contains("souveraineté"));
    }

    #[test]
    fn test_title_ii_president() {
        let title = title_ii_president();
        assert_eq!(title.number, 2);
        assert_eq!(title.articles, (5, 19));
        assert_eq!(title.article_count(), 15);
    }

    #[test]
    fn test_get_title() {
        let title = get_title(7).unwrap();
        assert_eq!(title.number, 7);
        assert!(title.title_fr.contains("Conseil constitutionnel"));
    }

    #[test]
    fn test_get_title_invalid() {
        assert!(get_title(17).is_none());
        assert!(get_title(0).is_none());
    }

    #[test]
    fn test_all_titles_have_descriptions() {
        for title in all_titles() {
            assert!(
                !title.description_fr.is_empty(),
                "Title {} missing French description",
                title.number
            );
            assert!(
                !title.description_en.is_empty(),
                "Title {} missing English description",
                title.number
            );
        }
    }
}
