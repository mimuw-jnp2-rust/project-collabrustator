use crate::response::Res;

pub enum Msg {
    InputChange(String, u32),
    SetContent(String),
    SendCode,
    SetResponse(Res),
    SetResponseNoWs(Res),
    Empty,
    SendMyId,
}
