use leptos::*;

use crate::utils::authorization::use_authorization;

pub const APP_PAGE_URL: &str = "/app";

#[component]
pub fn App() -> impl IntoView {
    // let Some(_authorization) = use_authorization() else {
    //     return view! {
    //         <Redirect path=AUTHENTICATION_PAGE_URL />
    //     }
    //     .into_view();
    // };

    let authorization = use_authorization();

    view! {
        <p>"Your authorization token is "{move || format!("{:?}", (authorization.token)())}</p>
    }
}
