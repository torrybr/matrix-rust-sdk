use ruma::{
    events::{beacon_info::BeaconInfoEventContent, location::LocationContent},
    MilliSecondsSinceUnixEpoch, OwnedUserId,
};
use tokio::sync::broadcast;

use crate::event_handler::EventHandlerHandle;

#[derive(Clone, Debug)]
pub struct LastLocation {
    /// The most recent location content of the user.
    pub location: LocationContent,
    /// The timestamp of when the location was updated.
    pub ts: MilliSecondsSinceUnixEpoch,
}

/// Details of a users live location share.
#[derive(Clone, Debug)]
pub struct LiveLocationShare {
    /// The user's last known location.
    pub last_location: LastLocation,
    /// Information about the associated beacon event.
    pub beacon_info: BeaconInfoEventContent,
    /// The user ID of the person sharing their live location.
    pub user_id: OwnedUserId,
}

/// A subscription to live location sharing events.
///
/// This struct holds the `EventHandlerHandle` and the
/// `Receiver<LiveLocationShare>` for live location shares.
#[derive(Debug)]
pub struct LiveLocationSubscription {
    /// Manages the event handler lifecycle.
    pub event_handler_handle: EventHandlerHandle,
    /// Receives live location shares.
    pub receiver: broadcast::Receiver<LiveLocationShare>,
}

impl Drop for LiveLocationSubscription {
    fn drop(&mut self) {}
}
