extern crate log;
use crate::controlled_textarea::ControlledTextArea;
use crate::message::Msg;
use crate::response::{Res, SpecificResponse};
use serde_json::json;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlTextAreaElement};
use yew::prelude::*;
use yew_websocket::macros::Json;
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
pub struct Room {
    code: String,
    ss: SyntaxSet,
    theme: Theme,
    syntax: SyntaxReference,
    html: String,
    code_response: Res,
    ws: WebSocketTask,
    cursor: u32,
}
#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub id: String,
}
const TAB_KEYCODE: u32 = 9;
impl Component for Room {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|Json(data): Json<Result<String, _>>| {
            let data = data.unwrap();
            let js: serde_json::Value = serde_json::from_str(&data).unwrap();
            if let Some(code) = js.get("code") {
                return Msg::SetContent(code.as_str().unwrap().to_string());
            }
            if let Some(_code) = js.get("start_running") {
                return Msg::Empty;
            }
            if let Some(response) = js.get("execution_response") {
                let res: Res = serde_json::from_value(response.clone()).unwrap();
                return Msg::SetResponseNoWs(res);
            }
            Msg::Empty
        });
        let status_callback = ctx.link().callback(|status| {
            if status == WebSocketStatus::Opened {
                return Msg::SendMyId;
            }
            Msg::Empty
        });
        let baseurl = web_sys::window().unwrap().origin().replace("http", "ws");
        let ws = WebSocketService::connect_text(
            format!("{baseurl}/ws/room").as_str(),
            callback,
            status_callback,
        )
        .unwrap();
        let code = "\n".to_string();
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = ts.themes["base16-ocean.dark"].clone();
        let syntax = ss.find_syntax_by_extension("rs").unwrap().to_owned();
        let html = highlighted_html_for_string(&code, &ss, &syntax, &theme).expect("Can't parse");
        let id = ctx.props().id.clone();
        let baseurl = web_sys::window().unwrap().origin();
        ctx.link().send_future(async move {
            let client = reqwest::Client::new();
            let res = client
                .get(format!("{baseurl}/mysite/api/room/{}/code", id))
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
            cursor: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChange(content, cursor) => {
                let message = json!({ "code": content });
                self.ws.send(Ok(serde_json::to_string(&message).unwrap()));
                ctx.link().send_message(Msg::SetContent(content));
                self.cursor = cursor;
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
                let message = json!({
                    "start_running": true
                });
                self.ws.send(Ok(serde_json::to_string(&message).unwrap()));
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
                let id = ctx.props().id.clone();
                ctx.link().send_future(async move {
                    let client = reqwest::Client::new();
                    let baseurl = web_sys::window().unwrap().origin();
                    let res = client
                        .post(format!("{baseurl}/mysite/api/code/{id}"))
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
                let message = json!({ "execution_response": res });
                self.ws.send(Ok(serde_json::to_string(&message).unwrap()));
                self.code_response = res;
                true
            }
            Msg::SetResponseNoWs(res) => {
                self.code_response = res;
                true
            }
            Msg::SendMyId => {
                let id = ctx.props().id.clone();
                self.ws.send(Ok(id));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let html = self.html.clone();
        let (new_area_height, new_area_width) = match window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("editor"))
        {
            None => ("2rem".to_string(), "100%".to_string()),
            Some(e) => {
                e.set_inner_html(&html);
                (
                    format!("{}px", e.client_height()),
                    format!("{}px" /* subtract left margin */, e.client_width()),
                )
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
                let cursor = text_area.selection_start().unwrap_or(None).unwrap_or(0);
                return Msg::InputChange(text_area.value(), cursor);
            }
            Msg::Empty
        };
        log::info!("Render");
        let cursor = self.cursor;
        html! {
            <div id="main">
                /*<textarea id="area" spellcheck="false" style={format!("height: {}; width: {}", new_area_height, new_area_width)} value={self.code.clone()} oninput={ctx.link().callback(|e: web_sys::InputEvent| Msg::InputChange(e.target_unchecked_into::<HtmlTextAreaElement>().value()))} onkeydown={ctx.link().callback(on_textarea_keydown)}/>*/
                <ControlledTextArea style={format!("height: {}; width: {}", new_area_height, new_area_width)} value={self.code.clone()} oninput={ctx.link().callback(|e: web_sys::InputEvent| {
                    let textarea = e.target_unchecked_into::<HtmlTextAreaElement>();
                    let cursor = textarea.selection_start().unwrap_or(None).unwrap_or(0);
                    Msg::InputChange(textarea.value(), cursor)
                })} onkeydown={ctx.link().callback(on_textarea_keydown)} cursor={cursor}/>
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
