use crate::{
    models::todo::Todo,
    server_functions::todo::{DeleteTodo, UpdateTodo},
};

use leptos::*;
use leptos_router::*;

#[component]
pub fn TodoItem(
    cx: Scope,
    todo_item: Todo,
    delete_callback: Action<DeleteTodo, Result<(), ServerFnError>>,
    update_callback: Action<UpdateTodo, Result<(), ServerFnError>>,
) -> impl IntoView {
    // Classes
    let span_class = move || {
        format!(
            "text-md {}",
            if todo_item.status { "line-through" } else { "" }
        )
    };
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

    view! { cx,
        <div class="flex justify-between items-center">
            <span class=span_class>{todo_item.task}</span>
            <div class="flex justify-between w-fit sm:w-1/3">
                <ActionForm action=update_callback>
                    <input type="hidden" name="status" value=format!("{}", ! todo_item.status)/>
                    <input type="hidden" name="id" value=todo_item.id/>
                    <input
                        type="submit"
                        value=move || if !todo_item.status { "Complete" } else { "Undo" }
                        class=completed_button_class
                    />
                </ActionForm>
                <ActionForm action=delete_callback>
                    <input type="hidden" name="id" value=todo_item.id/>
                    <input type="submit" value="Delete" class="hover:cursor-pointer ml-4 sm:ml-0"/>
                </ActionForm>
            </div>
        </div>
    }
}
