use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="fullscreen d-flex flex-row align-items-center">
            <div class="container">
                <div class="row justify-content-center">
                    <div class="col-md-12 text-center">
                        <span class="display-1 d-block">{404}</span>
                        <div class="mb-4 lead">{"Not Found"}</div>
                        <a href="/" class="link-style-light">
                            {"Back to Home Page"}
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
