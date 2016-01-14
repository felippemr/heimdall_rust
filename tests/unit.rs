extern crate heimdall;

#[test]
fn timestamp_array_has_anomaly(){
    let a_datapoint_vector = vec![(1452776170, 1.1), (1452776171, 1.1),
                               (1452776172, 1.1), (1452776173, 2.1)];
    assert_eq!(heimdall::has_anomaly(&a_datapoint_vector), true);
}

#[test]
fn timestamp_array_does_not_has_anomaly(){
    let a_datapoint_vector = vec![(1452776170, 1.1), (1452776171, 1.1),
                               (1452776172, 1.1), (1452776173, 1.1)];
    assert_eq!(heimdall::has_anomaly(&a_datapoint_vector), false);
}
