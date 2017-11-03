error_chain!{
    foreign_links {
        Reqwest(::reqwest::Error);
    }
    errors {
        Panic(inner: Box<::std::any::Any + Send + 'static>) {
            description("Thread Panicked")
                display("{}",
                        if let Some(s) = inner.downcast_ref::<String>() {
                            s.clone()
                        } else if let Some(s) = inner.downcast_ref::<&str>() {
                            s.to_string()
                        } else {
                            String::from("Thread Panicked")
                        })
        }
    }
}
