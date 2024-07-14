use actix_web::web;
use serde_json::Value;

// crate imports
use crate::config::ServiceConfig;

pub fn extract_param<'a>(
    params: &'a web::Query<Value>,
    body: &'a web::Json<Value>,
    key: &str,
) -> Option<&'a str> {
    params
        .get(key)
        .or_else(|| body.get(key))
        .and_then(|v| v.as_str())
}

pub fn extract_nested_param<'a>(
    params: &'a web::Query<Value>,
    body: &'a web::Json<Value>,
    key: &str,
    nested_key: &str,
) -> Option<&'a str> {
    params
        .get(key)
        .or_else(|| body.get(key))
        .and_then(|v| v.get(nested_key))
        .and_then(|v| v.as_str())
}

pub fn extract_first_event<'a>(
    params: &'a web::Query<Value>,
    body: &'a web::Json<Value>,
    hook_key: &str,
    events_key: &str,
    event_type: &str,
) -> Option<&'a Value> {
    params
        .get(hook_key)
        .or_else(|| body.get(hook_key))
        .and_then(|hook| hook.get(events_key))
        .and_then(|events| events.as_array())
        .and_then(|events| {
            events
                .iter()
                .find(|&event| event.as_str() == Some(event_type))
        })
}

pub fn is_authorized(
    service_config: &ServiceConfig,
    build_key: Option<&str>,
    repository_url: Option<&str>,
    first_event: Option<&Value>,
) -> bool {
    build_key == Some(service_config.WATCHDOG_RS_BUILD_KEY.as_str())
        && repository_url == Some(service_config.WATCHDOG_RS_REPOSITORY_URL.as_str())
        && first_event.is_some()
}
