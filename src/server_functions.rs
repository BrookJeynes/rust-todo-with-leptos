pub mod todo;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::ServerFn;

        pub fn register_server_functions() {
            _ = self::todo::GetTodos::register();
            _ = self::todo::AddTodo::register();
            _ = self::todo::DeleteTodo::register();
            _ = self::todo::UpdateTodo::register();
        }
    }
}
