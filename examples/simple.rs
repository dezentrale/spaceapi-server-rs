use spaceapi_dezentrale::{Contact, IssueReportChannel, Location, State, StatusBuilder};

use spaceapi_dezentrale_server::SpaceapiServerBuilder;

fn main() {
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
            twitter: Some("@coredump_ch".to_string()),
            ..Default::default()
        })
        .add_issue_report_channel(IssueReportChannel::Matrix)
        .add_issue_report_channel(IssueReportChannel::Twitter)
        .state(State::default())
        .build()
        .expect("Creating status failed");

    // Set up server
    let server = SpaceapiServerBuilder::new(status)
        .redis_connection_info("redis://127.0.0.1/")
        .build()
        .unwrap();

    // Serve!
    let _ = server.serve("127.0.0.1:8000");
}
