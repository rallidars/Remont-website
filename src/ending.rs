use gloo_timers::callback::{Interval, Timeout};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GuaranteeCategory {
    Quality,
    Terms,
    Materials,
    Service,
    Support,
}

#[derive(Debug, Clone)]
pub struct GuaranteeCard {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub category: GuaranteeCategory,
    pub icon: &'static str,
    pub color: String,
    pub details: Vec<String>,
    pub is_certified: bool,
    pub certificate_number: Option<String>,
    pub coverage_years: u8,
}

#[derive(Debug, Clone)]
pub struct WarrantyPeriod {
    pub id: usize,
    pub name: String,
    pub duration: String,
    pub coverage: Vec<String>,
    pub icon: &'static str,
    pub color: String,
    pub is_extendable: bool,
}

#[derive(Debug, Clone)]
pub struct QualityMetric {
    pub id: usize,
    pub name: String,
    pub value: u8, // 0-100
    pub target: u8,
    pub unit: String,
    pub trend: i8, // -10 to 10
    pub color: String,
}

#[derive(Debug, Clone, Default)]
pub struct SealOfQuality {
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: f32,
    pub glow_intensity: f32,
    pub is_verified: bool,
}

#[component]
pub fn FinalGuarantees() -> Html {
    let active_category = use_state(|| GuaranteeCategory::Quality);
    let selected_guarantee = use_state(|| Option::<usize>::None);
    let show_certificate = use_state(|| false);
    let seal_of_quality = use_state(|| SealOfQuality::default());
    let animation_state = use_state(|| "enter".to_string());
    let particles = use_state(|| Vec::<(f32, f32, f32, f32)>::new()); // x, y, size, speed
    let metrics_visible = use_state(|| false);
    let warranty_timeline = use_state(|| 0u32); // 0-100%
    let trust_score = use_state(|| 0u8); // 0-100

    let categories = vec![
        (GuaranteeCategory::Quality, "Якість", "#4299e1"),
        (GuaranteeCategory::Terms, "Терміни", "#38b2ac"),
        (GuaranteeCategory::Materials, "Матеріали", "#ed8936"),
        (GuaranteeCategory::Service, "Обслуговування", "#9f7aea"),
        (GuaranteeCategory::Support, "Підтримка", "#f56565"),
    ];

    let guarantees = vec![
        GuaranteeCard {
            id: 0,
            title: "Гарантія якості робіт".to_string(),
            description: "Гарантуємо відповідність всіх робіт вищим стандартам якості".to_string(),
            category: GuaranteeCategory::Quality,
            icon: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
            color: "#4299e1".to_string(),
            details: vec![
                "Контроль якості на кожному етапі".to_string(),
                "Відповідність ДБН та СНиП".to_string(),
                "Професійний нагляд інженера".to_string(),
                "Фото- та відеозвітність".to_string(),
            ],
            is_certified: true,
            certificate_number: Some("QC-2023-001".to_string()),
            coverage_years: 5,
        },
        GuaranteeCard {
            id: 1,
            title: "Точність термінів".to_string(),
            description: "Гарантуємо дотримання узгоджених термінів виконання робіт".to_string(),
            category: GuaranteeCategory::Terms,
            icon: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z",
            color: "#38b2ac".to_string(),
            details: vec![
                "Детальний графік робіт".to_string(),
                "Щоденний звіт про прогрес".to_string(),
                "Штраф за прострочення".to_string(),
                "Резервний план на випадок форс-мажору".to_string(),
            ],
            is_certified: true,
            certificate_number: Some("TD-2023-001".to_string()),
            coverage_years: 3,
        },
        GuaranteeCard {
            id: 2,
            title: "Оригінальність матеріалів".to_string(),
            description: "Гарантуємо оригінальність та якість всіх використовуваних матеріалів".to_string(),
            category: GuaranteeCategory::Materials,
            icon: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
            color: "#ed8936".to_string(),
            details: vec![
                "Сертифікати на всі матеріали".to_string(),
                "Прямі поставки від виробників".to_string(),
                "Перевірка перед використанням".to_string(),
                "Повернення неякісних матеріалів".to_string(),
            ],
            is_certified: true,
            certificate_number: Some("OM-2023-001".to_string()),
            coverage_years: 10,
        },
        GuaranteeCard {
            id: 3,
            title: "Післягарантійне обслуговування".to_string(),
            description: "Гарантуємо підтримку та обслуговування після завершення робіт".to_string(),
            category: GuaranteeCategory::Service,
            icon: "M18.364 5.636l-3.536 3.536m0 5.656l3.536 3.536M9.172 9.172L5.636 5.636m3.536 9.192l-3.536 3.536M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-5 0a4 4 0 11-8 0 4 4 0 018 0z",
            color: "#9f7aea".to_string(),
            details: vec![
                "Технічна підтримка 24/7".to_string(),
                "Екстрені виїзди".to_string(),
                "Планове технічне обслуговування".to_string(),
                "Консультації з експлуатації".to_string(),
            ],
            is_certified: true,
            certificate_number: Some("PS-2023-001".to_string()),
            coverage_years: 2,
        },
        GuaranteeCard {
            id: 4,
            title: "Повна прозорість".to_string(),
            description: "Гарантуємо повну прозорість процесу та фінансових розрахунків".to_string(),
            category: GuaranteeCategory::Support,
            icon: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
            color: "#f56565".to_string(),
            details: vec![
                "Детальний кошторис".to_string(),
                "Фіксована ціна договору".to_string(),
                "Відсутність прихованих платежів".to_string(),
                "Онлайн-доступ до звітів".to_string(),
            ],
            is_certified: true,
            certificate_number: Some("FT-2023-001".to_string()),
            coverage_years: 1,
        },
    ];

    let warranty_periods = vec![
        WarrantyPeriod {
            id: 0,
            name: "Базова гарантія".to_string(),
            duration: "12 місяців".to_string(),
            coverage: vec![
                "Усі виконані роботи".to_string(),
                "Установлене обладнання".to_string(),
                "Монтажні роботи".to_string(),
            ],
            icon: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
            color: "#4299e1".to_string(),
            is_extendable: true,
        },
        WarrantyPeriod {
            id: 1,
            name: "Розширена гарантія".to_string(),
            duration: "24 місяці".to_string(),
            coverage: vec![
                "Усе з базової гарантії".to_string(),
                "Обладнання Smart Home".to_string(),
                "Системи безпеки".to_string(),
                "Кондиціонування".to_string(),
            ],
            icon: "M13 10V3L4 14h7v7l9-11h-7z",
            color: "#38b2ac".to_string(),
            is_extendable: true,
        },
        WarrantyPeriod {
            id: 2,
            name: "Повна гарантія".to_string(),
            duration: "60 місяців".to_string(),
            coverage: vec![
                "Усе з розширеної гарантії".to_string(),
                "Конструктивні елементи".to_string(),
                "Інженерні системи".to_string(),
                "Фінішне оздоблення".to_string(),
            ],
            icon: "M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z",
            color: "#ed8936".to_string(),
            is_extendable: false,
        },
    ];

    let quality_metrics = vec![
        QualityMetric {
            id: 0,
            name: "Задоволеність клієнтів".to_string(),
            value: 98,
            target: 95,
            unit: "%".to_string(),
            trend: 2,
            color: "#48bb78".to_string(),
        },
        QualityMetric {
            id: 1,
            name: "Дотримання термінів".to_string(),
            value: 96,
            target: 90,
            unit: "%".to_string(),
            trend: 3,
            color: "#4299e1".to_string(),
        },
        QualityMetric {
            id: 2,
            name: "Якість матеріалів".to_string(),
            value: 99,
            target: 97,
            unit: "%".to_string(),
            trend: 1,
            color: "#ed8936".to_string(),
        },
        QualityMetric {
            id: 3,
            name: "Швидкість реагування".to_string(),
            value: 95,
            target: 85,
            unit: "%".to_string(),
            trend: 4,
            color: "#9f7aea".to_string(),
        },
    ];

    // Filter guarantees by category
    let filtered_guarantees = if *active_category == GuaranteeCategory::Quality {
        guarantees.clone()
    } else {
        guarantees
            .iter()
            .filter(|g| g.category == *active_category)
            .cloned()
            .collect::<Vec<_>>()
    };

    // Selected guarantee details
    let guarantee_details = selected_guarantee
        .as_ref()
        .and_then(|&id| guarantees.iter().find(|g| g.id == id));

    // Initialize particles
    {
        let particles = particles.clone();
        use_effect_with((), move |_| {
            let mut initial_particles = Vec::new();
            for _ in 0..50 {
                initial_particles.push((
                    js_sys::Math::random() as f32 * 100.0,
                    js_sys::Math::random() as f32 * 100.0,
                    js_sys::Math::random() as f32 * 4.0 + 1.0,
                    js_sys::Math::random() as f32 * 0.5 + 0.1,
                ));
            }
            particles.set(initial_particles);
            || ()
        });
    }

    // Animate particles
    {
        let particles = particles.clone();
        use_effect_with(particles.clone(), move |_| {
            let interval = Interval::new(50, move || {
                particles.set(
                    particles
                        .iter()
                        .map(|&(x, y, size, speed)| {
                            let new_y = (y + speed) % 100.0;
                            (x, new_y, size, speed)
                        })
                        .collect(),
                );
            });
            move || {
                interval.cancel();
            }
        });
    }

    // Animate warranty timeline
    {
        let warranty_timeline = warranty_timeline.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(30, move || {
                if *warranty_timeline < 100 {
                    warranty_timeline.set(*warranty_timeline + 1);
                }
            });
            move || {
                interval.cancel();
            }
        });
    }

    // Animate trust score
    {
        let trust_score = trust_score.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(20, move || {
                if *trust_score < 97 {
                    trust_score.set(*trust_score + 1);
                }
            });
            move || {
                interval.cancel();
            }
        });
    }

    // Animate seal of quality
    {
        let seal_of_quality = seal_of_quality.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(16, move || {
                let mut seal = (*seal_of_quality).clone();
                seal.rotation += 0.3;
                seal.glow_intensity = 0.5 + (seal.rotation * 0.01).sin().abs() * 0.5;
                seal_of_quality.set(seal);
            });
            move || {
                interval.cancel();
            }
        });
    }

    // Show metrics after delay
    {
        let metrics_visible = metrics_visible.clone();
        use_effect_with((), move |_| {
            Timeout::new(1000, move || {
                metrics_visible.set(true);
            })
            .forget();
            || ()
        });
    }

    let select_category = {
        let active_category = active_category.clone();
        let animation_state = animation_state.clone();
        Callback::from(move |category: GuaranteeCategory| {
            let active_category = active_category.clone();
            let animation_state = animation_state.clone();
            animation_state.set("exit".to_string());
            Timeout::new(200, move || {
                active_category.set(category);
                animation_state.set("enter".to_string());
            })
            .forget();
        })
    };

    let select_guarantee = {
        let selected_guarantee = selected_guarantee.clone();
        let show_certificate = show_certificate.clone();
        Callback::from(move |guarantee_id: usize| {
            selected_guarantee.set(Some(guarantee_id));
            show_certificate.set(false);
        })
    };

    let toggle_certificate = {
        let show_certificate = show_certificate.clone();
        Callback::from(move |_| {
            show_certificate.set(!*show_certificate);
        })
    };

    let verify_seal = {
        let seal_of_quality = seal_of_quality.clone();
        Callback::from(move |_| {
            let mut seal = (*seal_of_quality).clone();
            seal.is_verified = !seal.is_verified;
            seal.scale = if seal.is_verified { 1.2 } else { 1.0 };
            seal_of_quality.set(seal);
        })
    };

    let reset_animations = {
        let warranty_timeline = warranty_timeline.clone();
        let trust_score = trust_score.clone();
        Callback::from(move |_| {
            warranty_timeline.set(0);
            trust_score.set(0);
            Timeout::new(100, move || {
                // Restart animations
            })
            .forget();
        })
    };

    // Calculate overall guarantee score
    let guarantee_score = guarantees
        .iter()
        .map(|g| g.coverage_years as f32)
        .sum::<f32>()
        / guarantees.len() as f32;

    // Calculate total coverage years
    let total_coverage: u32 = guarantees.iter().map(|g| g.coverage_years as u32).sum();

    html! {
        <section class="guarantees-section">
            <div class="guarantees-background">
                {for particles.iter().map(|&(x, y, size, _)| {
                    html! {
                        <div
                            class="particle"
                            style={format!(
                                "left: {}%; top: {}%; width: {}px; height: {}px;",
                                x, y, size, size
                            )}
                        ></div>
                    }
                })}
                <div class="bg-gradient"></div>
                <div class="bg-sparkles"></div>
            </div>

            <div class="guarantees-container">
                <div class="guarantees-header">
                    <div class="header-content">
                        <h2 class="guarantees-title">
                            <span class="title-aura">{"НАШІ ГАРАНТІЇ"}</span>
                        </h2>
                        <p class="guarantees-subtitle">
                            {"Ми несемо повну відповідальність за кожен проект та гарантуємо найвищу якість"}
                        </p>
                    </div>

                    <div class="trust-badge">
                        <div class="badge-content">
                            <div class="badge-icon">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                    <path d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                                          stroke="currentColor" stroke-width="2"
                                          stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <div class="badge-stats">
                                <div class="score-value">{format!("{}%", *trust_score)}</div>
                                <div class="score-label">{"Рівень довіри"}</div>
                            </div>
                        </div>
                        <div class="badge-glow"></div>
                    </div>
                </div>

                <div class="guarantees-stats">
                    <div class="stats-grid">
                        <div class="stat-card">
                            <div class="stat-icon">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                    <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                          stroke="currentColor" stroke-width="2"
                                          stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <div class="stat-content">
                                <div class="stat-value">{format!("{:.1} років", guarantee_score)}</div>
                                <div class="stat-label">{"Середня гарантія"}</div>
                            </div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-icon">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                    <path d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                                          stroke="currentColor" stroke-width="2"
                                          stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <div class="stat-content">
                                <div class="stat-value">{guarantees.len()}</div>
                                <div class="stat-label">{"Види гарантій"}</div>
                            </div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-icon">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                    <path d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                                          stroke="currentColor" stroke-width="2"
                                          stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <div class="stat-content">
                                <div class="stat-value">{total_coverage}</div>
                                <div class="stat-label">{"Років покриття"}</div>
                            </div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-icon">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                    <path d="M13 10V3L4 14h7v7l9-11h-7z"
                                          stroke="currentColor" stroke-width="2"
                                          stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <div class="stat-content">
                                <div class="stat-value">{"100%"}</div>
                                <div class="stat-label">{"Сертифіковано"}</div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="guarantees-content">
                    <div class="content-left">
                        <div class="categories-panel">
                            <div class="panel-header">
                                <h3 class="panel-title">{"Гарантійні обіцянки"}</h3>
                                <div class="panel-subtitle">{"Оберіть категорію для детальної інформації"}</div>
                            </div>

                            <div class="categories-grid">
                                {for categories.iter().map(|(category, label, color)| {
                                    let is_active = *active_category == *category;
                                    let category_clone = category.clone();
                                    let select_category = select_category.clone();

                                    html! {
                                        <button
                                            class={if is_active { "category-btn active" } else { "category-btn" }}
                                            onclick={Callback::from(move |_| select_category.emit(category_clone.clone()))}
                                            style={format!("--category-color: {}", color)}
                                        >
                                            <div class="category-icon">
                                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                    {match category {
                                                        GuaranteeCategory::Quality => html! {
                                                            <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        },
                                                        GuaranteeCategory::Terms => html! {
                                                            <path d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        },
                                                        GuaranteeCategory::Materials => html! {
                                                            <path d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        },
                                                        GuaranteeCategory::Service => html! {
                                                            <path d="M18.364 5.636l-3.536 3.536m0 5.656l3.536 3.536M9.172 9.172L5.636 5.636m3.536 9.192l-3.536 3.536M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-5 0a4 4 0 11-8 0 4 4 0 018 0z"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        },
                                                        GuaranteeCategory::Support => html! {
                                                            <path d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        },
                                                    }}
                                                </svg>
                                            </div>
                                            <span class="category-label">{*label}</span>
                                            <div class="category-count">
                                                {guarantees.iter().filter(|g| g.category == *category).count()}
                                            </div>
                                        </button>
                                    }
                                })}
                            </div>

                            <div class="seal-container" onclick={verify_seal}>
                                <div
                                    class={classes!("seal-of-quality", if seal_of_quality.is_verified { "verified" } else { "" })}
                                    style={format!(
                                        "transform: rotate({}deg) scale({}); --glow-intensity: {};",
                                        seal_of_quality.rotation,
                                        seal_of_quality.scale,
                                        seal_of_quality.glow_intensity
                                    )}
                                >
                                    <div class="seal-ring">
                                        <div class="ring-text">{"ГАРАНТІЯ ЯКОСТІ • ПОВНА ВІДПОВІДАЛЬНІСТЬ"}</div>
                                    </div>
                                    <div class="seal-center">
                                        <div class="center-shield">
                                            <svg width="48" height="48" viewBox="0 0 24 24" fill="none">
                                                <path d="M12 2L4 5v6.09c0 5.05 3.41 9.76 8 10.91 4.59-1.15 8-5.86 8-10.91V5l-8-3z"
                                                      stroke="currentColor" stroke-width="2"
                                                      stroke-linecap="round" stroke-linejoin="round"/>
                                            </svg>
                                        </div>
                                        <div class="center-text">{"GRADPRUS"}</div>
                                    </div>
                                    {if seal_of_quality.is_verified {
                                        html! {
                                            <div class="seal-verification">
                                                <div class="verification-badge">
                                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                        <path d="M22 11.08V12a10 10 0 11-5.93-9.14"
                                                              stroke="currentColor" stroke-width="2"
                                                              stroke-linecap="round" stroke-linejoin="round"/>
                                                        <path d="M22 4L12 14.01l-3-3"
                                                              stroke="currentColor" stroke-width="2"
                                                              stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                                <div class="seal-hint">
                                    {"Натисніть для верифікації печатки"}
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="content-center">
                        <div class={classes!("guarantees-grid", format!("animation-{}", *animation_state))}>
                            {for filtered_guarantees.iter().map(|guarantee| {
                                let is_selected = *selected_guarantee == Some(guarantee.id);
                                let guarantee_id = guarantee.id;
                                let select_guarantee = select_guarantee.clone();

                                html! {
                                    <div
                                        class={classes!(
                                            "guarantee-card",
                                            if is_selected { "selected" } else { "" },
                                            if guarantee.is_certified { "certified" } else { "" }
                                        )}
                                        onclick={Callback::from(move |_| select_guarantee.emit(guarantee_id))}
                                        style={format!("--guarantee-color: {}", guarantee.color)}
                                    >
                                        <div class="card-badge">
                                            <div class="badge-years">{guarantee.coverage_years}</div>
                                            <div class="badge-label">{"років"}</div>
                                        </div>

                                        <div class="card-icon" style={format!("background: {}", guarantee.color)}>
                                            <svg width="28" height="28" viewBox="0 0 24 24" fill="none">
                                                <path d={guarantee.icon}
                                                      stroke="white"
                                                      stroke-width="2"
                                                      stroke-linecap="round"
                                                      stroke-linejoin="round"/>
                                            </svg>
                                        </div>

                                        <div class="card-content">
                                            <h4 class="card-title">{&guarantee.title}</h4>
                                            <p class="card-description">{&guarantee.description}</p>

                                            <div class="card-details">
                                                {for guarantee.details.iter().take(2).map(|detail| {
                                                    html! {
                                                        <div class="detail-item">
                                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                                <path d="M5 13l4 4L19 7"
                                                                      stroke="currentColor" stroke-width="2"
                                                                      stroke-linecap="round" stroke-linejoin="round"/>
                                                            </svg>
                                                            <span>{detail}</span>
                                                        </div>
                                                    }
                                                })}
                                            </div>
                                        </div>

                                        <div class="card-footer">
                                            {if guarantee.is_certified {
                                                html! {
                                                    <div class="footer-certified">
                                                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                            <path d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        </svg>
                                                        {"Сертифіковано"}
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                        </div>
                                    </div>
                                }
                            })}
                        </div>

                        {if *metrics_visible {
                            html! {
                                <div class="metrics-panel">
                                    <div class="metrics-header">
                                        <h4 class="metrics-title">{"Показники якості"}</h4>
                                        <div class="metrics-update">{"Оновлено сьогодні"}</div>
                                    </div>

                                    <div class="metrics-grid">
                                        {for quality_metrics.iter().map(|metric| {
                                            html! {
                                                <div class="metric-card">
                                                    <div class="metric-header">
                                                        <div class="metric-name">{&metric.name}</div>
                                                        <div class="metric-trend" style={format!(
                                                            "color: {}",
                                                            if metric.trend >= 0 { "#48bb78" } else { "#f56565" }
                                                        )}>
                                                            {if metric.trend >= 0 {
                                                                html! {
                                                                    <>
                                                                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                                            <path d="M5 10l7-7 7 7M5 21v-7a2 2 0 012-2h10a2 2 0 012 2v7"
                                                                                  stroke="currentColor" stroke-width="2"
                                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                                        </svg>
                                                                        {format!("+{}", metric.trend)}
                                                                    </>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <>
                                                                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                                            <path d="M19 14l-7 7-7-7M5 3v7a2 2 0 002 2h10a2 2 0 002-2V3"
                                                                                  stroke="currentColor" stroke-width="2"
                                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                                        </svg>
                                                                        {metric.trend.abs()}
                                                                    </>
                                                                }
                                                            }}
                                                        </div>
                                                    </div>

                                                    <div class="metric-progress">
                                                        <div class="progress-track">
                                                            <div
                                                                class="progress-fill"
                                                                style={format!(
                                                                    "width: {}%; background: {}",
                                                                    metric.value,
                                                                    metric.color
                                                                )}
                                                            ></div>
                                                            <div
                                                                class="progress-target"
                                                                style={format!("left: {}%", metric.target)}
                                                            ></div>
                                                        </div>
                                                        <div class="progress-labels">
                                                            <div class="label-value">{format!("{}{}", metric.value, metric.unit)}</div>
                                                            <div class="label-target">{format!("Ціль: {}{}", metric.target, metric.unit)}</div>
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        })}
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </div>

                    <div class="content-right">
                        {if guarantee_details.is_some() {
                            let guarantee = guarantee_details.unwrap();

                            html! {
                                <div class="details-panel">
                                    <div class="panel-header">
                                        <h3 class="panel-title">{&guarantee.title}</h3>
                                        <button
                                            class="panel-certificate"
                                            onclick={toggle_certificate}
                                        >
                                            {if *show_certificate {
                                                html! {
                                                    <>
                                                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                            <path d="M6 18L18 6M6 6l12 12"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        </svg>
                                                        {"Закрити"}
                                                    </>
                                                }
                                            } else {
                                                html! {
                                                    <>
                                                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                            <path d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        </svg>
                                                        {"Сертифікат"}
                                                    </>
                                                }
                                            }}
                                        </button>
                                    </div>

                                    {if *show_certificate && guarantee.certificate_number.is_some() {
                                        html! {
                                            <div class="certificate-view">
                                                <div class="certificate-header">
                                                    <div class="certificate-title">{"СЕРТИФІКАТ ГАРАНТІЇ"}</div>
                                                    <div class="certificate-number">
                                                        {format!("№ {}", guarantee.certificate_number.as_ref().unwrap())}
                                                    </div>
                                                </div>

                                                <div class="certificate-body">
                                                    <div class="certificate-field">
                                                        <div class="field-label">{"Тип гарантії:"}</div>
                                                        <div class="field-value">{&guarantee.title}</div>
                                                    </div>
                                                    <div class="certificate-field">
                                                        <div class="field-label">{"Термін дії:"}</div>
                                                        <div class="field-value">{format!("{} років", guarantee.coverage_years)}</div>
                                                    </div>
                                                    <div class="certificate-field">
                                                        <div class="field-label">{"Дата видачі:"}</div>
                                                        <div class="field-value">{"01.01.2024"}</div>
                                                    </div>
                                                    <div class="certificate-field">
                                                        <div class="field-label">{"Дійсний до:"}</div>
                                                        <div class="field-value">{format!("01.01.{}", 2024 + guarantee.coverage_years as u32)}</div>
                                                    </div>
                                                </div>

                                                <div class="certificate-footer">
                                                    <div class="footer-seal">
                                                        <div class="seal-circle"></div>
                                                        <div class="seal-text">{"ПЕЧАТКА"}</div>
                                                    </div>
                                                    <div class="footer-signature">
                                                        <div class="signature-line"></div>
                                                        <div class="signature-text">{"Підпис директора"}</div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            <>
                                                <div class="panel-description">
                                                    <p>{&guarantee.description}</p>
                                                </div>

                                                <div class="panel-coverage">
                                                    <div class="coverage-header">
                                                        <div class="coverage-title">{"Що покриває гарантія"}</div>
                                                        <div class="coverage-years">{format!("{} років", guarantee.coverage_years)}</div>
                                                    </div>

                                                    <ul class="coverage-list">
                                                        {for guarantee.details.iter().map(|detail| {
                                                            html! {
                                                                <li class="coverage-item">
                                                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                                        <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                                                              stroke="currentColor" stroke-width="2"
                                                                              stroke-linecap="round" stroke-linejoin="round"/>
                                                                    </svg>
                                                                    <span>{detail}</span>
                                                                </li>
                                                            }
                                                        })}
                                                    </ul>
                                                </div>

                                                <div class="panel-actions">
                                                    <button class="btn-download">
                                                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                            <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M7 10l5 5 5-5M12 15V3"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        </svg>
                                                        {"Завантажити гарантію"}
                                                    </button>
                                                    <button class="btn-print">
                                                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                            <path d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        </svg>
                                                        {"Друк"}
                                                    </button>
                                                </div>
                                            </>
                                        }
                                    }}
                                </div>
                            }
                        } else {
                            html! {
                                <div class="welcome-panel">
                                    <div class="welcome-content">
                                        <div class="welcome-icon">
                                            <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
                                                <path d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                                                      stroke="currentColor" stroke-width="2"
                                                      stroke-linecap="round" stroke-linejoin="round"/>
                                            </svg>
                                        </div>
                                        <h3 class="welcome-title">{"Оберіть гарантію"}</h3>
                                        <p class="welcome-text">
                                            {"Натисніть на будь-яку гарантійну картку для перегляду детальної інформації, сертифікатів та умов покриття"}
                                        </p>
                                    </div>
                                </div>
                            }
                        }}

                        <div class="warranty-timeline">
                            <div class="timeline-header">
                                <h4 class="timeline-title">{"Гарантійні періоди"}</h4>
                                <div class="timeline-progress">
                                    <div class="progress-label">{"Прогрес:"}</div>
                                    <div class="progress-value">{format!("{}%", *warranty_timeline)}</div>
                                </div>
                            </div>

                            <div class="timeline-track">
                                <div
                                    class="timeline-fill"
                                    style={format!("width: {}%", *warranty_timeline)}
                                ></div>

                                {for warranty_periods.iter().enumerate().map(|(i, period)| {
                                    let position = (i as f32 + 1.0) / (warranty_periods.len() as f32 + 1.0) * 100.0;
                                    let is_active = *warranty_timeline as f32 >= position;

                                    html! {
                                        <div
                                            class={classes!("timeline-marker", if is_active { "active" } else { "" })}
                                            style={format!("left: {}%; --marker-color: {}", position, period.color)}
                                        >
                                            <div class="marker-dot"></div>
                                            <div class="marker-tooltip">
                                                <div class="tooltip-name">{&period.name}</div>
                                                <div class="tooltip-duration">{&period.duration}</div>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>

                            <div class="timeline-periods">
                                {for warranty_periods.iter().map(|period| {
                                    html! {
                                        <div class="period-card" style={format!("--period-color: {}", period.color)}>
                                            <div class="period-icon">
                                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                    <path d={period.icon}
                                                          stroke="currentColor" stroke-width="2"
                                                          stroke-linecap="round" stroke-linejoin="round"/>
                                                </svg>
                                            </div>
                                            <div class="period-content">
                                                <div class="period-name">{&period.name}</div>
                                                <div class="period-duration">{&period.duration}</div>
                                                <div class="period-coverage">
                                                    {for period.coverage.iter().take(1).map(|item| {
                                                        html! {
                                                            <div class="coverage-item">{item}</div>
                                                        }
                                                    })}
                                                </div>
                                            </div>
                                            {if period.is_extendable {
                                                html! {
                                                    <div class="period-badge">
                                                        {"Можна продовжити"}
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    </div>
                </div>

                <div class="guarantees-commitment">
                    <div class="commitment-content">
                        <div class="commitment-icon">
                            <svg width="48" height="48" viewBox="0 0 24 24" fill="none">
                                <path d="M12 2L4 5v6.09c0 5.05 3.41 9.76 8 10.91 4.59-1.15 8-5.86 8-10.91V5l-8-3z"
                                      stroke="currentColor" stroke-width="2"
                                      stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        </div>
                        <div class="commitment-text">
                            <h3 class="commitment-title">{"Наш зобов'язання перед вами"}</h3>
                            <p class="commitment-description">
                                {"Ми не просто даємо гарантії – ми беремо на себе повну відповідальність за кожен проект. Наша мета – не лише виконати роботу, але й забезпечити довготривалу надійність та комфорт для вас та вашої родини."}
                            </p>
                        </div>
                    </div>

                    <div class="commitment-actions">
                        <button class="btn-commitment" onclick={reset_animations}>
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M2.5 2v6h6M21.5 22v-6h-6M2.5 2l10 10M2.5 2l-1 1M21.5 22L11.5 12M21.5 22l1-1M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2z"
                                      stroke="currentColor" stroke-width="2"
                                      stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                            {"Перевірити анімації"}
                        </button>
                        <button class="btn-contact">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M21 11.5a8.38 8.38 0 01-.9 3.8 8.5 8.5 0 01-7.6 4.7 8.38 8.38 0 01-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 01-.9-3.8 8.5 8.5 0 014.7-7.6 8.38 8.38 0 013.8-.9h.5a8.48 8.48 0 018 8v.5z"
                                      stroke="currentColor" stroke-width="2"
                                      stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                            {"Обговорити гарантії"}
                        </button>
                    </div>
                </div>
            </div>

        </section>
    }
}
