use gloo_timers::callback::Timeout;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessStep {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub icon: &'static str,
    pub color: String,
    pub details: Vec<String>,
    pub duration: String,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeamMember {
    pub id: usize,
    pub name: String,
    pub role: String,
    pub specialization: String,
    pub experience: String,
    pub avatar_color: String,
    pub projects_count: u32,
    pub is_available: bool,
}

#[component]
pub fn HowWeWork() -> Html {
    let active_step = use_state(|| 0usize);
    let hovered_member = use_state(|| Option::<usize>::None);
    let show_details = use_state(|| false);
    let animation_state = use_state(|| "enter".to_string());
    let parallax_offset = use_state(|| (0.0, 0.0));

    let processes = vec![
        ProcessStep {
            id: 0,
            title: "Консультація та заміри".to_string(),
            description: "Перша зустріч та обговорення всіх деталей".to_string(),
            icon: "M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z",
            color: "#4299e1".to_string(),
            details: vec![
                "Безкоштовний виїзд майстра".to_string(),
                "Детальні заміри приміщення".to_string(),
                "Обговорення бюджету та термінів".to_string(),
                "Попередній ескіз рішення".to_string(),
            ],
            duration: "1-2 дні".to_string(),
            active: true,
        },
        ProcessStep {
            id: 1,
            title: "Проектування".to_string(),
            description: "Розробка дизайн-проекту".to_string(),
            icon: "M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z",
            color: "#38b2ac".to_string(),
            details: vec![
                "3D-візуалізація інтер'єру".to_string(),
                "Планувальні рішення".to_string(),
                "Підбір матеріалів та кольорів".to_string(),
                "Розробка технічної документації".to_string(),
            ],
            duration: "3-7 днів".to_string(),
            active: true,
        },
        ProcessStep {
            id: 2,
            title: "Підготовка".to_string(),
            description: "Закупівля матеріалів та підготовка".to_string(),
            icon: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
            color: "#9f7aea".to_string(),
            details: vec![
                "Закупівля якісних матеріалів".to_string(),
                "Доставка на об'єкт".to_string(),
                "Підготовка робочих зон".to_string(),
                "Узгодження графіка робіт".to_string(),
            ],
            duration: "2-4 дні".to_string(),
            active: true,
        },
        ProcessStep {
            id: 3,
            title: "Чорнові роботи".to_string(),
            description: "Основні будівельні роботи".to_string(),
            icon: "M13 10V3L4 14h7v7l9-11h-7z",
            color: "#ed8936".to_string(),
            details: vec![
                "Демонтажні роботи".to_string(),
                "Електромонтаж".to_string(),
                "Сантехнічні роботи".to_string(),
                "Стяжка підлоги".to_string(),
                "Вирівнювання стін та стелі".to_string(),
            ],
            duration: "7-21 день".to_string(),
            active: false,
        },
        ProcessStep {
            id: 4,
            title: "Чистові роботи".to_string(),
            description: "Фінішне оздоблення".to_string(),
            icon: "M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
            color: "#f56565".to_string(),
            details: vec![
                "Поклейка шпалер".to_string(),
                "Покриття підлоги".to_string(),
                "Монтаж дверей та вікон".to_string(),
                "Установка сантехніки".to_string(),
                "Монтаж освітлення".to_string(),
            ],
            duration: "10-20 днів".to_string(),
            active: false,
        },
        ProcessStep {
            id: 5,
            title: "Завершення".to_string(),
            description: "Фінальні роботи та здача об'єкта".to_string(),
            icon: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
            color: "#48bb78".to_string(),
            details: vec![
                "Прибирання після ремонту".to_string(),
                "Установка меблів".to_string(),
                "Тестування всіх систем".to_string(),
                "Фінальна інспекція".to_string(),
                "Підписання актів виконаних робіт".to_string(),
            ],
            duration: "2-3 дні".to_string(),
            active: false,
        },
    ];

    let team_members = vec![
        TeamMember {
            id: 0,
            name: "Андрій Коваленко".to_string(),
            role: "Прораб".to_string(),
            specialization: "Координація робіт".to_string(),
            experience: "12+ років".to_string(),
            avatar_color: "#4299e1".to_string(),
            projects_count: 87,
            is_available: true,
        },
        TeamMember {
            id: 1,
            name: "Олексій Петренко".to_string(),
            role: "Дизайнер".to_string(),
            specialization: "Інтер'єр та планування".to_string(),
            experience: "8+ років".to_string(),
            avatar_color: "#9f7aea".to_string(),
            projects_count: 64,
            is_available: true,
        },
        TeamMember {
            id: 2,
            name: "Марія Шевченко".to_string(),
            role: "Інженер".to_string(),
            specialization: "Електромонтаж".to_string(),
            experience: "6+ років".to_string(),
            avatar_color: "#38b2ac".to_string(),
            projects_count: 42,
            is_available: false,
        },
        TeamMember {
            id: 3,
            name: "Віктор Бойко".to_string(),
            role: "Бригадир".to_string(),
            specialization: "Чорнові роботи".to_string(),
            experience: "15+ років".to_string(),
            avatar_color: "#ed8936".to_string(),
            projects_count: 103,
            is_available: true,
        },
    ];

    let set_active_step = {
        let active_step = active_step.clone();
        let animation_state = animation_state.clone();
        Callback::from(move |step_id: usize| {
            let active_step = active_step.clone();
            let animation_state = animation_state.clone();
            animation_state.set("exit".to_string());
            Timeout::new(200, move || {
                active_step.set(step_id);
                animation_state.set("enter".to_string());
            })
            .forget();
        })
    };

    //let toggle_details = {
    //    let show_details = show_details.clone();
    //    Callback::from(move |_| {
    //        show_details.set(!*show_details);
    //    })
    //};

    // Parallax effect on mouse move
    let on_mouse_move = {
        let parallax_offset = parallax_offset.clone();
        Callback::from(move |e: MouseEvent| {
            let x = (e.client_x() as f64 / window().inner_width().unwrap().as_f64().unwrap() - 0.5)
                * 20.0;
            let y = (e.client_y() as f64 / window().inner_height().unwrap().as_f64().unwrap()
                - 0.5)
                * 20.0;
            parallax_offset.set((x, y));
        })
    };

    // Reset parallax on mouse leave
    let on_mouse_leave = {
        let parallax_offset = parallax_offset.clone();
        Callback::from(move |_| {
            parallax_offset.set((0.0, 0.0));
        })
    };

    html! {
        <section
            class="processes-section"
            onmousemove={on_mouse_move}
            onmouseleave={on_mouse_leave}
        >
            <div class="processes-background">
                <div class="bg-grid" style={format!("transform: translate({}px, {}px)", parallax_offset.0, parallax_offset.1)}></div>
                <div class="bg-dots" style={format!("transform: translate({}px, {}px)", -parallax_offset.0 * 0.5, -parallax_offset.1 * 0.5)}></div>
                <div class="bg-lines" style={format!("transform: translate({}px, {}px)", parallax_offset.0 * 0.3, parallax_offset.1 * 0.3)}></div>
            </div>

            <div class="processes-container">
                <div class="processes-header">
                    <h2 class="processes-title">
                        <span class="title-glow">{"ЯК МИ ПРАЦЮЄМО"}</span>
                    </h2>
                    <p class="processes-subtitle">
                        {"Прозорий процес від першої зустрічі до здачі об'єкта"}
                    </p>
                </div>

                <div class="process-timeline">
                    <div class="timeline-track">
                        <div class="timeline-progress" style={format!("width: {}%", (*active_step + 1) as f32 / processes.len() as f32 * 100.0)}></div>
                        {for processes.iter().map(|step| {
                            let is_active = step.id == *active_step;
                            let is_passed = step.id < *active_step;
                            let set_step = set_active_step.clone();
                            let step_id = step.id;

                            html! {
                                <div
                                    class={classes!(
                                        "timeline-step",
                                        if is_active { "active" } else { "" },
                                        if is_passed { "passed" } else { "" }
                                    )}
                                    onclick={Callback::from(move |_| set_step.emit(step_id))}
                                >
                                    <div class="step-marker">
                                        <div class="marker-outer">
                                            <div class="marker-inner" style={format!("background: {}", step.color)}>
                                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                    <path d={step.icon}
                                                          stroke="white"
                                                          stroke-width="2"
                                                          stroke-linecap="round"
                                                          stroke-linejoin="round"/>
                                                </svg>
                                            </div>
                                        </div>
                                        <div class="step-tooltip">
                                            <div class="tooltip-title">{step.title.clone()}</div>
                                        </div>
                                    </div>
                                    <div class="step-info">
                                        <div class="step-duration">{step.duration.clone()}</div>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                </div>

                <div class={classes!("process-content", format!("animation-{}", *animation_state))}>
                    <div class="process-details">
                        <div class="process-header">
                            <div class="process-title-wrapper">
                                <div
                                    class="process-icon"
                                    style={format!("background: {}", processes[*active_step].color)}
                                >
                                    <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                        <path d={processes[*active_step].icon}
                                              stroke="white"
                                              stroke-width="2"
                                              stroke-linecap="round"
                                              stroke-linejoin="round"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="process-title">{&processes[*active_step].title}</h3>
                                    <p class="process-description">{&processes[*active_step].description}</p>
                                </div>
                            </div>
                            <div class="process-stats">
                                <div class="stat-item">
                                    <div class="stat-value">{processes[*active_step].duration.clone()}</div>
                                    <div class="stat-label">{"Тривалість"}</div>
                                </div>
                                <div class="stat-item">
                                    <div class="stat-value">{processes[*active_step].details.len()}</div>
                                    <div class="stat-label">{"Етапи"}</div>
                                </div>
                            </div>
                        </div>

                        <div class="process-list">
                            {for processes[*active_step].details.iter().enumerate().map(|(i, detail)| {
                                html! {
                                    <div class="process-item" key={i}>
                                        <div
                                            class="process-item-marker"
                                            style={format!("background: {}", processes[*active_step].color)}
                                        >
                                            <div class="item-number">{i + 1}</div>
                                        </div>
                                        <div class="process-item-content">
                                            <div class="item-text">{detail}</div>
                                            <div class="item-status">
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                    <path d="M5 13l4 4L19 7"
                                                          stroke="currentColor"
                                                          stroke-width="2"
                                                          stroke-linecap="round"
                                                          stroke-linejoin="round"/>
                                                </svg>
                                                {"Виконується"}
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>

                    <div class="process-visual">
                        <div class="visual-container">
                            <div class="visual-scene">
                                <div class="scene-element scene-room">
                                    <div class="room-walls"></div>
                                    <div class="room-floor"></div>
                                    <div class="room-window"></div>
                                    <div class="room-door"></div>
                                </div>

                                {match *active_step {
                                    0 => html! {
                                        <>
                                            <div class="scene-element scene-person scene-person-1">
                                                <div class="person-body"></div>
                                                <div class="person-head"></div>
                                                <div class="person-tool">
                                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                        <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                        <path d="M14 2v6h6"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                                <div class="person-dialog">
                                                    {"Обговорюємо деталі"}
                                                </div>
                                            </div>
                                            <div class="scene-element scene-person scene-person-2">
                                                <div class="person-body"></div>
                                                <div class="person-head"></div>
                                                <div class="person-tool">
                                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                        <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7z"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                        <circle cx="12" cy="12" r="3"
                                                                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                            </div>
                                        </>
                                    },
                                    1 => html! {
                                        <>
                                            <div class="scene-element scene-person scene-designer">
                                                <div class="person-body"></div>
                                                <div class="person-head"></div>
                                                <div class="person-tool">
                                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                        <path d="M12 20h9M16.5 3.5a2.121 2.121 0 013 3L7 19l-4 1 1-4L16.5 3.5z"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                                <div class="scene-blueprint">
                                                    <div class="blueprint-grid"></div>
                                                    <div class="blueprint-plan"></div>
                                                </div>
                                            </div>
                                            <div class="scene-element scene-monitor">
                                                <div class="monitor-screen">
                                                    <div class="screen-content">
                                                        {"3D Візуалізація"}
                                                    </div>
                                                </div>
                                            </div>
                                        </>
                                    },
                                    2 => html! {
                                        <>
                                            <div class="scene-element scene-truck">
                                                <div class="truck-body"></div>
                                                <div class="truck-cargo">
                                                    <div class="cargo-box"></div>
                                                    <div class="cargo-box"></div>
                                                    <div class="cargo-box"></div>
                                                </div>
                                            </div>
                                            <div class="scene-element scene-materials">
                                                <div class="material-stack"></div>
                                                <div class="material-stack"></div>
                                                <div class="material-stack"></div>
                                            </div>
                                            <div class="scene-element scene-person scene-worker">
                                                <div class="person-body"></div>
                                                <div class="person-head"></div>
                                                <div class="person-tool">
                                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                        <path d="M16 8v8m-4-5v5m-4-2v2m-2 4h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                            </div>
                                        </>
                                    },
                                    3 => html! {
                                        <>
                                            <div class="scene-element scene-tools">
                                                <div class="tool-hammer"></div>
                                                <div class="tool-drill"></div>
                                                <div class="tool-saw"></div>
                                            </div>
                                            <div class="scene-element scene-wires">
                                                <div class="wire"></div>
                                                <div class="wire"></div>
                                                <div class="wire"></div>
                                            </div>
                                            <div class="scene-element scene-person scene-builder">
                                                <div class="person-body"></div>
                                                <div class="person-head"></div>
                                                <div class="person-tool">
                                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                        <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                        <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                            </div>
                                        </>
                                    },
                                    4 => html! {
                                        <>
                                            <div class="scene-element scene-paint">
                                                <div class="paint-can"></div>
                                                <div class="paint-roller"></div>
                                            </div>
                                            <div class="scene-element scene-flooring">
                                                <div class="floor-plank"></div>
                                                <div class="floor-plank"></div>
                                                <div class="floor-plank"></div>
                                            </div>
                                            <div class="scene-element scene-person scene-painter">
                                                <div class="person-body"></div>
                                                <div class="person-head"></div>
                                                <div class="person-tool">
                                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                        <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                        <circle cx="9" cy="7" r="4"
                                                                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                        <path d="M23 21v-2a4 4 0 00-3-3.87"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                        <path d="M16 3.13a4 4 0 010 7.75"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                            </div>
                                        </>
                                    },
                                    5 => html! {
                                        <>
                                            <div class="scene-element scene-cleaning">
                                                <div class="clean-bucket"></div>
                                                <div class="clean-mop"></div>
                                            </div>
                                            <div class="scene-element scene-furniture">
                                                <div class="furniture-sofa"></div>
                                                <div class="furniture-table"></div>
                                            </div>
                                            <div class="scene-element scene-person scene-inspector">
                                                <div class="person-body"></div>
                                                <div class="person-head"></div>
                                                <div class="person-tool">
                                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                        <path d="M22 11.08V12a10 10 0 11-5.93-9.14"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                        <path d="M22 4L12 14.01l-3-3"
                                                              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                    </svg>
                                                </div>
                                                <div class="person-dialog">
                                                    {"Перевірка якості"}
                                                </div>
                                            </div>
                                        </>
                                    },
                                    _ => html! {},
                                }}
                            </div>
                        </div>
                    </div>
                </div>

                <div class="team-section">
                    <div class="team-header">
                        <h3 class="team-title">{"Відповідальні фахівці"}</h3>
                        <p class="team-subtitle">{"На кожному етапі з вами працюють професіонали"}</p>
                    </div>

                    <div class="team-grid">
                        {for team_members.iter().map(|member| {
                            let is_hovered = *hovered_member == Some(member.id);
                            let hovered_member_state = hovered_member.clone();
                            let member_id = member.id;

                            let on_mouse_enter = {
                                let hovered_member_state = hovered_member_state.clone();
        Callback::from(move |_| {
                                hovered_member_state.set(Some(member_id));
                            })};

                            let on_mouse_leave = {

                                let hovered_member_state = hovered_member_state.clone();
        Callback::from(move |_| {
                                hovered_member_state.set(None);
                            })};

                            html! {
                                <div
                                    class={classes!("team-card", if is_hovered { "hovered" } else { "" })}
                                    onmouseenter={on_mouse_enter}
                                    onmouseleave={on_mouse_leave}
                                >
                                    <div class="card-avatar" style={format!("--avatar-color: {}", member.avatar_color)}>
                                        <div class="avatar-initials">
                                            {member.name.split_whitespace()
                                                .map(|w| w.chars().next().unwrap())
                                                .take(2)
                                                .collect::<String>()}
                                        </div>
                                        {if member.is_available {
                                            html! {
                                                <div class="avatar-status available">
                                                    <div class="status-dot"></div>
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <div class="avatar-status busy">
                                                    <div class="status-dot"></div>
                                                </div>
                                            }
                                        }}
                                    </div>

                                    <div class="card-content">
                                        <div class="member-name">{&member.name}</div>
                                        <div class="member-role">{&member.role}</div>
                                        <div class="member-specialization">{&member.specialization}</div>

                                        <div class="member-stats">
                                            <div class="stat">
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                    <path d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                                                          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                </svg>
                                                <span>{&member.experience}</span>
                                            </div>
                                            <div class="stat">
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                                                    <path d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"
                                                          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                </svg>
                                                <span>{format!("{} проєктів", member.projects_count)}</span>
                                            </div>
                                        </div>
                                    </div>

                                    {if is_hovered {
                                        html! {
                                            <div class="card-hover">
                                                <div class="hover-content">
                                                    <div class="hover-text">{"Наразі працює над етапом"}</div>
                                                    <div class="hover-step">{&processes[*active_step].title}</div>
                                                </div>
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

                <div class="process-cta">
                    <div class="cta-content">
                        <div class="cta-icon">
                            <svg width="40" height="40" viewBox="0 0 24 24" fill="none">
                                <path d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z"
                                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        </div>
                        <div class="cta-text">
                            <div class="cta-title">{"Є питання щодо процесу?"}</div>
                            <div class="cta-subtitle">{"Зателефонуйте нам або напишіть у будь-який зручний месенджер"}</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn inner_width() -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    fn inner_height() -> JsValue;
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
