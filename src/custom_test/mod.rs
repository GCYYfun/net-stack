mod loopback_test;

// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Testable]) {
//     // println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }
// }

// pub trait Testable {
//     fn run(&self) -> ();
// }

// impl<T> Testable for T
// where
//     T: Fn(),
// {
//     fn run(&self) {
//         // print!("{}...\t", core::any::type_name::<T>());
//         self();
//         // println!("[ok]");
//     }
// }
