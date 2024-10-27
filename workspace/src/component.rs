use dioxus::prelude::*;
use js_sys::JsString;
use wasm_bindgen::JsCast;
use web_sys::{console, window};

use crate::structs::Inputs;

#[component]
pub fn ResultView(prop: Vec<Inputs>) -> Element {
    let form_eval = eval(r#"
        let msg = await dioxus.recv();
        let pdf = document.getElementById(msg)
        html2pdf(pdf).save("sample.pdf")
    "#);
    let to_pdf = move |event: MouseEvent| {
        let document = window().unwrap().document().unwrap();
        let title = document.get_element_by_id("content-title").unwrap().text_content().unwrap();
        let descripton = document.get_element_by_id("content-description").unwrap().text_content().unwrap();
        let opt_info = document.get_element_by_id("content-opt-info").unwrap().text_content().unwrap();
        form_eval.send("pdf_elem".into());
    };
    
    match prop.len() {
        0 => rsx!{ article {
            h1 {
                "No Content" 
            }
        }},
        _ => {
            let first_content =  prop.pop().unwrap().clone();
            return rsx!{
                div {
                    article {
                        id: "pdf_elem",
                        r"class": "card",
                        margin_top: "5vh",
                        h1 {
                            id: "content-title",
                            "{first_content.title.unwrap()}"
                        }
                        p {
                            id: "content-description",
                            "{first_content.description.unwrap()}"
                        }
                        p {
                            id: "content-opt-info",
                            "{first_content.optional_info.unwrap()}"
                        }
                    }
                    button {
                        onclick: to_pdf,
                        "PDF化する!"
                    }
                }
            }
        }
    }

}


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


        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("form")
            .unwrap()
            .dyn_into::<web_sys::HtmlFormElement>()
            .unwrap()
            .reset();
    };

    rsx! {
        article {
            script {
                src: "https://cdnjs.cloudflare.com/ajax/libs/html2pdf.js/0.10.1/html2pdf.bundle.min.js",
                integrity: "sha512-GsLlZN/3F2ErC5ifS5QtgpiJtWd43JWSuIgh7mbzZ8zBps+dvLusV+eNQATqgA/HdeKFVgA5v3S/cIrLF7QnIg==",
                crossorigin: "anonymous",
            }
            form {
                onsubmit: submitter,
                margin: "auto",
                id: "form",
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
            ResultView {
                prop: input_list.read().clone()
            }
        }
    }
}

