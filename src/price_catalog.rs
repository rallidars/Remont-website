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
    let sections = vec![
        PriceSection {
            id: 1,
            title: "Демонтажні роботи".to_string(),
            icon: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
            color: "#ef4444",
            positions: vec![
                PricePosition {
                    id: 101,
                    code: "DEM-001".to_string(),
                    name: "Демонтаж стін".to_string(),
                    description: "Розбирання перегородок з цегли, бетону або гіпсокартону з виносом сміття".to_string(),
                    unit: "м²".to_string(),
                    base_price: 250.0,
                    material_price: 0.0,
                    complexity: Complexity::Medium,
                    min_quantity: 5,
                    discount_percent: 10,
                    popular: true,
                    guarantee_months: 0,
                },
                PricePosition {
                    id: 102,
                    code: "DEM-002".to_string(),
                    name: "Демонтаж підлоги".to_string(),
                    description: "Зняття паркету, ламінату, плитки, демонтаж стяжки".to_string(),
                    unit: "м²".to_string(),
                    base_price: 180.0,
                    material_price: 0.0,
                    complexity: Complexity::Simple,
                    min_quantity: 10,
                    discount_percent: 5,
                    popular: false,
                    guarantee_months: 0,
                },
                PricePosition {
                    id: 103,
                    code: "DEM-003".to_string(),
                    name: "Демонтаж сантехніки".to_string(),
                    description: "Демонтаж ванни, унітазу, раковин, змішувачів".to_string(),
                    unit: "комплект".to_string(),
                    base_price: 800.0,
                    material_price: 0.0,
                    complexity: Complexity::Simple,
                    min_quantity: 1,
                    discount_percent: 0,
                    popular: false,
                    guarantee_months: 0,
                },
                PricePosition {
                    id: 104,
                    code: "DEM-004".to_string(),
                    name: "Демонтаж вікон та дверей".to_string(),
                    description: "Зняття старих віконних та дверних блоків".to_string(),
                    unit: "шт".to_string(),
                    base_price: 350.0,
                    material_price: 0.0,
                    complexity: Complexity::Simple,
                    min_quantity: 2,
                    discount_percent: 15,
                    popular: true,
                    guarantee_months: 0,
                },
            ],
        },
        PriceSection {
            id: 2,
            title: "Монтаж вікон та дверей".to_string(),
            icon: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
            color: "#3b82f6",
            positions: vec![
                PricePosition {
                    id: 201,
                    code: "WIN-001".to_string(),
                    name: "Встановлення металопластикового вікна".to_string(),
                    description: "Монтаж вікна, запінювання, встановлення підвіконня, відливів, регулювання фурнітури".to_string(),
                    unit: "шт".to_string(),
                    base_price: 2200.0,
                    material_price: 350.0,
                    complexity: Complexity::Medium,
                    min_quantity: 1,
                    discount_percent: 5,
                    popular: true,
                    guarantee_months: 12,
                },
                PricePosition {
                    id: 202,
                    code: "WIN-002".to_string(),
                    name: "Встановлення вхідних дверей".to_string(),
                    description: "Монтаж металевих дверей, врізка замків, запінювання".to_string(),
                    unit: "шт".to_string(),
                    base_price: 2800.0,
                    material_price: 400.0,
                    complexity: Complexity::Medium,
                    min_quantity: 1,
                    discount_percent: 0,
                    popular: false,
                    guarantee_months: 12,
                },
                PricePosition {
                    id: 203,
                    code: "WIN-003".to_string(),
                    name: "Встановлення міжкімнатних дверей".to_string(),
                    description: "Монтаж дверної коробки, навішування полотна, встановлення наличників".to_string(),
                    unit: "шт".to_string(),
                    base_price: 1500.0,
                    material_price: 200.0,
                    complexity: Complexity::Simple,
                    min_quantity: 2,
                    discount_percent: 10,
                    popular: true,
                    guarantee_months: 6,
                },
                PricePosition {
                    id: 204,
                    code: "WIN-004".to_string(),
                    name: "Скління балкона".to_string(),
                    description: "Монтаж алюмінієвого або ПВХ профілю, склопакетів".to_string(),
                    unit: "м.пог.".to_string(),
                    base_price: 1800.0,
                    material_price: 250.0,
                    complexity: Complexity::Complex,
                    min_quantity: 3,
                    discount_percent: 7,
                    popular: false,
                    guarantee_months: 12,
                },
            ],
        },
        PriceSection {
            id: 3,
            title: "Електромонтажні роботи".to_string(),
            icon: "M13 10V3L4 14h7v7l9-11h-7z",
            color: "#eab308",
            positions: vec![
                PricePosition {
                    id: 301,
                    code: "ELC-001".to_string(),
                    name: "Заміна електропроводки".to_string(),
                    description: "Штроблення стін, прокладка кабелю, монтаж розподільчих коробок".to_string(),
                    unit: "точка".to_string(),
                    base_price: 380.0,
                    material_price: 120.0,
                    complexity: Complexity::Complex,
                    min_quantity: 10,
                    discount_percent: 12,
                    popular: true,
                    guarantee_months: 24,
                },
                PricePosition {
                    id: 302,
                    code: "ELC-002".to_string(),
                    name: "Встановлення розеток та вимикачів".to_string(),
                    description: "Монтаж підрозетників, підключення, встановлення механізмів".to_string(),
                    unit: "шт".to_string(),
                    base_price: 180.0,
                    material_price: 0.0,
                    complexity: Complexity::Simple,
                    min_quantity: 3,
                    discount_percent: 5,
                    popular: true,
                    guarantee_months: 12,
                },
                PricePosition {
                    id: 303,
                    code: "ELC-003".to_string(),
                    name: "Монтаж електрощитка".to_string(),
                    description: "Встановлення щитка, монтаж автоматів, УЗО, збірка схеми".to_string(),
                    unit: "шт".to_string(),
                    base_price: 2200.0,
                    material_price: 500.0,
                    complexity: Complexity::Complex,
                    min_quantity: 1,
                    discount_percent: 0,
                    popular: false,
                    guarantee_months: 24,
                },
                PricePosition {
                    id: 304,
                    code: "ELC-004".to_string(),
                    name: "Монтаж люстри".to_string(),
                    description: "Збірка, підключення, монтаж на стелю".to_string(),
                    unit: "шт".to_string(),
                    base_price: 450.0,
                    material_price: 0.0,
                    complexity: Complexity::Simple,
                    min_quantity: 1,
                    discount_percent: 0,
                    popular: false,
                    guarantee_months: 6,
                },
            ],
        },
        PriceSection {
            id: 4,
            title: "Сантехнічні роботи".to_string(),
            icon: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
            color: "#10b981",
            positions: vec![
                PricePosition {
                    id: 401,
                    code: "PLM-001".to_string(),
                    name: "Заміна стояка водопостачання".to_string(),
                    description: "Демонтаж старого стояка, монтаж нового, врізка в систему".to_string(),
                    unit: "шт".to_string(),
                    base_price: 3200.0,
                    material_price: 600.0,
                    complexity: Complexity::Complex,
                    min_quantity: 1,
                    discount_percent: 0,
                    popular: false,
                    guarantee_months: 12,
                },
                PricePosition {
                    id: 402,
                    code: "PLM-002".to_string(),
                    name: "Встановлення унітазу".to_string(),
                    description: "Монтаж, підключення до каналізації, герметизація".to_string(),
                    unit: "шт".to_string(),
                    base_price: 950.0,
                    material_price: 150.0,
                    complexity: Complexity::Simple,
                    min_quantity: 1,
                    discount_percent: 5,
                    popular: true,
                    guarantee_months: 6,
                },
                PricePosition {
                    id: 403,
                    code: "PLM-003".to_string(),
                    name: "Встановлення ванни".to_string(),
                    description: "Монтаж, підключення, герметизація".to_string(),
                    unit: "шт".to_string(),
                    base_price: 1800.0,
                    material_price: 200.0,
                    complexity: Complexity::Medium,
                    min_quantity: 1,
                    discount_percent: 0,
                    popular: false,
                    guarantee_months: 6,
                },
                PricePosition {
                    id: 404,
                    code: "PLM-004".to_string(),
                    name: "Встановлення змішувача".to_string(),
                    description: "Монтаж, підключення до водопроводу".to_string(),
                    unit: "шт".to_string(),
                    base_price: 450.0,
                    material_price: 0.0,
                    complexity: Complexity::Simple,
                    min_quantity: 1,
                    discount_percent: 10,
                    popular: true,
                    guarantee_months: 3,
                },
            ],
        },
        PriceSection {
            id: 5,
            title: "Оздоблювальні роботи".to_string(),
            icon: "M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z",
            color: "#a855f7",
            positions: vec![
                PricePosition {
                    id: 501,
                    code: "FIN-001".to_string(),
                    name: "Штукатурка стін".to_string(),
                    description: "Вирівнювання стін по маяках, нанесення штукатурки".to_string(),
                    unit: "м²".to_string(),
                    base_price: 320.0,
                    material_price: 80.0,
                    complexity: Complexity::Medium,
                    min_quantity: 20,
                    discount_percent: 15,
                    popular: true,
                    guarantee_months: 12,
                },
                PricePosition {
                    id: 502,
                    code: "FIN-002".to_string(),
                    name: "Шпаклівка стін".to_string(),
                    description: "Нанесення шпаклівки, шліфування".to_string(),
                    unit: "м²".to_string(),
                    base_price: 150.0,
                    material_price: 30.0,
                    complexity: Complexity::Simple,
                    min_quantity: 20,
                    discount_percent: 10,
                    popular: true,
                    guarantee_months: 6,
                },
                PricePosition {
                    id: 503,
                    code: "FIN-003".to_string(),
                    name: "Поклейка шпалер".to_string(),
                    description: "Підготовка, нарізка, поклейка, стикування малюнка".to_string(),
                    unit: "м²".to_string(),
                    base_price: 120.0,
                    material_price: 0.0,
                    complexity: Complexity::Simple,
                    min_quantity: 15,
                    discount_percent: 5,
                    popular: true,
                    guarantee_months: 3,
                },
                PricePosition {
                    id: 504,
                    code: "FIN-004".to_string(),
                    name: "Фарбування стін".to_string(),
                    description: "Ґрунтовка, фарбування в 2 шари".to_string(),
                    unit: "м²".to_string(),
                    base_price: 90.0,
                    material_price: 20.0,
                    complexity: Complexity::Simple,
                    min_quantity: 20,
                    discount_percent: 0,
                    popular: false,
                    guarantee_months: 3,
                },
            ],
        },
        PriceSection {
            id: 6,
            title: "Підлогові покриття".to_string(),
            icon: "M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z",
            color: "#ec4899",
            positions: vec![
                PricePosition {
                    id: 601,
                    code: "FLR-001".to_string(),
                    name: "Укладка ламінату".to_string(),
                    description: "Підготовка основи, підкладка, укладка, монтаж плінтусів".to_string(),
                    unit: "м²".to_string(),
                    base_price: 280.0,
                    material_price: 50.0,
                    complexity: Complexity::Simple,
                    min_quantity: 15,
                    discount_percent: 10,
                    popular: true,
                    guarantee_months: 6,
                },
                PricePosition {
                    id: 602,
                    code: "FLR-002".to_string(),
                    name: "Укладка плитки на підлогу".to_string(),
                    description: "Підготовка, укладка, затирання швів".to_string(),
                    unit: "м²".to_string(),
                    base_price: 550.0,
                    material_price: 100.0,
                    complexity: Complexity::Medium,
                    min_quantity: 10,
                    discount_percent: 5,
                    popular: true,
                    guarantee_months: 12,
                },
                PricePosition {
                    id: 603,
                    code: "FLR-003".to_string(),
                    name: "Стяжка підлоги".to_string(),
                    description: "Заливка цементно-піщаної стяжки, вирівнювання".to_string(),
                    unit: "м²".to_string(),
                    base_price: 380.0,
                    material_price: 120.0,
                    complexity: Complexity::Medium,
                    min_quantity: 20,
                    discount_percent: 8,
                    popular: false,
                    guarantee_months: 24,
                },
                PricePosition {
                    id: 604,
                    code: "FLR-004".to_string(),
                    name: "Паркетні роботи".to_string(),
                    description: "Укладка штучного паркету, шліфовка, лакування".to_string(),
                    unit: "м²".to_string(),
                    base_price: 850.0,
                    material_price: 150.0,
                    complexity: Complexity::Complex,
                    min_quantity: 10,
                    discount_percent: 0,
                    popular: false,
                    guarantee_months: 24,
                },
            ],
        },
    ];

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
        <div class="price-catalog-main">
    <div class="price-list-app">
        <div class="pl-main">
            <div class="pl-header">
                <h1 class="pl-title">
                    <span class="pl-title-main">{"Прайс-лист"}</span>
                    <span class="pl-title-year">{"2026"}</span>
                </h1>
            </div>

            <div class="pl-banner">
                <div class="banner-icon">
                    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <path d="M12 2v4M12 22v-4M4 12H2h2m16 0h2-2M5.636 5.636l2.828 2.828M15.536 15.536l2.828 2.828M18.364 5.636l-2.828 2.828M8.464 15.536l-2.828 2.828" stroke-width="2"/>
                    </svg>
                </div>
                <div class="banner-text">
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
                                <div class="section-header-left">
                                    <div class="section-icon" style={format!("background: {}", section.color)}>
                                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white">
                                            <path d={section.icon} stroke-width="2"/>
                                        </svg>
                                    </div>
                                    <h2 class="section-title">{&section.title}</h2>
                                </div>
                                <div class="section-header-right">
                                    <span class="section-count">{section.positions.len()} {if section.positions.len() == 1 { "позиція" } else { "позицій" }}</span>
                                    <svg
                                        class="section-arrow"
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
                                        <div class="price-table-header">
                                            <div class="col-code">{"Код"}</div>
                                            <div class="col-name">{"Назва роботи"}</div>
                                            <div class="col-unit">{"Од."}</div>
                                            <div class="col-price">{"Ціна, грн"}</div>
                                            <div class="col-materials">{"Матеріали"}</div>
                                            <div class="col-complexity">{"Складність"}</div>
                                            <div class="col-guarantee">{"Гарантія"}</div>
                                            <div class="col-actions"></div>
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
                                                <div class={classes!("price-row", if pos.popular { "popular" } else { "" })}>
                                                    <div class="col-code">
                                                        <span class="pos-code">{&pos.code}</span>
                                                    </div>

                                                    <div class="col-name">
                                                        <div class="pos-name">{&pos.name}</div>
                                                        <div class="pos-desc">{&pos.description}</div>
                                                    </div>

                                                    <div class="col-unit">
                                                        <span class="pos-unit">{&pos.unit}</span>
                                                    </div>

                                                    <div class="col-price">
                                                        <div class={classes!("price-value", if *show_discount { "with-discount" } else { "" })}>
                                                            {if *show_discount {
                                                                html! {
                                                                    <>
                                                                        <span class="old-price">{format!("{:.0}", pos.base_price)}</span>
                                                                        <span class="new-price">{format!("{:.0}", final_price)}</span>
                                                                    </>
                                                                }
                                                            } else {
                                                                html! { format!("{:.0}", pos.base_price) }
                                                            }}
                                                        </div>
                                                    </div>

                                                    <div class="col-materials">
                                                        {if pos.material_price > 0.0 {
                                                            html! { format!("{:.0} грн", pos.material_price) }
                                                        } else {
                                                            html! { <span class="no-materials">{"включено"}</span> }
                                                        }}
                                                    </div>

                                                    <div class="col-complexity">
                                                        <div class="complexity-indicator" title={pos.complexity.as_str()}>
                                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                                                <path d={pos.complexity.icon()} stroke-width="2"/>
                                                            </svg>
                                                        </div>
                                                    </div>

                                                    <div class="col-guarantee">
                                                        {if pos.guarantee_months > 0 {
                                                            html! { format!("{} міс.", pos.guarantee_months) }
                                                        } else {
                                                            html! { <span class="no-guarantee">{"немає"}</span> }
                                                        }}
                                                    </div>

                                                    <div class="col-actions">
                                                        {if pos.discount_percent > 0 {
                                                            html! {
                                                                <button
                                                                    class={classes!("action-btn", "discount-btn", if *show_discount { "active" } else { "" })}
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
                                                            class="action-btn info-btn"
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
                <div class="info-grid">
                    <div class="info-card">
                        <div class="info-card-icon" style="background: #3b82f6">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white">
                                <path d="M12 2l7 4v6l-7 4-7-4V6l7-4z" stroke-width="2"/>
                            </svg>
                        </div>
                        <h3>{"Гарантія на роботи"}</h3>
                        <p>{"Від 3 до 24 місяців на всі види робіт"}</p>
                    </div>

                    <div class="info-card">
                        <div class="info-card-icon" style="background: #10b981">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white">
                                <path d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" stroke-width="2"/>
                            </svg>
                        </div>
                        <h3>{"Сертифіковані майстри"}</h3>
                        <p>{"Працюємо тільки з перевіреними фахівцями"}</p>
                    </div>

                    <div class="info-card">
                        <div class="info-card-icon" style="background: #a855f7">
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
                                <div class="modal-header">
                                    <h2>{&pos.name}</h2>
                                    <button class="modal-close" onclick={close_modal.clone()}>
                                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                            <path d="M6 18L18 6M6 6l12 12" stroke-width="2"/>
                                        </svg>
                                    </button>
                                </div>

                                <div class="modal-body">
                                    <div class="modal-row">
                                        <span class="modal-label">{"Код:"}</span>
                                        <span class="modal-value">{&pos.code}</span>
                                    </div>
                                    <div class="modal-row">
                                        <span class="modal-label">{"Опис:"}</span>
                                        <span class="modal-value">{&pos.description}</span>
                                    </div>
                                    <div class="modal-row">
                                        <span class="modal-label">{"Одиниця виміру:"}</span>
                                        <span class="modal-value">{&pos.unit}</span>
                                    </div>
                                    <div class="modal-row">
                                        <span class="modal-label">{"Ціна за роботу:"}</span>
                                        <span class="modal-value price">{format!("{:.0} грн", pos.base_price)}</span>
                                    </div>
                                    <div class="modal-row">
                                        <span class="modal-label">{"Вартість матеріалів:"}</span>
                                        <span class="modal-value">{if pos.material_price > 0.0 { format!("{:.0} грн", pos.material_price) } else { "включено в ціну".to_string() }}</span>
                                    </div>
                                    <div class="modal-row">
                                        <span class="modal-label">{"Складність:"}</span>
                                        <span class="modal-value">{pos.complexity.as_str()}</span>
                                    </div>
                                    <div class="modal-row">
                                        <span class="modal-label">{"Мінімальне замовлення:"}</span>
                                        <span class="modal-value">{format!("{} {}", pos.min_quantity, pos.unit)}</span>
                                    </div>
                                    <div class="modal-row">
                                        <span class="modal-label">{"Гарантія:"}</span>
                                        <span class="modal-value">{if pos.guarantee_months > 0 { format!("{} місяців", pos.guarantee_months) } else { "без гарантії".to_string() }}</span>
                                    </div>
                                    {if pos.discount_percent > 0 {
                                        html! {
                                            <div class="modal-row highlight">
                                                <span class="modal-label">{"Знижка:"}</span>
                                                <span class="modal-value">{format!("{}% при замовленні від {} {}", pos.discount_percent, pos.min_quantity * 2, pos.unit)}</span>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>

                                <div class="modal-footer">
                                    <button class="modal-btn primary">
                                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                            <path d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" stroke-width="2"/>
                                        </svg>
                                        {"Замовити"}
                                    </button>
                                    <button class="modal-btn secondary" onclick={close_modal.clone()}>
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
