use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys, window};
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum NavbarState {
    Home,
    Calculator,
    Gallery,
    Process,
    WorkPrices,
    CTA,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NavbarProps {
    pub current_page: NavbarState,
    pub on_navigate: Callback<NavbarState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    pub id: usize,
    pub name: String,
    pub path: NavbarState,
    pub icon: &'static str,
    pub color: String,
    pub notification_count: Option<u8>,
}

#[component]
pub fn Navbar(props: &NavbarProps) -> Html {
    let nav_items = vec![
        NavItem {
            id: 0,
            name: "Головна".to_string(),
            path: NavbarState::Home,
            icon: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
            color: "#4299e1".to_string(),
            notification_count: None,
        },
        NavItem {
            id: 1,
            name: "Калькулятор".to_string(),
            path: NavbarState::Calculator,
            icon: "M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z",
            color: "#38b2ac".to_string(),
            notification_count: Some(3),
        },
        NavItem {
            id: 2,
            name: "Галерея".to_string(),
            path: NavbarState::Gallery,
            icon: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
            color: "#ed8936".to_string(),
            notification_count: None,
        },
        NavItem {
            id: 3,
            name: "Як ми працюємо".to_string(),
            path: NavbarState::Process,
            icon: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01",
            color: "#9f7aea".to_string(),
            notification_count: Some(1),
        },
        NavItem {
            id: 4,
            name: "Ціни".to_string(),
            path: NavbarState::WorkPrices,
            icon: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
            color: "#f56565".to_string(),
            notification_count: Some(5),
        },
        NavItem {
            id: 5,
            name: "Зв'язок".to_string(),
            path: NavbarState::CTA,
            icon: "M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
            color: "#48bb78".to_string(),
            notification_count: Some(2),
        },
    ];

    let is_mobile_menu_open = use_state(|| false);
    let hovered_item = use_state(|| Option::<usize>::None);

    let toggle_mobile_menu = {
        let is_mobile_menu_open = is_mobile_menu_open.clone();
        Callback::from(move |_| {
            is_mobile_menu_open.set(!*is_mobile_menu_open);
        })
    };

    let navigate = {
        let on_navigate = props.on_navigate.clone();
        let is_mobile_menu_open = is_mobile_menu_open.clone();

        Callback::from(move |page: NavbarState| {
            on_navigate.emit(page.clone());
            is_mobile_menu_open.set(false);

            match page {
                NavbarState::Home => scroll_to("home"),
                NavbarState::Calculator => scroll_to("calculator"),
                NavbarState::Gallery => scroll_to("portfolio"),
                NavbarState::Process => scroll_to("process"),
                NavbarState::WorkPrices => scroll_to("price-catalog"),
                NavbarState::CTA => scroll_to("cta"),
            }
        })
    };
    let set_hovered_item = {
        let hovered_item = hovered_item.clone();
        Callback::from(move |id: Option<usize>| {
            hovered_item.set(id);
        })
    };

    // Calculate navbar background opacity based on scroll

    html! {
        <div class="navbar-main">
        <nav class={classes!(
            "modern-navbar",
            if *is_mobile_menu_open { "menu-open" } else { "" }
        )}>
            <div class="navbar-background">
                <div
                    class="navbar-blur"
                    style={format!("opacity: {}", 0.8)}
                ></div>
                <div class="navbar-gradient"></div>
            </div>

            <div class="navbar-container">
                <div class="navbar-logo" onclick={navigate.reform(|_| NavbarState::Home)}>
                    <div class="logo-icon">
                        <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                            <path d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"
                                  stroke="currentColor" stroke-width="2"
                                  stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                    </div>
                    <div class="logo-text">
                        <div class="logo-name">{"GRADPRUS"}</div>
                        <div class="logo-tagline">{"Професійний ремонт"}</div>
                    </div>
                </div>

                <div class="navbar-desktop">
                    <div class="nav-items">
                        {for nav_items.iter().enumerate().map(|(index, item)| {
                            let is_active = props.current_page == item.path;
                            let item_id = item.id;
                            let item_path = item.path.clone();
                            let navigate = navigate.clone();
                            let set_hovered_item = set_hovered_item.clone();

                            html! {
                                <div
                                    class={classes!(
                                        "nav-item",
                                        if is_active { "active" } else { "" },
                                        if *hovered_item == Some(item_id) { "hovered" } else { "" }
                                    )}
                                    onmouseenter={
        let set_hovered_item = set_hovered_item.clone();

                                    Callback::from(move |_| set_hovered_item.emit(Some(item_id)))}
                                    onmouseleave={

                                    let set_hovered_item = set_hovered_item.clone();
                                    Callback::from(move |_| set_hovered_item.emit(None))}
                                    onclick={Callback::from(move |_| navigate.emit(item_path.clone()))}
                                    style={format!("--item-color: {}", item.color)}
                                >
                                    <div class="item-content">
                                        <div class="item-icon">
                                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                <path d={item.icon}
                                                      stroke="currentColor" stroke-width="2"
                                                      stroke-linecap="round" stroke-linejoin="round"/>
                                            </svg>
                                        </div>
                                        <div class="item-name">{&item.name}</div>
                                    </div>

                                    {if is_active {
                                        html! {
                                            <div class="item-active-indicator">
                                                <div class="active-glow"></div>
                                                <div class="active-pulse"></div>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}

                                    <div class="item-hover-effect"></div>
                                </div>
                            }
                        })}
                    </div>

                </div>

                <button
                    class={classes!(
                        "mobile-menu-button",
                        if *is_mobile_menu_open { "open" } else { "" }
                    )}
                    onclick={toggle_mobile_menu.clone()}
                    aria-label={if *is_mobile_menu_open { "Close menu" } else { "Open menu" }}
                >
                    <div class="menu-icon">
                        <span class="line line1"></span>
                        <span class="line line2"></span>
                        <span class="line line3"></span>
                    </div>
                </button>

            </div>

            <div class={classes!(
                "navbar-mobile",
                if *is_mobile_menu_open { "open" } else { "" }
            )}>
                <div class="mobile-backdrop" onclick={toggle_mobile_menu.clone()}></div>
                <div class="mobile-menu">
                    <div class="mobile-header">
                        <div class="mobile-logo">{"GRADPRUS"}</div>
                    </div>

                    <div class="mobile-items">
                        {for nav_items.iter().map(|item| {
                            let is_active = props.current_page == item.path;
                            let item_path = item.path.clone();
                            let navigate = navigate.clone();

                            html! {
                                <div
                                    class={classes!(
                                        "mobile-item",
                                        if is_active { "active" } else { "" }
                                    )}
                                    onclick={Callback::from(move |_| navigate.emit(item_path.clone()))}
                                    style={format!("--item-color: {}", item.color)}
                                >
                                    <div class="mobile-item-content">
                                        <div class="mobile-item-icon">
                                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                <path d={item.icon}
                                                      stroke="currentColor" stroke-width="2"
                                                      stroke-linecap="round" stroke-linejoin="round"/>
                                            </svg>
                                        </div>
                                        <div class="mobile-item-name">{&item.name}</div>
                                    </div>

                                    {if is_active {
                                        html! {
                                            <div class="mobile-active-indicator">
                                                <div class="active-dot"></div>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            }
                        })}
                    </div>

                    <div class="mobile-contact">
                        <div class="contact-info">
                            <div class="contact-title">{"Замовити консультацію"}</div>
                            <div class="contact-phone">{"+38 (063) 535-31-30"}</div>
                            <div class="contact-hours">{"Пн-Нд: 9:00-18:00"}</div>
                        </div>
                        <button
                            class="mobile-contact-button"
                            onclick={navigate.reform(|_| NavbarState::CTA)}
                        >
                            <div class="button-icon">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                    <path d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"
                                          stroke="currentColor" stroke-width="2"
                                          stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <span>{"Зв'язатися"}</span>
                        </button>
                    </div>
                </div>
            </div>
        </nav>
        </div>
    }
}

use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

fn scroll_to(id: &str) {
    if let Some(document) = window().unwrap().document() {
        if let Some(element) = document.get_element_by_id(id) {
            let options = ScrollIntoViewOptions::new();
            options.set_behavior(ScrollBehavior::Smooth);

            element.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }
}
