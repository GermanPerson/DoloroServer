// @generated automatically by Diesel CLI.

diesel::table! {
    pishock_devices (id) {
        id -> Integer,
        name -> Text,
        sharecode -> Text,
        max_intensity -> Integer,
        max_duration -> Integer,
    }
}
