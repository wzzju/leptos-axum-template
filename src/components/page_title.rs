use leptos::*;
use leptos_meta::Title;

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    view! { <Title text=format!("{} | {{project-name}}", text.get())/> }
}
