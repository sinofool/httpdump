use log::info;
use proxy_wasm as wasm;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_http_context(
        |context_id, _root_context_id| -> Box<dyn wasm::traits::HttpContext> {
            Box::new(AuthHandler{ context_id })
        },
    )
}

struct AuthHandler {
    context_id: u32,
}

impl wasm::traits::Context for AuthHandler {}

impl wasm::traits::HttpContext for AuthHandler {
    fn on_http_request_headers(&mut self, num_headers: usize) -> wasm::types::Action {
        info!("Got {} HTTP headers in #{}.", num_headers, self.context_id);
        let headers = self.get_http_request_headers();
        let mut token = "";

        for (name, value) in &headers {
            if name == "x-skillz-token" {
                token = value;
                break;
            }
        }
        return if token == "" {
            self.send_http_response(403, vec!(), None);
            wasm::types::Action::Pause
        } else {
            self.set_http_request_header("x-skillz-token", Some(&format!("Hello from {}", token)));
            wasm::types::Action::Continue
        }

    }
}
