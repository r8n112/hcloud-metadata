//! Integration tests using an in-memory transport; no network access.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use hcloud_metadata::{metadata, Error, HttpResponse, MetadataClient, Transport};

#[derive(Clone, Default)]
struct MockTransport {
    state: Rc<RefCell<State>>,
}

#[derive(Default)]
struct State {
    responses: VecDeque<HttpResponse>,
    urls: Vec<String>,
}

impl MockTransport {
    fn with_responses(responses: Vec<HttpResponse>) -> Self {
        let transport = Self::default();
        transport.state.borrow_mut().responses = responses.into();
        transport
    }

    fn urls(&self) -> Vec<String> {
        self.state.borrow().urls.clone()
    }
}

impl Transport for MockTransport {
    fn get(&self, url: &str) -> hcloud_metadata::Result<HttpResponse> {
        let mut state = self.state.borrow_mut();
        state.urls.push(url.to_owned());
        state
            .responses
            .pop_front()
            .ok_or_else(|| Error::Transport("no mock response queued".to_owned()))
    }
}

fn client_with(responses: Vec<HttpResponse>) -> (MetadataClient<MockTransport>, MockTransport) {
    let transport = MockTransport::with_responses(responses);
    let client = MetadataClient::with_transport("http://metadata.example", transport.clone());
    (client, transport)
}

#[test]
fn reads_full_metadata() {
    let body = r#"{"instance-id":"42","hostname":"srv","availability-zone":"fsn1-dc14",
                   "region":"fsn1","public-ipv4":"192.0.2.1","public-ipv6":"2001:db8::1",
                   "public-keys":["ssh-ed25519 AAAA"],"private-networks":[]}"#;
    let (client, transport) = client_with(vec![HttpResponse {
        status: 200,
        body: body.to_owned(),
    }]);

    let info = metadata::get(&client).unwrap();
    assert_eq!(info.instance_id.as_deref(), Some("42"));
    assert_eq!(info.availability_zone.as_deref(), Some("fsn1-dc14"));
    assert_eq!(info.public_keys.len(), 1);
    assert_eq!(
        transport.urls()[0],
        "http://metadata.example/hetzner/v1/metadata"
    );
}

#[test]
fn reads_a_single_field() {
    let (client, transport) = client_with(vec![HttpResponse {
        status: 200,
        body: "srv-1".to_owned(),
    }]);

    assert_eq!(metadata::field(&client, "hostname").unwrap(), "srv-1");
    assert_eq!(
        transport.urls()[0],
        "http://metadata.example/hetzner/v1/metadata/hostname"
    );
}

#[test]
fn reads_user_data() {
    let (client, transport) = client_with(vec![HttpResponse {
        status: 200,
        body: "#cloud-config\n".to_owned(),
    }]);

    assert_eq!(metadata::user_data(&client).unwrap(), "#cloud-config\n");
    assert_eq!(
        transport.urls()[0],
        "http://metadata.example/hetzner/v1/userdata"
    );
}

#[test]
fn maps_a_missing_key_to_not_found() {
    let (client, _transport) = client_with(vec![HttpResponse {
        status: 404,
        body: String::new(),
    }]);

    assert!(matches!(
        metadata::field(&client, "nope").unwrap_err(),
        Error::NotFound(_)
    ));
}

#[test]
fn normalises_a_trailing_slash_in_the_base_url() {
    let transport = MockTransport::with_responses(vec![HttpResponse {
        status: 200,
        body: "{}".to_owned(),
    }]);
    let client = MetadataClient::with_transport("http://metadata.example/", transport.clone());

    assert_eq!(client.base_url(), "http://metadata.example");
    let _info = metadata::get(&client).unwrap();
    assert_eq!(
        transport.urls()[0],
        "http://metadata.example/hetzner/v1/metadata"
    );
}
