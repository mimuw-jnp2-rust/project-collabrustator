use crate::response::Res;

pub enum Msg {
    SetContent(String),
    SendCode,
    SetResponse(Res),
}
