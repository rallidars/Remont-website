use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyType {
    Apartment,
    House,
    Office,
    Commercial,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalculatorStep {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub current: bool,
}

#[derive(Debug, Clone)]
pub struct PricePackage {
    pub name: String,
    pub description: String,
    pub price_per_sqm: f64,
    pub features: Vec<String>,
    pub color: String,
}

#[component]
pub fn Calculator() -> Html {
    let current_step = use_state(|| 0usize);
    let property_type = use_state(|| PropertyType::Apartment);
    let area = use_state(|| 100.0f64);
    let has_project = use_state(|| false);
    let urgency_level = use_state(|| 1);
    let selected_package = use_state(|| 0usize);
    let animation_state = use_state(|| "enter".to_string());
    let is_complete = use_state(|| false);
    let show_gift = use_state(|| false);

    // ============================================================
    // AREA DRAG STATE
    // ============================================================

    let drag_start_y = use_state(|| 0.0f64);
    let drag_start_area = use_state(|| 100.0f64);
    let is_dragging_area = use_state(|| false);

    // ============================================================
    // STEPS
    // ============================================================

    let steps = vec![
        CalculatorStep {
            id: 0,
            title: "Тип об'єкта".to_string(),
            description: "Що плануєте ремонтувати?".to_string(),
            completed: false,
            current: true,
        },
        CalculatorStep {
            id: 1,
            title: "Площа".to_string(),
            description: "Загальна площа приміщення".to_string(),
            completed: false,
            current: false,
        },
        CalculatorStep {
            id: 2,
            title: "Проект".to_string(),
            description: "Наявність дизайн-проекту".to_string(),
            completed: false,
            current: false,
        },
        CalculatorStep {
            id: 3,
            title: "Терміни".to_string(),
            description: "Терміновість виконання".to_string(),
            completed: false,
            current: false,
        },
        CalculatorStep {
            id: 4,
            title: "Пакет".to_string(),
            description: "Виберіть тип ремонту".to_string(),
            completed: false,
            current: false,
        },
    ];

    // ============================================================
    // PACKAGES
    // ============================================================

    let packages = vec![
        PricePackage {
            name: "Економ".to_string(),
            description: "Базовий ремонт з якісними матеріалами".to_string(),
            price_per_sqm: 4000.0,
            features: vec![
                "Чорнова обробка".to_string(),
                "Електромонтаж".to_string(),
                "Сантехніка".to_string(),
                "Стяжка підлоги".to_string(),
            ],
            color: "#4299e1".to_string(),
        },
        PricePackage {
            name: "Стандарт".to_string(),
            description: "Повний ремонт під ключ".to_string(),
            price_per_sqm: 7000.0,
            features: vec![
                "Усе з пакету Економ".to_string(),
                "Фінішне покриття стін".to_string(),
                "Натяжні стелі".to_string(),
                "Покриття підлоги".to_string(),
                "Освітлення".to_string(),
            ],
            color: "#38b2ac".to_string(),
        },
        PricePackage {
            name: "Преміум".to_string(),
            description: "Дизайнерський ремонт з авторським наглядом".to_string(),
            price_per_sqm: 12000.0,
            features: vec![
                "Усе з пакету Стандарт".to_string(),
                "Індивідуальний дизайн".to_string(),
                "Преміум матеріали".to_string(),
                "Smart-home система".to_string(),
                "Монтаж меблів".to_string(),
                "Прибирання після ремонту".to_string(),
            ],
            color: "#9f7aea".to_string(),
        },
    ];

    // ============================================================
    // TOTAL PRICE
    // ============================================================

    let total_price = {
        let base_price = packages[*selected_package].price_per_sqm * *area;

        let project_discount = if *has_project { 0.9 } else { 1.0 };

        let urgency_multiplier = match *urgency_level {
            1 => 1.0,
            2 => 1.15,
            3 => 1.3,
            _ => 1.0,
        };

        (base_price * project_discount * urgency_multiplier).round()
    };

    // ============================================================
    // ANIMATION
    // ============================================================

    {
        let animation_state = animation_state.clone();

        use_effect_with((), move |_| {
            Timeout::new(300, move || {
                animation_state.set("active".to_string());
            })
            .forget();

            || ()
        });
    }

    // ============================================================
    // NEXT STEP
    // ============================================================

    let go_next_step = {
        let current_step = current_step.clone();
        let animation_state = animation_state.clone();
        let steps = steps.clone();

        Callback::from(move |_| {
            let steps = steps.clone();
            let current_step = current_step.clone();
            let animation_state = animation_state.clone();

            animation_state.set("exit".to_string());

            Timeout::new(300, move || {
                if *current_step < steps.len() - 1 {
                    current_step.set(*current_step + 1);
                    animation_state.set("enter".to_string());
                }
            })
            .forget();
        })
    };

    // ============================================================
    // PREVIOUS STEP
    // ============================================================

    let go_prev_step = {
        let current_step = current_step.clone();
        let animation_state = animation_state.clone();

        Callback::from(move |_| {
            let current_step = current_step.clone();
            let animation_state = animation_state.clone();

            animation_state.set("exit".to_string());

            Timeout::new(300, move || {
                if *current_step > 0 {
                    current_step.set(*current_step - 1);
                    animation_state.set("enter".to_string());
                }
            })
            .forget();
        })
    };

    // ============================================================
    // COMPLETE
    // ============================================================

    let calculate_complete = {
        let is_complete = is_complete.clone();
        let show_gift = show_gift.clone();
        let animation_state = animation_state.clone();

        Callback::from(move |_| {
            let animation_state = animation_state.clone();
            let show_gift = show_gift.clone();
            let is_complete = is_complete.clone();

            animation_state.set("exit".to_string());

            Timeout::new(300, move || {
                is_complete.set(true);
                animation_state.set("enter".to_string());

                Timeout::new(1500, move || {
                    show_gift.set(true);
                })
                .forget();
            })
            .forget();
        })
    };

    // ============================================================
    // RESET
    // ============================================================

    let reset_calculator = {
        let current_step = current_step.clone();
        let is_complete = is_complete.clone();
        let show_gift = show_gift.clone();
        let animation_state = animation_state.clone();

        Callback::from(move |_| {
            let current_step = current_step.clone();
            let is_complete = is_complete.clone();
            let show_gift = show_gift.clone();
            let animation_state = animation_state.clone();

            animation_state.set("exit".to_string());

            Timeout::new(300, move || {
                current_step.set(0);
                is_complete.set(false);
                show_gift.set(false);
                animation_state.set("enter".to_string());
            })
            .forget();
        })
    };

    // ============================================================
    // AREA DRAG START
    // ============================================================

    let on_area_pointer_down = {
        let drag_start_y = drag_start_y.clone();
        let drag_start_area = drag_start_area.clone();
        let is_dragging_area = is_dragging_area.clone();
        let area = area.clone();

        Callback::from(move |e: PointerEvent| {
            e.prevent_default();

            drag_start_y.set(e.client_y() as f64);
            drag_start_area.set(*area);
            is_dragging_area.set(true);
        })
    };

    // ============================================================
    // AREA DRAG MOVE
    // ============================================================

    let on_area_pointer_move = {
        let drag_start_y = drag_start_y.clone();
        let drag_start_area = drag_start_area.clone();
        let is_dragging_area = is_dragging_area.clone();
        let area = area.clone();

        Callback::from(move |e: PointerEvent| {
            if !*is_dragging_area {
                return;
            }

            e.prevent_default();

            let current_y = e.client_y() as f64;

            // Вгору = позитивне значення
            // Вниз = негативне значення
            let delta_y = *drag_start_y - current_y;

            // 1 px = 1 м²
            let new_area = (*drag_start_area + delta_y).round().clamp(10.0, 200.0);

            area.set(new_area);
        })
    };

    // ============================================================
    // AREA DRAG END
    // ============================================================

    let on_area_pointer_up = {
        let is_dragging_area = is_dragging_area.clone();

        Callback::from(move |e: PointerEvent| {
            e.prevent_default();
            is_dragging_area.set(false);
        })
    };

    // ============================================================
    // PROGRESS
    // ============================================================

    let progress = ((*current_step as f32 + 1.0) / steps.len() as f32) * 100.0;

    // ============================================================
    // HTML
    // ============================================================

    html! {
        <div class="calculator-main" id="calculator">

            <section class="calculator-section">

                <div class="calculator-container">

                    // ====================================================
                    // HEADER
                    // ====================================================

                    <div class="calculator-header">

                        <h2 class="calculator-title">
                            <span class="title-gradient">
                                {"РОЗРАХУЄМО ВАРТІСТЬ РЕМОНТУ"}
                            </span>
                        </h2>

                        <p class="calculator-subtitle">
                            {"Відповідайте на питання та отримайте точну вартість за 2 хвилини"}
                        </p>

                    </div>

                    // ====================================================
                    // PROGRESS
                    // ====================================================

                    <div class="calculator-progress">

                        <div class="progress-track">

                            <div
                                class="progress-fill"
                                style={format!("width: {}%", progress)}
                            />

                        </div>

                        <div class="progress-steps">

                            {
                                for steps.iter().enumerate().map(|(i, step)| {

                                    let step_class =
                                        if i <= *current_step {
                                            "progress-step active"
                                        } else {
                                            "progress-step"
                                        };

                                    html! {

                                        <div class={step_class}>

                                            <div class="step-number">
                                                {i + 1}
                                            </div>

                                            <div class="step-title">
                                                {step.title.clone()}
                                            </div>

                                        </div>

                                    }
                                })
                            }

                        </div>

                    </div>

                    // ====================================================
                    // CONTENT
                    // ====================================================

                    <div class={
                        classes!(
                            "calculator-content",
                            format!("animation-{}", *animation_state)
                        )
                    }>

                        {
                            if !*is_complete {

                                match *current_step {

                                    // ==================================================
                                    // STEP 0 — PROPERTY TYPE
                                    // ==================================================

                                    0 => html! {

                                        <div class="calculator-step">

                                            <h3 class="step-question">
                                                {&steps[0].description}
                                            </h3>

                                            <div class="options-grid">

                                                {
                                                    [
                                                        (
                                                            "Квартира",
                                                            PropertyType::Apartment,
                                                            "#f56565"
                                                        ),
                                                        (
                                                            "Будинок",
                                                            PropertyType::House,
                                                            "#38a169"
                                                        ),
                                                        (
                                                            "Офіс",
                                                            PropertyType::Office,
                                                            "#d69e2e"
                                                        ),
                                                        (
                                                            "Комерція",
                                                            PropertyType::Commercial,
                                                            "#9f7aea"
                                                        ),
                                                    ]
                                                    .iter()
                                                    .map(
                                                        |(label, prop_type, color)| {

                                                            let is_selected =
                                                                *property_type == *prop_type;

                                                            let prop_type_clone =
                                                                prop_type.clone();

                                                            let property_type_state =
                                                                property_type.clone();

                                                            let on_select =
                                                                Callback::from(
                                                                    move |_| {
                                                                        property_type_state
                                                                            .set(
                                                                                prop_type_clone
                                                                                    .clone()
                                                                            );
                                                                    }
                                                                );

                                                            html! {

                                                                <button
                                                                    class={
                                                                        if is_selected {
                                                                            "option-card selected"
                                                                        } else {
                                                                            "option-card"
                                                                        }
                                                                    }
                                                                    onclick={on_select}
                                                                    style={
                                                                        format!(
                                                                            "--card-color: {}",
                                                                            color
                                                                        )
                                                                    }
                                                                >

                                                                    <div class="option-icon">

                                                                        <svg
                                                                            width="40"
                                                                            height="40"
                                                                            viewBox="0 0 24 24"
                                                                            fill="none"
                                                                        >

                                                                            <path
                                                                                d="M3 9L12 2L21 9V20C21 20.5304 20.7893 21.0391 20.4142 21.4142C20.0391 21.7893 19.5304 22 19 22H5C4.46957 22 3.96086 21.7893 3.58579 21.4142C3.21071 21.0391 3 20.5304 3 20V9Z"
                                                                                stroke="currentColor"
                                                                                stroke-width="2"
                                                                                stroke-linecap="round"
                                                                                stroke-linejoin="round"
                                                                            />

                                                                            <path
                                                                                d="M9 22V12H15V22"
                                                                                stroke="currentColor"
                                                                                stroke-width="2"
                                                                                stroke-linecap="round"
                                                                                stroke-linejoin="round"
                                                                            />

                                                                        </svg>

                                                                    </div>

                                                                    <div class="option-label">
                                                                        {*label}
                                                                    </div>

                                                                </button>

                                                            }
                                                        }
                                                    )
                                                    .collect::<Html>()
                                                }

                                            </div>

                                        </div>

                                    },

                                    // ==================================================
                                    // STEP 1 — AREA
                                    // ==================================================

                                    1 => html! {

                                        <div class="calculator-step">

                                            <h3 class="step-question">
                                                {&steps[1].description}
                                            </h3>

                                            <div class="area-input-container">

                                                <div
                                                    class="area-box-container"

                                                    onpointerdown={
                                                        on_area_pointer_down.clone()
                                                    }

                                                    onpointermove={
                                                        on_area_pointer_move.clone()
                                                    }

                                                    onpointerup={
                                                        on_area_pointer_up.clone()
                                                    }

                                                    onpointercancel={
                                                        on_area_pointer_up.clone()
                                                    }
                                                >

                                                    <div class="area-box">

                                                        // ============================================
                                                        // FILL
                                                        // ============================================

                                                        <div
                                                            class="area-fill"

                                                            style={format!(
                                                                "height: {}%",
                                                                ((*area - 10.0)
                                                                    / 190.0
                                                                    * 100.0)
                                                                    .clamp(0.0, 100.0)
                                                            )}
                                                        />

                                                        // ============================================
                                                        // VALUE
                                                        // ============================================

                                                        <div class="area-value-display">

                                                            <span class="area-number">
                                                                {format!("{:.0}", *area)}
                                                            </span>

                                                            <span class="area-unit">
                                                                {"м²"}
                                                            </span>

                                                        </div>

                                                        // ============================================
                                                        // HANDLE
                                                        // ============================================

                                                        <div class="drag-handle">

                                                            <svg
                                                                width="28"
                                                                height="28"
                                                                viewBox="0 0 24 24"
                                                                fill="none"
                                                            >

                                                                <path
                                                                    d="M12 5V19"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                />

                                                                <path
                                                                    d="M7 9L12 4L17 9"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                />

                                                                <path
                                                                    d="M7 15L12 20L17 15"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                />

                                                            </svg>

                                                        </div>

                                                    </div>

                                                    // ============================================
                                                    // LABELS
                                                    // ============================================

                                                    <div class="area-labels-vertical">

                                                        <span>
                                                            {"200 м²"}
                                                        </span>

                                                        <span>
                                                            {"150 м²"}
                                                        </span>

                                                        <span>
                                                            {"100 м²"}
                                                        </span>

                                                        <span>
                                                            {"50 м²"}
                                                        </span>

                                                        <span>
                                                            {"10 м²"}
                                                        </span>

                                                    </div>

                                                </div>

                                                <div class="area-hint">
                                                    {"Потягніть вгору або вниз, щоб змінити площу"}
                                                </div>

                                            </div>

                                        </div>

                                    },

                                    // ==================================================
                                    // STEP 2 — PROJECT
                                    // ==================================================

                                    2 => html! {

                                        <div class="calculator-step">

                                            <h3 class="step-question">
                                                {&steps[2].description}
                                            </h3>

                                            <div class="toggle-container">

                                                <div
                                                    class={
                                                        if *has_project {
                                                            "toggle-option selected"
                                                        } else {
                                                            "toggle-option"
                                                        }
                                                    }
                                                >

                                                    <button
                                                        class="toggle-btn"

                                                        onclick={
                                                            let has_project =
                                                                has_project.clone();

                                                            Callback::from(
                                                                move |_| {
                                                                    has_project.set(true)
                                                                }
                                                            )
                                                        }
                                                    >

                                                        <div class="toggle-icon">

                                                            <svg
                                                                width="32"
                                                                height="32"
                                                                viewBox="0 0 24 24"
                                                                fill="none"
                                                            >

                                                                <path
                                                                    d="M22 11.08V12C21.9988 14.1564 21.3005 16.2547 20.0093 17.9818C18.718 19.709 16.9033 20.9725 14.8354 21.5839C12.7674 22.1953 10.5573 22.1219 8.53447 21.3746C6.51168 20.6273 4.78465 19.2461 3.61096 17.4371C2.43727 15.628 1.87979 13.4881 2.02168 11.3363C2.16356 9.18455 2.99721 7.13631 4.39828 5.49706C5.79979 3.85781 7.69279 2.71537 9.79619 2.24013C11.8996 1.7649 14.1003 1.98232 16.07 2.85999"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                />

                                                                <path
                                                                    d="M22 4L12 14.01L9 11.01"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                />

                                                            </svg>

                                                        </div>

                                                        <div class="toggle-label">
                                                            {"Так, є готовий проект"}
                                                        </div>

                                                        <div class="toggle-benefit">
                                                            {"-10% до вартості"}
                                                        </div>

                                                    </button>

                                                </div>

                                                <div
                                                    class={
                                                        if !*has_project {
                                                            "toggle-option selected"
                                                        } else {
                                                            "toggle-option"
                                                        }
                                                    }
                                                >

                                                    <button
                                                        class="toggle-btn"

                                                        onclick={
                                                            let has_project =
                                                                has_project.clone();

                                                            Callback::from(
                                                                move |_| {
                                                                    has_project.set(false)
                                                                }
                                                            )
                                                        }
                                                    >

                                                        <div class="toggle-icon">

                                                            <svg
                                                                width="32"
                                                                height="32"
                                                                viewBox="0 0 24 24"
                                                                fill="none"
                                                            >

                                                                <path
                                                                    d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                />

                                                                <path
                                                                    d="M15 9L9 15"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                />

                                                                <path
                                                                    d="M9 9L15 15"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                />

                                                            </svg>

                                                        </div>

                                                        <div class="toggle-label">
                                                            {"Ні, потрібен проект"}
                                                        </div>

                                                        <div class="toggle-benefit">
                                                            {"Пропонуємо послуги дизайнера"}
                                                        </div>

                                                    </button>

                                                </div>

                                            </div>

                                        </div>

                                    },

                                    // ==================================================
                                    // STEP 3 — URGENCY
                                    // ==================================================

                                    3 => html! {

                                        <div class="calculator-step">

                                            <h3 class="step-question">
                                                {&steps[3].description}
                                            </h3>

                                            <div class="urgency-levels">

                                                {
                                                    [
                                                        (
                                                            "Стандартний",
                                                            1,
                                                            "1-3 місяці"
                                                        ),
                                                        (
                                                            "Пришвидшений",
                                                            2,
                                                            "3-6 тижнів"
                                                        ),
                                                        (
                                                            "Експрес",
                                                            3,
                                                            "2-4 тижні"
                                                        ),
                                                    ]
                                                    .iter()
                                                    .map(
                                                        |(label, level, timeframe)| {

                                                            let is_selected =
                                                                *urgency_level == *level;

                                                            let level_clone =
                                                                *level;

                                                            let urgency_level_state =
                                                                urgency_level.clone();

                                                            let on_select =
                                                                Callback::from(
                                                                    move |_| {
                                                                        urgency_level_state
                                                                            .set(level_clone);
                                                                    }
                                                                );

                                                            html! {

                                                                <button
                                                                    class={
                                                                        if is_selected {
                                                                            "urgency-option selected"
                                                                        } else {
                                                                            "urgency-option"
                                                                        }
                                                                    }

                                                                    onclick={on_select}
                                                                >

                                                                    <div class="urgency-header">

                                                                        <div class="urgency-label">
                                                                            {*label}
                                                                        </div>

                                                                        <div class="urgency-indicator">

                                                                            {
                                                                                (1..=3)
                                                                                    .map(|i| {

                                                                                        html! {

                                                                                            <div
                                                                                                class={
                                                                                                    if i <= level_clone {
                                                                                                        "dot active"
                                                                                                    } else {
                                                                                                        "dot"
                                                                                                    }
                                                                                                }
                                                                                            />

                                                                                        }

                                                                                    })
                                                                                    .collect::<Html>()
                                                                            }

                                                                        </div>

                                                                    </div>

                                                                    <div class="urgency-timeframe">
                                                                        {*timeframe}
                                                                    </div>

                                                                    <div class="urgency-percent">

                                                                        {
                                                                            if *level == 1 {
                                                                                "+0%"
                                                                            } else if *level == 2 {
                                                                                "+15%"
                                                                            } else {
                                                                                "+30%"
                                                                            }
                                                                        }

                                                                    </div>

                                                                </button>

                                                            }
                                                        }
                                                    )
                                                    .collect::<Html>()
                                                }

                                            </div>

                                        </div>

                                    },

                                    // ==================================================
                                    // STEP 4 — PACKAGE
                                    // ==================================================

                                    4 => html! {

                                        <div class="calculator-step">

                                            <h3 class="step-question">
                                                {&steps[4].description}
                                            </h3>

                                            <div class="package-selection">

                                                {
                                                    packages.iter().enumerate().map(
                                                        |(i, package)| {

                                                            let is_selected =
                                                                *selected_package == i;

                                                            let package_clone = i;

                                                            let selected_package_state =
                                                                selected_package.clone();

                                                            let on_select =
                                                                Callback::from(
                                                                    move |_| {
                                                                        selected_package_state
                                                                            .set(package_clone);
                                                                    }
                                                                );

                                                            html! {

                                                                <div
                                                                    class={
                                                                        if is_selected {
                                                                            "package-card selected"
                                                                        } else {
                                                                            "package-card"
                                                                        }
                                                                    }

                                                                    onclick={on_select}

                                                                    style={
                                                                        format!(
                                                                            "--package-color: {}",
                                                                            package.color
                                                                        )
                                                                    }
                                                                >

                                                                    <div class="package-header">

                                                                        <div class="package-name">
                                                                            {&package.name}
                                                                        </div>

                                                                        <div class="package-price">
                                                                            {format!(
                                                                                "{:.0} грн/м²",
                                                                                package.price_per_sqm
                                                                            )}
                                                                        </div>

                                                                    </div>

                                                                    <div class="package-description">
                                                                        {&package.description}
                                                                    </div>

                                                                    <ul class="package-features">

                                                                        {
                                                                            package.features.iter().map(
                                                                                |feature| {

                                                                                    html! {

                                                                                        <li class="package-feature">

                                                                                            <svg
                                                                                                width="20"
                                                                                                height="20"
                                                                                                viewBox="0 0 24 24"
                                                                                                fill="none"
                                                                                            >

                                                                                                <path
                                                                                                    d="M20 6L9 17L4 12"
                                                                                                    stroke="currentColor"
                                                                                                    stroke-width="2"
                                                                                                    stroke-linecap="round"
                                                                                                    stroke-linejoin="round"
                                                                                                />

                                                                                            </svg>

                                                                                            <span>
                                                                                                {feature}
                                                                                            </span>

                                                                                        </li>

                                                                                    }
                                                                                }
                                                                            )
                                                                            .collect::<Html>()
                                                                        }

                                                                    </ul>

                                                                    {
                                                                        if is_selected {

                                                                            html! {

                                                                                <div class="package-selected-badge">

                                                                                    <svg
                                                                                        width="20"
                                                                                        height="20"
                                                                                        viewBox="0 0 24 24"
                                                                                        fill="none"
                                                                                    >

                                                                                        <path
                                                                                            d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z"
                                                                                            stroke="currentColor"
                                                                                            stroke-width="2"
                                                                                            stroke-linecap="round"
                                                                                            stroke-linejoin="round"
                                                                                        />

                                                                                    </svg>

                                                                                </div>

                                                                            }

                                                                        } else {

                                                                            html! {}

                                                                        }
                                                                    }

                                                                </div>

                                                            }
                                                        }
                                                    )
                                                    .collect::<Html>()
                                                }

                                            </div>

                                        </div>

                                    },

                                    _ => html! {},
                                }

                            } else {

                                // ==================================================
                                // RESULT
                                // ==================================================

                                html! {

                                    <div class="result-container">

                                        <div class="result-header">

                                            <div class="result-icon">

                                                <svg
                                                    width="80"
                                                    height="80"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                >

                                                    <path
                                                        d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z"
                                                        stroke="currentColor"
                                                        stroke-width="2"
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                    />

                                                </svg>

                                            </div>

                                            <h3 class="result-title">
                                                {"Ваш попередній розрахунок готовий!"}
                                            </h3>

                                            <p class="result-subtitle">
                                                {"Залишився один крок до точного кошторису"}
                                            </p>

                                        </div>

                                        <div class="result-price">

                                            <div class="result-price-label">
                                                {"ОРІЄНТОВНА ВАРТІСТЬ РЕМОНТУ"}
                                            </div>

                                            <div class="price-amount">

                                                {format!("{:.0}", total_price)}

                                                <span class="price-currency">
                                                    {" грн"}
                                                </span>

                                            </div>

                                            <div class="price-details">

                                                {format!(
                                                    "{} м² × {:.0} грн/м²",
                                                    *area,
                                                    packages[*selected_package]
                                                        .price_per_sqm
                                                )}

                                            </div>

                                            <p class="result-price-note">
                                                {"Попередня оцінка. Фінальна вартість залежить від обсягу робіт і матеріалів."}
                                            </p>

                                        </div>

                                        <div class="result-contact-cta">

                                            <h4>{"Хочете знати точну вартість?"}</h4>

                                            <p>
                                                {"Уточнимо деталі вашого ремонту та підготуємо кошторис."}
                                            </p>

                                            <a class="result-main-cta" href="#contact">
                                                {"Отримати точний розрахунок за 15 хвилин"}
                                                <span>{" →"}</span>
                                            </a>

                                            <div class="result-contact-caption">
                                                {"Або зв’яжіться з нами напряму"}
                                            </div>

                                            <div class="result-contact-links">
                                                <a class="result-contact-link result-contact-phone" href="tel:+380XXXXXXXXX">
                                                    <span>{"☎"}</span>
                                                    {"Телефон"}
                                                </a>

                                                <a class="result-contact-link result-contact-telegram" href="https://t.me/YOUR_USERNAME" target="_blank" rel="noopener noreferrer">
                                                    <span>{"➤"}</span>
                                                    {"Telegram"}
                                                </a>

                                                <a class="result-contact-link result-contact-viber" href="viber://chat?number=%2B380XXXXXXXXX">
                                                    <span>{"✆"}</span>
                                                    {"Viber"}
                                                </a>
                                            </div>

                                            <div class="result-response-note">
                                                {"Швидко відповімо на ваш запит"}
                                            </div>

                                        </div>

                                        <div class="result-summary">

                                            <div class="summary-item">

                                                <span class="summary-label">
                                                    {"Тип об'єкта:"}
                                                </span>

                                                <span class="summary-value">

                                                    {
                                                        match *property_type {

                                                            PropertyType::Apartment =>
                                                                "Квартира",

                                                            PropertyType::House =>
                                                                "Будинок",

                                                            PropertyType::Office =>
                                                                "Офіс",

                                                            PropertyType::Commercial =>
                                                                "Комерційне приміщення",
                                                        }
                                                    }

                                                </span>

                                            </div>

                                            <div class="summary-item">

                                                <span class="summary-label">
                                                    {"Площа:"}
                                                </span>

                                                <span class="summary-value">
                                                    {format!("{} м²", *area)}
                                                </span>

                                            </div>

                                            <div class="summary-item">

                                                <span class="summary-label">
                                                    {"Пакет ремонту:"}
                                                </span>

                                                <span class="summary-value">
                                                    {&packages[*selected_package].name}
                                                </span>

                                            </div>

                                            <div class="summary-item">

                                                <span class="summary-label">
                                                    {"Наявність проекту:"}
                                                </span>

                                                <span class="summary-value">

                                                    {
                                                        if *has_project {
                                                            "Так"
                                                        } else {
                                                            "Ні"
                                                        }
                                                    }

                                                </span>

                                            </div>

                                            <div class="summary-item">

                                                <span class="summary-label">
                                                    {"Терміни:"}
                                                </span>

                                                <span class="summary-value">

                                                    {
                                                        match *urgency_level {

                                                            1 =>
                                                                "Стандартні (1-3 місяці)",

                                                            2 =>
                                                                "Пришвидшені (3-6 тижнів)",

                                                            3 =>
                                                                "Експрес (2-4 тижні)",

                                                            _ => "",
                                                        }
                                                    }

                                                </span>

                                            </div>

                                        </div>

                                        {
                                            if *show_gift {

                                                html! {

                                                    <div class="gift-offer animation-pulse">

                                                        <div class="gift-icon">

                                                            <svg
                                                                width="40"
                                                                height="40"
                                                                viewBox="0 0 24 24"
                                                                fill="none"
                                                            >

                                                                <path
                                                                    d="M20 12V22H4V12M22 7H2V12H22V7ZM12 22V7M12 7H7.5C6.83696 7 6.20107 6.73661 5.73223 6.26777C5.26339 5.79893 5 5.16304 5 4.5C5 3.83696 5.26339 3.20107 5.73223 2.73223C6.20107 2.26339 6.83696 2 7.5 2C11 2 12 7 12 7ZM12 7H16.5C17.163 7 17.7989 6.73661 18.2678 6.26777C18.7366 5.79893 19 5.16304 19 4.5C19 3.83696 18.7366 3.20107 18.2678 2.73223C18.2678 2.26339 17.163 2 16.5 2C13 2 12 7 12 7Z"
                                                                    stroke="currentColor"
                                                                    stroke-width="2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                />

                                                            </svg>

                                                        </div>

                                                        <div class="gift-content">

                                                            <div class="gift-title">
                                                                {"Подарунок при замовленні!"}
                                                            </div>

                                                            <div class="gift-description">
                                                                {"Отримайте безкоштовний подарунок при першому замовленні"}
                                                            </div>

                                                        </div>

                                                    </div>

                                                }

                                            } else {

                                                html! {}

                                            }
                                        }

                                    </div>

                                }

                            }
                        }

                    </div>

                    // ====================================================
                    // CONTROLS
                    // ====================================================

                    <div class="calculator-controls">

                        {
                            if !*is_complete {

                                html! {

                                    <>

                                        <button
                                            class="btn-secondary"
                                            onclick={go_prev_step}
                                            disabled={*current_step == 0}
                                        >

                                            <svg
                                                width="20"
                                                height="20"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                            >

                                                <path
                                                    d="M15 18L9 12L15 6"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                />

                                            </svg>

                                            {"Назад"}

                                        </button>

                                        {
                                            if *current_step == steps.len() - 1 {

                                                html! {

                                                    <button
                                                        class="btn-primary"
                                                        onclick={calculate_complete}
                                                    >

                                                        {"Розрахувати вартість"}

                                                        <svg
                                                            width="20"
                                                            height="20"
                                                            viewBox="0 0 24 24"
                                                            fill="none"
                                                        >

                                                            <path
                                                                d="M5 12H19M19 12L12 5M19 12L12 19"
                                                                stroke="currentColor"
                                                                stroke-width="2"
                                                                stroke-linecap="round"
                                                                stroke-linejoin="round"
                                                            />

                                                        </svg>

                                                    </button>

                                                }

                                            } else {

                                                html! {

                                                    <button
                                                        class="btn-primary"
                                                        onclick={go_next_step}
                                                    >

                                                        {"Далі"}

                                                        <svg
                                                            width="20"
                                                            height="20"
                                                            viewBox="0 0 24 24"
                                                            fill="none"
                                                        >

                                                            <path
                                                                d="M5 12H19M19 12L12 5M19 12L12 19"
                                                                stroke="currentColor"
                                                                stroke-width="2"
                                                                stroke-linecap="round"
                                                                stroke-linejoin="round"
                                                            />

                                                        </svg>

                                                    </button>

                                                }

                                            }
                                        }

                                    </>

                                }

                            } else {

                                html! {

                                    <>

                                        <button
                                            class="btn-secondary"
                                            onclick={reset_calculator}
                                        >

                                            <svg
                                                width="20"
                                                height="20"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                            >

                                                <path
                                                    d="M2.5 2v6h6M21.5 22v-6h-6M2.5 2l10 10M2.5 2l-1 1M21.5 22L11.5 12M21.5 22l1-1M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2z"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                />

                                            </svg>

                                            {"Новий розрахунок"}

                                        </button>

                                    </>

                                }

                            }
                        }

                    </div>

                    // ====================================================
                    // DISCLAIMER
                    // ====================================================

                    <div class="calculator-disclaimer">

                        <svg
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="none"
                        >

                            <path
                                d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />

                            <path
                                d="M12 8V12"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />

                            <path
                                d="M12 16H12.01"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />

                        </svg>

                        {
                            "Це орієнтовна вартість. Для точного розрахунку зателефонуйте нам або залиште заявку"
                        }

                    </div>

                </div>

            </section>

        </div>
    }
}
