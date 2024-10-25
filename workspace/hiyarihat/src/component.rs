use dioxus::prelude::*;
use js_sys::JsString;
use wasm_bindgen::JsValue;
use web_sys::console;

pub fn FailForm() -> Element {
    
    let submitter = move |event: FormEvent| {
        let form_data= event.data().values();
        let title = form_data.get("title").expect("Title Getting Error")
            .first().expect("Get Error");
        console::log_1(&js_sys::JsString::from(format!("{}:{:?}","結果",title)));
        // event.data
    };

    rsx!(
        form {
            onsubmit: submitter,
            margin: "auto",
            div {
                display: "flex",
                flex_direction: "column",
                label {
                    "タイトル"
                }
                input {
                    name: "title",
                    r#type: "text",
    
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                label {
                    "内容"
                }
                textarea {
                    name: "description"
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                label {
                    "補足情報"
                }
                textarea {
                    name: "optional_info"
                }
            }
            button {
                r"type": "submit",
                "すっきり！"
            }
        }
    )
}