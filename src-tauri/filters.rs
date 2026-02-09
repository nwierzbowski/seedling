pub fn filter_think_tag(input: &str) -> String {
    let mut reply = input.to_string();
    if let Some(index) = reply.find("</think>") {
        let start = index + "</think>".len();
        reply = reply[start..].to_string();
    }
    reply
}
