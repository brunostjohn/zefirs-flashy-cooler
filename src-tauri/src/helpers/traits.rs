use std::{
    sync::mpsc::Receiver,
    time::{Duration, SystemTime},
};

use crate::{rendering::ThemeConfigItem, sensors::SensorWithDetails};

pub trait CustomSerialise {
    fn custom_serialise(&self) -> String
    where
        Self: Sized;
}

impl CustomSerialise for Vec<ThemeConfigItem> {
    #[inline(always)]
    fn custom_serialise(&self) -> String
    where
        Self: Sized,
    {
        let theme_config = self;

        let everything_else: Vec<ThemeConfigItem> = theme_config
            .iter()
            .filter(|x| x.r#type != "sensor")
            .map(|x| x.to_owned())
            .collect::<Vec<ThemeConfigItem>>();

        let mut serialised = "{".to_owned();

        for item in everything_else {
            serialised += &("\"".to_owned() + &item.name.clone() + "\":");
            let item_string = serde_json::to_string::<ThemeConfigItem>(&item)
                .or::<Result<String, &'static str>>(Ok("[]".to_string()))
                .unwrap();
            serialised += &item_string;
            serialised += ",";
        }

        serialised.pop();
        serialised += "}";

        serialised
    }
}

impl CustomSerialise for Vec<String> {
    #[inline(always)]
    fn custom_serialise(&self) -> String
    where
        Self: Sized,
    {
        let mut all_sensor_string = "{".to_owned();

        for sensor in self {
            all_sensor_string += &sensor;
            all_sensor_string += ",";
        }

        all_sensor_string.pop();
        all_sensor_string += "}";

        all_sensor_string
    }
}

impl CustomSerialise for Vec<SensorWithDetails> {
    #[inline(always)]
    fn custom_serialise(&self) -> String
    where
        Self: Sized,
    {
        let mut all_sensor_string = "{".to_owned();

        for sensor in self {
            all_sensor_string += &("\"".to_owned() + &sensor.code_name + "\":");
            let name = &sensor.sensor;
            let r#type = &sensor.r#type;
            let value = &sensor.value;
            let parent_hw = (sensor.parent_hw_type.clone())
                .or(Some("".to_string()))
                .unwrap();

            let sensor_string = format!(
                r#"{{"sensor":"{name}","value":"{value}","type":"{type}","parent_hw_type":"{parent_hw}"}}"#
            );
            all_sensor_string += &sensor_string;
            all_sensor_string += ",";
        }

        all_sensor_string.pop();
        all_sensor_string += "}";

        all_sensor_string
    }
}

pub trait Reassign<T> {
    fn reassign(self, channel: T) -> Self
    where
        Self: Sized;
}

impl Reassign<Result<Vec<String>, &'static str>> for Vec<String> {
    #[inline(always)]
    fn reassign(self, channel: Result<Vec<String>, &'static str>) -> Self
    where
        Self: Sized,
    {
        match channel {
            Ok(result) => {
                let checked = &result[0];
                if checked.contains(r#"{"sensor": "noWayInFuckingHell", "value": "anyLegitimateSensorValue", "type": "WouldContainThis"}"#) {
                    return self;
                } else {
                    return result;
                }
            }
            Err(_) => return self,
        }
    }
}

impl Reassign<Result<Vec<SensorWithDetails>, &'static str>> for Vec<SensorWithDetails> {
    #[inline(always)]
    fn reassign(self, channel: Result<Vec<SensorWithDetails>, &'static str>) -> Self
    where
        Self: Sized,
    {
        match channel {
            Ok(result) => {
                if result[0].value != "a" && result[0].r#type != "a" && result[0].value != "3" {
                    return result;
                } else {
                    return self;
                }
            }
            Err(_) => return self,
        }
    }
}

impl Reassign<&Receiver<Duration>> for Duration {
    #[inline(always)]
    fn reassign(self, channel: &Receiver<Self>) -> Self
    where
        Self: Sized,
    {
        match channel.try_recv() {
            Ok(result) => return Self::from(result),
            Err(_) => return self,
        }
    }
}

pub trait TryElapsed<T> {
    fn try_elapsed(&self, duration: T) -> bool
    where
        Self: Sized;
}

impl TryElapsed<u64> for SystemTime {
    #[inline(always)]
    fn try_elapsed(&self, duration_ms: u64) -> bool
    where
        Self: Sized,
    {
        match self.elapsed() {
            Ok(time) => {
                if time >= Duration::from_millis(duration_ms) {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        }
    }
}

impl TryElapsed<Duration> for SystemTime {
    #[inline(always)]
    fn try_elapsed(&self, duration: Duration) -> bool
    where
        Self: Sized,
    {
        match self.elapsed() {
            Ok(time) => {
                if time >= duration {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        }
    }
}
