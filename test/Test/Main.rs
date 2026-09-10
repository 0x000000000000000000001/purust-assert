use std::sync::atomic::{AtomicUsize, Ordering};

static CALLS: AtomicUsize = AtomicUsize::new(0);

pub fn Test_Main_scenario() -> i64 {
    // Keep the timeout in the test executable, without a separate JS runner.
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(10));
        eprintln!("Assertion test timed out after 10 seconds");
        std::process::exit(124);
    });
    std::env::args().nth(1).expect("scenario argument").parse().expect("integer scenario")
}

pub fn Test_Main_throwing(_: ()) -> i64 {
    CALLS.fetch_add(1, Ordering::SeqCst);
    panic!("callback exception");
}

pub fn Test_Main_checkDeferredFailure(factory: purust_core::Func1<(), UnknownType>) -> UnknownType {
    Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |_| {
        // Constructing a failing assertion must not throw; executing it must.
        let action = factory(());
        for _ in 0..2 {
            let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                action.unwrap_func1()(Value::Unit);
            })).expect_err("false assertion must throw when executed");
            let message = thrown.downcast_ref::<String>().map(String::as_str)
                .or_else(|| thrown.downcast_ref::<&str>().copied());
            assert_eq!(message, Some("deferred assertion"));
        }
        Value::Unit
    })))
}

pub fn Test_Main_checkDeferredThrows(factory: purust_core::Func1<(), UnknownType>) -> UnknownType {
    Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |_| {
        CALLS.store(0, Ordering::SeqCst);
        let action = factory(());
        assert_eq!(CALLS.load(Ordering::SeqCst), 0, "checkThrows must be deferred");
        for expected in 1..=2 {
            action.unwrap_func1()(Value::Unit);
            assert_eq!(CALLS.load(Ordering::SeqCst), expected, "one callback call per execution");
        }
        Value::Unit
    })))
}
