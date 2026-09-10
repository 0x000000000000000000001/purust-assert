pub fn Test_Assert_assertImpl(message: String, success: bool) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |_| {
        if !success {
            panic!("{}", message);
        }
        crate::Value::Unit
    })))
}

pub fn Test_Assert_checkThrows(callback: purust_core::Func1<(), crate::UnknownType>) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |_| {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(())));
        crate::mk_bool(result.is_err())
    })))
}
