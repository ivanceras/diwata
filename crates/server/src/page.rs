use actix_files::NamedFile;
use actix_web::{
    dev,
    middleware::errhandlers::ErrorHandlerResponse,
    web,
    Error,
    HttpRequest,
    HttpResponse,
    Responder,
    Result,
};
use futures::future::{
    self,
    Future,
};

pub fn index(
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);

    future::ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Hello")),
    )
}

pub fn bad_request<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn not_found<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn internal_server_error<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}
