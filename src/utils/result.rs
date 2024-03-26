pub fn unwrap<T, E>(result: Result<T, E>, desc: &str) -> T {
    match result {
        Ok(t) => t,
        Err(_) => {
            let msg = format!("{}", desc);
            error!("{}", msg);
            panic!("{}", msg)
        }
    }
}
