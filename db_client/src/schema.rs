
diesel::table! {
    manufacturers (id) {
        id -> Integer,
        #[max_length = 256]
        name -> VarChar,
        #[max_length = 128]
        register_number -> VarChar,
    }
}

diesel::table! {
    devices (id) {
        id -> Integer,
        #[max_length = 256]
        name -> VarChar,
        #[max_length = 512]
        description -> VarChar,
    }
}

diesel::table! {
    communication_protocols (id) {
        id -> Integer,
        #[max_length = 256]
        name -> VarChar,
    }
}

diesel::table! {
    magnitudes (id) {
        id -> Integer,
        #[max_length = 256]
        name -> VarChar,
        #[max_length = 128]
        unit -> VarChar,
    }
}

diesel::table! {
    states (id) {
        id -> Integer,
        #[max_length = 256]
        name -> VarChar,
        #[max_length = 512]
        description -> Nullable<VarChar>,
    }
}

diesel::table! {
    priority_levels (id) {
        id -> Integer,
        #[max_length = 256]
        label -> VarChar,
    }
}

diesel::table! {
    alarms (id) {
        id -> Integer,
        priority_level_id -> Integer,
        #[max_length = 256]
        name -> VarChar,
        #[max_length = 512]
        description -> Nullable<VarChar>,
        #[max_length = 512]
        probable_causes -> Nullable<VarChar>,
        #[max_length = 512]
        probable_solutions -> Nullable<VarChar>,
    }
}

diesel::table! {
    device_state_transitions (id) {
        id -> Integer,
        #[max_length = 256]
        name -> VarChar,
        #[max_length = 512]
        description -> Nullable<VarChar>,
        initial_state -> Integer,
        final_state -> Integer
    }
}

diesel::table! {
    device_compatible_protocols (id) {
        id -> Integer,
    }
}
