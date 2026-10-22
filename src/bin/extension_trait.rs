/*The extension trait pattern lets us add new methods to an existing type without modifying its original implementation. Unlike traditional inheritance based extension in OOP, rust achieves this through traits.
 */

use std::fmt::Debug;

trait StatisticsExt {
    fn average(&self) -> Option<f64>;
}

impl StatisticsExt for Vec<u64> {
    fn average(&self) -> Option<f64> {
        if self.is_empty() {
            return None;
        }

        let sum: f64 = self.iter().map(|&n| n as f64).sum();

        Some(sum / self.len() as f64)
    }
}

fn main() {
    let numbers = vec![10, 20, 30, 40];
    println!("Average: {:?}", numbers.average());

    let result: Result<u64, String> = Ok(100);

    result.log_success().log_error();

    let failed: Result<u64, String> = Err("Connection timeout".into());

    failed.log_success().log_error();
}

trait ResultExt<T, E>
where
    T: Debug,
    E: Debug,
{
    fn log_success(&self) -> &Self;
    fn log_error(&self) -> &Self;
}
impl<T, E> ResultExt<T, E> for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn log_success(&self) -> &Result<T, E> {
        if let Ok(value) = self {
            println!("Success: {:?}", value);
        }
        self
    }

    fn log_error(&self) -> &Result<T, E> {
        if let Err(err) = self {
            println!("Error: {:?}", err);
        }

        self
    }
}
