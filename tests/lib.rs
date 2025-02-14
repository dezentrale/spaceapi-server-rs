use std::io::ErrorKind;
use std::net::Ipv4Addr;
use std::net::TcpStream;

use spaceapi_dezentrale::{Contact, IssueReportChannel, Location, State, Status, StatusBuilder};

use spaceapi_dezentrale_server::{SpaceapiServer, SpaceapiServerBuilder};

/// Create a new status object containing test data.
fn get_status() -> Status {
    StatusBuilder::new("ourspace")
        .logo("https://example.com/logo.png")
        .url("https://example.com/")
        .location(Location {
            address: Some("Street 1, Zürich, Switzerland".to_string()),
            osm_link: None,
            lat: 47.123,
            lon: 8.88,
            timezone: None,
        })
        .contact(Contact {
            email: Some("hi@example.com".to_string()),
            ..Contact::default()
        })
        .add_issue_report_channel(IssueReportChannel::Matrix)
        .add_issue_report_channel(IssueReportChannel::Twitter)
        .state(State {
            open: Some(false),
            ..State::default()
        })
        .build()
        .unwrap()
}

/// Create a new SpaceapiServer instance listening on the specified port.
fn get_server(status: Status) -> SpaceapiServer {
    SpaceapiServerBuilder::new(status)
        .redis_connection_info("redis://127.0.0.1/")
        .build()
        .unwrap()
}

#[test]
fn server_starts() {
    //! Test that the spaceapi server starts at all.

    // Ip / port for test server
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 3344;

    // Test data
    let status = get_status();

    // Connection to port should fail right now
    let connect_result = TcpStream::connect((ip, port));
    assert!(connect_result.is_err());
    assert_eq!(connect_result.unwrap_err().kind(), ErrorKind::ConnectionRefused);

    // Instantiate and start server
    let server = get_server(status);
    let mut listening = server.serve((ip, port)).unwrap();

    // Connecting to server should work now
    let connect_result = TcpStream::connect((ip, port));
    assert!(connect_result.is_ok());

    // Close server
    listening.close().unwrap();
}
