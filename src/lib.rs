use std::collections::HashMap;

use exports::edgee::components::data_collection::*;
use payload_builder::{IsComplete, IsUnset, SetProps, SetReferrer, SetUrl, State};

wit_bindgen::generate!({
    path: ".edgee/wit",
    world: "data-collection",
    generate_all,
});

macro_rules! dict {
    {
        $($key:expr => $value:expr),*$(,)?
    } => {
        vec![
            $(($key.to_string(), $value.to_string()),)*
        ]
    }
}

struct Component;
export!(Component);

impl Guest for Component {
    fn page(event: Event, settings: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::try_from(settings)?;

        let Data::Page(page) = event.data else {
            return Err("Not a page event!".to_string());
        };

        Ok(Payload::builder()
            .name("pageview")
            .url(page.url)
            .referrer(page.referrer)
            .convert_props(page.properties)
            .build_request(settings))
    }

    fn user(event: Event, settings: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::try_from(settings)?;

        let Data::User(user) = event.data else {
            return Err("Not an user event!".to_string());
        };

        Ok(Payload::builder()
            .name("user")
            .import_context(&event.context)
            .convert_props(user.properties)
            .build_request(settings))
    }

    fn track(event: Event, settings: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::try_from(settings)?;

        let Data::Track(track) = event.data else {
            return Err("Not a track event!".to_string());
        };

        Ok(Payload::builder()
            .name(track.name)
            .import_context(&event.context)
            .convert_props(track.properties)
            .build_request(settings))
    }
}

#[derive(Debug, serde::Serialize, bon::Builder)]
#[builder(on(String, into))]
struct Payload {
    name: String,
    #[builder(default)]
    domain: String,
    url: String,
    referrer: String,
    props: HashMap<String, String>,
}

impl<S: State> PayloadBuilder<S> {
    fn import_context(self, context: &Context) -> PayloadBuilder<SetReferrer<SetUrl<S>>>
    where
        S::Url: IsUnset,
        S::Referrer: IsUnset,
    {
        self.url(&context.page.url).referrer(&context.page.referrer)
    }

    fn convert_props(self, props: Dict) -> PayloadBuilder<SetProps<S>>
    where
        S::Props: IsUnset,
    {
        self.props(props.into_iter().collect())
    }

    fn build_request(self, settings: Settings) -> EdgeeRequest
    where
        S: IsComplete,
        S::Domain: IsUnset,
    {
        let payload = self.domain(settings.domain).build();
        let body = serde_json::to_string(&payload).expect("A valid payload");

        EdgeeRequest {
            method: HttpMethod::Post,
            url: format!("{}/api/event", settings.instance_url),
            headers: dict! {
                "Content-Type" => "application/json",
            },
            body,
            forward_client_headers: true,
        }
    }
}

struct Settings {
    instance_url: String,
    domain: String,
}

impl TryFrom<Dict> for Settings {
    type Error = String;

