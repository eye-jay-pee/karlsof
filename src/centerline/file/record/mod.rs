use serde::{Deserialize, Serialize};

mod point_num;
pub use point_num::PointID;

mod kind;
pub use kind::CLRecordKind;

mod api;

#[derive(Debug, Serialize, Deserialize)]
pub struct CLRecord {
    pub id: PointID, // <- this needs to be changed to a String (i think).
    pub distance: f64,
    pub kind: CLRecordKind,
    pub northing: f64,
    pub easting: f64,
}

// # POINT STRUCT
//
// The fields of this struct match the position of their corisponding column in a carlson *.cl
// file. This is no accident. When they match, serialiZation and deserializaiton cen be derived.
// More efficent structs can be defined elsewhere. These are for packaging data for saving or
// unpacking it from a file.
//
// it would be nice to define something like this here:
// ``` snippet A
//  struct Point {
//      id: Option<i32>
//      x: f64,
//      y: f64,
//  }
// ```
// or
// ``` snippet B
//  struct Point {
//  id: PointID,
//      x: f64,
//      y: f64,
//  }
// ```
// ...... BUT .......
//
// this makes serialization more complicated. `CLRecord`, in it's current form, can be serialized
// and deserialized *IMPLICITLY*. This is *VERRRRY NICEEE*. Wa wa we wah. After all, this is
// `..centerline/file/..`, so the primary concern is centerlines interacting with files. If it
// makes serialization or deserializtion more complex, it does not belong in or under the
// `file` module. It happens that I will (or perhaps have, depending on when you are reading this),
// define a struct very similar to snippets A and B, but not here.
//
//
//
