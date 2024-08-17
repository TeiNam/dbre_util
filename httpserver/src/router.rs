use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{http_request, http_request::HttpRequest, http_response::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        let resp: HttpResponse = match req.method {
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => WebServiceHandler::handle(&req),
                        _ => StaticPageHandler::handle(&req),
                    }
                }
            },
            _ => PageNotFoundHandler::handle(&req),
        };

        // HTTP 버전에 따른 추가 처리를 여기에 추가할 수 있습니다.
        // 예: req.version()이 HTTP/2.0인 경우 특별한 처리를 할 수 있습니다.

        let _ = resp.send_response(stream);
    }
}