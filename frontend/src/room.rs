extern crate log;
use crate::message::Msg;
use crate::response::{Res, SpecificResponse};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlTextAreaElement};
use yew::prelude::*;
use yew_websocket::macros::Json;
use yew_websocket::websocket::{WebSocketService, WebSocketTask};
pub struct Room {
    code: String,
    ss: SyntaxSet,
    theme: Theme,
    syntax: SyntaxReference,
    html: String,
    code_response: Res,
    ws: WebSocketTask,
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}
const TAB_KEYCODE: u32 = 9;
impl Component for Room {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|Json(data): Json<Result<Vec<u8>, _>>| {
            //log::info!("Received message from websocket: {:?}", data);
            data.map(|recv| String::from_utf8(recv))
                .map(|code| {
                    code.map(|code_str| Msg::SetContent(code_str))
                        .unwrap_or(Msg::Empty)
                })
                .unwrap_or(Msg::Empty)
            //data.map(|recv| Msg::SetContent(String::from_utf8(recv.clone()).unwrap_or(String::from("")))).unwrap_or_else(|_| Msg::Empty)
        });
        let status_callback = ctx.link().callback(|_| Msg::Empty);
        let ws = WebSocketService::connect_text(
            format!("ws://localhost:8000/room/{}", ctx.props().id).as_str(),
            callback,
            status_callback,
        )
        .unwrap();
        let code = "\n".repeat(1);
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = ts.themes["base16-ocean.dark"].clone();
        let syntax = ss.find_syntax_by_extension("rs").unwrap().to_owned();
        let html = highlighted_html_for_string(&code, &ss, &syntax, &theme).expect("Can't parse");
        let id = ctx.props().id.clone();
        ctx.link().send_future(async move {
            let client = reqwest::Client::new();
            let res = client
                .get(format!("http://127.0.0.1:8000/room/{}/code", id))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            Msg::SetContent(res)
        });
        Self {
            code,
            ss,
            theme,
            syntax,
            html,
            ws,
            code_response: Res {
                ..Default::default()
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChange(content) => {
                self.ws.send(Ok(content.clone()));
                ctx.link().send_message(Msg::SetContent(content));
                false
            }
            Msg::Empty => false,
            Msg::SetContent(content) => {
                self.code = content;
                let code_with_endline = self.code.clone() + "\n";
                self.html = highlighted_html_for_string(
                    &code_with_endline,
                    &self.ss,
                    &self.syntax,
                    &self.theme,
                )
                .expect("Can't parse");
                true
            }
            Msg::SendCode => {
                let code = self.code.clone();
                ctx.link().send_message(Msg::SetResponse(Res {
                    errors: SpecificResponse {
                        compile: String::from(""),
                        run: String::from(""),
                    },
                    outputs: SpecificResponse {
                        compile: String::from(""),
                        run: String::from("Compiling/Running..."),
                    },
                }));
                ctx.link().send_future(async move {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let html = self.html.clone();
        let new_area_height = match window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("editor"))
        {
            None => "2rem".to_string(),
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
            if e.key_code() == TAB_KEYCODE {
                let spaces_in_tab: u32 = 4;
                e.prevent_default();
                let start = text_area.selection_start().unwrap_or(None).unwrap_or(0);
                let end = text_area.selection_end().unwrap_or(None).unwrap_or(0);
                let current_text = text_area.value();
                text_area.set_value(
                    format!(
                        "{}{}{}",
                        &current_text
                            .chars()
                            .into_iter()
                            .take(start as usize)
                            .collect::<String>()
                            .as_str(),
                        " ".repeat(spaces_in_tab as usize),
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
                    .set_selection_range(start + spaces_in_tab, end + spaces_in_tab)
                    .unwrap_or_default();
                return Msg::SetContent(text_area.value());
            }
            Msg::Empty
        };
        log::info!("Render");
        html! {
            <div id="main">
                <textarea id="area" spellcheck="false" style={format!("height: {}", new_area_height)} value={self.code.clone()} oninput={ctx.link().callback(|e: web_sys::InputEvent| Msg::InputChange(e.target_unchecked_into::<HtmlTextAreaElement>().value()))} onkeydown={ctx.link().callback(on_textarea_keydown)}/>
                <div id="editor-line-numbers">
                {arr.iter().map(|x| html! { <p>{format!("{}", x)}</p> }).collect::<Html>()}
                </div>
                <div id="editor-wrap">
                <div id="editor"/>
                <button id="send-code" onclick={ctx.link().callback(move |_| {
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
