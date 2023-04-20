use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

use crate::components::page_wrapper::*;
use crate::components::todo_item::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // Signals
    let todo_task_input_ref: NodeRef<Input> = create_node_ref(cx);
    let (todo_items, set_todo_items) = create_signal(
        cx,
        vec![
            TodoItem {
                id: 0,
                task: String::from("Take out the trash"),
                status: false,
            },
            TodoItem {
                id: 1,
                task: String::from("Make the bed"),
                status: false,
            },
            TodoItem {
                id: 2,
                task: String::from("Mow the lawn"),
                status: true,
            },
            TodoItem {
                id: 3,
                task: String::from("Wash the dishes"),
                status: false,
            },
        ],
    );

    // Helpers
    let last_todo_id = move || todo_items().iter().map(|todo_item| todo_item.id).max();

    // Handlers
    let delete_todo_item = move |todo_id: u32| {
        set_todo_items
            .update(move |todo_items| todo_items.retain(|todo_item| todo_item.id != todo_id))
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let mut new_todo_items = todo_items();
        let todo_id = last_todo_id().unwrap_or_default() + 1;

        new_todo_items.push(TodoItem {
            id: todo_id,
            task: todo_task_input_ref().expect("<input> to exist").value(),
            status: false,
        });

        set_todo_items.set(new_todo_items);
    };

    view! { cx,
        <PageWrapper>
            <div id="add-task" class="flex flex-col rounded mb-20 text-black">
                <h2 class="text-2xl font-medium mb-4">"Add Task"</h2>
                <form class="w-full flex flex-col" on:submit=on_submit>
                    <div class="flex items-center justify-between">
                        <input
                            class="w-2/3 px-2 py-1 border-b-2 border-black focus:outline-none"
                            type="text"
                            placeholder="Add a new task"
                            node_ref=todo_task_input_ref
                        />
                        <input class="hover:cursor-pointer" type="submit" value="Submit" />
                    </div>
                </form>
            </div>

            <div id="tasks">
                <h2 class="text-2xl font-medium mb-4">"Tasks"</h2>
                <div class="flex flex-col gap-4">
                    <For
                        each={todo_items}
                        key=|item| item.id
                        view=move |cx, item: TodoItem| {
                          view! {
                            cx,
                                <TodoItem todo_item={item} delete_callback=delete_todo_item />
                          }
                        }
                    />
                </div>
            </div>
        </PageWrapper>
    }
}
