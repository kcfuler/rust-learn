// 对 Command 的处理抽象
pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
  match cmd.request_data {
    Some(RequestData::Hget(param)) => param.execute(store),
    Some(RequestData::Hgetall(param)) => param.execute(store),
    Some(RequestData::Hset(param)),
    None => KvError::InvalidCommand("request has no data".into()).into(),
    _ => KvError::Internal("Not implemented".into()).into(),
  }
}