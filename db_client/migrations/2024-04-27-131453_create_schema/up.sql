-- Your SQL goes here
CREATE TABLE `device_valid_states`(
	`device_identifier_id` INTEGER NOT NULL,
	`device_state_id` INTEGER NOT NULL,
	PRIMARY KEY(`device_identifier_id`, `device_state_id`)
);

CREATE TABLE `device_compatible_protocols`(
	`device_identifier_id` INTEGER NOT NULL,
	`communication_protocol_id` INTEGER NOT NULL,
	PRIMARY KEY(`device_identifier_id`, `communication_protocol_id`)
);

CREATE TABLE `device_state_transitions`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` VARCHAR(256) NOT NULL,
	`description` VARCHAR(512),
	`initial_state` INTEGER NOT NULL,
	`final_state` INTEGER NOT NULL
);

CREATE TABLE `state_transition_execution_log`(
	`valid_state_transition_id` INTEGER NOT NULL,
	`name` VARCHAR(256) NOT NULL,
	`report_details` VARCHAR(512) NOT NULL,
	`timestamp` DATETIME NOT NULL,
	PRIMARY KEY(`valid_state_transition_id`, `timestamp`)
);

CREATE TABLE `priority_levels`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`label` VARCHAR(256) NOT NULL
);

CREATE TABLE `device_alarms`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`priority_level_id` INTEGER NOT NULL,
	`name` VARCHAR(256) NOT NULL,
	`description` VARCHAR(512),
	`probable_causes` VARCHAR(512),
	`probable_solutions` VARCHAR(512)
);

CREATE TABLE `device_supported_magnitudes`(
	`device_identifier_id` INTEGER NOT NULL,
	`magnitude_id` INTEGER NOT NULL,
	PRIMARY KEY(`device_identifier_id`, `magnitude_id`)
);

CREATE TABLE `device_valid_state_transitions`(
	`device_identifier_id` INTEGER NOT NULL,
	`device_state_transition_id` INTEGER NOT NULL,
	PRIMARY KEY(`device_identifier_id`, `device_state_transition_id`)
);

CREATE TABLE `status_log`(
	`current_valid_status` INTEGER NOT NULL,
	`timestamp` DATETIME NOT NULL,
	PRIMARY KEY(`current_valid_status`, `timestamp`)
);

CREATE TABLE `manufacturers`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` VARCHAR(256) NOT NULL,
	`register_number` VARCHAR(128) NOT NULL
);

CREATE TABLE `device_states`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` VARCHAR(256) NOT NULL,
	`description` VARCHAR(512)
);

CREATE TABLE `device_valid_alarms`(
	`device_identifier_id` INTEGER NOT NULL,
	`device_alarm_id` INTEGER NOT NULL,
	PRIMARY KEY(`device_identifier_id`, `device_alarm_id`)
);

CREATE TABLE `communication_protocols`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` VARCHAR(256) NOT NULL
);

CREATE TABLE `magnitudes`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` VARCHAR(256) NOT NULL,
	`unit` VARCHAR(128) NOT NULL
);

CREATE TABLE `device_identifiers`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` VARCHAR(256) NOT NULL,
	`description` VARCHAR(512) NOT NULL
);

CREATE TABLE `fired_alarms_log`(
	`valid_fired_alarm_id` INTEGER NOT NULL,
	`timestamp` DATETIME NOT NULL,
	PRIMARY KEY(`valid_fired_alarm_id`, `timestamp`)
);

