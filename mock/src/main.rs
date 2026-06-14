use axum::{
    Form, Json, Router,
    http::{HeaderMap, header},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use rand::Rng;
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;

const COOKIE_NAME: &str = "MOCK_USER";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/xtgl/login_slogin.html",
            get(login_slogin_get).post(login_slogin_post),
        )
        .route("/xtgl/login_getPublicKey.html", get(public_key))
        .route("/xsxk/zzxkyzb_cxZzxkYzbIndex.html", get(index_html))
        .route("/xsxk/zzxkyzb_cxZzxkYzbDisplay.html", post(display))
        .route(
            "/xsxk/zzxkyzb_cxZzxkYzbPartDisplay.html",
            post(part_display),
        )
        .route(
            "/xsxk/zzxkyzbjk_cxJxbWithKchZzxkYzb.html",
            post(query_do_with_course_id),
        )
        .route("/xsxk/zzxkyzb_xkBcZyZzxkYzb.html", post(select_course));

    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();

    println!("http://127.0.0.1:8081");

    axum::serve(listener, app).await.unwrap();
}

fn page(body: &str) -> Html<String> {
    Html(format!("<!DOCTYPE html><html><body>{body}</body></html>"))
}

fn use_user(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get(header::COOKIE)?.to_str().ok()?;
    cookie.split(';').find_map(|kv| {
        let (k, v) = kv.split_once('=')?;
        if k.trim() == COOKIE_NAME {
            Some(v.trim().to_string())
        } else {
            None
        }
    })
}

async fn login_slogin_get(headers: HeaderMap) -> Html<String> {
    let mut body =
        format!(r#"<input type="hidden" id="csrftoken" name="csrftoken" value="MOCK_CSRFTOKEN"/>"#);

    if let Some(user) = use_user(&headers) {
        body.push_str(&format!(
            r#"<input type="hidden" id="sessionUserKey" name="sessionUserKey" value="{user}"/>"#
        ));
    }

    page(&body)
}

#[derive(Deserialize)]
struct LoginForm {
    yhm: String,
}

async fn login_slogin_post(form: Form<LoginForm>) -> impl IntoResponse {
    let user = form.0.yhm;

    let body = format!(
        r#"<input type="hidden" id="sessionUserKey" name="sessionUserKey" value="{user}"/>"#
    );

    (
        [(header::SET_COOKIE, format!("{COOKIE_NAME}={user}; Path=/"))],
        page(&body),
    )
}

async fn public_key() -> impl IntoResponse {
    Json(json!({
        "modulus": "AMgAC4t3eQZGSI9nsRvYBSJjBHWEKtmAQU3qyB+iogdWVARn1la1kS4gibudexLjDmnyXW3Lf3gci305N4KoJ8HX3CGSk3J4nDNVGipC3o8KAf6klDjxsjjanv2pI40h3YECv2zGezaw0jaCxo23sGlxhhRVhRiJm6U82Tpn0p69",
        "exponent": "AQAB"
    }))
}

async fn index_html(headers: HeaderMap) -> Html<String> {
    let xh_id = use_user(&headers).unwrap_or_default();

    let mut fields: Vec<(&str, &str)> = vec![
        ("firstXkkzId", "N253512"),
        ("firstKklxdm", "10"),
        ("xkkz_id", "N253512"),
        ("xbm", "1"),
        ("ccdm", "3"),
        ("xkxnm", "2023"),
        ("xkxqm", "2"),
        ("jg_id_1", "0001"),
        ("xsbj", "mockbj"),
        ("mzm", "01"),
        ("xz", "4"),
        ("bh_id", "mockbh"),
        ("xqh_id", "1"),
        ("zyfx_id", "mockzyfx"),
        ("xslbdm", "1"),
        ("bklx_id", "0"),
        ("njdm_id", "2023"),
    ];
    fields.push(("xh_id", xh_id.as_ref()));

    let body = fields
        .iter()
        .map(|(name, value)| format!(r#"<input type="hidden" name="{name}" value="{value}"/>"#))
        .collect::<String>();

    page(&body)
}

async fn display() -> Html<String> {
    page("")
}

async fn part_display() -> impl IntoResponse {
    Json(json!({
        "tmpList": [
            { "kch_id": "MOCKKCH001" },
            { "kch_id": "MOCKKCH002" }
        ]
    }))
}

async fn query_do_with_course_id() -> impl IntoResponse {
    Json(json!([
        {
            "do_jxb_id": "MOCKDOJXB001",
            "jsxx": "Mock Teacher",
            "jxb_id": "MOCKKCH001",
            "sksj": "星期四第9-10节{1-8周}"
        },
        {
            "do_jxb_id": "MOCKDOJXB002",
            "jsxx": "Mock Teacher2",
            "jxb_id": "MOCKKCH001",
            "sksj": "星期四第9-10节{9-16周}"
        }
    ]))
}

async fn select_course() -> impl IntoResponse {
    let mut rng = rand::thread_rng();
    let choice = rng.gen_range(0..5);
    match choice {
        1 => Json(json!({
            "flag": "0",
            "msg": "对不起，当前未开放选课！"
        })),
        2 => Json(json!({
            "flag": "0",
            "msg": "选课频率过高，请稍后重试！"
        })),
        3 => Json(json!({
            "flag": "0",
            "msg": "一门课程只能选一个教学班，不可再选！"
        })),
        4 => Json(json!({
            "flag": "0",
            "msg": "超过体育分项本学期本专业最高选课门次限制，不可选！"
        })),
        _ => Json(json!({
            "flag": "1",
            "msg": null
        })),
    }
}
