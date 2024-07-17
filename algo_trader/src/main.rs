mod structs;
mod impls;
mod enums;

mod test;

pub use structs::Order;
use structs::Price;
pub use enums::BidOrAsk;
pub use test::test;

fn main() -> Result<(), String> {
    match test() {
        Ok(_) => {Ok(())},
        Err(err) => {
            return Err(format!("{}", err));
        }
    }
}