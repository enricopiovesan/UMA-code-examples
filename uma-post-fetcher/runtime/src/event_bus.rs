use service::model::Event;
use serde_json::{json, Value};

/// Deterministic event bus.  Maintains a logical clock and appends events
/// to an internal vector.  Each call to `emit` increments the clock and
/// returns a reference to the stored event.
pub struct EventBus {
    pub events: Vec<Event>,
    pub clock: u64,
}

impl EventBus {
    /// Create a new event bus with logical clock starting at zero.
    pub fn new() -> Self {
        Self { events: Vec::new(), clock: 0 }
    }

    /// Emit an event of the given type with arbitrary data.  The logical
    /// timestamp is converted to a string.
    pub fn emit(&mut self, event_type: &str, data: Value) {
        let t = self.clock.to_string();
        let event = Event {
            t,
            type_: event_type.to_string(),
            data,
        };
        self.events.push(event);
        self.clock += 1;
    }
}