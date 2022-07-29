struct Message {
    id: u64,
    message: String,
}

pub fn add(id: u64, message: String) -> u64 {
    return id;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_messages() {
        let id = 5;
        let message = "Test message";
        let res = add(id, message.to_string());
        assert_eq!(res, id)
    }
}
