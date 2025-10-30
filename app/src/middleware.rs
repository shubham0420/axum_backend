// use std::time::Duration;
// use tower::{limit::RateLimitLayer, timeout::TimeoutLayer};
//
// pub fn timeout_layer(d: Duration) -> TimeoutLayer {
//     TimeoutLayer::new(d)
// }
//
// pub fn rate_limit_layer(n: u64, window: Duration) -> RateLimitLayer {
//     // simple global token bucket
//     RateLimitLayer::new(n, window)
// }
