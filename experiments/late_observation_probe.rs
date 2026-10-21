// Independent exploratory test appended to Kolibrie's existing integration tests.
// Uses existing imports and public APIs in rsp_engine_test.rs, pinned to v0.2.0.
// This probe observes behaviour; it does not assert a particular late-event policy.
#[test]
fn external_probe_late_probabilistic_observation() {
    let consumer = ResultConsumer {
        function: Arc::new(|_row: Vec<(String, String)>| {}),
    };
    let mut engine: RSPEngine<Triple, Vec<(String, String)>> = RSPBuilder::new()
        .add_rsp_ql_query(hybrid_rsp_query())
        .add_sparql_rules(vec![hybrid_rsp_rule()])
        .add_consumer(consumer)
        .add_r2r(Box::new(SimpleR2R::with_execution_mode(QueryExecutionMode::Volcano)))
        .set_operation_mode(OperationMode::SingleThread)
        .build()
        .expect("constructing exploratory engine");
    let newer = engine.parse_data(
        "<http://test/newer> <http://test/input> <http://test/yes> ."
    ).remove(0);
    let older = engine.parse_data(
        "<http://test/older> <http://test/input> <http://test/yes> ."
    ).remove(0);
    let id_newer = engine.add_probabilistic_to_stream("stream", newer, 0.8, 2)
        .expect("newer event ingestion");
    let id_older = engine.add_probabilistic_to_stream("stream", older, 0.9, 1)
        .expect("older event ingestion");
    assert_ne!(id_newer, id_older, "distinct ingestions require distinct ids");
    let window = engine.get_window_info()[0].window_iri.clone();
    let latest = engine.latest_hybrid_results(&window);
    println!("late-observation probe: newest event ts=2 before older event ts=1");
    println!("latest_hybrid_results exists: {}", latest.is_some());
    if let Some(values) = latest {
        println!("derived result count: {}", values.len());
        for (fact, outcome) in &values {
            println!("derived fact: {:?}; result: {:?}", fact, outcome);
        }
    }
}
