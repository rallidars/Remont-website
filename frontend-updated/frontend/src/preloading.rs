use yew::prelude::*;
use yew_hooks::use_interval;

pub enum PhaseState {
    OneByOne,
    AllTheRow,
    Complete,
}

#[component]
pub fn Preloading() -> Html {
    let step = use_state(|| 0usize);
    let total = 4;

    let phase_state = use_state(|| PhaseState::OneByOne);
    let show_title = use_state(|| false);
    let title_animation_step = use_state(|| 0usize);

    let images = use_state(|| {
        vec![
            "image_remont1.jpeg",
            "image_remont2.jpeg",
            "image_remont3.jpeg",
            "image_remont4.jpeg",
        ]
    });

    {
        let step = step.clone();
        let phase_state = phase_state.clone();
        let show_title = show_title.clone();
        let title_animation_step = title_animation_step.clone();

        use_interval(
            move || {
                let current = *step;

                match *phase_state {
                    PhaseState::OneByOne => {
                        if current < total {
                            step.set(current + 1);
                        } else {
                            phase_state.set(PhaseState::AllTheRow);
                        }
                    }

                    PhaseState::AllTheRow => {
                        let current_step = *title_animation_step;

                        if !*show_title && current_step == 0 {
                            show_title.set(true);
                            title_animation_step.set(1);
                        } else if current_step < 3 {
                            title_animation_step.set(current_step + 1);
                        } else {
                            phase_state.set(PhaseState::Complete);
                        }
                    }

                    PhaseState::Complete => {}
                }
            },
            700,
        );
    }

    html! {
        <section id="home">
            <div class="gp-preloader-main">

                <div class={classes!(
                    "gp-preloader",
                    "gp-preloader-active",
                    if matches!(*phase_state, PhaseState::Complete) {
                        ""
                    } else {
                        "gp-preloader-completed"
                    }
                )}>

                    // Progress
                    <div class="gp-preloader-progress-bar">
                        <div
                            class="gp-preloader-progress"
                            style={format!(
                                "width: {}%",
                                (*step as f32 / total as f32) * 100.0
                            )}
                        />
                    </div>

                    // Images
                    <div class={match *phase_state {
                        PhaseState::AllTheRow =>
                            "gp-preloader-images gp-preloader-images-show-all",

                        PhaseState::Complete =>
                            "gp-preloader-images gp-preloader-images-show-all gp-preloader-images-fade-out",

                        PhaseState::OneByOne =>
                            "gp-preloader-images",
                    }}>
                        {
                            for (0..total).map(|i| {
                                let class = match *phase_state {
                                    PhaseState::AllTheRow
                                    | PhaseState::Complete => {
                                        format!("gp-preloader-image gp-preloader-image-{}", i + 1)
                                    }

                                    PhaseState::OneByOne => {
                                        if *step > i {
                                            "gp-preloader-image gp-preloader-image-active".to_string()
                                        } else {
                                            "gp-preloader-image".to_string()
                                        }
                                    }
                                };

                                if let Some(src) = images.get(i).cloned() {
                                    html! {
                                        <div class={class}>
                                            <img
                                                src={format!("static/{}", src)}
                                                alt=""
                                            />
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            })
                        }
                    </div>

                    // Title
                    if matches!(
                        *phase_state,
                        PhaseState::AllTheRow | PhaseState::Complete
                    ) && *show_title {
                        <div class={classes!(
                            "gp-preloader-title-overlay",
                            format!(
                                "gp-preloader-title-animation-step-{}",
                                *title_animation_step
                            )
                        )}>
                            <div class="gp-preloader-title-wrapper">

                                <div class="gp-preloader-title-gradient">
                                    {"GRADPRUS"}
                                </div>

                                <div class={classes!(
                                    "gp-preloader-title-subtext",
                                    if *title_animation_step >= 2 {
                                        "gp-preloader-title-subtext-visible"
                                    } else {
                                        ""
                                    }
                                )}>
                                    {"Професійний ремонт квартир та будинків"}
                                    <p> {"в Києві та Київській області"}</p>
                                    <div class="hero-trust-line">{"Гарантія • Фіксована ціна • Здача в термін"}</div>
                                    <div class="hero-actions">
                                        <a class="hero-button hero-button-primary" href="tel:+380635353130">{"Зателефонувати"}</a>
                                        <a class="hero-button hero-button-secondary" href="https://t.me/bkgradprus" target="_blank" rel="noopener noreferrer">{"Написати в Telegram"}</a>
                                        <a class="hero-button hero-button-quiet" href="#calculator">{"Отримати кошторис"}</a>
                                    </div>
                                </div>

                            </div>
                        </div>
                    }

                    // Scroll indicator
                    if matches!(*phase_state, PhaseState::Complete) {
                        <div class="gp-preloader-scroll-indicator">

                            <div class="gp-preloader-scroll-text">
                                {"Прокрутіть вниз"}
                            </div>

                            <div class="gp-preloader-scroll-arrow">
                                <svg
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                >
                                    <path
                                        d="M7 10L12 15L17 10"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                </svg>
                            </div>

                        </div>
                    }

                </div>
            </div>
        </section>
    }
}
