use crate::components::page_wrapper::*;
use crate::components::todo_item::*;

use crate::models::todo::Todo;

use crate::server_functions::todo::get_todos;
use crate::server_functions::todo::AddTodo;
use crate::server_functions::todo::DeleteTodo;
use crate::server_functions::todo::UpdateTodo;

use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // Resources
    let add_todo = create_server_multi_action::<AddTodo>(cx);
    let delete_todo = create_server_action::<DeleteTodo>(cx);
    let update_todo = create_server_action::<UpdateTodo>(cx);

    let todos = create_resource(
        cx,
        move || {
            (
                add_todo.version().get(),
                delete_todo.version().get(),
                update_todo.version().get(),
            )
        },
        move |_| get_todos(cx),
    );

    view! { cx,
        <PageWrapper>
            <div id="add-task" class="flex flex-col rounded mb-20 text-black">
                <h2 class="text-2xl font-medium mb-4">"Add Task"</h2>
                <MultiActionForm
                    class="w-full flex flex-col"
                    on:submit=move |ev| {
                        AddTodo::from_event(&ev).expect("to parse form data");
                    }
                    action=add_todo
                >
                    <div class="flex items-center justify-between">
                        <input
                            class="w-2/3 px-2 py-1 border-b-2 border-black focus:outline-none"
                            type="text"
                            placeholder="Add a new task"
                            name="task"
                        />
                        <input class="hover:cursor-pointer" type="submit" value="Submit" />
                    </div>
                </MultiActionForm>
            </div>

            <div id="tasks">
                <h2 class="text-2xl font-medium mb-4">"Tasks"</h2>
                <div class="flex flex-col gap-4">
                    <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
                        {move ||
                            todos.read(cx).map(move |todos| match todos {
                                Ok(todos) => {
                                    view! {
                                        cx,
                                        <div>
                                            <For
                                                each={move || todos.clone()}
                                                key=|item| item.id
                                                view=move |cx, item: Todo| {
                                                  view! {
                                                    cx,
                                                        <TodoItem todo_item={item} delete_callback=delete_todo update_callback=update_todo />
                                                  }
                                                }
                                            />
                                        </div>
                                    }
                                }
                                Err(_) => view! {cx, <div />},
                            })
                        }
                    </Transition>
                </div>
            </div>
        </PageWrapper>
    }
}
