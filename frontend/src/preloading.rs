use yew::prelude::*;
use yew_hooks::use_interval;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PhaseState {
    OneByOne,
    AllTheRow,
    Complete,
}

#[component]
pub fn Preloading() -> Html {
    let step = use_state(|| 0usize);
    let total = 4usize;

    let phase_state = use_state(|| PhaseState::OneByOne);
    let show_title = use_state(|| false);
    let title_animation_step = use_state(|| 0usize);

    let images = vec![
        "image_remont1.jpeg",
        "image_remont2.jpeg",
        "image_remont3.jpeg",
        "image_remont4.jpeg",
    ];

    {
        let step = step.clone();
        let phase_state = phase_state.clone();
        let show_title = show_title.clone();
        let title_animation_step = title_animation_step.clone();

        use_interval(
            move || {
                match *phase_state {
                    PhaseState::OneByOne => {
                        if *step < total {
                            step.set(*step + 1);
                        } else {
                            phase_state.set(PhaseState::AllTheRow);
                        }
                    }
                    PhaseState::AllTheRow => {
                        let current = *title_animation_step;

                        if !*show_title {
                            show_title.set(true);
                            title_animation_step.set(1);
                        } else if current < 3 {
                            title_animation_step.set(current + 1);
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

    let phase_class = match *phase_state {
        PhaseState::OneByOne => "gp-preloader-images",
        PhaseState::AllTheRow => "gp-preloader-images gp-preloader-images-show-all",
        PhaseState::Complete => {
            "gp-preloader-images gp-preloader-images-show-all gp-preloader-images-fade-out"
        }
    };

    html! {
        <section id="home">
            <div class="gp-preloader-main">
                <div class={classes!(
                    "gp-preloader",
                    "gp-preloader-active",
                    if *phase_state == PhaseState::Complete {
                        ""
                    } else {
                        "gp-preloader-completed"
                    }
                )}>
                    <div class="gp-preloader-progress-bar">
                        <div
                            class="gp-preloader-progress"
                            style={format!("width: {}%", (*step as f32 / total as f32) * 100.0)}
                        />
                    </div>

                    <div class={phase_class}>
                        {
                            for images.iter().enumerate().map(|(i, src)| {
                                let class = match *phase_state {
                                    PhaseState::OneByOne => {
                                        if *step > i {
                                            "gp-preloader-image gp-preloader-image-active".to_string()
                                        } else {
                                            "gp-preloader-image".to_string()
                                        }
                                    }
                                    PhaseState::AllTheRow | PhaseState::Complete => {
                                        format!("gp-preloader-image gp-preloader-image-{}", i + 1)
                                    }
                                };

                                html! {
                                    <div class={class}>
                                        <img
                                            src={format!("static/{}", src)}
                                            alt="Ремонт квартир у Києві"
                                        />
                                    </div>
                                }
                            })
                        }
                    </div>

                    if matches!(*phase_state, PhaseState::AllTheRow | PhaseState::Complete) && *show_title {
                        <div class={classes!(
                            "gp-preloader-title-overlay",
                            format!("gp-preloader-title-animation-step-{}", *title_animation_step)
                        )}>
                            <div class="gp-preloader-title-wrapper">
                                <div class="gp-preloader-title-gradient">
                                    {"GRADPRUS"}
                                </div>

                                <div class="gp-preloader-title-subtext gp-preloader-hero-content">
                                    <h1 class="gp-preloader-hero-heading">
                                        {"Професійний ремонт квартир і офісів у Києві"}
                                    </h1>

                                    <div class="gp-preloader-hero-benefits">
                                        <span>{"Гарантія"}</span>
                                        <span class="gp-preloader-benefit-dot">{"•"}</span>
                                        <span>{"Фіксована ціна"}</span>
                                        <span class="gp-preloader-benefit-dot">{"•"}</span>
                                        <span>{"Здача в термін"}</span>
                                    </div>

                                    <div class="gp-preloader-hero-actions">
                                        // Замініть номер і username на свої реальні контакти.
                                        <a
                                            class="gp-preloader-cta gp-preloader-cta-phone"
                                            href="tel:+380635353130"
                                        >
                                            <span class="gp-preloader-cta-icon">{"☎"}</span>
                                            <span>{"Зателефонувати"}</span>
                                        </a>

                                        <a
                                            class="gp-preloader-cta gp-preloader-cta-telegram"
                                            href="https://t.me/bkgradprus"
                                            target="_blank"
                                            rel="noopener noreferrer"
                                        >
                                            <span class="gp-preloader-cta-icon">{"➤"}</span>
                                            <span>{"Написати в Telegram"}</span>
                                        </a>

                                        <a
                                            class="gp-preloader-cta gp-preloader-cta-estimate"
                                            href="#calculator"
                                        >
                                            {"Отримати кошторис"}
                                            <span>{" →"}</span>
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }

                    if *phase_state == PhaseState::Complete {
                        <div class="gp-preloader-scroll-indicator">
                            <div class="gp-preloader-scroll-text">
                                {"Прокрутіть вниз"}
                            </div>
                            <div class="gp-preloader-scroll-arrow">
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
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
