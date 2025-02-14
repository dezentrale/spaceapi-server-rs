//! Modifiers which can be injected by the application logic to change the
//! state dynamically per request.

use spaceapi_dezentrale::sensors::{PeopleNowPresentSensor, Sensors};
use spaceapi_dezentrale::Status;

/// `StatusModifier`s are used to modify the status
pub trait StatusModifier: Send + Sync {
    /// Called after all registered sensors are read
    fn modify(&self, status: &mut Status);
}

/// This modifier updates the opening state based on the
/// first people now present sensor (if present).
pub struct StateFromPeopleNowPresent;

impl StatusModifier for StateFromPeopleNowPresent {
    fn modify(&self, status: &mut Status) {
        // Update state depending on number of people present
        let people_now_present: Option<u64> = status
            .sensors
            .as_ref()
            .and_then(|sensors: &Sensors| sensors.people_now_present.first())
            .map(|sensor: &PeopleNowPresentSensor| sensor.value);
        if let Some(count) = people_now_present {
            let mut state = status.state.clone().unwrap_or_default();
            state.open = Some(count > 0);
            // comparison chain is actually cleaner here IMO
            #[allow(clippy::comparison_chain)]
            if count == 1 {
                state.message = Some(format!("{} person here right now", count));
            } else if count > 1 {
                state.message = Some(format!("{} people here right now", count));
            }
            status.state = Some(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod state_from_people_now_present {
        use spaceapi_dezentrale::{State, Status};
        use spaceapi_dezentrale::sensors::Sensors;

        use super::*;

        #[test]
        fn no_sensors() {
            let mut status = Status {
                sensors: None,
                ..Status::default()
            };
            assert_eq!(status.state, None);
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(status.sensors, None);
            assert_eq!(status.state, None);
        }

        #[test]
        fn no_people_present_sensor() {
            let mut status = Status {
                sensors: Some(Sensors {
                    people_now_present: vec![],
                    temperature: vec![],
                    humidity: vec![],
                    power_consumption: vec![],
                }),
                ..Status::default()
            };
            assert_eq!(status.state, None);
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(status.state, None);
        }

        fn make_pnp_sensor(value: u64) -> PeopleNowPresentSensor {
            PeopleNowPresentSensor {
                metadata: Default::default(),
                names: None,
                value,
            }
        }

        #[test]
        fn zero_people_present() {
            let mut status = Status {
                sensors: Some(Sensors {
                    people_now_present: vec![make_pnp_sensor(0)],
                    temperature: vec![],
                    humidity: vec![],
                    power_consumption: vec![],
                }),
                state: Some(State::default()),
                ..Status::default()
            };
            status.state.as_mut().unwrap().message = Some("This will remain unchanged.".to_string());
            assert_eq!(
                status.state.as_ref().unwrap().message,
                Some("This will remain unchanged.".to_string())
            );
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(
                status.state.unwrap().message,
                Some("This will remain unchanged.".to_string())
            );
        }

        #[test]
        fn one_person_present() {
            let mut status = Status {
                sensors: Some(Sensors {
                    people_now_present: vec![make_pnp_sensor(1)],
                    temperature: vec![],
                    humidity: vec![],
                    power_consumption: vec![],
                }),
                ..Status::default()
            };
            assert_eq!(status.state, None);
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(
                status.state.unwrap().message,
                Some("1 person here right now".to_string())
            );
        }

        #[test]
        fn two_people_present() {
            let mut status = Status {
                sensors: Some(Sensors {
                    people_now_present: vec![make_pnp_sensor(2)],
                    temperature: vec![],
                    humidity: vec![],
                    power_consumption: vec![],
                }),
                ..Status::default()
            };
            assert_eq!(status.state, None);
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(
                status.state.as_ref().unwrap().message,
                Some("2 people here right now".to_string())
            );
        }
    }
}
