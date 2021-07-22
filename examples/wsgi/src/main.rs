fn main() {
    rouille::start_server_with_pool("0.0.0.0:8000", None, move |request| {
        let mut who = request.url();
        if let Some(rest) = who.strip_prefix("/") {
            who = rest.to_string();
        }
        rouille::Response::text(format!("hello {}", who))
    });
}
