pub const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

pub fn count(times: Vec<usize>, distances: Vec<usize>) -> usize {
    times
        .into_iter()
        .zip(distances)
        .map(|(max_time, distance)| {
            (0..=max_time)
                .filter(|time| time * (max_time - time) > distance)
                .count()
        })
        .product::<usize>()
}
