mod api;
mod calculator;
mod cta;
mod gallery;
mod how_we_work;
mod navbar;
mod preloading;
mod price_catalog;

use crate::calculator::Calculator;
use crate::cta::CTASection;
use crate::gallery::PortfolioGallery;
use crate::how_we_work::HowWeWork;
use crate::navbar::{Navbar, NavbarState};
use crate::preloading::Preloading;
use crate::price_catalog::PriceCatalog;
use yew::prelude::*;
use web_sys::window;
use yew_hooks::{UseAsyncOptions, use_async, use_async_with_options};

pub const BACKEND_URL: &'static str = "http://127.0.0.1:3000/api";

#[component]
fn App() -> Html {
    let current_page = use_state(|| NavbarState::Home);
    let should_scroll=use_state(|| true);


    let on_navigate = {
        let current_page = current_page.clone();
        Callback::from(move |page: NavbarState| {
            current_page.set(page.clone());
        })
    };
    html! {
    <>
        <Navbar current_page={(*current_page).clone()} on_navigate={on_navigate} />
        <Preloading />
        <Calculator />
        <PortfolioGallery />
        <HowWeWork />
        <PriceCatalog />
        <CTASection />
    </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
