use yew::prelude::*;
use yew_hooks::{use_async_with_options, use_interval, UseAsyncOptions};

use crate::{api::ApiRequest, BACKEND_URL};

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
    let images = use_state(|| vec!["image_remont1.jpeg", "image_remont2.jpeg", "image_remont3.jpeg", "image_remont4.jpeg"]);


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
                            // After showing all individually, transition to showing all in a row
                            phase_state.set(PhaseState::AllTheRow);
                        }
                    }
                    PhaseState::AllTheRow => {
                        let current_step = *title_animation_step;

                        if !*show_title && current_step == 0 {
                            // Start showing title
                            show_title.set(true);
                            title_animation_step.set(1);
                        } else if current_step < 3 {
                            // Progress through animation steps
                            title_animation_step.set(current_step + 1);
                        } else if current_step >= 3 {
                            // After title animation is complete, mark for scroll
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
    <div class="preloader-main">
    <div class={format!("preloader preloader-active {}", if matches!(*phase_state, PhaseState::Complete){""} else{"completed"})}>
        <div class="progress-bar">
            <div
                class="progress"
                style={format!("width: {}%", (*step as f32 / total as f32) * 100.0)}
            />
        </div>


        <div class={match *phase_state {
            PhaseState::AllTheRow => "preloader-images show-all",
            PhaseState::Complete => "preloader-images show-all fade-out",
            PhaseState::OneByOne => "preloader-images"
        }}>
            { for (0..total).map(|i| {
                let class = match *phase_state{
                PhaseState::AllTheRow | PhaseState::Complete =>
                    format!("img img-{}", i + 1),
                PhaseState::OneByOne => if *step > i {
                    "img active".to_string()
                } else {
                    "img".to_string()
                }

                };

            {if let Some(src) = images.get(i).clone(){

                html! {
                    <div class={class}>
                        <img src={format!("static/{}", src.clone())} />
                    </div>
                }
            }else{html!{}}}
            })}
        </div>

        // Title overlay for AllTheRow phase
        if matches!(*phase_state, PhaseState::AllTheRow | PhaseState::Complete) && *show_title {
            <div class={classes!("title-overlay", format!("animation-step-{}", *title_animation_step))}>
                <div class="title-text-wrapper">
                    <div class="title-gradient-text">
                        {"GRADPRUS"}
                    </div>
                    <div class={classes!("title-subtext", if *title_animation_step >= 2 { "visible" } else { "" })}>
                        {"Професійний ремонт квартир та будинків"}
                    </div>
                </div>
            </div>
        }
     // Scroll indicator
                if matches!(*phase_state, PhaseState::Complete) {
                    <div class="scroll-indicator">
                        <div class="scroll-text">{"Прокрутіть вниз"}</div>
                        <div class="scroll-arrow">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                <path d="M7 10L12 15L17 10" stroke="currentColor" stroke-width="2"
                                      stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        </div>
                    </div>
                }
    </div>
        </div>
        </section>
    }
}
