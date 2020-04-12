use std::sync::Once;

static START: Once = Once::new();

pub fn initialize() {
    START.call_once(|| {
        println!("Venus initialized!");
    })
}

#[cfg(test)]
mod tests {
    use crate::initialize;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_initializes() {
        initialize();
    }
}