    fn try_from(value: Dict) -> Result<Self, Self::Error> {
        let dict: HashMap<String, String> = value.into_iter().collect();

        macro_rules! dict_get {
            ($dict:expr, $key:ident) => {
                $dict
                    .get(stringify!($key))
                    .filter(|s| !s.is_empty())
                    .cloned()
                    .ok_or_else(|| format!("Key not found in settings: {}", stringify!($key)))?
            };
            ($dict:expr, $key:ident, $default:expr) => {
                $dict
                    .get(stringify!($key))
                    .filter(|s| !s.is_empty())
                    .cloned()
                    .unwrap_or_else(|| $default.into())
            };
        }

        Ok(Settings {
            instance_url: dict_get!(dict, instance_url, "https://plausible.io"),
            domain: dict_get!(dict, domain),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::exports::edgee::components::data_collection::{
        Campaign, Client, Context, Data, EventType, PageData, Session, UserData,
    };
    use exports::edgee::components::data_collection::Consent;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    fn sample_user_data(edgee_id: String) -> UserData {
        UserData {
            user_id: "123".to_string(),
            anonymous_id: "456".to_string(),
            edgee_id,
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
            ],
        }
    }

    fn sample_context(edgee_id: String, locale: String, session_start: bool) -> Context {
        Context {
            page: sample_page_data(),
            user: sample_user_data(edgee_id),
            client: Client {
                city: "Paris".to_string(),
                ip: "192.168.0.1".to_string(),
                locale,
                timezone: "CET".to_string(),
                user_agent: "Chrome".to_string(),
                user_agent_architecture: "fuck knows".to_string(),
                user_agent_bitness: "64".to_string(),
                user_agent_full_version_list: "abc".to_string(),
                user_agent_version_list: "abc".to_string(),
                user_agent_mobile: "mobile".to_string(),
                user_agent_model: "don't know".to_string(),
                os_name: "MacOS".to_string(),
                os_version: "latest".to_string(),
                screen_width: 1024,
                screen_height: 768,
                screen_density: 2.0,
                continent: "Europe".to_string(),
                country_code: "FR".to_string(),
                country_name: "France".to_string(),
                region: "West Europe".to_string(),
            },
            campaign: Campaign {
                name: "random".to_string(),
                source: "random".to_string(),
                medium: "random".to_string(),
                term: "random".to_string(),
                content: "random".to_string(),
                creative_format: "random".to_string(),
                marketing_tactic: "random".to_string(),
            },
            session: Session {
                session_id: "random".to_string(),
                previous_session_id: "random".to_string(),
                session_count: 2,
                session_start,
                first_seen: 123,
                last_seen: 123,
            },
        }
    }

    fn sample_page_data() -> PageData {
        PageData {
            name: "page name".to_string(),
            category: "category".to_string(),
            keywords: vec!["value1".to_string(), "value2".into()],
            title: "page title".to_string(),
            url: "https://example.com/full-url?test=1".to_string(),
            path: "/full-path".to_string(),
            search: "?test=1".to_string(),
            referrer: "https://example.com/another-page".to_string(),
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ],
        }
    }

    fn sample_page_event(
        consent: Option<Consent>,
        edgee_id: String,
        locale: String,
        session_start: bool,
    ) -> Event {
        Event {
            uuid: Uuid::new_v4().to_string(),
            timestamp: 123,
            timestamp_millis: 123,
            timestamp_micros: 123,
            event_type: EventType::Page,
            data: Data::Page(sample_page_data()),
            context: sample_context(edgee_id, locale, session_start),
            consent,
        }
    }

    #[test]
    fn page_works_fine() {
        let event = sample_page_event(
            Some(Consent::Granted),
            "abc".to_string(),
            "fr".to_string(),
            true,
        );
        let settings = vec![
            (
                "instance_url".to_string(),
                "https://plausible.io".to_string(),
            ),
            ("domain".to_string(), "edgee.cloud".to_string()),
        ];
        let result = Component::page(event, settings);

        assert_eq!(result.is_err(), false);
        let edgee_request = result.unwrap();
        assert_eq!(edgee_request.method, HttpMethod::Post);
        assert_eq!(edgee_request.body.is_empty(), false);
        assert_eq!(edgee_request.url.starts_with("https://plausible.io"), true);
    }

    #[test]
    fn page_works_fine_without_instance_url() {
        let event = sample_page_event(
            Some(Consent::Granted),
            "abc".to_string(),
            "fr".to_string(),
            true,
        );
        let settings = vec![
            ("instance_url".to_string(), String::new()),
            ("domain".to_string(), "edgee.cloud".to_string()),
        ];
        let result = Component::page(event, settings);

        assert_eq!(result.is_err(), false);
        let edgee_request = result.unwrap();
        assert_eq!(edgee_request.method, HttpMethod::Post);
        assert_eq!(edgee_request.body.is_empty(), false);
        assert_eq!(edgee_request.url.starts_with("https://plausible.io"), true);
    }
}
