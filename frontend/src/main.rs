use reqwasm::http::Request;
use yew::prelude::*;

pub async fn make_cookie() {
    // Makes a request to the /api/cookie endpoint
    Request::get("/api/cookie").send().await.unwrap();
}

pub async fn delete_cookie() {
    // Makes a request to the /api/cookie endpoint
    Request::delete("/api/cookie").send().await.unwrap();
}

#[function_component]
/// A simple component that makes a request to the /api/cookie endpoint
/// with a button to trigger the request.
pub fn App() -> Html {
    let make_cookie = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            make_cookie().await;
        });
    });

    let delete_cookie = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            delete_cookie().await;
        });
    });

    html! {
        <div>
            <button onclick={make_cookie}>{"Make Cookie"}</button>
            <button onclick={delete_cookie}>{"Delete Cookie"}</button>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
