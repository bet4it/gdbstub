use super::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub struct qsThreadInfo;

impl<'a> ParseCommand<'a> for qsThreadInfo {
    fn from_packet(buf: PacketBuf<'a>) -> Option<Self> {
        let body = buf.into_body_str();
        if !body.is_empty() {
            return None;
        }
        Some(qsThreadInfo)
    }
}
