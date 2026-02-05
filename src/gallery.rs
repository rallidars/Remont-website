use gloo_timers::callback::Timeout;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectCategory {
    All,
    Apartments,
    Houses,
    Offices,
    Commercial,
    Renovation,
    Design,
}

#[derive(Debug, Clone)]
pub struct PortfolioProject {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub category: ProjectCategory,
    pub area: u32,
    pub duration: String,
    pub before_images: Vec<String>,
    pub after_images: Vec<String>,
    pub features: Vec<String>,
    pub color: String,
    pub year: u32,
    pub rating: f32,
    pub is_featured: bool,
}

#[derive(Debug, Clone)]
pub struct ClientReview {
    pub id: usize,
    pub name: String,
    pub project: String,
    pub text: String,
    pub rating: u32,
    pub date: String,
    pub avatar_color: String,
    pub is_verified: bool,
}

#[component]
pub fn PortfolioGallery() -> Html {
    let active_category = use_state(|| ProjectCategory::All);
    let view_mode = use_state(|| "grid".to_string()); // grid, masonry, list
    let selected_project = use_state(|| Option::<usize>::None);
    let compare_mode = use_state(|| false);
    let active_comparison = use_state(|| Option::<usize>::None);
    let hovered_project = use_state(|| Option::<usize>::None);
    let animation_state = use_state(|| "enter".to_string());
    let transform_state = use_state(|| TransformState::default());
    let show_details = use_state(|| false);

    let categories = vec![
        (ProjectCategory::All, "Всі роботи", "#4299e1"),
        (ProjectCategory::Apartments, "Квартири", "#38b2ac"),
        (ProjectCategory::Houses, "Будинки", "#ed8936"),
        (ProjectCategory::Offices, "Офіси", "#9f7aea"),
        (ProjectCategory::Commercial, "Комерція", "#f56565"),
        (ProjectCategory::Renovation, "Реконструкція", "#48bb78"),
        (ProjectCategory::Design, "Дизайн", "#d69e2e"),
    ];

    let projects = vec![
        PortfolioProject {
            id: 0,
            title: "Лофт апартаменти в центрі".to_string(),
            description: "Сучасний лофт з відкритим плануванням та промисловими елементами"
                .to_string(),
            category: ProjectCategory::Apartments,
            area: 85,
            duration: "45 днів".to_string(),
            before_images: vec!["/static/image_remont1.jpeg".to_string()],
            after_images: vec!["/static/image_remont1.jpeg".to_string()],
            features: vec![
                "Відкрите планування".to_string(),
                "Промисловий стиль".to_string(),
                "Smart-home система".to_string(),
                "Авторські меблі".to_string(),
            ],
            color: "#4299e1".to_string(),
            year: 2023,
            rating: 4.9,
            is_featured: true,
        },
        PortfolioProject {
            id: 1,
            title: "Заміський котедж".to_string(),
            description: "Повний ремонт двоповерхового будинку з терасою".to_string(),
            category: ProjectCategory::Houses,
            area: 240,
            duration: "90 днів".to_string(),
            before_images: vec!["/api/placeholder/400/300".to_string()],
            after_images: vec!["/api/placeholder/400/300".to_string()],
            features: vec![
                "Камін з груби".to_string(),
                "Тераса з навісом".to_string(),
                "Сауна".to_string(),
                "Ландшафтний дизайн".to_string(),
            ],
            color: "#ed8936".to_string(),
            year: 2023,
            rating: 5.0,
            is_featured: true,
        },
        PortfolioProject {
            id: 2,
            title: "IT-офіс open space".to_string(),
            description: "Створення комфортного робочого простору для IT-компанії".to_string(),
            category: ProjectCategory::Offices,
            area: 320,
            duration: "60 днів".to_string(),
            before_images: vec!["/api/placeholder/400/300".to_string()],
            after_images: vec!["/api/placeholder/400/300".to_string()],
            features: vec![
                "Open-space зона".to_string(),
                "Конференц-кімнати".to_string(),
                "Relax зона".to_string(),
                "Енергоефективне освітлення".to_string(),
            ],
            color: "#9f7aea".to_string(),
            year: 2023,
            rating: 4.8,
            is_featured: false,
        },
        PortfolioProject {
            id: 3,
            title: "Кав'ярня мінімалізм".to_string(),
            description: "Створення унікального атмосферного простору для кав'ярні".to_string(),
            category: ProjectCategory::Commercial,
            area: 120,
            duration: "30 днів".to_string(),
            before_images: vec!["/api/placeholder/400/300".to_string()],
            after_images: vec!["/api/placeholder/400/300".to_string()],
            features: vec![
                "Мінімалістичний дизайн".to_string(),
                "Авторська барна стійка".to_string(),
                "Зона зелених рослин".to_string(),
                "Акустичне облицювання".to_string(),
            ],
            color: "#f56565".to_string(),
            year: 2023,
            rating: 4.7,
            is_featured: false,
        },
        PortfolioProject {
            id: 4,
            title: "Історична реконструкція".to_string(),
            description: "Відновлення історичної будівлі з сучасними комунікаціями".to_string(),
            category: ProjectCategory::Renovation,
            area: 180,
            duration: "120 днів".to_string(),
            before_images: vec!["/api/placeholder/400/300".to_string()],
            after_images: vec!["/api/placeholder/400/300".to_string()],
            features: vec![
                "Збереження історичних деталей".to_string(),
                "Сучасні комунікації".to_string(),
                "Відновлення паркету".to_string(),
                "Розумна система вентиляції".to_string(),
            ],
            color: "#48bb78".to_string(),
            year: 2023,
            rating: 4.9,
            is_featured: true,
        },
        PortfolioProject {
            id: 5,
            title: "Скандинавська квартира".to_string(),
            description: "Світлий та затишний простір у скандинавському стилі".to_string(),
            category: ProjectCategory::Design,
            area: 65,
            duration: "40 днів".to_string(),
            before_images: vec!["/api/placeholder/400/300".to_string()],
            after_images: vec!["/api/placeholder/400/300".to_string()],
            features: vec![
                "Скандинавський стиль".to_string(),
                "Натуральні матеріали".to_string(),
                "Максимальне природне світло".to_string(),
                "Мультифункціональні меблі".to_string(),
            ],
            color: "#d69e2e".to_string(),
            year: 2023,
            rating: 4.8,
            is_featured: false,
        },
        PortfolioProject {
            id: 6,
            title: "Пентхаус з панорамним видом".to_string(),
            description: "Розкішний пентхаус на останньому поверсі з терасою".to_string(),
            category: ProjectCategory::Apartments,
            area: 210,
            duration: "75 днів".to_string(),
            before_images: vec!["/api/placeholder/400/300".to_string()],
            after_images: vec!["/api/placeholder/400/300".to_string()],
            features: vec![
                "Панорамні вікна".to_string(),
                "Приватна тераса".to_string(),
                "Домашній кінотеатр".to_string(),
                "Винний погріб".to_string(),
            ],
            color: "#4299e1".to_string(),
            year: 2023,
            rating: 5.0,
            is_featured: true,
        },
        PortfolioProject {
            id: 7,
            title: "Медичний центр".to_string(),
            description: "Створення сучасного медичного центру з зонами очікування".to_string(),
            category: ProjectCategory::Commercial,
            area: 280,
            duration: "65 днів".to_string(),
            before_images: vec!["/api/placeholder/400/300".to_string()],
            after_images: vec!["/api/placeholder/400/300".to_string()],
            features: vec![
                "Стерильні зони".to_string(),
                "Комфортні зони очікування".to_string(),
                "Сучасне обладнання".to_string(),
                "Безбар'єрне середовище".to_string(),
            ],
            color: "#f56565".to_string(),
            year: 2023,
            rating: 4.8,
            is_featured: false,
        },
    ];

    let reviews = vec![
        ClientReview {
            id: 0,
            name: "Ольга Петренко".to_string(),
            project: "Лофт апартаменти".to_string(),
            text: "Команда працювала професійно та вчасно. Особливо вразила увага до деталей та якість матеріалів. Рекомендую!".to_string(),
            rating: 5,
            date: "15.11.2023".to_string(),
            avatar_color: "#4299e1".to_string(),
            is_verified: true,
        },
        ClientReview {
            id: 1,
            name: "Андрій Ковальчук".to_string(),
            project: "Заміський котедж".to_string(),
            text: "Весь ремонт від початку до кінця пройшов гладко. Прораб постійно був на зв'язку, показував фото з процесу. Результатом дуже задоволений.".to_string(),
            rating: 5,
            date: "02.11.2023".to_string(),
            avatar_color: "#ed8936".to_string(),
            is_verified: true,
        },
        ClientReview {
            id: 2,
            name: "Марія Сидоренко".to_string(),
            project: "Скандинавська квартира".to_string(),
            text: "Дизайнер вразив своїми ідеями! Перетворили стару квартиру на сучасний затишний простір. Дякую за чудову роботу!".to_string(),
            rating: 4,
            date: "28.10.2023".to_string(),
            avatar_color: "#d69e2e".to_string(),
            is_verified: true,
        },
        ClientReview {
            id: 3,
            name: "Віктор Бойко".to_string(),
            project: "IT-офіс".to_string(),
            text: "Потрібно було швидко зробити офіс для нашої команди. Вклались у терміни та бюджет. Співробітники дуже задоволені новим простором.".to_string(),
            rating: 5,
            date: "12.10.2023".to_string(),
            avatar_color: "#9f7aea".to_string(),
            is_verified: true,
        },
    ];

    // Filter projects by category
    let filtered_projects = if *active_category == ProjectCategory::All {
        projects.clone()
    } else {
        projects
            .iter()
            .filter(|p| p.category == *active_category)
            .cloned()
            .collect::<Vec<_>>()
    };

    // Featured projects
    let featured_projects = projects
        .iter()
        .filter(|p| p.is_featured)
        .cloned()
        .collect::<Vec<_>>();

    let select_category = {
        let active_category = active_category.clone();
        let animation_state = animation_state.clone();
        Callback::from(move |category: ProjectCategory| {
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

    let toggle_view_mode = {
        let view_mode = view_mode.clone();
        Callback::from(move |mode: String| {
            view_mode.set(mode);
        })
    };

    let open_project_details = {
        let selected_project = selected_project.clone();
        let show_details = show_details.clone();
        Callback::from(move |project_id: usize| {
            selected_project.set(Some(project_id));
            show_details.set(true);
        })
    };

    let close_project_details = {
        let selected_project = selected_project.clone();
        let show_details = show_details.clone();
        Callback::from(move |_| {
            let selected_project = selected_project.clone();
            let show_details = show_details.clone();
            show_details.set(false);
            Timeout::new(300, move || {
                selected_project.set(None);
            })
            .forget();
        })
    };

    let toggle_compare_mode = {
        let compare_mode = compare_mode.clone();
        Callback::from(move |_| {
            compare_mode.set(!*compare_mode);
        })
    };

    let start_comparison = {
        let active_comparison = active_comparison.clone();
        Callback::from(move |project_id: usize| {
            active_comparison.set(Some(project_id));
        })
    };

    // 3D transform controls
    let rotate_left = {
        let transform_state = transform_state.clone();
        Callback::from(move |_| {
            let mut state = (*transform_state).clone();
            state.rotate_y -= 15.0;
            transform_state.set(state);
        })
    };

    let rotate_right = {
        let transform_state = transform_state.clone();
        Callback::from(move |_| {
            let mut state = (*transform_state).clone();
            state.rotate_y += 15.0;
            transform_state.set(state);
        })
    };

    let zoom_in = {
        let transform_state = transform_state.clone();
        Callback::from(move |_| {
            let mut state = (*transform_state).clone();
            state.scale += 0.1;
            state.scale = state.scale.min(2.0);
            transform_state.set(state);
        })
    };

    let zoom_out = {
        let transform_state = transform_state.clone();
        Callback::from(move |_| {
            let mut state = (*transform_state).clone();
            state.scale -= 0.1;
            state.scale = state.scale.max(0.5);
            transform_state.set(state);
        })
    };

    let reset_transform = {
        let transform_state = transform_state.clone();
        Callback::from(move |_| {
            transform_state.set(TransformState::default());
        })
    };

    // Stats
    let total_area: u32 = projects.iter().map(|p| p.area).sum();
    let avg_rating: f32 = projects.iter().map(|p| p.rating).sum::<f32>() / projects.len() as f32;

    html! {
        <section class="portfolio-section">
            <div class="portfolio-background">
                <div class="bg-particles"></div>
                <div class="bg-gradient"></div>
            </div>

            <div class="portfolio-container">
                <div class="portfolio-header">
                    <div class="header-content">
                        <h2 class="portfolio-title">
                            <span class="title-shine">{"НАШІ РОБОТИ"}</span>
                        </h2>
                        <p class="portfolio-subtitle">
                            {"Понад 200 успішно реалізованих проектів різної складності"}
                        </p>
                    </div>

                    <div class="portfolio-stats">
                        <div class="stat-card">
                            <div class="stat-number">{projects.len()}</div>
                            <div class="stat-label">{"Проектів"}</div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-number">{format!("{} м²", total_area)}</div>
                            <div class="stat-label">{"Загальна площа"}</div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-number">{format!("{:.1}", avg_rating)}</div>
                            <div class="stat-label">{"Середня оцінка"}</div>
                        </div>
                    </div>
                </div>

                <div class="portfolio-controls">
                    <div class="category-filters">
                        <div class="filter-scroll">
                            {for categories.iter().map(|(category, label, color)| {
                                let is_active = *active_category == *category;
                                let category_clone = category.clone();
                                let select_category = select_category.clone();

                                html! {
                                    <button
                                        class={if is_active { "filter-btn active" } else { "filter-btn" }}
                                        onclick={Callback::from(move |_| select_category.emit(category_clone.clone()))}
                                        style={format!("--btn-color: {}", color)}
                                    >
                                        <span class="filter-dot" style={format!("background: {}", color)}></span>
                                        {*label}
                                    </button>
                                }
                            })}
                        </div>
                    </div>

                    <div class="view-controls">
                        <button
                            class={classes!("view-btn", if *view_mode == "grid" { "active" } else { "" })}
                            onclick={
                            let toggle_view_mode = toggle_view_mode.clone();
        Callback::from(move |_| toggle_view_mode.emit("grid".to_string()))}
                            title="Сітка"
                        >
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M3 3H10V10H3V3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M14 3H21V10H14V3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M14 14H21V21H14V14Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M3 14H10V21H3V14Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        </button>
                        <button
                            class={classes!("view-btn", if *view_mode == "masonry" { "active" } else { "" })}
                            onclick={Callback::from(move |_| toggle_view_mode.emit("masonry".to_string()))}
                            title="Мозаїка"
                        >
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M3 3H12V12H3V3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M3 16H12V21H3V16Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M16 3H21V8H16V3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                <path d="M16 12H21V21H16V12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        </button>
                        <button
                            class={classes!("view-btn", if *compare_mode { "active" } else { "" })}
                            onclick={toggle_compare_mode}
                            title="Порівняння до/після"
                        >
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M8 3H4a2 2 0 00-2 2v14a2 2 0 002 2h4M16 3h4a2 2 0 012 2v14a2 2 0 01-2 2h-4M8 3v18M16 3v18"
                                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        </button>
                    </div>
                </div>

                <div class={classes!(
                    "projects-container",
                    format!("view-{}", *view_mode),
                    format!("animation-{}", *animation_state)
                )}>
                    {for filtered_projects.iter().map(|project| {
                        let is_hovered = *hovered_project == Some(project.id);
                        let is_comparing = *active_comparison == Some(project.id);
                        let project_id = project.id;
                        let hovered_project_state = hovered_project.clone();
                        let open_project_details = open_project_details.clone();
                        let start_comparison = start_comparison.clone();

                        let on_mouse_enter ={

                        Callback::from(move |_| {
                            hovered_project_state.set(Some(project_id));
                        })};

                        let on_mouse_leave ={
                        let hovered_project_state = hovered_project.clone();
                        Callback::from(move |_| {
                            hovered_project_state.set(None);
                        })};

                        html! {
                            <div
                                class={classes!(
                                    "project-card",
                                    if project.is_featured { "featured" } else { "" },
                                    if is_hovered { "hovered" } else { "" },
                                    if is_comparing { "comparing" } else { "" }
                                )}
                                onmouseenter={on_mouse_enter}
                                onmouseleave={on_mouse_leave}
                                style={format!("--project-color: {}", project.color)}
                            >
                                <div class="project-badge">
                                    <div class="badge-year">{project.year}</div>
                                    {if project.is_featured {
                                        html! {
                                            <div class="badge-featured">
                                                <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
                                                    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"
                                                          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                </svg>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>

                                <div class="project-image">
                                    <div class="image-container">
                                        <div class="image-overlay"><img src="/static/image_remont1.jpeg" /></div>
                                        <div class="image-placeholder">

                                            <div class="placeholder-content">

                                                <div class="placeholder-icon">
                                                    <svg width="48" height="48" viewBox="0 0 24 24" fill="none">
                                                        <path d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                                <div class="placeholder-text">{"Натисніть для перегляду"}</div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="project-content">
                                    <div class="project-header">
                                        <h3 class="project-title">{&project.title}</h3>
                                        <div class="project-rating">
                                            <div class="rating-stars">
                                                {(0..5).map(|i| {
                                                    let is_filled = i < project.rating as i32;
                                                    html! {
                                                        <svg
                                                            width="16"
                                                            height="16"
                                                            viewBox="0 0 24 24"
                                                            fill={if is_filled { "currentColor" } else { "none" }}
                                                            stroke="currentColor"
                                                            stroke-width="2"
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                        >
                                                            <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
                                                        </svg>
                                                    }
                                                }).collect::<Html>()}
                                            </div>
                                            <div class="rating-value">{format!("{:.1}", project.rating)}</div>
                                        </div>
                                    </div>

                                    <p class="project-description">{&project.description}</p>

                                    <div class="project-meta">
                                        <div class="meta-item">
                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                <path d="M19 21H5a2 2 0 01-2-2V5a2 2 0 012-2h11l5 5v11a2 2 0 01-2 2z"
                                                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                <path d="M17 21v-8H7v8"
                                                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                <path d="M7 3v5h8"
                                                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                            </svg>
                                            <span>{format!("{} м²", project.area)}</span>
                                        </div>
                                        <div class="meta-item">
                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                <path d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                                                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                            </svg>
                                            <span>{&project.duration}</span>
                                        </div>
                                    </div>

                                    <div class="project-features">
                                        {project.features.iter().take(2).map(|feature| {
                                            html! {
                                                <span class="feature-tag">{feature}</span>
                                            }
                                        }).collect::<Html>()}
                                    </div>

                                    <div class="project-actions">
                                        <button
                                            class="btn-view"
                                            onclick={Callback::from(move |_| open_project_details.emit(project_id))}
                                        >
                                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                                                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"
                                                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                <circle cx="12" cy="12" r="3"
                                                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                            </svg>
                                            {"Детальніше"}
                                        </button>
                                        {if *compare_mode {
                                            html! {
                                                <button
                                                    class="btn-compare"
                                                    onclick={Callback::from(move |_| start_comparison.emit(project_id))}
                                                >
                                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                                                        <path d="M21 21l-6-6m6 6v-4.8m0 4.8h-4.8M3 16.2V21m0 0h4.8M3 21l6-6M3 12.9V3m0 0h4.8M3 3l6 6m11.1 6.9V21m0 0h-4.8m4.8 0l-6-6"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </button>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                </div>
                            </div>
                        }
                    })}
                </div>

                {if *compare_mode && active_comparison.is_some() {
                    let project_id = active_comparison.unwrap();
                    let project = projects.iter().find(|p| p.id == project_id).unwrap();

                    html! {
                        <div class="comparison-modal">
                            <div class="comparison-content">
                                <div class="comparison-header">
                                    <h3 class="comparison-title">
                                        {"Порівняння: "}{&project.title}
                                    </h3>
                                    <button class="btn-close" onclick={Callback::from(move |_| active_comparison.set(None))}>
                                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                            <path d="M18 6L6 18M6 6l12 12"
                                                  stroke="currentColor" stroke-width="2"
                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                    </button>
                                </div>

                                <div class="comparison-slider">
                                    <div class="slider-before">
                                        <div class="slider-label">{"ДО"}</div>
                                        <div class="slider-image"></div>
                                    </div>
                                    <div class="slider-after">
                                        <div class="slider-label">{"ПІСЛЯ"}</div>
                                        <div class="slider-image"></div>
                                    </div>
                                    <div class="slider-handle"></div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}

                <div class="reviews-section">
                    <div class="reviews-header">
                        <h3 class="reviews-title">{"Відгуки клієнтів"}</h3>
                        <p class="reviews-subtitle">{"Що кажуть про нас ті, хто вже скористався нашими послугами"}</p>
                    </div>

                    <div class="reviews-slider">
                        <div class="slider-track">
                            {for reviews.iter().map(|review| {
                                html! {
                                    <div class="review-card">
                                        <div class="review-header">
                                            <div class="reviewer-avatar" style={format!("--avatar-color: {}", review.avatar_color)}>
                                                <div class="avatar-initials">
                                                    {review.name.split_whitespace()
                                                        .map(|w| w.chars().next().unwrap())
                                                        .take(2)
                                                        .collect::<String>()}
                                                </div>
                                                {if review.is_verified {
                                                    html! {
                                                        <div class="avatar-verified">
                                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                                <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                                                      stroke="currentColor" stroke-width="2"
                                                                      stroke-linecap="round" stroke-linejoin="round"/>
                                                            </svg>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }}
                                            </div>
                                            <div class="reviewer-info">
                                                <div class="reviewer-name">{&review.name}</div>
                                                <div class="reviewer-project">{&review.project}</div>
                                            </div>
                                            <div class="review-rating">
                                                {(0..5).map(|i| {
                                                    let is_filled = i < review.rating;
                                                    html! {
                                                        <svg
                                                            width="16"
                                                            height="16"
                                                            viewBox="0 0 24 24"
                                                            fill={if is_filled { "#fbbf24" } else { "none" }}
                                                            stroke="#fbbf24"
                                                            stroke-width="2"
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                        >
                                                            <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
                                                        </svg>
                                                    }
                                                }).collect::<Html>()}
                                            </div>
                                        </div>

                                        <div class="review-content">
                                            <p class="review-text">{&review.text}</p>
                                        </div>

                                        <div class="review-footer">
                                            <div class="review-date">{&review.date}</div>
                                            <div class="review-actions">
                                                <button class="btn-like">
                                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                                                        <path d="M14 9V5a3 3 0 00-3-3l-4 9v11h11.28a2 2 0 002-1.7l1.38-9a2 2 0 00-2-2.3H7z"
                                                              stroke="currentColor" stroke-width="2"
                                                              stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </button>
                                                <button class="btn-share">
                                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                                                        <path d="M4 12v8a2 2 0 002 2h12a2 2 0 002-2v-8M16 6l-4-4-4 4M12 2v13"
                                                              stroke="currentColor" stroke-width="2"
                                                              stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>

                <div class="portfolio-cta">
                    <div class="cta-3d">
                        <div class="scene-container" style={format!(
                            "transform: rotateX({}deg) rotateY({}deg) scale({})",
                            transform_state.rotate_x,
                            transform_state.rotate_y,
                            transform_state.scale
                        )}>
                            <div class="scene-house">
                                <div class="house-wall house-front"></div>
                                <div class="house-wall house-back"></div>
                                <div class="house-wall house-left"></div>
                                <div class="house-wall house-right"></div>
                                <div class="house-roof"></div>
                                <div class="house-window"></div>
                                <div class="house-door"></div>
                            </div>
                        </div>
                        <div class="scene-controls">
                            <button onclick={rotate_left} title="Повернути вліво">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                    <path d="M15 18l-6-6 6-6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </button>
                            <button onclick={rotate_right} title="Повернути вправо">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                    <path d="M9 18l6-6-6-6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </button>
                            <button onclick={zoom_in} title="Збільшити">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                    <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM10 7v6m3-3H7" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </button>
                            <button onclick={zoom_out} title="Зменшити">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                    <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM7 10h6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </button>
                            <button onclick={reset_transform} title="Скинути">
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                    <path d="M2.5 2v6h6M21.5 22v-6h-6M2.5 2l10 10M2.5 2l-1 1M21.5 22L11.5 12M21.5 22l1-1" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                            </button>
                        </div>
                    </div>

                    <div class="cta-content">
                        <h3 class="cta-title">{"Готові перетворити вашу ідею в реальність?"}</h3>
                        <p class="cta-subtitle">
                            {"Перегляньте наше повне портфоліо або замовить безкоштовну консультацію з нашим дизайнером"}
                        </p>
                    </div>
                </div>
            </div>

            {if *show_details && selected_project.is_some() {
                let project_id = selected_project.unwrap();
                let project = projects.iter().find(|p| p.id == project_id).unwrap();

                html! {
                    <div class={classes!("project-modal", if *show_details { "visible" } else { "" })}>
                        <div class="modal-overlay" onclick={close_project_details.clone()}></div>
                        <div class="modal-content">
                            <div class="modal-header">
                                <h3 class="modal-title">{&project.title}</h3>
                                <button class="modal-close" onclick={close_project_details}>
                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                        <path d="M18 6L6 18M6 6l12 12"
                                              stroke="currentColor" stroke-width="2"
                                              stroke-linecap="round" stroke-linejoin="round"/>
                                    </svg>
                                </button>
                            </div>

                            <div class="modal-body">
                                <div class="modal-gallery">
                                    <div class="gallery-main">
                                        <div class="main-image"><img  src="/static/image_remont1.jpeg" /></div>
                                    </div>
                                    <div class="gallery-thumbs">
                                        {(0..4).map(|i| {
                                            html! {
                                                <div class="thumb-item"><img  src="/static/image_remont2.jpeg" /></div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                </div>

                                <div class="modal-details">
                                    <div class="details-grid">
                                        <div class="detail-item">
                                            <div class="detail-label">{"Площа"}</div>
                                            <div class="detail-value">{format!("{} м²", project.area)}</div>
                                        </div>
                                        <div class="detail-item">
                                            <div class="detail-label">{"Тривалість"}</div>
                                            <div class="detail-value">{&project.duration}</div>
                                        </div>
                                        <div class="detail-item">
                                            <div class="detail-label">{"Рік"}</div>
                                            <div class="detail-value">{project.year}</div>
                                        </div>
                                        <div class="detail-item">
                                            <div class="detail-label">{"Категорія"}</div>
                                            <div class="detail-value">{match project.category {
                                                ProjectCategory::All => "Всі",
                                                ProjectCategory::Apartments => "Квартири",
                                                ProjectCategory::Houses => "Будинки",
                                                ProjectCategory::Offices => "Офіси",
                                                ProjectCategory::Commercial => "Комерція",
                                                ProjectCategory::Renovation => "Реконструкція",
                                                ProjectCategory::Design => "Дизайн",
                                            }}</div>
                                        </div>
                                    </div>

                                    <div class="details-features">
                                        <h4 class="features-title">{"Особливості проекту"}</h4>
                                        <ul class="features-list">
                                            {project.features.iter().map(|feature| {
                                                html! {
                                                    <li class="feature-item">
                                                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                            <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                                                  stroke="currentColor" stroke-width="2"
                                                                  stroke-linecap="round" stroke-linejoin="round"/>
                                                        </svg>
                                                        <span>{feature}</span>
                                                    </li>
                                                }
                                            }).collect::<Html>()}
                                        </ul>
                                    </div>

                                    <div class="details-description">
                                        <p>{&project.description}</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </section>
    }
}

#[derive(Debug, Clone, Default)]
struct TransformState {
    rotate_x: f32,
    rotate_y: f32,
    scale: f32,
}
