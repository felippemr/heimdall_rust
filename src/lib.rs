pub fn has_anomaly(datapoint_vector: &Vec<(i32, f32)>) -> bool{
    for datapoint in datapoint_vector.iter().enumerate() {
        let previous: f32 = match datapoint.0 {
            0 => (datapoint.1).1,
            _ => datapoint_vector[datapoint.0 - 1].1
        };
        if previous != (datapoint.1).1 {
            return true;
        };
    };
    return false;
}
