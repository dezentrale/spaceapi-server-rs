use spaceapi_dezentrale::{Contact, IssueReportChannel, Location, State, StatusBuilder};
use spaceapi_dezentrale::sensors::{PeopleNowPresentSensorTemplate, TemperatureSensorTemplate};

use spaceapi_dezentrale_server::modifiers::StateFromPeopleNowPresent;
use spaceapi_dezentrale_server::SpaceapiServerBuilder;

fn main() {
    env_logger::init();

    // Create new minimal Status instance compatible with v0.13 and v14
    let status = StatusBuilder::mixed("coredump")
        .logo("https://www.coredump.ch/logo.png")
        .url("https://www.coredump.ch/")
        .location(Location {
            address: Some("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".to_string()),
            osm_link: None,
            lat: 47.22936,
            lon: 8.82949,
            timezone: None,
        })
        .contact(Contact {
            irc: Some("irc://freenode.net/#coredump".to_string()),
            twitter: Some("None@coredump_ch".to_string()),
            ..Default::default()
        })
        .add_issue_report_channel(IssueReportChannel::Email)
        .add_issue_report_channel(IssueReportChannel::Twitter)
        .state(State::default())
        .build()
        .expect("Creating status failed");

    // Set up server
    let server = SpaceapiServerBuilder::new(status)
        .redis_connection_info("redis://127.0.0.1/")
        .add_status_modifier(StateFromPeopleNowPresent)
        .add_sensor(
            PeopleNowPresentSensorTemplate {
                metadata: Default::default(),
            },
            "people_now_present".into(),
        )
        .add_sensor(
            TemperatureSensorTemplate {
                metadata: Default::default(),
                unit: "°C".to_string(),
            },
            "temp_room1".into(),
        )
        .add_sensor(
            TemperatureSensorTemplate {
                metadata: Default::default(),
                unit: "°C".to_string(),

            },
            "temp_room2".into(),
        )
        .build()
        .expect("Could not initialize server");

    // Serve!
    server
        .serve("127.0.0.1:8000")
        .expect("Could not start the server");
}
