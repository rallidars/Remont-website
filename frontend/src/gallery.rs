use gloo_timers::callback::Timeout;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

use crate::BACKEND_URL;

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

#[derive(Clone, Debug)]
pub struct ComparisonImage {
    before_image: String,
    after_image: String,
}

#[derive(Debug, Clone)]
pub struct PortfolioProject {
    pub id: usize,
    pub images: Vec<String>,
    pub title: String,
    pub description: String,
    pub category: ProjectCategory,
    pub area: u32,
    pub duration: String,
    pub comparison_images: Vec<ComparisonImage>,
    pub features: Vec<String>,
    pub color: String,
    pub year: u32,
    pub is_featured: bool,
}

#[component]
pub fn PortfolioGallery() -> Html {
    let active_category = use_state(|| ProjectCategory::All);
    let view_mode = use_state(|| "grid".to_string()); // grid, masonry, list
    let selected_project = use_state(|| Option::<usize>::None);
    let active_comparison = use_state(|| Option::<usize>::None);
    let hovered_project = use_state(|| Option::<usize>::None);
    let animation_state = use_state(|| "enter".to_string());
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

    //let projects = use_state(|| None::<Vec<PortfolioProject>>);
    let projects = Vec::<PortfolioProject>::new();


    //use_async_with_options(
    //    async move {
    //        match ApiRequest::get(format!("{}/portfolio_gallery", BACKEND_URL))
    //            .json_response::<Vec<PortfolioProject>>()
    //            .await
    //        {
    //            Ok(data) => projects.set(data),
    //            Err(_) => {}
    //        }
    //        Ok::<(), ()>(());
    //    },
    //    UseAsyncOptions::enable_auto(),
    //);
    //// Filter projects by category
    let filtered_projects = if *active_category == ProjectCategory::All {
        projects.clone()
    } else {
        projects
            .iter()
            .filter(|p| p.category == *active_category)
            .cloned()
            .collect::<Vec<_>>()
    };

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

    let start_comparison = {
        let active_comparison = active_comparison.clone();
        Callback::from(move |project_id: usize| {
            active_comparison.set(Some(project_id));
        })
    };

    // Stats
    let total_area: u32 = projects.iter().map(|p| p.area).sum();

    // In your component, add these states
    // Add these to your component's state initialization
    let comparison_image_index = use_state(|| 0);
    let gallery_image_index = use_state(|| 0);
    let is_dragging = use_state(|| false);
    let slider_position = use_state(|| 50); // percentage from left

    // Mouse events
    let start_drag = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            is_dragging.set(true);
        })
    };

    let stop_drag = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_| {
            is_dragging.set(false);
        })
    };

    let on_drag = {
        let slider_position = slider_position.clone();
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: MouseEvent| {
            if *is_dragging {
                e.prevent_default();

                // Get the slider container
                if let Some(target) = e.target() {
                    if let Some(element) = target.dyn_ref::<HtmlElement>() {
                        if let Ok(Some(slider)) = element.closest(".comparison-slider") {
                            let rect = slider.get_bounding_client_rect();
                            let x = (e.client_x() as f64 - rect.left()).clamp(0.0, rect.width());
                            let percentage = ((x / rect.width()) * 100.0) as i32;
                            slider_position.set(percentage);
                        }
                    }
                }
            }
        })
    };

    // Touch events for mobile
    let touch_start = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: TouchEvent| {
            e.prevent_default();
            is_dragging.set(true);
        })
    };

    let touch_end = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_| {
            is_dragging.set(false);
        })
    };

    let touch_move = {
        let slider_position = slider_position.clone();
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: TouchEvent| {
            if *is_dragging {
                e.prevent_default();

                if let Some(touch) = e.touches().get(0) {
                    if let Some(target) = e.target() {
                        if let Some(element) = target.dyn_ref::<HtmlElement>() {
                            if let Ok(Some(slider)) = element.closest(".comparison-slider") {
                                let rect = slider.get_bounding_client_rect();
                                let x = (touch.client_x() as f64 - rect.left())
                                    .clamp(0.0, rect.width());
                                let percentage = ((x / rect.width()) * 100.0) as i32;
                                slider_position.set(percentage);
                            }
                        }
                    }
                }
            }
        })
    };

    html! {
                    <div class="portfolio-main">
                    <section class="portfolio-section" id="portfolio">
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
                                        {"Понад 200 успішно реалізованих проектів різної складності, деякі з них можна побачити в нашій галереї"}
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
                                                    <div class="image-overlay"><img src={project.images.first().map(|s| s.to_string()).unwrap_or("".to_string())} /></div>
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
                                                    {if project.comparison_images.is_empty(){
                                                        html!{}

                                                        }else{
                                                        html!{
                    <button
        class="btn-compare"
        onclick={Callback::from(move |_| start_comparison.emit(project_id))}
    >
        {"Порівняти"}
    </button>
                                                                    }
                                                                    }
                                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                    // In your comparison modal section:
        {if active_comparison.is_some() {
            let project_id = active_comparison.unwrap();
            if let Some(project) = projects.iter().find(|p| p.id == project_id) {
                let total_images = project.comparison_images.len();

                html! {
                    <div class="comparison-modal">
                        <div class="comparison-content">
                            <div class="comparison-header">
                                <div class="comparison-title-wrapper">
                                    <h3 class="comparison-title">
                                        {format!("Порівняння: {}", project.title)}
                                    </h3>
                                    {if total_images > 1 {
                                        html! {
                                            <div class="image-counter">
                                                {format!("{}/{}", *comparison_image_index + 1, total_images)}
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                                <button class="btn-close" onclick={Callback::from(move |_| active_comparison.set(None))}>
                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                        <path d="M18 6L6 18M6 6l12 12"
                                              stroke="currentColor" stroke-width="2"
                                              stroke-linecap="round" stroke-linejoin="round"/>
                                    </svg>
                                </button>
                            </div>

                            <div class="comparison-body">
                                // Image Navigation (if multiple images)
                                {if total_images > 1 {
                                    html! {
                                        <div class="comparison-navigation">
                                            <button
                                                class="nav-btn prev-btn"
                                                disabled={*comparison_image_index == 0}
                                                onclick={{
                                                    let comparison_image_index = comparison_image_index.clone();
                                                    let slider_position = slider_position.clone();
                                                    Callback::from(move |_| {
                                                        let new_index = if *comparison_image_index > 0 { *comparison_image_index - 1 } else { 0 };
                                                        comparison_image_index.set(new_index);
                                                        // Reset slider position when changing images
                                                        slider_position.set(50);
                                                    })
                                                }}
                                            >
                                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                    <path d="M15 18l-6-6 6-6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                </svg>
                                            </button>
                                            <span class="nav-label">
                                                {format!("Фото {} з {}", *comparison_image_index + 1, total_images)}
                                            </span>
                                            <button
                                                class="nav-btn next-btn"
                                                disabled={*comparison_image_index == total_images - 1}
                                                onclick={{
                                                    let comparison_image_index = comparison_image_index.clone();
                                                    let slider_position = slider_position.clone();
                                                    Callback::from(move |_| {
                                                        let new_index = if *comparison_image_index < total_images - 1 { *comparison_image_index + 1 } else { total_images - 1 };
                                                        comparison_image_index.set(new_index);
                                                        // Reset slider position when changing images
                                                        slider_position.set(50);
                                                    })
                                                }}
                                            >
                                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                                    <path d="M9 18l6-6-6-6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                </svg>
                                            </button>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}

                                // Main Revealing Slider
                                <div
                                    class="comparison-slider"
                                    onmousemove={on_drag.clone()}
                                    onmouseup={stop_drag.clone()}
                                    onmouseleave={stop_drag.clone()}
                                    ontouchmove={touch_move.clone()}
                                    ontouchend={touch_end.clone()}
                                    ontouchcancel={touch_end.clone()}
                                >
                                    // After image (full width, underneath)
                                    <div class="slider-after">
                                        <div class="slider-label after-label">{"ПІСЛЯ"}</div>
                                        <img
                                            src={project.comparison_images[*comparison_image_index].after_image.clone()}
                                            alt="After"
                                            class="comparison-img after-img"
                                        />
                                    </div>

                                    // Before image (revealed by clip-path)
                                    <div
                                        class="slider-before"
                                        style={format!("clip-path: polygon(0% 0%, {}% 0%, {}% 100%, 0% 100%)",
                                            *slider_position, *slider_position)}
                                    >
                                        <div class="slider-label before-label">{"ДО"}</div>
                                        <img
                                            src={project.comparison_images[*comparison_image_index].before_image.clone()}
                                            alt="Before"
                                            class="comparison-img before-img"
                                        />
                                    </div>

                                    // Draggable handle
                                    <div
                                        class="slider-handle"
                                        onmousedown={start_drag.clone()}
                                        ontouchstart={touch_start.clone()}
                                        style={format!("left: {}%", *slider_position)}
                                    >
                                        <div class="handle-icon">
                                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                                <path d="M8 7v10M16 7v10M12 3v18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                                            </svg>
                                        </div>
                                        <div class="handle-line"></div>
                                    </div>
                                </div>

                                // Thumbnails (if multiple images)
                                {if total_images > 1 {
                                    html! {
                                        <div class="comparison-thumbnails">
                                            {(0..total_images).map(|i| {
                                                let is_active = *comparison_image_index == i;
                                                let comparison_image_index = comparison_image_index.clone();
                                                let slider_position = slider_position.clone();
                                                html! {
                                                    <button
                                                        class={classes!("thumbnail-btn", is_active.then_some("active"))}
                                                        onclick={Callback::from(move |_| {
                                                            comparison_image_index.set(i);
                                                            slider_position.set(50); // Reset position
                                                        })}
                                                    >
                                                        <div class="thumbnail-before">
                                                            <img src={project.comparison_images[i].before_image.clone()} alt="Before" />
                                                            <span class="thumbnail-label">{"До"}</span>
                                                        </div>
                                                        <div class="thumbnail-after">
                                                            <img src={project.comparison_images[i].after_image.clone()} alt="After" />
                                                            <span class="thumbnail-label">{"Після"}</span>
                                                        </div>
                                                    </button>
                                                }
                                            }).collect::<Html>()}
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        }}



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
                                                    <div class="main-image"><img  src={project.images.get(*gallery_image_index).map(|s| s.to_string()).unwrap_or("".to_string())} /></div>
                                                </div>
                                                <div class="gallery-thumbs">
                                                    {project.images.iter().enumerate().map(|(i,image)| {
                                                let is_active = *gallery_image_index == i;
                                                let gallery_image_index = gallery_image_index.clone();
                                                        html! {
                                                    <button
                                                        class={classes!("thumbnail-btn", is_active.then_some("active"))}
                                                        onclick={Callback::from(move |_| {
                                                            gallery_image_index.set(i);
                                                        })}
                                                    >
                                                            <div class="thumb-item"><img src={image.clone()} /></div>
                                                    </button>
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
                    </div>
                }
}
