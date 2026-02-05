mod calculator;
mod cta;
mod ending;
mod gallery;
mod how_we_work;
mod navbar;
mod preloading;

use crate::ending::FinalGuarantees;
use crate::navbar::{Navbar, NavbarState};
use crate::{
    calculator::Calculator, cta::CTASection, gallery::PortfolioGallery, how_we_work::HowWeWork,
    preloading::Preloading,
};
use yew::prelude::*;

#[component]
fn App() -> Html {
    let current_page = use_state(|| NavbarState::Home);

    let on_navigate = {
        let current_page = current_page.clone();
        Callback::from(move |page: NavbarState| {
            current_page.set(page.clone());
        })
    };
    html! {
    <>
        <div class="navbar-main">
        <Navbar current_page={(*current_page).clone()} on_navigate={on_navigate} />
        </div>
        {match *current_page{
        NavbarState::Home => { html!{
        <div>
            <Preloading />
        </div>
        }}
        NavbarState::Calculator => { html!{
        <div class="calculator-main">
        <Calculator />
        </div>
        }}
        NavbarState::Gallery => { html!{
        <div class="gallery-main">
        <PortfolioGallery />
        </div>
        }}
        NavbarState::Process => { html!{
        <div class="hometowork-main">
        <HowWeWork />
        </div>

        }}
        NavbarState::CTA => { html!{
        <div class="cta-main">
        <CTASection />
        </div>
        }}
        NavbarState::Guaratees => { html!{
        <div class="guarantees-main">
        <FinalGuarantees />
        </div>
        }}
        }}
        //<Footer />
    </>

    }
}

// components/footer.rs
use yew::prelude::*;

#[component]
pub fn Footer() -> Html {
    html! {
    <div class="guarantees-footer">
        <div class="footer-content">
            <div class="footer-logo">
                <div class="logo-text">{"GRADPRUS"}</div>
                <div class="logo-slogan">{"Професійний ремонт з гарантією"}</div>
            </div>

            <div class="footer-assurance">
                <div class="assurance-item">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                        <path
                            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                    </svg>
                    <span>{"Сертифіковано"}</span>
                </div>
                <div class="assurance-item">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                        <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" stroke="currentColor" stroke-width="2"
                            stroke-linecap="round" stroke-linejoin="round" />
                    </svg>
                    <span>{"Гарантовано"}</span>
                </div>
                <div class="assurance-item">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                        <path d="M13 10V3L4 14h7v7l9-11h-7z" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                            stroke-linejoin="round" />
                    </svg>
                    <span>{"Інноваційно"}</span>
                </div>
            </div>

            <div class="footer-note">
                {"© 2024 GRADPRUS. Усі гарантії захищені. Інформація на сайті не є публічною офертою."}
            </div>
        </div>
    </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
