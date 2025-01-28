pub fn format_message(message: &str) -> String {
    format!("Message: {}", message)
}

pub fn validate_input(input: &str) -> bool {
    !input.trim().is_empty()
}
