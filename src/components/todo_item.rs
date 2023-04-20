use leptos::*;

#[derive(Clone)]
pub struct TodoItem {
    pub id: u32,
    pub task: String,
    pub status: bool,
}

#[component]
pub fn TodoItem<F>(cx: Scope, todo_item: TodoItem, delete_callback: F) -> impl IntoView
where
    F: Fn(u32) + 'static,
{
    // Signals
    let (status, set_status) = create_signal(cx, todo_item.status);

    // Handlers
    let on_click = move |_| {
        set_status.update(|val| *val = !*val);
    };

    // Classes
    let span_class = move || format!("text-md {}", if status() { "line-through" } else { "" });
    let completed_button_class = move || {
        format!(
            "hover:cursor-pointer {}",
            if !status() {
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
                <button on:click=on_click class=completed_button_class>{move || if !status() { "Complete" } else { "Undo" }}</button>
                <button on:click=move |_| delete_callback(todo_item.id) class="hover:cusor-pointer ml-4 sm:ml-0">"Delete"</button>
            </div>
        </div>
    }
}
