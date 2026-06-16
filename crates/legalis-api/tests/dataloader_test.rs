use legalis_api::graphql::{GraphQLState, create_schema};
use legalis_core::{Effect, EffectType, Statute};

fn make_statute(id: &str, jurisdiction: &str) -> Statute {
    Statute::new(
        id,
        format!("Statute {id}"),
        Effect::new(EffectType::Obligation, "test"),
    )
    .with_jurisdiction(jurisdiction)
}

async fn make_state_with_statutes(count: usize) -> GraphQLState {
    let state = GraphQLState::new();
    {
        let mut statutes = state.statutes.write().await;
        for i in 0..count {
            statutes.push(make_statute(&format!("statute-{i}"), "TEST"));
        }
    }
    state
}

#[tokio::test]
async fn schema_builds_with_dataloaders() {
    let state = make_state_with_statutes(10).await;
    let schema = create_schema(state);
    let result = schema.execute("{ statutes { id title } }").await;
    assert!(
        result.errors.is_empty(),
        "GraphQL errors: {:?}",
        result.errors
    );
}

#[tokio::test]
async fn statute_lookup_by_id_via_dataloader() {
    let state = make_state_with_statutes(50).await;
    let schema = create_schema(state);
    let result = schema
        .execute(r#"{ statute(id: "statute-5") { id title } }"#)
        .await;
    assert!(
        result.errors.is_empty(),
        "GraphQL errors: {:?}",
        result.errors
    );
    // Verify the returned data contains the expected statute
    let data = result.data.into_json().expect("No data");
    let statute_id = data["statute"]["id"].as_str().expect("No statute id");
    assert_eq!(statute_id, "statute-5");
}

#[tokio::test]
async fn statute_lookup_missing_returns_null() {
    let state = make_state_with_statutes(5).await;
    let schema = create_schema(state);
    let result = schema
        .execute(r#"{ statute(id: "nonexistent-999") { id title } }"#)
        .await;
    assert!(
        result.errors.is_empty(),
        "GraphQL errors: {:?}",
        result.errors
    );
    let data = result.data.into_json().expect("No data");
    assert!(
        data["statute"].is_null(),
        "Expected null for missing statute"
    );
}

#[tokio::test]
async fn statutes_by_jurisdiction_via_dataloader() {
    let state = GraphQLState::new();
    {
        let mut statutes = state.statutes.write().await;
        statutes.push(make_statute("us-1", "US"));
        statutes.push(make_statute("us-2", "US"));
        statutes.push(make_statute("eu-1", "EU"));
    }
    let schema = create_schema(state);
    let result = schema
        .execute(r#"{ statutesByJurisdiction(jurisdiction: "US") { id } }"#)
        .await;
    assert!(
        result.errors.is_empty(),
        "GraphQL errors: {:?}",
        result.errors
    );
    let data = result.data.into_json().expect("No data");
    let statutes = data["statutesByJurisdiction"]
        .as_array()
        .expect("Expected array");
    assert_eq!(statutes.len(), 2, "Expected 2 US statutes");
}
