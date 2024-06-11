use crate::timeline::beacons::BeaconState;
use crate::timeline::tests::beacon::fakes::create_beacon_info;
use crate::timeline::tests::TestTimeline;
use crate::timeline::{EventTimelineItem, TimelineItemContent};
use matrix_sdk_test::{async_test, ALICE, BOB};
use ruma::events::room::beacon_info::BeaconInfoEventContent;
use ruma::{server_name, EventId, OwnedEventId, UserId};

use ruma::events::beacon::BeaconEventContent;

use ruma::events::location::LocationContent;
use ruma::events::AnyMessageLikeEventContent;
#[async_test]
async fn beacon_info_is_correctly_processed_in_timeline() {
    let timeline = TestTimeline::new();
    let alice_beacon = create_beacon_info("Alice's Live location", 2300);

    timeline.send_beacon_info(&ALICE, alice_beacon).await;
    let beacon_state = timeline.beacon_state().await;

    assert_beacon_info(
        &beacon_state.beacon_info_event_content,
        &create_beacon_info("Alice's Live location", 2300),
    );
    assert!(beacon_state.last_location.is_none());
}

#[async_test]
async fn beacon_updates_location() {
    let geo_uri = "geo:51.5008,0.1247;u=35".to_string();

    let timeline = TestTimeline::new();
    timeline.send_beacon_info(&ALICE, create_beacon_info("Alice's Live location", 2300)).await;
    let beacon_info_id = timeline.beacon_info_event().await.event_id().unwrap().to_owned();

    // Alice sends her location beacon
    timeline.send_beacon(&ALICE, &beacon_info_id, geo_uri).await;
    let beacon_state = timeline.beacon_state().await;

    //assert last_location is populated
    println!("{:#?}", beacon_state);
    assert!(beacon_state.last_location.is_some());
    assert_eq!(beacon_state.beacon_info_event_content.is_live(), true);
}

#[async_test]
async fn beacon_updates_location_with_multiple_beacons() {
    let geo_uri = "geo:51.5008,0.1247;u=35";
    let geo_uri2 = "geo:51.5009,0.1248;u=36";

    let timeline = TestTimeline::new();
    timeline.send_beacon_info(&ALICE, create_beacon_info("Alice's Live location", 2300)).await;
    let beacon_info_id = timeline.beacon_info_event().await.event_id().unwrap().to_owned();

    // Alice sends her location beacon
    timeline.send_beacon(&ALICE, &beacon_info_id, geo_uri.to_string()).await;
    let beacon_state = timeline.beacon_state().await;

    //assert last_location is populated
    assert!(beacon_state.last_location.is_some());
    assert_eq!(beacon_state.last_location.unwrap().uri, geo_uri);
    assert_beacon_info(
        &beacon_state.beacon_info_event_content,
        &create_beacon_info("Alice's Live location", 1),
    );
    assert_eq!(beacon_state.beacon_info_event_content.is_live(), true);

    timeline.send_beacon(&ALICE, &beacon_info_id, geo_uri2.to_string()).await;
    let beacon_state = timeline.beacon_state().await;

    //assert last_location is populated
    assert!(beacon_state.last_location.is_some());
    assert_eq!(beacon_state.last_location.unwrap().uri, geo_uri2);
    assert_beacon_info(
        &beacon_state.beacon_info_event_content,
        &create_beacon_info("Alice's Live location", 2300),
    );
    assert_eq!(beacon_state.beacon_info_event_content.is_live(), true);
}

#[async_test]
async fn multiple_people_sharing_location() {
    let geo_uri = "geo:51.5008,0.1247;u=35";
    let geo_uri2 = "geo:51.5009,0.1248;u=36";

    let timeline = TestTimeline::new();

    //Alice starts sharing her location
    timeline.send_beacon_info(&ALICE, create_beacon_info("Alice's Live location", 2300)).await;

    //Bob starts sharing his location
    timeline.send_beacon_info(&BOB, create_beacon_info("Bob's Live location", 2300)).await;

    let alice_beacon_info_event_id =
        timeline.event_items().await[0].clone().event_id().unwrap().to_owned();

    let bob_beacon_info_event_id =
        timeline.event_items().await[1].clone().event_id().unwrap().to_owned();

    // Alice sends her location beacon
    timeline.send_beacon(&ALICE, &alice_beacon_info_event_id, geo_uri.to_string()).await;
    let alice_beacon_state = timeline.event_items().await[0].clone().beacon_state();
    assert!(alice_beacon_state.last_location.is_some());
    assert_beacon_info(
        &alice_beacon_state.beacon_info_event_content,
        &create_beacon_info("Alice's Live location", 1),
    );
    assert_beacon(
        &alice_beacon_state.last_location.as_ref().unwrap(),
        &LocationContent::new(geo_uri.to_string()),
    );
    assert_eq!(alice_beacon_state.beacon_info_event_content.is_live(), true);

    //Bobs sends his location beacon
    timeline.send_beacon(&BOB, &bob_beacon_info_event_id, geo_uri2.to_string()).await;
    let bobs_beacon_state = timeline.event_items().await[1].clone().beacon_state();
    assert!(bobs_beacon_state.last_location.is_some());
    assert_beacon_info(
        &bobs_beacon_state.beacon_info_event_content,
        &create_beacon_info("Bob's Live location", 1),
    );
    assert_beacon(
        &bobs_beacon_state.last_location.as_ref().unwrap(),
        &LocationContent::new(geo_uri2.to_string()),
    );
    assert_eq!(bobs_beacon_state.beacon_info_event_content.is_live(), true);
}

