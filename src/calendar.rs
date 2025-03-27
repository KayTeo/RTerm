use google_calendar3::{CalendarHub, oauth2, Event as GoogleEvent};
use hyper;
use hyper_tls;

pub struct Event {
    pub summary: String,
    pub start_time: String,
}

impl Event {
    pub fn from_google_event(event: GoogleEvent) -> Self {
        Self {
            summary: event.summary.unwrap_or_default(),
            start_time: event.start
                .and_then(|s| s.date_time)
                .map(|dt| dt.to_string())
                .unwrap_or_else(|| "No date".to_string()),
        }
    }

    pub fn format_for_display(&self) -> Spans {
        Spans::from(vec![
            Span::raw(format!("{}: ", self.start_time)),
            Span::raw(&self.summary),
        ])
    }
}

pub struct CalendarClient {
    hub: CalendarHub<hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>>,
}

impl CalendarClient {
    pub fn new(token: oauth2::AccessToken) -> Self {
        let hub = CalendarHub::new(
            hyper::Client::builder().build(hyper_tls::HttpsConnector::new()),
            token,
        );
        Self { hub }
    }

    pub async fn fetch_events(&self) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let result = self.hub
            .events()
            .list("primary")
            .max_results(10)
            .order_by("startTime")
            .single_events(true)
            .execute()
            .await?;

        Ok(result
            .items
            .unwrap_or_default()
            .into_iter()
            .map(Event::from_google_event)
            .collect())
    }
}
