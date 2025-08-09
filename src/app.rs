use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local, *};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use crate::models::{HttpRequest, HttpResponse};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (url, set_url) = signal(String::new());
    let (method, set_method) = signal("GET".to_string());
    let (body, set_body) = signal(String::new());
    let (response, set_response) = signal::<Option<HttpResponse>>(None);
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let send_click = move |_| {
        let req = HttpRequest {
            url: url.get().to_string(),
            method: method.get().to_string(),
            headers: vec![],
            body: Some(body.get().to_string()),
        };

        spawn_local(async move {
            let res = invoke_tauri("send_request", req).await;
            if let Ok(resp) = res {
                set_response.set(Some(resp));
            }
        });
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="container">
            <div>
                <select
                    class="border p-2 mt-2"
                    on:input=move |ev| set_method.set(event_target_value(&ev))
                >
                    <option value="GET">"GET"</option>
                    <option value="POST">"POST"</option>
                    <option value="PUT">"PUT"</option>
                    <option value="DELETE">"DELETE"</option>
                </select>

                <input
                    class="border p-2 w-full"
                    placeholder="Enter URL"
                    on:input=move |ev| set_url.set(event_target_value(&ev))
                />
                <button
                    class="bg-blue-500 text-white px-4 py-2 mt-2"
                    on:click=send_click
                >
                    "Send"
                </button>
                {move || response.get().map(|resp| view!{ cx,
                    <div class="mt-4 border p-2">
                        <div>"Status: " {resp.status}</div>
                        <div class="mt-2 font-bold">"Headers:"</div>
                        <ul>
                            {resp.headers.into_iter().map(|(k,v)| view!{ cx,
                                <li>{k}": "{v}</li>
                            }).collect_view()}
                        </ul>
                        <div class="mt-2 font-bold">"Body:"</div>
                        <pre class="whitespace-pre-wrap">{resp.body}</pre>
                    </div>
                })}
            </div>
            <textarea
                class="border p-2 w-full mt-2 h-32"
                placeholder="Request body (JSON)"
                on:input=move |ev| set_body.set(event_target_value(&ev))
            ></textarea>
        </main>
    }
}

async fn invoke_tauri(cmd: &str, payload: HttpRequest) -> Result<HttpResponse, String> {
    let val = to_value(&payload).unwrap();
    let js_val = invoke(cmd, val).await;
    from_value(js_val).map_err(|e| e.to_string())
}
