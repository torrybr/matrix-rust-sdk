use std::collections::HashMap;

use ruma::{
    events::{
        beacon::BeaconEventContent, beacon_info::BeaconInfoEventContent, location::LocationContent,
    },
    EventId, MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedUserId,
};

#[derive(Clone, Debug)]
pub struct BeaconState {
    pub(super) beacon_info_event_content: BeaconInfoEventContent,
    pub(super) last_location: Option<LocationContent>,
    pub(super) end_event_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    pub(super) user_id: OwnedUserId,
}

impl BeaconState {
    // pub(super) fn new(
    //     beacon_info_event_content: BeaconInfoEventContent,
    //     user_id: OwnedUserId,
    // ) -> Self {
    //     Self { beacon_info_event_content, last_location: None, end_event_timestamp: None, user_id }
    // }

    pub(super) fn update_beacon(&self, content: &BeaconEventContent) -> Self {
        let mut clone = self.clone();
        clone.last_location = Some(content.location.clone());
        clone
    }

    /// Get the last location of the beacon state (if any)
    pub fn last_location(&self) -> Option<LocationContent> {
        self.last_location.clone()
    }

    pub fn user_id(&self) -> OwnedUserId {
        self.user_id.clone()
    }
}

impl From<BeaconState> for BeaconEventContent {
    fn from(value: BeaconState) -> Self {
        unimplemented!("(tb): implement this")
    }
}

impl From<BeaconState> for BeaconInfoEventContent {
    fn from(value: BeaconState) -> Self {
        BeaconInfoEventContent::new(
            value.beacon_info_event_content.description.clone(),
            value.beacon_info_event_content.timeout.clone(),
            value.beacon_info_event_content.live.clone(),
            None,
        )
    }
}

/// Acts as a cache for poll response and poll end events handled before their
/// start event has been handled.
#[derive(Clone, Debug, Default)]
pub(super) struct BeaconPendingEvents {
    pub(super) pending_beacons: HashMap<OwnedEventId, BeaconEventContent>,
    // pub(super) pending_poll_ends: HashMap<OwnedEventId, MilliSecondsSinceUnixEpoch>,
}

impl BeaconPendingEvents {
    /// (tb) handle the newest location update by storing new beacon event content
    pub(super) fn apply(&mut self, start_event_id: &EventId, beacon_state: &mut BeaconState) {
        if let Some(newest_beacon) = self.pending_beacons.get_mut(start_event_id) {
            beacon_state.last_location = Some(newest_beacon.location.clone());
        }
    }
}
