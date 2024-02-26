//! A simple example of making a request to the /api/cookie endpoint
extern crate serde_derive;
extern crate serde_qs;

use commons::OauthData;
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

    let state = web_sys::window().unwrap().location().href().unwrap();

    let query = OauthData {
        client_id: "our_muckup_server".to_string(),
        scope: "read".to_string(),
        state,
    };

    let query = serde_qs::to_string(&query).unwrap();

    let oauth_url = format!("http://localhost:9999/oauth/authorize?{}", query);

    html! {
        <ul>
            <li><button onclick={make_cookie}>{"Make Cookie"}</button></li>
            <li><button onclick={delete_cookie}>{"Delete Cookie"}</button></li>
            <li><a href={oauth_url}>{"Authorize"}</a></li>
            <li><a href="/api/logout">{"Logout"}</a></li>
        </ul>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
