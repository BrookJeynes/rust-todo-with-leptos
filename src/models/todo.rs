use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
        pub struct Todo {
            pub id: u32,
            pub task: String,
            pub status: bool,
        }
    } else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Todo {
            pub id: u32,
            pub task: String,
            pub status: bool,
        }
    }
}
