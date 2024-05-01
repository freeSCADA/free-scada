/*
 INFO: Database tables:
- communication_protocols,
- device_alarms,
- device_compatible_protocols,
- device_identifiers,
- device_state_transitions,
- device_states,
- device_supported_magnitudes,
- device_valid_alarms,
- device_valid_state_transitions,
- device_valid_states,
- fired_alarms_log,
- magnitudes,
- manufacturers,
- priority_levels,
- state_transition_execution_log,
- status_log
*/

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::magnitudes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Magnitude {
    pub id: i32,
    pub name: String,
    pub unit: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::device_identifiers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DeviceIdentifier {
    pub id: i32,
    pub name: String,
    pub description: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::communication_protocols)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct CommunicationProtocol {
    pub id: i32,
    pub name: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::device_alarms)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DeviceAlarm {
    pub id: i32,
    pub priority_level_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub probable_causes: Option<String>,
    pub probable_solutions: Option<String>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::device_states)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DeviceState {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::device_state_transitions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DeviceStateTransition {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub initial_state: i32,
    pub final_state: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::priority_levels)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct PriorityLevel {
    pub id: i32,
    pub label: String
}



