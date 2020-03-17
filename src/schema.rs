table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        stock -> Float8,
        price -> Nullable<Int4>,
    }
}

table! {
    users (email) {
        email -> Varchar,
        password -> Varchar,
        company -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