#[async_test]
async fn beacon_info_is_stopped_by_user() {
    let timeline = TestTimeline::new();

    timeline.send_beacon_info(&ALICE, create_beacon_info("Alice's Live location", 2300)).await;
    let beacon_info_id = timeline.beacon_info_event().await.event_id().unwrap().to_owned();

    // Alice sends her location beacon
    timeline.send_beacon(&ALICE, &beacon_info_id, "geo:51.5008,0.1247;u=35".to_string()).await;
    let mut beacon_state = timeline.beacon_state().await;

    // Alice stops sharing her location
    beacon_state.beacon_info_event_content.stop();

    assert_eq!(&beacon_state.beacon_info_event_content.is_live(), &false);
}

#[async_test]
async fn beacon_info_is_stopped_by_timeout() {
    let timeline = TestTimeline::new();

    timeline.send_beacon_info(&ALICE, create_beacon_info("Alice's Live location", 0)).await;
    let beacon_info_id = timeline.beacon_info_event().await.event_id().unwrap().to_owned();

    // Alice sends her location beacon
    timeline.send_beacon(&ALICE, &beacon_info_id, "geo:51.5008,0.1247;u=35".to_string()).await;
    let beacon_state = timeline.beacon_state().await;

    assert_eq!(beacon_state.beacon_info_event_content.is_live(), false);
}

#[async_test]
async fn events_received_before_start_are_not_lost() {
    let timeline = TestTimeline::new();

    let beacon_info_id: OwnedEventId = EventId::new(server_name!("dummy.server"));

    // Alice sends her live location beacon
    timeline.send_beacon(&ALICE, &beacon_info_id, "geo:51.5008,0.1247;u=35".to_string()).await;

    // Alice starts her live location share
    timeline.send_beacon_info(&ALICE, create_beacon_info("Alice's Live location", 2300)).await;

    let beacon_state = timeline.beacon_state().await;

    assert_beacon_info(
        &beacon_state.beacon_info_event_content,
        &create_beacon_info("Alice's Live location", 2300),
    );
}

fn assert_beacon_info(a: &BeaconInfoEventContent, b: &BeaconInfoEventContent) {
    assert_eq!(a.description, b.description);
    assert_eq!(a.live, b.live);
    assert_eq!(a.timeout, b.timeout);
    assert_eq!(a.asset, b.asset);
}

fn assert_beacon(a: &LocationContent, b: &LocationContent) {
    assert_eq!(a.uri, b.uri);
}

impl TestTimeline {
    async fn send_beacon_info(&self, user: &UserId, content: BeaconInfoEventContent) {
        // Send a beacon info state event to the room

        let event =
            self.event_builder.make_sync_state_event(user, "@example:localhost", content, None);
        self.handle_live_event(event).await;
    }

    async fn send_beacon(&self, user: &UserId, event_id: &EventId, geo_uri: String) {
        let owner = OwnedEventId::from(event_id);

        let beacon = BeaconEventContent::new(owner.clone(), geo_uri);

        let event_content = AnyMessageLikeEventContent::Beacon(beacon.clone());

        self.handle_live_message_event(user, event_content).await;
    }

    async fn beacon_state(&self) -> BeaconState {
        self.event_items().await[0].clone().beacon_state()
    }
    async fn beacon_info_event(&self) -> EventTimelineItem {
        self.event_items().await[0].clone()
    }
}

impl EventTimelineItem {
    fn beacon_state(self) -> BeaconState {
        match self.content() {
            TimelineItemContent::TestBeaconModel(beacon_state) => beacon_state.clone(),
            _ => panic!("Not a beacon state"),
        }
    }
}

mod fakes {
    use ruma::events::room::beacon_info::BeaconInfoEventContent;
    use std::time::Duration;

    pub fn create_beacon_info(desc: &str, duration: u64) -> BeaconInfoEventContent {
        BeaconInfoEventContent::new(
            Option::from(desc.to_string()),
            Duration::from_millis(duration),
            true,
            None,
        )
    }
}
