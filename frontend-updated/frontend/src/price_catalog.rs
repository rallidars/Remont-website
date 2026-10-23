use gloo_timers::callback::{Interval, Timeout};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use yew::prelude::*;

// ===== МОДЕЛІ ДАНИХ =====

#[derive(Debug, Clone, PartialEq)]
pub enum Complexity {
    Simple,
    Medium,
    Complex,
}

impl Complexity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Complexity::Simple => "Проста",
            Complexity::Medium => "Середня",
            Complexity::Complex => "Складна",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            // Single checkmark (simple task)
            Complexity::Simple => "M5 13l4 4L19 7",

            // Two horizontal lines (medium complexity)
            Complexity::Medium => "M4 7h16M4 12h12",

            // Branching structure (complex system)
            Complexity::Complex => "M4 6h6M14 6h6M10 6v6M4 18h6M14 18h6M10 12v6",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PricePosition {
    pub id: usize,
    pub code: String,
    pub name: String,
    pub description: String,
    pub unit: String,
    pub base_price: f32,
    pub material_price: f32,
    pub complexity: Complexity,
    pub min_quantity: u32,
    pub discount_percent: u8,
    pub popular: bool,
    pub guarantee_months: u8,
}

#[derive(Debug, Clone)]
pub struct PriceSection {
    pub id: usize,
    pub title: String,
    pub icon: &'static str,
    pub color: &'static str,
    pub positions: Vec<PricePosition>,
}

// ===== ОСНОВНИЙ КОМПОНЕНТ =====

