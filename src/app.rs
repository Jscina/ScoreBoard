use leptos::prelude::*;
use thaw::ConfigProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <main>
                <p>"Hello, World!"</p>
            </main>
        </ConfigProvider>
    }
}
