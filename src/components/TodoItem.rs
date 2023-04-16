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
        let current_status = status();
        set_status.update(|val| *val = !current_status);
    };

    // Classes
    let span_class = move || format!("text-lg {}", if status() { "line-through" } else { "" });
    let completed_button_class = move || {
        format!(
            "px-3 py-2 rounded-md border-solid border-2 {}",
            if !status() {
                "border-[#7FB069]"
            } else {
                "border-[#FED766]"
            }
        )
    };

    view! {
        cx,
        <div class="flex flex-col justify-between items-center bg-white py-2 px-4 rounded-md sm:flex-row">
            <span class=span_class>
                {todo_item.task}
            </span>
            <div class="flex justify-between mt-5 w-full sm:w-fit sm:mt-0">
                <button on:click=on_click class=completed_button_class>{move || if !status() { "Complete" } else { "Undo" }}</button>
                <button on:click=move |_| delete_callback(todo_item.id) class="bg-[#D05353] px-3 py-2 rounded-md text-white sm:ml-2">"Delete"</button>
            </div>
        </div>
    }
}
