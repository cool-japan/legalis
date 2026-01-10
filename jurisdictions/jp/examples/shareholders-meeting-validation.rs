//! Shareholders Meeting Validation Example (株主総会の検証例)
//!
//! This example demonstrates validation of shareholders meetings and resolutions
//! according to the Companies Act (会社法).
//!
//! # Usage
//! ```bash
//! cargo run --example shareholders-meeting-validation
//! ```

use chrono::Utc;
use legalis_jp::commercial_law::*;

fn main() {
    println!("=== Shareholders Meeting Validation Example ===\n");
    println!("株主総会の検証例 - Shareholders Meeting Validation\n");

    // Example 1: Valid Ordinary General Meeting
    println!("📋 Example 1: Ordinary General Meeting (定時株主総会)");
    println!("{}", "=".repeat(70));

    let ordinary_meeting = ShareholdersMeeting {
        meeting_type: MeetingType::OrdinaryGeneralMeeting,
        meeting_date: Utc::now(),
        agenda_items: vec![
            AgendaItem {
                item_number: 1,
                description: "承認財務諸表 (Approval of Financial Statements)".to_string(),
                resolution_type: ResolutionType::OrdinaryResolution,
                votes_favor: 800,
                votes_against: 100,
                abstentions: 100,
                result: Some(ResolutionResult::Approved),
            },
            AgendaItem {
                item_number: 2,
                description: "取締役選任 (Election of Directors)".to_string(),
                resolution_type: ResolutionType::OrdinaryResolution,
                votes_favor: 750,
                votes_against: 150,
                abstentions: 100,
                result: Some(ResolutionResult::Approved),
            },
        ],
        quorum_met: true,
        voting_rights_present: 1000,
        voting_rights_total: 1500,
    };

    println!("Meeting Type: Ordinary General Meeting (定時株主総会)");
    println!(
        "Quorum: {} / {} voting rights ({:.1}%)",
        ordinary_meeting.voting_rights_present,
        ordinary_meeting.voting_rights_total,
        (ordinary_meeting.voting_rights_present as f64
            / ordinary_meeting.voting_rights_total as f64)
            * 100.0
    );
    println!("\nAgenda Items:");

    for item in &ordinary_meeting.agenda_items {
        println!("\n  Item {}: {}", item.item_number, item.description);
        println!("    Resolution Type: {:?}", item.resolution_type);
        println!(
            "    Votes - For: {}, Against: {}, Abstain: {}",
            item.votes_favor, item.votes_against, item.abstentions
        );
        if let Some(result) = item.result {
            println!("    Result: {:?}", result);
        }
    }

    match validate_shareholders_meeting_resolution(&ordinary_meeting) {
        Ok(()) => println!("\n✅ Validation: PASSED - All resolutions are valid!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 2: Special Resolution (2/3 majority required)
    println!("📋 Example 2: Special Resolution (特別決議)");
    println!("{}", "=".repeat(70));
    println!("Required: 2/3 majority of voting rights present (Article 309-2)\n");

    let special_resolution_meeting = ShareholdersMeeting {
        meeting_type: MeetingType::ExtraordinaryGeneralMeeting,
        meeting_date: Utc::now(),
        agenda_items: vec![AgendaItem {
            item_number: 1,
            description: "定款変更 (Amendment to Articles of Incorporation)".to_string(),
            resolution_type: ResolutionType::SpecialResolution,
            votes_favor: 700, // 70% of 1000 present
            votes_against: 200,
            abstentions: 100,
            result: Some(ResolutionResult::Approved),
        }],
        quorum_met: true,
        voting_rights_present: 1000,
        voting_rights_total: 1500,
    };

    for item in &special_resolution_meeting.agenda_items {
        println!("  Item {}: {}", item.item_number, item.description);
        println!(
            "    Resolution Type: {:?} (2/3 majority required)",
            item.resolution_type
        );
        println!(
            "    Votes - For: {}, Against: {}, Abstain: {}",
            item.votes_favor, item.votes_against, item.abstentions
        );
        println!(
            "    Approval Rate: {:.1}%",
            (item.votes_favor as f64 / special_resolution_meeting.voting_rights_present as f64)
                * 100.0
        );
        if let Some(result) = item.result {
            println!("    Result: {:?}", result);
        }
    }

    match validate_shareholders_meeting_resolution(&special_resolution_meeting) {
        Ok(()) => println!("\n✅ Validation: PASSED - Special resolution approved!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 3: Failed Resolution (Insufficient Votes)
    println!("📋 Example 3: Failed Resolution (Insufficient Votes)");
    println!("{}", "=".repeat(70));

    let failed_meeting = ShareholdersMeeting {
        meeting_type: MeetingType::OrdinaryGeneralMeeting,
        meeting_date: Utc::now(),
        agenda_items: vec![AgendaItem {
            item_number: 1,
            description: "役員報酬増額 (Increase in Executive Compensation)".to_string(),
            resolution_type: ResolutionType::OrdinaryResolution,
            votes_favor: 400, // Only 40% - insufficient
            votes_against: 500,
            abstentions: 100,
            result: Some(ResolutionResult::Rejected),
        }],
        quorum_met: true,
        voting_rights_present: 1000,
        voting_rights_total: 1500,
    };

    for item in &failed_meeting.agenda_items {
        println!("  Item {}: {}", item.item_number, item.description);
        println!(
            "    Votes - For: {}, Against: {}, Abstain: {}",
            item.votes_favor, item.votes_against, item.abstentions
        );
        println!(
            "    Approval Rate: {:.1}% (majority required)",
            (item.votes_favor as f64 / failed_meeting.voting_rights_present as f64) * 100.0
        );
        if let Some(result) = item.result {
            println!("    Result: {:?}", result);
        }
    }

    match validate_shareholders_meeting_resolution(&failed_meeting) {
        Ok(()) => println!("\n✅ Validation: PASSED - Rejection properly recorded!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 4: Quorum Not Met
    println!("📋 Example 4: Quorum Not Met (定足数未達)");
    println!("{}", "=".repeat(70));

    let no_quorum_meeting = ShareholdersMeeting {
        meeting_type: MeetingType::OrdinaryGeneralMeeting,
        meeting_date: Utc::now(),
        agenda_items: vec![AgendaItem {
            item_number: 1,
            description: "Test Resolution".to_string(),
            resolution_type: ResolutionType::OrdinaryResolution,
            votes_favor: 300,
            votes_against: 100,
            abstentions: 50,
            result: None,
        }],
        quorum_met: false, // Quorum not met!
        voting_rights_present: 450,
        voting_rights_total: 1500,
    };

    println!("Quorum Status: NOT MET");
    println!(
        "Voting Rights Present: {} / {} ({:.1}%)",
        no_quorum_meeting.voting_rights_present,
        no_quorum_meeting.voting_rights_total,
        (no_quorum_meeting.voting_rights_present as f64
            / no_quorum_meeting.voting_rights_total as f64)
            * 100.0
    );

    match validate_shareholders_meeting_resolution(&no_quorum_meeting) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED (as expected)\n   Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 5: Board of Directors Validation
    println!("📋 Example 5: Board of Directors Validation (取締役会の検証)");
    println!("{}", "=".repeat(70));

    let board = BoardOfDirectors {
        directors: vec![
            Director {
                name: "山田太郎 (President)".to_string(),
                position: DirectorPosition::President,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + chrono::Duration::days(365)),
            },
            Director {
                name: "佐藤花子 (Executive Director)".to_string(),
                position: DirectorPosition::ExecutiveDirector,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + chrono::Duration::days(365)),
            },
            Director {
                name: "鈴木一郎 (Outside Director)".to_string(),
                position: DirectorPosition::OutsideDirector,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + chrono::Duration::days(365)),
            },
        ],
        meeting_frequency: Some("Monthly".to_string()),
    };

    println!("Board Members: {}", board.directors.len());
    println!("Meeting Frequency: {:?}\n", board.meeting_frequency);

    for (i, director) in board.directors.iter().enumerate() {
        println!("  {}. {} - {:?}", i + 1, director.name, director.position);
    }

    match validate_board_of_directors(&board, true) {
        Ok(()) => println!("\n✅ Validation: PASSED - Board composition is valid!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 6: Corporate Auditors Validation
    println!("📋 Example 6: Corporate Auditors (監査役の検証)");
    println!("{}", "=".repeat(70));

    let auditors = CorporateAuditors {
        auditors: vec![
            CorporateAuditor {
                name: "田中次郎 (Full-time)".to_string(),
                is_full_time: true,
                is_outside: false,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + chrono::Duration::days(365 * 4)),
            },
            CorporateAuditor {
                name: "高橋三郎 (Outside)".to_string(),
                is_full_time: false,
                is_outside: true,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + chrono::Duration::days(365 * 4)),
            },
            CorporateAuditor {
                name: "伊藤四郎 (Outside)".to_string(),
                is_full_time: false,
                is_outside: true,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + chrono::Duration::days(365 * 4)),
            },
        ],
    };

    println!("Auditors: {}", auditors.auditors.len());
    let outside_count = auditors.auditors.iter().filter(|a| a.is_outside).count();
    println!(
        "Outside Auditors: {} / {} ({:.0}%)\n",
        outside_count,
        auditors.auditors.len(),
        (outside_count as f64 / auditors.auditors.len() as f64) * 100.0
    );

    for (i, auditor) in auditors.auditors.iter().enumerate() {
        println!(
            "  {}. {} - {} / {}",
            i + 1,
            auditor.name,
            if auditor.is_full_time {
                "Full-time"
            } else {
                "Part-time"
            },
            if auditor.is_outside {
                "Outside"
            } else {
                "Inside"
            }
        );
    }

    match validate_corporate_auditors(&auditors, true, true) {
        Ok(()) => println!("\n✅ Validation: PASSED - Auditor composition is valid!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}", "=".repeat(70));
    println!("\n✨ Shareholders Meeting Validation Examples Complete!");
    println!("   All examples demonstrate proper compliance with");
    println!("   the Companies Act (会社法) governance requirements.\n");
}
