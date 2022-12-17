use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlTextAreaElement};
use yew::prelude::*;
pub mod message;
pub mod response;
use crate::message::Msg;
use crate::response::{Res, SpecificResponse};
pub struct Main {
    code: String,
    ss: SyntaxSet,
    theme: Theme,
    syntax: SyntaxReference,
    html: String,
    div: Element,
    code_response: Res,
}
impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let code = "\n".repeat(20);
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = ts.themes["base16-ocean.dark"].clone();
        let syntax = ss.find_syntax_by_extension("rs").unwrap().to_owned();
        let html = highlighted_html_for_string(&code, &ss, &syntax, &theme).expect("Can't parse");
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&html);
        Self {
            code,
            ss,
            theme,
            syntax,
            html,
            div,
            code_response: Res {
                errors: SpecificResponse {
                    compile: String::from(""),
                    run: String::from(""),
                },
                outputs: SpecificResponse {
                    compile: String::from(""),
                    run: String::from(""),
                },
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetContent(content) => {
                self.code = content + "\n";
                self.html =
                    highlighted_html_for_string(&self.code, &self.ss, &self.syntax, &self.theme)
                        .expect("Can't parse");
                self.div.set_inner_html(&self.html);
                true
            }
            Msg::SendCode => {
                let code = self.code.clone();
                _ctx.link().send_message(Msg::SetResponse(Res {
                    errors: SpecificResponse {
                        compile: String::from(""),
                        run: String::from(""),
                    },
                    outputs: SpecificResponse {
                        compile: String::from(""),
                        run: String::from("Compiling/Running..."),
                    },
                }));
                _ctx.link().send_future(async move {
                    let client = reqwest::Client::new();
                    let res = client
                        .post("http://127.0.0.1:8000/code")
                        .header("Content-Type", "application/json")
                        .body(format!(
                            "{{\"code\": \"{}\"}}",
                            code.clone().replace('\"', "\\\"").replace('\n', "")
                        ))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    let res_json: Res =
                        serde_json::from_str(&res).expect("Failed to parse response");
                    Msg::SetResponse(res_json)
                });
                true
            }
            Msg::SetResponse(res) => {
                self.code_response = res;
                true
            }
        }
    }

    fn rendered(&mut self, _: &Context<Self>, first_render: bool) {
        if first_render {
            window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("editor"))
                .unwrap()
                .set_inner_html(&self.html.clone());
            window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("area"))
                .unwrap()
                .set_inner_html(&self.code.clone());
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let html = self.html.clone();
        let new_area_height = match window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("editor"))
        {
            None => "30rem".to_string(),
            Some(e) => {
                e.set_inner_html(&html);
                format!("{}px", e.client_height())
            }
        };
        let rows = &html.lines().count() - 2;
        let mut arr = vec![];
        let mut i = 0;
        while i < rows {
            arr.push(i + 1);
            i += 1;
        }
        let on_textarea_keydown = |e: KeyboardEvent| {
            let text_area = e.target().unwrap().unchecked_into::<HtmlTextAreaElement>();
            if e.key_code() == 9 {
                e.prevent_default();
                let start = text_area.selection_start().unwrap_or(None).unwrap_or(0);
                let end = text_area.selection_end().unwrap_or(None).unwrap_or(0);
                let current_text = text_area.value();
                text_area.set_value(
                    format!(
                        "{}\t{}",
                        &current_text
                            .chars()
                            .into_iter()
                            .take(start as usize)
                            .collect::<String>()
                            .as_str(),
                        &current_text
                            .chars()
                            .into_iter()
                            .skip(end as usize)
                            .collect::<String>()
                            .as_str()
                    )
                    .as_str(),
                );
                text_area
                    .set_selection_range(start + 1, end + 1)
                    .unwrap_or_default();
                //let _r = text_area.set_range_text_with_start_and_end("\t", start, end);
            }
            Msg::SetContent(text_area.value())
        };
        html! {
            <div id="main">
                <textarea id="area" spellcheck="false" style={format!("height: {}", new_area_height)} oninput={ctx.link().callback(|e: web_sys::InputEvent| Msg::SetContent(e.target_unchecked_into::<HtmlTextAreaElement>().value()))} onkeydown={ctx.link().callback(on_textarea_keydown)}/>
                <div id="editor-line-numbers">
                {arr.iter().map(|x| html! { <p>{format!("{}", x)}</p> }).collect::<Html>()}
                </div>
                <div id="editor-wrap">
                <div id="editor"/>
                <button onclick={ctx.link().callback(move |_| {
                    Msg::SendCode
                })}>{"Run/Compile"}</button>
                </div>
                <div id="terminal">
                  {self.code_response.to_message()}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
