use chrono::Datelike;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="position-relative bottom text-center text-bg-light">
            <section class="text-center py-5 mt-3">
                <a
                    href="https://www.facebook.com/p/Flow-Car-Studio-100084819075644/"
                    class="me-4"
                >
                    <i class="bi bi-facebook"></i>
                </a>
                <a href="https://www.instagram.com/flowcarstudio/" class="me-4">
                    <i class="bi bi-instagram"></i>
                </a>
            </section>
            <div class="text-center p-3 text-bg-light">
                {format!("© {} Flow Car Studio, created by: ", chrono::Utc::now().year())}
                <a
                    href="https://fraguinha.com/"
                    class="me-4"
                >
                    {"Fraguinha"}
                </a>
            </div>
        </footer>
    }
}
