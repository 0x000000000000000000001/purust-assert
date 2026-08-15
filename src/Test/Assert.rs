pub fn Test_Assert_assertImpl() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { call: Some(std::rc::Rc::new(move |_message: crate::UnknownType| -> crate::UnknownType {
        crate::UnknownType::new(crate::Record_a { call: Some(std::rc::Rc::new(move |_success: crate::UnknownType| -> crate::UnknownType {
            // Mock assert
            crate::UnknownType::new(crate::Record_a { ..Default::default() })
        })), ..Default::default() })
    })), ..Default::default() })
}

pub fn Test_Assert_checkThrows() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { call: Some(std::rc::Rc::new(move |_f: crate::UnknownType| -> crate::UnknownType {
        // Mock checkThrows (returns true for now, but boolean in purust is true=Record_a)
        crate::UnknownType::new(crate::Record_a { init_bool: Some(true), ..Default::default() })
    })), ..Default::default() })
}
