extern crate heimdall;

#[test]
fn timestamp_array_has_anomaly(){
    let timestamp_array = vec![("100001", 1), ("100002", 1),
                               ("100003", 1), ("100004", 2)];

    assert!(heimdall::has_anomaly(&timestamp_array), true);
}
