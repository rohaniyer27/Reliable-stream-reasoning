// Experimental integration test appended to Kolibrie's v0.2.0 upstream test module.
// Uses a public Rust method, latest_hybrid_results, whose result shape is
// implementation-specific and not presented as a stable external API.
fn external_run_arrival_order(reverse: bool) -> Vec<f64> {
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
        .expect("constructing engine");
    let older = engine.parse_data(
        "<http://test/older> <http://test/input> <http://test/yes> ."
    ).remove(0);
    let newer = engine.parse_data(
        "<http://test/newer> <http://test/input> <http://test/yes> ."
    ).remove(0);
    let trigger = engine.parse_data(
        "<http://test/trigger> <http://test/input> <http://test/yes> ."
    ).remove(0);
    if reverse {
        engine.add_probabilistic_to_stream("stream", newer, 0.8, 2).unwrap();
        engine.add_probabilistic_to_stream("stream", older, 0.9, 1).unwrap();
    } else {
        engine.add_probabilistic_to_stream("stream", older, 0.9, 1).unwrap();
        engine.add_probabilistic_to_stream("stream", newer, 0.8, 2).unwrap();
    }
    // Trigger another window firing at a common event time.
    engine.add_probabilistic_to_stream("stream", trigger, 0.95, 3).unwrap();
    let window = engine.get_window_info()[0].window_iri.clone();
    let values = engine.latest_hybrid_results(&window)
        .expect("hybrid result snapshot should be available");
    let mut probabilities: Vec<f64> = values.values().filter_map(|result| {
        match result {
            shared::hybrid::HybridProbabilityResult::Exact { probability, .. } => Some(*probability),
            _ => None,
        }
    }).collect();
    probabilities.sort_by(|a, b| a.total_cmp(b));
    println!("reverse={reverse}: exact probabilities={probabilities:?}");
    probabilities
}

#[test]
fn external_probe_late_probabilistic_observation() {
    let ordered = external_run_arrival_order(false);
    let reversed = external_run_arrival_order(true);
    assert!(!ordered.is_empty(), "ordered control must yield nonempty exact probabilistic conclusions");
    assert!(!reversed.is_empty(), "reversed control must yield nonempty exact probabilistic conclusions");
    assert_eq!(ordered.len(), reversed.len(), "same number of derived conclusions expected");
    for (a, b) in ordered.iter().zip(reversed.iter()) {
        assert!((a - b).abs() < 1e-9, "ordered vs reversed probability differs: {a} vs {b}");
    }
}
