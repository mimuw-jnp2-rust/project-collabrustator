use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::{function_component, Properties};
#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: String,
    pub oninput: yew::Callback<web_sys::InputEvent>,
    pub onkeydown: yew::Callback<web_sys::KeyboardEvent>,
    pub style: String,
    pub cursor: u32,
}
#[function_component(ControlledTextArea)]
pub fn text_area(props: &Props) -> Html {
    let ref_ = use_node_ref();
    let ref_2 = ref_.clone();

    let cursor = props.cursor;

    use_effect(move || {
        if let Some(input) = ref_.cast::<HtmlTextAreaElement>() {
            let _ = input.set_selection_range(cursor, cursor);
        }
        || {}
    });
    let style = props.style.clone();
    let oninput = props.oninput.clone();
    let onkeydown = props.onkeydown.clone();

    html! {
        <textarea
            id="area" spellcheck="false"
            ref={ref_2}
            value={props.value.clone()}
            oninput={move |e: InputEvent| {
                oninput.emit(e);
            }}
            onkeydown={move |e: KeyboardEvent| {
                onkeydown.emit(e);
            }}
            style={style}
        >
        </textarea>
    }
}
