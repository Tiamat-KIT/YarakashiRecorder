use dioxus::prelude::*;
use js_sys::JsString;
use web_sys::console;
use crate::structs::Inputs;

#[component]
pub fn FailForm() -> Element {
    let mut input_list = use_signal(Vec::<Inputs>::new);
    let submitter = move |event: FormEvent| {
        let _ = match Inputs::new(&event.data.values()) {
            Ok(safes) => {
                console::log_1(&JsString::from(format!("{:?}",safes)));
                web_sys::window().unwrap().alert_with_message("すっきりしたね！ ^_^");
                input_list.push(safes);
            },
            Err(e) => {
                console::error_1(&JsString::from(format!("{:?}",e.errors())));
                web_sys::window().unwrap().alert_with_message("めんどくさいかもだけど、ちゃんと書いてね > < ");
            },
        };

    };
 
    rsx! {
        article {
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
            div {
                id: "append-space",
                display: "flex",
                justify_content: "center",
                align_items: "center",
                margin_top: "2rem",
                flex_direction: "column",
                for contents in input_list.read().clone().into_iter() {
                    
                    article {
                        h1 {
                            "{contents.title.unwrap()}"
                        }
                        p {
                            "{contents.description.unwrap()}"
                        }
                        p {
                            "{contents.optional_info.unwrap()}"
                        }
                    }
                }
            }
        }
    }
}

