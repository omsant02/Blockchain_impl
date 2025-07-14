mod balances;
mod system;

pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}
impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let runtime = Runtime::new();
    println!("Hello, world!");
}

#[test]
fn init_balances() {
    let mut balances = balances::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()), 0);
    balances.set_balance(&"alice".to_string(), 100);
    
    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);
}