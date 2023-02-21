use r2d2::Pool;
use redis::{Commands, RedisResult};
use spaceapi_dezentrale::{Contact, IssueReportChannel, Location, State, Status, StatusBuilder};

use spaceapi_dezentrale_server::modifiers::StatusModifier;
use spaceapi_dezentrale_server::SpaceapiServerBuilder;

type RedisPool = Pool<redis::Client>;

struct OpenStatusFromRedisModifier {
    pool: RedisPool,
}

impl OpenStatusFromRedisModifier {
    fn new(pool: RedisPool) -> OpenStatusFromRedisModifier {
        OpenStatusFromRedisModifier { pool }
    }
}

impl StatusModifier for OpenStatusFromRedisModifier {
    fn modify(&self, status: &mut Status) {
        let mut conn = self.pool.get().unwrap();
        let redis_state: RedisResult<String> = conn.get("state_open");
        if let Some(state) = &mut status.state {
            state.open = match redis_state {
                Ok(v) => Some(v == "open"),
                Err(_) => None,
            };
            state.lastchange = conn.get("state_lastchange").ok();
            state.trigger_person = conn.get("state_triggerperson").ok();
        }
    }
}

fn main() {
    env_logger::init();

    let status = StatusBuilder::mixed("Mittelab")
        .logo("https://www.mittelab.org/images/logo.svg")
        .url("https://www.mittelab.org")
        .location(Location {
            address: Some("Piazza Libertà 5/B, 34132 Trieste (TS), Italy".to_string()),
            osm_link: None,
            lat: 45.656_652_6,
            lon: 13.773_387_2,
            timezone: None,
        })
        .contact(Contact {
            email: Some("info@mittelab.org".to_string()),
            irc: Some("irc://irc.freenode.net/#mittelab".to_string()),
            twitter: Some("@mittelab".to_string()),
            facebook: Some("https://facebook.com/mittelab".to_string()),
            phone: Some("+390409776431".to_string()),
            issue_mail: Some("sysadmin@mittelab.org".to_string()),
            ..Default::default()
        })
        .add_issue_report_channel(IssueReportChannel::Matrix)
        .add_project("https://git.mittelab.org")
        .add_project("https://github.com/mittelab")
        .add_project("https://wiki.mittelab.org/progetti/")
        .state(State::default())
        .build()
        .expect("Creating status failed");

    // get redis client
    let client: redis::Client = redis::Client::open("redis://127.0.0.1/")
        .unwrap_or_else(|e| panic!("Error connecting to redis: {}", e));

    // create r2d2 pool
    let pool: r2d2::Pool<redis::Client> = r2d2::Pool::builder()
        .max_size(15)
        .build(client)
        .unwrap_or_else(|e| panic!("Error building redis pool: {}", e));

    // Set up server
    let server = SpaceapiServerBuilder::new(status)
        // .add_redis_pool(pool) waiting for
        .redis_pool(pool.clone())
        .add_status_modifier(OpenStatusFromRedisModifier::new(pool))
        .build()
        .expect("Could not initialize server");

    // Serve!
    server
        .serve("127.0.0.1:8000")
        .expect("Could not start the server");
}
