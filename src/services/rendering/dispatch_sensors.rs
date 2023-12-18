use super::message::SensorSubscriptionNotification;
use ultralight::ULView;

pub fn dispatch_sensors(view: &mut ULView<'_>, values: &[SensorSubscriptionNotification]) {
    let mut stringified = "{".to_string();

    for value in values {
        stringified.push('"');
        stringified.push_str(&value.code_name);
        stringified.push_str("\":");
        let serialised = serde_json::to_string(value).unwrap_or("{}".to_string());
        stringified.push_str(&serialised);
        stringified.push(',');
    }

    stringified.pop();
    stringified.push('}');

    let script = format!(
        "document.dispatchEvent(new CustomEvent('sensorValues', {{ detail: JSON.parse('{}') }}));",
        stringified
    );

    view.evaluate_script(script);
}
