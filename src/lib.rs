#[cfg(feature = "schedule")]
pub mod schedule;
#[cfg(feature = "realtime")]
pub mod realtime;

pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
