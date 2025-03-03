use serde::{Deserialize, Serialize};

// all event builders and their event methods
pub mod command;
pub mod generic;
pub mod repo;
pub mod task;

/// All possible telemetry events must be included in this enum.
/// These events must be added to the backend (telemetry.vercel.com)
/// before they can be tracked - invalid or unknown events will be
/// ignored.
pub use turborepo_vercel_api::TelemetryEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Sensitive,
    NonSensitive,
}

/// Key-value pairs that are sent with each even - if the value is
/// sensitive, it will be hashed and anonymized before being sent
/// using the users private salt.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    key: String,
    value: String,
    is_sensitive: EventType,
}

pub trait Identifiable {
    fn get_id(&self) -> &String;
}

/// Public trait that can be used for building telemetry events.
/// Supports connecting events via a parent-child relationship
/// to aid in connecting events together.
pub trait EventBuilder {
    fn with_parent<U: Identifiable>(self, parent_event: &U) -> Self;
    fn track(&self, event: Event);
    fn child(&self) -> Self;
}

#[macro_export]
macro_rules! track_usage {
    ($tel:expr, $field:expr, $is_used:expr) => {
        if $is_used($field) {
            $tel.track_arg_usage(
                stringify!($field)
                    .trim_start_matches("&")
                    .trim_start_matches("self.")
                    .replace("_", "-")
                    .as_str(),
                true,
            );
        }
    };
}
