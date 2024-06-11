// FFI types for passing location from device to the sdk
// Shoutout to https://github.com/stadiamaps/ferrostar for the implementation

use serde::Serialize;
use std::time::SystemTime;

/// A geographic coordinate in WGS84.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, uniffi::Record)]
#[cfg_attr(test, derive(Serialize))]
pub struct GeographicCoordinate {
    pub lat: f64,
    pub lng: f64,
}

// /// The direction in which the user/device is observed to be traveling.
// #[derive(Clone, Copy, PartialEq, PartialOrd, Debug, uniffi::Record)]
// #[cfg_attr(test, derive(Serialize))]
// pub struct CourseOverGround {
//     /// The direction in which the user's device is traveling, measured in clockwise degrees from
//     /// true north (N = 0, E = 90, S = 180, W = 270).
//     pub degrees: u16,
//     /// The accuracy of the course value, measured in degrees.
//     pub accuracy: u16,
// }
//
// impl CourseOverGround {
//     pub fn new(degrees: u16, accuracy: u16) -> Self {
//         Self { degrees, accuracy }
//     }
// }

/// The location of the user
///
/// In addition to coordinates, this includes estimated accuracy and course information,
/// which can influence navigation logic and UI.
///
/// NOTE: Heading is absent on purpose.
/// Heading updates are not related to a change in the user's location.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, uniffi::Record)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserLocation {
    pub coordinates: GeographicCoordinate,
    /// The estimated accuracy of the coordinate (in meters)
    pub horizontal_accuracy: f64,
    // pub course_over_ground: Option<CourseOverGround>,
    pub timestamp: SystemTime,
}

impl UserLocation {
    /// Generates a geoUri string from the UserLocation struct
    pub fn to_geo_uri(&self) -> String {
        format!("geo:{},{}", self.coordinates.lat, self.coordinates.lng)
    }
}
