use leptos::*;

use crate::components::PageWrapper::*;
use crate::components::TodoItem::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
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

    let delete_todo_item = move |todo_id: u32| {
        set_todo_items.update(|todo_items| todo_items.retain(|todo_item| todo_item.id != todo_id))
    };

    view! { cx,
        <PageWrapper>
            <h1 class="text-center text-4xl mb-6 font-medium text-white">"Tasks"</h1>

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
        </PageWrapper>
    }
}
