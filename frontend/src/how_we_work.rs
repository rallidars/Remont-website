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
    pub image_path: String,
}

#[component]
pub fn HowWeWork() -> Html {
    let active_step = use_state(|| 0usize);
    let animation_state = use_state(|| "enter".to_string());

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
            image_path: "/static/image_remont1.jpeg".to_string(),
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
            image_path: "".to_string(),
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
            image_path: "".to_string(),
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
            image_path: "".to_string(),
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
            image_path: "".to_string(),
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
            image_path: "".to_string(),
        },
    ];

    let steps_count = processes.len();

    // Add style tag for dynamic step positioning
    let style = {
        let mut styles = String::new();
        for (i, _) in processes.iter().enumerate() {
            let position = (i as f32 * 100.0) / (steps_count as f32 - 1.0);
            styles.push_str(&format!(
                ".how-we-work-main .timeline-step:nth-child({}) {{ left: {}%; }}",
                i + 2, // +2 because first child is the progress bar
                position
            ));
        }
        styles
    };
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

    html! {
        <div class="ap-how-we-work-main">
            <section id="process" class="ap-processes-section">
                <div class="ap-processes-container">
                    <div class="ap-processes-header">
                        <h2 class="ap-processes-title">
                            <span class="ap-title-glow">{"ЯК МИ ПРАЦЮЄМО"}</span>
                        </h2>
                    </div>

                    <div class="ap-process-timeline">
                        <div class="ap-timeline-track">
                            <div class="ap-timeline-progress" style={format!(
                                "width: {}%;",
                                if processes.len() > 1 {
                                    (*active_step as f32 / (processes.len() - 1) as f32) * 100.0
                                } else { 0.0 }
                            )} />

                            {for processes.iter().enumerate().map(|(index, step)| {
                                let is_active = index == *active_step;
                                let is_passed = index < *active_step;
                                let set_step = set_active_step.clone();
                                let step_id = index;

                                let percent = if processes.len() > 1 {
                                    index as f32 / (processes.len() - 1) as f32 * 100.0
                                } else { 0.0 };

                                html! {
                                    <div
                                        class={classes!(
                                            "ap-timeline-step",
                                            if is_active { "active" } else { "" },
                                            if is_passed { "passed" } else { "" }
                                        )}
                                        style={format!("left: {}%;", percent)}
                                        onclick={Callback::from(move |_| set_step.emit(step_id))}
                                    >
                                        <div class="ap-step-marker">
                                            <div class="ap-marker-outer">
                                                <div class="ap-marker-inner" style={format!("background: {}", step.color)}>
                                                    <span class="ap-step-number">{index + 1}</span>
                                                </div>
                                            </div>
                                            <div class="ap-step-tooltip">
                                                <div class="ap-tooltip-duration">{&step.duration}</div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>

                    <div class={classes!("ap-process-content", format!("animation-{}", *animation_state))}>
                        <div class="ap-process-details">
                            <div class="ap-process-header">
                                <div class="ap-process-title-wrapper">
                                    <div class="ap-process-icon" style={format!("background: {}", processes[*active_step].color)}>
                                        <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                                            <path d={processes[*active_step].icon}
                                                  stroke="white"
                                                  stroke-width="2"
                                                  stroke-linecap="round"
                                                  stroke-linejoin="round"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <h3 class="ap-process-title">{&processes[*active_step].title}</h3>
                                        <p class="ap-process-description">{&processes[*active_step].description}</p>
                                    </div>
                                </div>
                                <div class="ap-process-stats">
                                    <div class="ap-stat-item">
                                        <div class="ap-stat-value">{processes[*active_step].duration.clone()}</div>
                                        <div class="ap-stat-label">{"Тривалість"}</div>
                                    </div>
                                    <div class="ap-stat-item">
                                        <div class="ap-stat-value">{processes[*active_step].details.len()}</div>
                                        <div class="ap-stat-label">{"Етапи"}</div>
                                    </div>
                                </div>
                            </div>

                            <div class="ap-process-list">
                                {for processes[*active_step].details.iter().enumerate().map(|(i, detail)| {
                                    html! {
                                        <div class="ap-process-item">
                                            <div class="ap-process-item-marker" style={format!("background: {}", processes[*active_step].color)}>
                                                <div class="ap-item-number">{i + 1}</div>
                                            </div>
                                            <div class="ap-process-item-content">
                                                <div class="ap-item-text">{detail}</div>
                                                <div class="ap-item-status">
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
                    </div>
                </div>
            </section>
        </div>
    }
}