#[component]
pub fn PriceCatalog() -> Html {
    // Стан для акордеону
    let open_sections = use_state(|| vec![1, 2, 3, 4]);
    let selected_position = use_state(|| None::<usize>);
    let show_modal = use_state(|| false);
    let discount_visible = use_state(|| HashMap::<usize, bool>::new());

    // Дані для прайс-листа
    let sections = get_sections();

    // Розрахунок загальної статистики
    let total_positions = sections.iter().map(|s| s.positions.len()).sum::<usize>();

    // Обробник для акордеону
    let toggle_section = {
        let open_sections = open_sections.clone();
        Callback::from(move |id: usize| {
            let mut new_open = (*open_sections).clone();
            if new_open.contains(&id) {
                new_open.retain(|&x| x != id);
            } else {
                new_open.push(id);
            }
            open_sections.set(new_open);
        })
    };

    // Обробник для показу деталей
    let show_position_details = {
        let selected_position = selected_position.clone();
        let show_modal = show_modal.clone();
        Callback::from(move |id: usize| {
            selected_position.set(Some(id));
            show_modal.set(true);
        })
    };

    // Обробник для закриття модалки
    let close_modal = {
        let selected_position = selected_position.clone();
        let show_modal = show_modal.clone();
        Callback::from(move |_| {
            show_modal.set(false);
            let selected_position = selected_position.clone();
            Timeout::new(300, move || {
                selected_position.set(None);
            })
            .forget();
        })
    };

    // Обробник для показу знижки
    let toggle_discount = {
        let discount_visible = discount_visible.clone();
        Callback::from(move |id: usize| {
            let mut new_map = (*discount_visible).clone();
            new_map.insert(id, !new_map.get(&id).unwrap_or(&false));
            discount_visible.set(new_map);
        })
    };
    html! {
        <div class="pl-price-catalog-main" id="price-catalog">
            <div class="pl-price-list-app">
                <div class="pl-main">
                    <div class="pl-header">
                        <h1 class="pl-title">
                            <span class="pl-title-main">{"Прайс-лист"}</span>
                            <span class="pl-title-year">{"2026"}</span>
                        </h1>
                    </div>

                    <div class="pl-banner">
                        <div class="pl-banner-icon">
                            <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                <path d="M12 2v4M12 22v-4M4 12H2h2m16 0h2-2M5.636 5.636l2.828 2.828M15.536 15.536l2.828 2.828M18.364 5.636l-2.828 2.828M8.464 15.536l-2.828 2.828" stroke-width="2"/>
                            </svg>
                        </div>
                        <div class="pl-banner-text">
                            <strong>{"Ціни актуальні на 2026 рік"}</strong>
                            {" • Вказана вартість робіт без матеріалів • Можливі індивідуальні знижки"}
                        </div>
                    </div>

                    <div class="pl-sections">
                        {for sections.iter().map(|section| {
                            let is_open = open_sections.contains(&section.id);
                            let on_toggle = toggle_section.clone();
                            let section_id = section.id;

                            html! {
                                <div class={classes!("pl-section", is_open.then(|| "open"))}>
                                    <div
                                        class="pl-section-header"
                                        style={format!("border-left-color: {}", section.color)}
                                        onclick={Callback::from(move |_| on_toggle.emit(section_id))}
                                    >
                                        <div class="pl-section-header-left">
                                            <div class="pl-section-icon" style={format!("background: {}", section.color)}>
                                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white">
                                                    <path d={section.icon} stroke-width="2"/>
                                                </svg>
                                            </div>
                                            <h2 class="pl-section-title">{&section.title}</h2>
                                        </div>
                                        <div class="pl-section-header-right">
                                            <span class="pl-section-count">{section.positions.len()} {if section.positions.len() == 1 { "позиція" } else { "позицій" }}</span>
                                            <svg
                                                class="pl-section-arrow"
                                                width="24"
                                                height="24"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                style={format!("transform: rotate({}deg)", if is_open { "180" } else { "0" })}
                                            >
                                                <path d="M19 9l-7 7-7-7" stroke-width="2"/>
                                            </svg>
                                        </div>
                                    </div>

                                    {if is_open {
                                        html! {
                                            <div class="pl-section-content">
                                                <div class="pl-price-table-header">
                                                    <div class="pl-col-code">{"Код"}</div>
                                                    <div class="pl-col-name">{"Назва роботи"}</div>
                                                    <div class="pl-col-unit">{"Од."}</div>
                                                    <div class="pl-col-price">{"Ціна, грн"}</div>
                                                    <div class="pl-col-materials">{"Матеріали"}</div>
                                                    <div class="pl-col-complexity">{"Складність"}</div>
                                                    <div class="pl-col-guarantee">{"Гарантія"}</div>
                                                    <div class="pl-col-actions"></div>
                                                </div>

                                                {for section.positions.iter().map(|pos| {
                                                    let show_discount = discount_visible.get(&pos.id).unwrap_or(&false);
                                                    let on_show_details = show_position_details.clone();
                                                    let on_toggle_discount = toggle_discount.clone();
                                                    let pos_id = pos.id;
                                                    let final_price = if *show_discount {
                                                        pos.base_price * (1.0 - pos.discount_percent as f32 / 100.0)
                                                    } else {
                                                        pos.base_price
                                                    };

                                                    html! {
                                                        <div class={classes!("pl-price-row", if pos.popular { "popular" } else { "" })}>
                                                            <div class="pl-col-code">
                                                                <span class="pl-pos-code">{&pos.code}</span>
                                                            </div>

                                                            <div class="pl-col-name">
                                                                <div class="pl-pos-name">{&pos.name}</div>
                                                                <div class="pl-pos-desc">{&pos.description}</div>
                                                            </div>

                                                            <div class="pl-col-unit">
                                                                <span class="pl-pos-unit">{&pos.unit}</span>
                                                            </div>

                                                            <div class="pl-col-price">
                                                                <div class={classes!("pl-price-value", if *show_discount { "with-discount" } else { "" })}>
                                                                    {if *show_discount {
                                                                        html! {
                                                                            <>
                                                                                <span class="pl-old-price">{format!("{:.0}", pos.base_price)}</span>
                                                                                <span class="pl-new-price">{format!("{:.0}", final_price)}</span>
                                                                            </>
                                                                        }
                                                                    } else {
                                                                        html! { format!("{:.0}", pos.base_price) }
                                                                    }}
                                                                </div>
                                                            </div>

                                                            <div class="pl-col-materials">
                                                                {if pos.material_price > 0.0 {
                                                                    html! { format!("{:.0} грн", pos.material_price) }
                                                                } else {
                                                                    html! { <span class="pl-no-materials">{"за уточненням"}</span> }
                                                                }}
                                                            </div>

                                                            <div class="pl-col-complexity">
                                                                <div class="pl-complexity-indicator" title={pos.complexity.as_str()}>
                                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                                                        <path d={pos.complexity.icon()} stroke-width="2"/>
                                                                    </svg>
                                                                </div>
                                                            </div>

                                                            <div class="pl-col-guarantee">
                                                                {if pos.guarantee_months > 0 {
                                                                    html! { format!("{} міс.", pos.guarantee_months) }
                                                                } else {
                                                                    html! { <span class="pl-no-guarantee">{"немає"}</span> }
                                                                }}
                                                            </div>

                                                            <div class="pl-col-actions">
                                                                {if pos.discount_percent > 0 {
                                                                    html! {
                                                                        <button
                                                                            class={classes!("pl-action-btn", "pl-discount-btn", if *show_discount { "active" } else { "" })}
                                                                            onclick={Callback::from(move |e: MouseEvent| {
                                                                                e.stop_propagation();
                                                                                on_toggle_discount.emit(pos_id);
                                                                            })}
                                                                        >
                                                                            <span>{format!("-{}%", pos.discount_percent)}</span>
                                                                        </button>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }}

                                                                <button
                                                                    class="pl-action-btn pl-info-btn"
                                                                    onclick={Callback::from(move |_| on_show_details.emit(pos_id))}
                                                                >
                                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                                                        <path d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" stroke-width="2"/>
                                                                    </svg>
                                                                </button>
                                                            </div>
                                                        </div>
                                                    }
                                                })}

                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            }
                        })}
                    </div>

                    <div class="pl-info">
                        <div class="pl-info-grid">
                            <div class="pl-info-card">
                                <div class="pl-info-card-icon" style="background: #3b82f6">
                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white">
                                        <path d="M12 2l7 4v6l-7 4-7-4V6l7-4z" stroke-width="2"/>
                                    </svg>
                                </div>
                                <h3>{"Гарантія на роботи"}</h3>
                                <p>{"Від 3 до 24 місяців на всі види робіт"}</p>
                            </div>

                            <div class="pl-info-card">
                                <div class="pl-info-card-icon" style="background: #10b981">
                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white">
                                        <path d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" stroke-width="2"/>
                                    </svg>
                                </div>
                                <h3>{"Сертифіковані майстри"}</h3>
                                <p>{"Працюємо тільки з перевіреними фахівцями"}</p>
                            </div>

                            <div class="pl-info-card">
                                <div class="pl-info-card-icon" style="background: #a855f7">
                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white">
                                        <path d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z" stroke-width="2"/>
                                    </svg>
                                </div>
                                <h3>{"Зручна оплата"}</h3>
                                <p>{"Готівковий та безготівковий розрахунок"}</p>
                            </div>
                        </div>
                    </div>
                </div>

                {if *show_modal {
                    if let Some(id) = *selected_position {
                        if let Some(pos) = sections.iter().flat_map(|s| &s.positions).find(|p| p.id == id) {
                            html! {
                                <div class="pl-modal" onclick={close_modal.clone()}>
                                    <div class="pl-modal-content" onclick={|e: MouseEvent| e.stop_propagation()}>
                                        <div class="pl-modal-header">
                                            <h2>{&pos.name}</h2>
                                            <button class="pl-modal-close" onclick={close_modal.clone()}>
                                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                                    <path d="M6 18L18 6M6 6l12 12" stroke-width="2"/>
                                                </svg>
                                            </button>
                                        </div>

                                        <div class="pl-modal-body">
                                            <div class="pl-modal-row">
                                                <span class="pl-modal-label">{"Код:"}</span>
                                                <span class="pl-modal-value">{&pos.code}</span>
                                            </div>
                                            <div class="pl-modal-row">
                                                <span class="pl-modal-label">{"Опис:"}</span>
                                                <span class="pl-modal-value">{&pos.description}</span>
                                            </div>
                                            <div class="pl-modal-row">
                                                <span class="pl-modal-label">{"Одиниця виміру:"}</span>
                                                <span class="pl-modal-value">{&pos.unit}</span>
                                            </div>
                                            <div class="pl-modal-row">
                                                <span class="pl-modal-label">{"Ціна за роботу:"}</span>
                                                <span class="pl-modal-value price">{format!("{:.0} грн", pos.base_price)}</span>
                                            </div>
                                            <div class="pl-modal-row">
                                                <span class="pl-modal-label">{"Вартість матеріалів:"}</span>
                                                <span class="pl-modal-value">{if pos.material_price > 0.0 { format!("{:.0} грн", pos.material_price) } else { "за уточненням".to_string() }}</span>
                                            </div>
                                            <div class="pl-modal-row">
                                                <span class="pl-modal-label">{"Складність:"}</span>
                                                <span class="pl-modal-value">{pos.complexity.as_str()}</span>
                                            </div>
                                            <div class="pl-modal-row">
                                                <span class="pl-modal-label">{"Мінімальне замовлення:"}</span>
                                                <span class="pl-modal-value">{format!("{} {}", pos.min_quantity, pos.unit)}</span>
                                            </div>
                                            <div class="pl-modal-row">
                                                <span class="pl-modal-label">{"Гарантія:"}</span>
                                                <span class="pl-modal-value">{if pos.guarantee_months > 0 { format!("{} місяців", pos.guarantee_months) } else { "без гарантії".to_string() }}</span>
                                            </div>
                                            {if pos.discount_percent > 0 {
                                                html! {
                                                    <div class="pl-modal-row highlight">
                                                        <span class="pl-modal-label">{"Знижка:"}</span>
                                                        <span class="pl-modal-value">{format!("{}% при замовленні від {} {}", pos.discount_percent, pos.min_quantity * 2, pos.unit)}</span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                        </div>

                                        <div class="pl-modal-footer">
                                            <button class="pl-modal-btn primary">
                                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                                    <path d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" stroke-width="2"/>
                                                </svg>
                                                {"Замовити"}
                                            </button>
                                            <button class="pl-modal-btn secondary" onclick={close_modal.clone()}>
                                                {"Закрити"}
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    } else {
                        html! {}
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}

fn get_sections() -> Vec<PriceSection> {
    vec![
PriceSection {
    id: 2,
    title: "Сантехнічні роботи".to_string(),
    icon: "M12 2v20M5 7h14M5 17h14",
    color: "#3b82f6",
    positions: vec![
        PricePosition {
            id: 201,
            code: "SAN-001".to_string(),
            name: "Штроблення під труби".to_string(),
            description: "Штроблення стін та підлоги під прокладання сантехнічних труб".to_string(),
            unit: "м.п".to_string(),
            base_price: 350.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 202,
            code: "SAN-002".to_string(),
            name: "Точка ХВ/ГВ/каналізація".to_string(),
            description: "Монтаж точки холодного водопостачання, гарячого водопостачання або каналізації".to_string(),
            unit: "шт".to_string(),
            base_price: 900.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 203,
            code: "SAN-003".to_string(),
            name: "Прокладання труби з утеплювачем".to_string(),
            description: "Прокладання трубопроводу з теплоізоляцією".to_string(),
            unit: "м.п".to_string(),
            base_price: 100.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 204,
            code: "SAN-004".to_string(),
            name: "Точка опалення".to_string(),
            description: "Монтаж точки підключення системи опалення".to_string(),
            unit: "шт".to_string(),
            base_price: 1900.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 205,
            code: "SAN-005".to_string(),
            name: "Укладання утеплювача 10–50 мм".to_string(),
            description: "Укладання теплоізоляційного матеріалу товщиною від 10 до 50 мм".to_string(),
            unit: "м²".to_string(),
            base_price: 180.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 206,
            code: "SAN-006".to_string(),
            name: "Укладання утеплювача до 100 мм".to_string(),
            description: "Укладання теплоізоляційного матеріалу товщиною до 100 мм".to_string(),
            unit: "м²".to_string(),
            base_price: 230.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 207,
            code: "SAN-007".to_string(),
            name: "Укладання теплої підлоги (водяна)".to_string(),
            description: "Монтаж труб водяної системи теплої підлоги".to_string(),
            unit: "м²".to_string(),
            base_price: 420.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 208,
            code: "SAN-008".to_string(),
            name: "Складання та монтаж гребінки".to_string(),
            description: "Складання та монтаж колекторної гребінки системи опалення".to_string(),
            unit: "шт".to_string(),
            base_price: 3000.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 209,
            code: "SAN-009".to_string(),
            name: "Підключення гребінки (контур)".to_string(),
            description: "Підключення одного контуру до колекторної гребінки".to_string(),
            unit: "шт".to_string(),
            base_price: 850.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 210,
            code: "SAN-010".to_string(),
            name: "Збірка ввідного вузла".to_string(),
            description: "Монтаж редуктора, фільтра та лічильника".to_string(),
            unit: "шт".to_string(),
            base_price: 3500.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 211,
            code: "SAN-011".to_string(),
            name: "Монтаж інсталяції".to_string(),
            description: "Монтаж сантехнічної інсталяції".to_string(),
            unit: "шт".to_string(),
            base_price: 2500.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 212,
            code: "SAN-012".to_string(),
            name: "Монтаж душового трапу".to_string(),
            description: "Монтаж та підключення душового трапу".to_string(),
            unit: "шт".to_string(),
            base_price: 2500.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 213,
            code: "SAN-013".to_string(),
            name: "Монтаж ванни".to_string(),
            description: "Монтаж та підключення ванни".to_string(),
            unit: "шт".to_string(),
            base_price: 3500.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 214,
            code: "SAN-014".to_string(),
            name: "Монтаж умивальника / раковини".to_string(),
            description: "Монтаж та підключення умивальника або раковини".to_string(),
            unit: "шт".to_string(),
            base_price: 1000.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 215,
            code: "SAN-015".to_string(),
            name: "Монтаж змішувача".to_string(),
            description: "Монтаж та підключення сантехнічного змішувача".to_string(),
            unit: "шт".to_string(),
            base_price: 800.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 216,
            code: "SAN-016".to_string(),
            name: "Монтаж унітазу".to_string(),
            description: "Монтаж та підключення унітазу".to_string(),
            unit: "шт".to_string(),
            base_price: 1500.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 217,
            code: "SAN-017".to_string(),
            name: "Підключення пральної машини / посудомийки".to_string(),
            description: "Підключення пральної машини або посудомийної машини".to_string(),
            unit: "шт".to_string(),
            base_price: 800.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 218,
            code: "SAN-018".to_string(),
            name: "Погрузка та підключення насосу (свердловинна)".to_string(),
            description: "Підйом, монтаж та підключення свердловинного насоса".to_string(),
            unit: "послуга".to_string(),
            base_price: 5000.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 219,
            code: "SAN-019".to_string(),
            name: "Монтаж гідроакумулятора та підключення автоматики".to_string(),
            description: "Монтаж гідроакумулятора та підключення автоматики системи водопостачання".to_string(),
            unit: "послуга".to_string(),
            base_price: 3000.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 220,
            code: "SAN-020".to_string(),
            name: "Монтаж та обв'язка котла".to_string(),
            description: "Монтаж та обв'язка електричного або газового котла. Вартість узгоджується окремо".to_string(),
            unit: "шт".to_string(),
            base_price: 0.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
    ],
},

PriceSection {
    id: 3,
    title: "Електромонтажні роботи".to_string(),
    icon: "M13 2L3 14h9l-1 8 10-12h-9l1-8z",
    color: "#f59e0b",
    positions: vec![
        PricePosition {
            id: 301,
            code: "ELE-001".to_string(),
            name: "Штроблення під кабель".to_string(),
            description: "Штроблення стін та інших поверхонь під прокладання електричного кабелю".to_string(),
            unit: "м.п".to_string(),
            base_price: 250.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 302,
            code: "ELE-002".to_string(),
            name: "Прокладання кабелів живлення".to_string(),
            description: "Прокладання силових електричних кабелів".to_string(),
            unit: "м.п".to_string(),
            base_price: 50.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 303,
            code: "ELE-003".to_string(),
            name: "Прокладання кабелів у металорукав / гофру".to_string(),
            description: "Прокладання кабелю в металорукаві або захисній гофрі".to_string(),
            unit: "м.п".to_string(),
            base_price: 35.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 304,
            code: "ELE-004".to_string(),
            name: "Складання розподільчої коробки".to_string(),
            description: "Монтаж та комутація розподільчої коробки".to_string(),
            unit: "шт".to_string(),
            base_price: 350.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 305,
            code: "ELE-005".to_string(),
            name: "Вирізання отворів підрозетників".to_string(),
            description: "Вирізання та підготовка отворів під електричні підрозетники".to_string(),
            unit: "шт".to_string(),
            base_price: 250.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 306,
            code: "ELE-006".to_string(),
            name: "Ніша під щит 12 модулів".to_string(),
            description: "Підготовка ніші під електричний щит на 12 модулів".to_string(),
            unit: "шт".to_string(),
            base_price: 1000.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 307,
            code: "ELE-007".to_string(),
            name: "Монтаж щита 12 модулів".to_string(),
            description: "Монтаж щита та підключення автоматичних вимикачів".to_string(),
            unit: "шт".to_string(),
            base_price: 4600.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 308,
            code: "ELE-008".to_string(),
            name: "Ніша під щит 24 модулі".to_string(),
            description: "Підготовка ніші під електричний щит на 24 модулі".to_string(),
            unit: "шт".to_string(),
            base_price: 1500.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 309,
            code: "ELE-009".to_string(),
            name: "Монтаж щита 24 модулі".to_string(),
            description: "Монтаж щита та підключення автоматичних вимикачів".to_string(),
            unit: "шт".to_string(),
            base_price: 8200.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 310,
            code: "ELE-010".to_string(),
            name: "Ніша під щит 36 модулів".to_string(),
            description: "Підготовка ніші під електричний щит на 36 модулів".to_string(),
            unit: "шт".to_string(),
            base_price: 2500.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 311,
            code: "ELE-011".to_string(),
            name: "Монтаж щита 36 модулів".to_string(),
            description: "Монтаж щита та підключення автоматичних вимикачів".to_string(),
            unit: "шт".to_string(),
            base_price: 12100.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 312,
            code: "ELE-012".to_string(),
            name: "Монтаж та підключення бойлера".to_string(),
            description: "Монтаж та електричне підключення бойлера".to_string(),
            unit: "шт".to_string(),
            base_price: 3200.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 313,
            code: "ELE-013".to_string(),
            name: "Монтаж електрофурнітури".to_string(),
            description: "Монтаж розеток, вимикачів та іншої електрофурнітури".to_string(),
            unit: "шт".to_string(),
            base_price: 200.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
    ],
},

PriceSection {
    id: 4,
    title: "Основні роботи".to_string(),
    icon: "M4 6h16M4 12h16M4 18h16",
    color: "#8b5cf6",
    positions: vec![
        PricePosition {
            id: 401,
            code: "BUD-001".to_string(),
            name: "Штукатурка стін (машинна)".to_string(),
            description: "Машинне нанесення штукатурки на стіни".to_string(),
            unit: "м²".to_string(),
            base_price: 360.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 402,
            code: "BUD-002".to_string(),
            name: "Штукатурка стін (ручна)".to_string(),
            description: "Ручне нанесення штукатурки на стіни".to_string(),
            unit: "м²".to_string(),
            base_price: 550.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 403,
            code: "BUD-003".to_string(),
            name: "Монтаж ГКЛВ стелі".to_string(),
            description: "Монтаж конструкцій стелі з вологостійкого гіпсокартону".to_string(),
            unit: "м²".to_string(),
            base_price: 650.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 404,
            code: "BUD-004".to_string(),
            name: "Монтаж ГКЛВ стін".to_string(),
            description: "Монтаж конструкцій стін з вологостійкого гіпсокартону".to_string(),
            unit: "м²".to_string(),
            base_price: 500.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 405,
            code: "BUD-005".to_string(),
            name: "Утеплення / шумоізоляція мінватою".to_string(),
            description: "Монтаж мінеральної вати для утеплення або шумоізоляції".to_string(),
            unit: "м²".to_string(),
            base_price: 250.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 406,
            code: "BUD-006".to_string(),
            name: "Монтаж OSB плит".to_string(),
            description: "Монтаж плит OSB на стіни, підлогу або інші поверхні".to_string(),
            unit: "м²".to_string(),
            base_price: 300.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 407,
            code: "BUD-007".to_string(),
            name: "Перегородки з ГКЛВ".to_string(),
            description: "Влаштування перегородок з вологостійкого гіпсокартону".to_string(),
            unit: "м²".to_string(),
            base_price: 800.0,
            material_price: 0.0,
            complexity: Complexity::Medium,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
        PricePosition {
            id: 408,
            code: "BUD-008".to_string(),
            name: "Заробляння штроб".to_string(),
            description: "Заповнення та вирівнювання штроб після прокладання комунікацій".to_string(),
            unit: "м.п".to_string(),
            base_price: 220.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 409,
            code: "BUD-009".to_string(),
            name: "Заробляння швів в ГКЛВ".to_string(),
            description: "Шпаклювання та армування швів гіпсокартонних конструкцій".to_string(),
            unit: "м²".to_string(),
            base_price: 180.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 410,
            code: "BUD-010".to_string(),
            name: "Посилення кутів перфорованим кутником".to_string(),
            description: "Монтаж перфорованого кутника для армування зовнішніх кутів".to_string(),
            unit: "м.п".to_string(),
            base_price: 180.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 411,
            code: "BUD-011".to_string(),
            name: "Ґрунтування стін / стелі (1 шар)".to_string(),
            description: "Ґрунтування поверхні стін або стелі в один шар".to_string(),
            unit: "м²".to_string(),
            base_price: 45.0,
            material_price: 0.0,
            complexity: Complexity::Simple,
            min_quantity: 1,
            discount_percent: 0,
            popular: false,
            guarantee_months: 0,
        },
        PricePosition {
            id: 412,
            code: "BUD-012".to_string(),
            name: "Влаштування стяжки підлоги до 8 см".to_string(),
            description: "Влаштування цементно-піщаної стяжки товщиною до 8 см, площа до 70 м²".to_string(),
            unit: "послуга".to_string(),
            base_price: 26500.0,
            material_price: 0.0,
            complexity: Complexity::Complex,
            min_quantity: 1,
            discount_percent: 0,
            popular: true,
            guarantee_months: 0,
        },
PricePosition {
    id: 413,
    code: "BUD-013".to_string(),
    name: "Влаштування стяжки підлоги (від 70 м², до 8 см)".to_string(),
    description: "Влаштування стяжки підлоги товщиною до 8 см при площі від 70 м²".to_string(),
    unit: "м²".to_string(),
    base_price: 350.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 70,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 414,
    code: "BUD-014".to_string(),
    name: "Влаштування самовирівнюючої суміші".to_string(),
    description: "Підготовка та заливка підлоги самовирівнюючою сумішшю".to_string(),
    unit: "м²".to_string(),
    base_price: 320.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 415,
    code: "BUD-015".to_string(),
    name: "Укладання теплої підлоги (електрична)".to_string(),
    description: "Монтаж електричної системи теплої підлоги".to_string(),
    unit: "м²".to_string(),
    base_price: 250.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 416,
    code: "BUD-016".to_string(),
    name: "Гідроізоляція ванної кімнати".to_string(),
    description: "Влаштування гідроізоляції підлоги та стін у ванній кімнаті".to_string(),
    unit: "м²".to_string(),
    base_price: 250.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 417,
    code: "BUD-017".to_string(),
    name: "Формування душового піддону з плитки (під ключ)".to_string(),
    description: "Повне формування та оздоблення душового піддону з плитки".to_string(),
    unit: "шт".to_string(),
    base_price: 15000.0,
    material_price: 0.0,
    complexity: Complexity::Complex,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 418,
    code: "BUD-018".to_string(),
    name: "Укладання плитки на стіни".to_string(),
    description: "Укладання керамічної плитки на стіни".to_string(),
    unit: "м²".to_string(),
    base_price: 900.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 419,
    code: "BUD-019".to_string(),
    name: "Укладання плитки на підлогу".to_string(),
    description: "Укладання керамічної плитки на підлогу".to_string(),
    unit: "м²".to_string(),
    base_price: 900.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 420,
    code: "BUD-020".to_string(),
    name: "Укладання мозаїки".to_string(),
    description: "Укладання декоративної мозаїки на стіни або інші поверхні".to_string(),
    unit: "м²".to_string(),
    base_price: 2100.0,
    material_price: 0.0,
    complexity: Complexity::Complex,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 421,
    code: "BUD-021".to_string(),
    name: "Вирізання отворів у плитці".to_string(),
    description: "Точне вирізання отворів у керамічній плитці".to_string(),
    unit: "шт".to_string(),
    base_price: 300.0,
    material_price: 0.0,
    complexity: Complexity::Simple,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 422,
    code: "BUD-022".to_string(),
    name: "Зарізання плитки під 45°".to_string(),
    description: "Формування запилу плитки під кутом 45 градусів".to_string(),
    unit: "м.п".to_string(),
    base_price: 650.0,
    material_price: 0.0,
    complexity: Complexity::Complex,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 423,
    code: "BUD-023".to_string(),
    name: "Затирання швів цементною затіркою".to_string(),
    description: "Заповнення плиткових швів цементною затіркою".to_string(),
    unit: "м²".to_string(),
    base_price: 200.0,
    material_price: 0.0,
    complexity: Complexity::Simple,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 424,
    code: "BUD-024".to_string(),
    name: "Затирання швів епоксидною затіркою".to_string(),
    description: "Заповнення плиткових швів двокомпонентною епоксидною затіркою".to_string(),
    unit: "м²".to_string(),
    base_price: 400.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 425,
    code: "BUD-025".to_string(),
    name: "Силікон по периметру ванної кімнати".to_string(),
    description: "Нанесення санітарного силікону по внутрішніх кутах ванної кімнати".to_string(),
    unit: "м.п".to_string(),
    base_price: 250.0,
    material_price: 0.0,
    complexity: Complexity::Simple,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 426,
    code: "BUD-026".to_string(),
    name: "Підготовка стін / стелі під склополотно".to_string(),
    description: "Підготовка поверхонь стін та стелі під подальше нанесення склополотна".to_string(),
    unit: "м²".to_string(),
    base_price: 350.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 427,
    code: "BUD-027".to_string(),
    name: "Підготовка стін під шпалери".to_string(),
    description: "Підготовка та вирівнювання стін під поклейку шпалер".to_string(),
    unit: "м²".to_string(),
    base_price: 400.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 428,
    code: "BUD-028".to_string(),
    name: "Виготовлення відкосів".to_string(),
    description: "Підготовка та оздоблення віконних і дверних відкосів".to_string(),
    unit: "м.п".to_string(),
    base_price: 400.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 429,
    code: "BUD-029".to_string(),
    name: "Поклейка склополотна".to_string(),
    description: "Поклейка склополотна на стіни, стелю або відкоси".to_string(),
    unit: "м²".to_string(),
    base_price: 220.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 430,
    code: "BUD-030".to_string(),
    name: "Підготовка стін / стелі під фарбування".to_string(),
    description: "Комплексна підготовка поверхні стін та стелі під фарбування".to_string(),
    unit: "м²".to_string(),
    base_price: 500.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 431,
    code: "BUD-031".to_string(),
    name: "Фарбування стін / стелі / відкосів".to_string(),
    description: "Фарбування підготовлених поверхонь стін, стелі та відкосів".to_string(),
    unit: "м²".to_string(),
    base_price: 250.0,
    material_price: 0.0,
    complexity: Complexity::Simple,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 432,
    code: "BUD-032".to_string(),
    name: "Декоративне оздоблення".to_string(),
    description: "Декоративне оздоблення: травертин, мокрий шовк, отточенто та інші декоративні покриття".to_string(),
    unit: "м²".to_string(),
    base_price: 650.0,
    material_price: 0.0,
    complexity: Complexity::Complex,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 433,
    code: "BUD-033".to_string(),
    name: "Поклейка шпалер без підбору".to_string(),
    description: "Поклейка шпалер без необхідності підбору малюнка".to_string(),
    unit: "м²".to_string(),
    base_price: 350.0,
    material_price: 0.0,
    complexity: Complexity::Simple,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 434,
    code: "BUD-034".to_string(),
    name: "Поклейка шпалер з підбором".to_string(),
    description: "Поклейка шпалер з підбором малюнка".to_string(),
    unit: "м²".to_string(),
    base_price: 450.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 435,
    code: "BUD-035".to_string(),
    name: "Укладання ламінату".to_string(),
    description: "Укладання ламінованого покриття на підготовлену основу".to_string(),
    unit: "м²".to_string(),
    base_price: 350.0,
    material_price: 0.0,
    complexity: Complexity::Simple,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 436,
    code: "BUD-036".to_string(),
    name: "Укладання кварц-вінілу".to_string(),
    description: "Укладання кварц-вінілового підлогового покриття".to_string(),
    unit: "м²".to_string(),
    base_price: 400.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 437,
    code: "BUD-037".to_string(),
    name: "Монтаж плінтуса МДФ".to_string(),
    description: "Монтаж плінтуса з МДФ".to_string(),
    unit: "м.п".to_string(),
    base_price: 350.0,
    material_price: 0.0,
    complexity: Complexity::Simple,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 438,
    code: "BUD-038".to_string(),
    name: "Монтаж плінтуса прихованого монтажу".to_string(),
    description: "Монтаж плінтуса системи прихованого монтажу".to_string(),
    unit: "м.п".to_string(),
    base_price: 700.0,
    material_price: 0.0,
    complexity: Complexity::Complex,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 439,
    code: "BUD-039".to_string(),
    name: "Монтаж міжкімнатних дверей".to_string(),
    description: "Монтаж та регулювання міжкімнатного дверного блоку".to_string(),
    unit: "шт".to_string(),
    base_price: 3000.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
PricePosition {
    id: 440,
    code: "BUD-040".to_string(),
    name: "Заміна вхідних дверей".to_string(),
    description: "Демонтаж старих та монтаж нових вхідних дверей".to_string(),
    unit: "шт".to_string(),
    base_price: 4800.0,
    material_price: 0.0,
    complexity: Complexity::Complex,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 441,
    code: "BUD-041".to_string(),
    name: "Підвісна стеля Армстронг".to_string(),
    description: "Монтаж підвісної стелі типу Армстронг".to_string(),
    unit: "м²".to_string(),
    base_price: 400.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 442,
    code: "BUD-042".to_string(),
    name: "Стеля Грильято".to_string(),
    description: "Монтаж підвісної стелі типу Грильято".to_string(),
    unit: "м²".to_string(),
    base_price: 450.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: false,
    guarantee_months: 0,
},
PricePosition {
    id: 443,
    code: "BUD-043".to_string(),
    name: "Натяжна стеля (робота + матеріал)".to_string(),
    description: "Монтаж натяжної стелі з матеріалом".to_string(),
    unit: "м²".to_string(),
    base_price: 800.0,
    material_price: 0.0,
    complexity: Complexity::Medium,
    min_quantity: 1,
    discount_percent: 0,
    popular: true,
    guarantee_months: 0,
},
    ],
}]
}
