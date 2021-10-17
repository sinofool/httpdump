use log::info;
use proxy_wasm as wasm;
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::ContextType;
use serde::{Deserialize, Serialize};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_root_context(
        |_context_id| -> Box<dyn wasm::traits::RootContext> {
            Box::new(RootHandler { config: None })
        },
    )
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub valid_keys: Vec<String>,
}

struct RootHandler {
    config: Option<Config>,
}

impl Context for RootHandler {}

impl RootContext for RootHandler {
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        let data = self.get_configuration().unwrap();
        let config = serde_json::from_slice::<Config>(&data).unwrap();
        self.config = Some(config);
        true
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(AuthHandler {
            context_id: _context_id,
            config: self.config.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct AuthHandler {
    context_id: u32,
    config: Option<Config>,
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
        if token == "" {
            self.send_http_response(403, vec!(), None);
            return wasm::types::Action::Pause;
        }

        for valid_token in &self.config.as_ref().unwrap().valid_keys {
            if valid_token == token {
                self.set_http_request_header("x-skillz-token", Some(&format!("Token is {}", token)));
                return wasm::types::Action::Continue;
            }
        }
        self.send_http_response(403, vec!(), None);
        return wasm::types::Action::Pause;
    }
}
