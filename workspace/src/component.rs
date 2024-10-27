use dioxus::prelude::*;
use js_sys::JsString;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlFormElement};

use crate::structs::Inputs;

#[component]
pub fn ResultView(prop: Vec<Inputs>) -> Element {
    let form_eval = eval(r#"
        let msg = await dioxus.recv();
        let pdf = document.getElementById(msg)
        html2pdf().set({
            margin: 0,
            filename: "失敗記録PDF.pdf",
            pagebreak		: { mode: 'legacy'},
			image			: {
				type		: 'png',
				quality		: 1
			},
			enableLinks		: true,
			html2canvas		: {
				dpi				: 90,
				scale			: 1,
				letterRendering	: true
			},
			jsPDF			: {
				unit			: 'mm',
				format			: 'a4',
				orientation		: 'portrait',
				x				: 0,
				y				: 0,
			}
        }).from(pdf).save()
    "#);

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
                    id: "pdf_elem",
                    article {
                        id: "pdf_content",
                        r"class": "card pdf_content",
                        margin_top: "5vh",
                        h1 {
                            id: "content-title",
                            "失敗談記録書"
                        }
                        div {
                            h3 {
                                "件名"
                            }
                            p {
                                "{first_content.title.unwrap()}"
                            }
                        }
                        
                        div {
                            display: "flex",
                            flex_direction: "column",
                            h3 {
                                "内容"
                            }
                            p {
                                id: "content-description",
                                "{first_content.description.unwrap()}"
                            }
                        }
                        div {
                            display: "flex",
                            flex_direction: "column",
                            h3 {
                                "補足情報"
                            }
                            p {
                                id: "content-opt-info",
                                "{first_content.optional_info.unwrap()}"
                            }
                        }
                    }
                    button {
                        onclick: move |event: MouseEvent| {
                        let title = element_in_text("content-title");
                        let descripton = element_in_text("content-description");
                        let opt_info = element_in_text("content-opt-info");
                        form_eval.send("pdf_content".into());
                    },
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
                get_window().alert_with_message("書いててえらいね ^ ^ ");
                input_list.push(safes);
            },
            Err(e) => {
                console::error_1(&JsString::from(format!("{:?}",e.errors())));
                get_window().alert_with_message("めんどくさいかもだけど、ちゃんと書いてね > < ");
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

