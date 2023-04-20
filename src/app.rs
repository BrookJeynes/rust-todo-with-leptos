use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/todo-leptos.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home /> }/>
            </Routes>
        </Router>
    }
}
