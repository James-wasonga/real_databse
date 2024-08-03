// @generated automatically by Diesel CLI.

diesel::table! {
    loans (id) {
        id -> Int4,
        user_id -> Int4,
        amount -> Nullable<Numeric>,
        interest_rate -> Nullable<Numeric>,
        status -> Nullable<Bool>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_types -> Nullable<Int4>,
        #[max_length = 150]
        name -> Nullable<Varchar>,
        #[max_length = 150]
        email -> Nullable<Varchar>,
        password -> Text,
        phone_number -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        is_blocked -> Nullable<Bool>,
        blocked_reason -> Nullable<Text>,
        address -> Nullable<Text>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    loans,
    users,
);
