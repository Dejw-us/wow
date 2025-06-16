pub enum HttpMethod {
  Get,
  Post,
}

pub struct RequestAction {
  url: String,
  method: HttpMethod,
}
