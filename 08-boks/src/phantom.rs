use std::marker::PhantomData;

// Imagine this is a specialized allocator or a factory
// for a specific type T.
struct Factory<T> {
    factory_id: u32,
    // We don't store T, but we want the compiler to treat
    // Factory as if it "leads to" or "produces" T.
    // We use fn() -> T to indicate Covariance.
    _marker: PhantomData<fn() -> T>,
}

impl<T> Factory<T> {
    fn new(id: u32) -> Self {
        Self {
            factory_id: id,
            _marker: PhantomData,
        }
    }

    // In a real scenario, this might interact with
    // low-level memory or a network stream.
    fn produce_info(&self) {
        println!(
            "Factory {} is configured for type: {}",
            self.factory_id,
            std::any::type_name::<T>()
        );
    }
}

fn main() {
    // 1. Basic usage
    let string_factory: Factory<String> = Factory::new(101);
    string_factory.produce_info();

    // 2. The "Why": Subtyping and Variance
    // 'static lifetimes are subtypes of shorter lifetimes (like 'a).
    // Because fn() -> T is COVARIANT, a Factory<&'static str>
    // can be treated as a Factory<&'a str>.

    let factory_static: Factory<&'static str> = Factory::new(202);

    {
        let short_lifetime = String::from("short");
        let _short_ref: &str = &short_lifetime;

        // This works because of the covariance provided by fn() -> T
        let _test_variance: Factory<&str> = factory_static;
        _test_variance.produce_info();
    }
}
