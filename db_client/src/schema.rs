// @generated automatically by Diesel CLI.

diesel::table! {
    communication_protocols (id) {
        id -> Integer,
        #[max_length = 256]
        name -> Varchar,
    }
}

diesel::table! {
    device_alarms (id) {
        id -> Integer,
        priority_level_id -> Integer,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 512]
        description -> Nullable<Varchar>,
        #[max_length = 512]
        probable_causes -> Nullable<Varchar>,
        #[max_length = 512]
        probable_solutions -> Nullable<Varchar>,
    }
}

diesel::table! {
    device_compatible_protocols (device_identifier_id, communication_protocol_id) {
        device_identifier_id -> Integer,
        communication_protocol_id -> Integer,
    }
}

diesel::table! {
    device_identifiers (id) {
        id -> Integer,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 512]
        description -> Varchar,
    }
}

diesel::table! {
    device_state_transitions (id) {
        id -> Integer,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 512]
        description -> Nullable<Varchar>,
        initial_state -> Integer,
        final_state -> Integer,
    }
}

diesel::table! {
    device_states (id) {
        id -> Integer,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 512]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    device_supported_magnitudes (device_identifier_id, magnitude_id) {
        device_identifier_id -> Integer,
        magnitude_id -> Integer,
    }
}

diesel::table! {
    device_valid_alarms (device_identifier_id, device_alarm_id) {
        device_identifier_id -> Integer,
        device_alarm_id -> Integer,
    }
}

diesel::table! {
    device_valid_state_transitions (device_identifier_id, device_state_transition_id) {
        device_identifier_id -> Integer,
        device_state_transition_id -> Integer,
    }
}

diesel::table! {
    device_valid_states (device_identifier_id, device_state_id) {
        device_identifier_id -> Integer,
        device_state_id -> Integer,
    }
}

diesel::table! {
    fired_alarms_log (valid_fired_alarm_id, timestamp) {
        valid_fired_alarm_id -> Integer,
        timestamp -> Datetime,
    }
}

diesel::table! {
    magnitudes (id) {
        id -> Integer,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 128]
        unit -> Varchar,
    }
}

diesel::table! {
    manufacturers (id) {
        id -> Integer,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 128]
        register_number -> Varchar,
    }
}

diesel::table! {
    priority_levels (id) {
        id -> Integer,
        #[max_length = 256]
        label -> Varchar,
    }
}

diesel::table! {
    state_transition_execution_log (valid_state_transition_id, timestamp) {
        valid_state_transition_id -> Integer,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 512]
        report_details -> Varchar,
        timestamp -> Datetime,
    }
}

diesel::table! {
    status_log (current_valid_status, timestamp) {
        current_valid_status -> Integer,
        timestamp -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    communication_protocols,
    device_alarms,
    device_compatible_protocols,
    device_identifiers,
    device_state_transitions,
    device_states,
    device_supported_magnitudes,
    device_valid_alarms,
    device_valid_state_transitions,
    device_valid_states,
    fired_alarms_log,
    magnitudes,
    manufacturers,
    priority_levels,
    state_transition_execution_log,
    status_log,
);
