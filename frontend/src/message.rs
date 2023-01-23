use crate::response::Res;

pub enum Msg {
    InputChange(String),
    SetContent(String),
    SendCode,
    SetResponse(Res),
    Empty,
}
