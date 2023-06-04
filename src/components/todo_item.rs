use crate::{
    models::todo::Todo,
    server_functions::todo::{DeleteTodo, UpdateTodo},
};

use cfg_if::cfg_if;

use leptos::*;
use leptos_router::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection, SqlitePool};

        pub fn pool(cx: Scope) -> Result<SqlitePool, ServerFnError> {
          use_context::<SqlitePool>(cx)
            .ok_or("Pool missing.")
            .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))
        }

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            SqliteConnection::connect("sqlite:Todos.db").await.map_err(|e| ServerFnError::ServerError(e.to_string()))
        }
    }
}

#[component]
pub fn TodoItem(
    cx: Scope,
    todo_item: Todo,
    delete_callback: Action<DeleteTodo, Result<(), ServerFnError>>,
    update_callback: Action<UpdateTodo, Result<(), ServerFnError>>,
) -> impl IntoView {
    // Classes
    let span_class = move || format!("text-md {}", if todo_item.status { "line-through" } else { "" });
    let completed_button_class = move || {
        format!(
            "hover:cursor-pointer {}",
            if !todo_item.status {
                "opacity-100"
            } else {
                "opacity-50"
            }
        )
    };

    view! {
        cx,
        <div class="flex justify-between items-center">
            <span class=span_class>
                {todo_item.task}
            </span>
            <div class="flex justify-between w-fit sm:w-1/3">
                <ActionForm action=update_callback>
                    <input type="hidden" name="status" value={format!("{}", !todo_item.status)}/>
                    <input type="hidden" name="id" value={todo_item.id}/>
                    <input
                        type="submit"
                        value=move || if !todo_item.status { "Complete" } else { "Undo" }
                        class=completed_button_class
                    />
                </ActionForm>

                <ActionForm action=delete_callback>
                    <input type="hidden" name="id" value={todo_item.id}/>
                    <input
                        type="submit"
                        value="Delete"
                        class="hover:cursor-pointer ml-4 sm:ml-0"
                    />
                </ActionForm>
            </div>
        </div>
    }
}
