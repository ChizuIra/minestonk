// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (user_id, item_id) {
        user_id -> Integer,
        item_id -> Integer,
        quantity -> Integer,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

diesel::joinable!(inventory -> items (item_id));
diesel::joinable!(inventory -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(inventory, items, users,);
