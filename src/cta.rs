// cta.rs
use yew::prelude::*;

#[component]
pub fn CTASection() -> Html {
    let show_form = use_state(|| false);
    let phone_hover = use_state(|| false);
    let email_hover = use_state(|| false);
    let form_data = use_state(|| FormData::default());

    let on_show_form = {
        let show_form = show_form.clone();
        Callback::from(move |_| {
            show_form.set(true);
        })
    };

    let on_close_form = {
        let show_form = show_form.clone();
        Callback::from(move |_| {
            show_form.set(true);
        })
    };

    let on_phone_hover = {
        let phone_hover = phone_hover.clone();
        Callback::from(move |hover| {
            phone_hover.set(hover);
        })
    };

    let on_email_hover = {
        let email_hover = email_hover.clone();
        Callback::from(move |hover| {
            email_hover.set(hover);
        })
    };

    html! {
        <div class="cta">
        <section class={classes!("cta-section", if *show_form { "form-active" } else { "" })}>
            // Background elements
            <div class="cta-background">
                <div class="background-grid"></div>
                <div class="floating-shapes">
                    <div class="shape shape-1"></div>
                    <div class="shape shape-2"></div>
                    <div class="shape shape-3"></div>
                </div>
                <div class="light-beam"></div>
            </div>

            // Main content container
            <div class="cta-container">
                // Left side - Main message
                <div class="cta-content">
                    <div class="badge">
                        <span class="badge-dot"></span>
                        {"ОБМОВИМО ПРОЄКТ ЗА 24 ГОДИНИ"}
                    </div>

                    <h1 class="cta-headline">
                        <span class="headline-line headline-line-1">
                            <span class="text-reveal">{"Готові до ремонту?"}</span>
                        </span>
                        <span class="headline-line headline-line-2">
                            <span class="text-highlight">
                                {"Зробіть перший крок"}
                                <span class="highlight-underline"></span>
                            </span>
                        </span>
                    </h1>

                    <p class="cta-description">
                        {"Наші експерти допоможуть реалізувати ваш проект від ідеї до ідеального результату.
                        Професійний підхід, гарантія якості та прозора цінова політика."}
                    </p>

                    // Stats counter
                    <div class="stats-counter">
                        <div class="stat-item">
                            <div class="stat-number" data-count="150">{"0"}</div>
                            <div class="stat-label">{"виконаних проектів"}</div>
                        </div>
                        <div class="stat-item">
                            <div class="stat-number" data-count="98">{"0"}</div>
                            <div class="stat-label">{"задоволених клієнтів"}</div>
                        </div>
                        <div class="stat-item">
                            <div class="stat-number" data-count="5">{"0"}</div>
                            <div class="stat-label">{"років досвіду"}</div>
                        </div>
                    </div>
                </div>

                // Right side - Contact options
                <div class="cta-actions">
                    // Main CTA button
                    <button
                        class={classes!("cta-button", if *show_form { "button-expanded" } else { "" })}
                        onclick={on_show_form}
                    >
                        <span class="button-text">{"Зв'язатися з нами"}</span>
                        <span class="button-arrow">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                                <path d="M5 12H19M19 12L12 5M19 12L12 19"
                                      stroke="currentColor"
                                      stroke-width="2"
                                      stroke-linecap="round"
                                      stroke-linejoin="round"/>
                            </svg>
                        </span>
                        <span class="button-ring"></span>
                        <span class="button-sparkle"></span>
                    </button>

                    // Contact methods
                    <div class="contact-methods">
                        <a
                            href="tel:+380XXXXXXXXX"
                            class={classes!("contact-item", if *phone_hover { "hover" } else { "" })}
                            onmouseenter={Callback::from({
                            let on_phone_hover = on_phone_hover.clone();
                            move |_| on_phone_hover.emit(true)})}
                            onmouseleave={Callback::from(
        {
                            let on_phone_hover = on_phone_hover.clone();
        move |_| on_phone_hover.emit(false)})}
                        >
                            <div class="contact-icon">
                                <div class="phone-ring">
                                    <div class="phone-ring-inner"></div>
                                </div>
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                    <path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07 19.5 19.5 0 01-6-6 19.79 19.79 0 01-3.07-8.67A2 2 0 014.11 2h3a2 2 0 012 1.72c.127.96.361 1.903.7 2.81a2 2 0 01-.45 2.11L8.09 9.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45c.907.339 1.85.573 2.81.7A2 2 0 0122 16.92z"
                                          stroke="currentColor"
                                          stroke-width="2"
                                          stroke-linecap="round"
                                          stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <div class="contact-details">
                                <div class="contact-label">{"Телефон"}</div>
                                <div class="contact-value">{"+380 (XX) XXX-XX-XX"}</div>
                            </div>
                        </a>

                        <a
                            href="mailto:info@gradprus.com"
                            class={classes!("contact-item", if *email_hover { "hover" } else { "" })}
                            onmouseenter={
                            let on_email_hover = on_email_hover.clone();
        Callback::from(move |_| on_email_hover.emit(true))}
                            onmouseleave={
                            let on_email_hover = on_email_hover.clone();
        Callback::from(move |_| on_email_hover.emit(false))}
                        >
                            <div class="contact-icon">
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"
                                          stroke="currentColor"
                                          stroke-width="2"
                                          stroke-linecap="round"
                                          stroke-linejoin="round"/>
                                    <polyline points="22,6 12,13 2,6"
                                             stroke="currentColor"
                                             stroke-width="2"
                                             stroke-linecap="round"
                                             stroke-linejoin="round"/>
                                </svg>
                            </div>
                            <div class="contact-details">
                                <div class="contact-label">{"Email"}</div>
                                <div class="contact-value">{"info@gradprus.com"}</div>
                            </div>
                        </a>
                    </div>

                    // Social proof
                    <div class="social-proof">
                        <div class="avatars">
                            <div class="avatar avatar-1"></div>
                            <div class="avatar avatar-2"></div>
                            <div class="avatar avatar-3"></div>
                            <div class="avatar avatar-4"></div>
                        </div>
                        <div class="proof-text">
                            <span class="proof-number">{"120+"}</span>
                            {" клієнтів обирають нас щомісяця"}
                        </div>
                    </div>
                </div>
            </div>

            // Contact form overlay
            if *show_form {
                <div class="contact-form-overlay">
                    <div class="contact-form-container">
                        <button class="form-close" onclick={on_close_form}>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                                <path d="M18 6L6 18M6 6l12 12"
                                      stroke="currentColor"
                                      stroke-width="2"
                                      stroke-linecap="round"
                                      stroke-linejoin="round"/>
                            </svg>
                        </button>

                        <div class="form-header">
                            <h3>{"Заповніть форму"}</h3>
                            <p>{"Ми зв'яжемося з вами протягом 30 хвилин"}</p>
                        </div>

                        <form class="contact-form">
                            <div class="form-group">
                                <input
                                    type="text"
                                    placeholder=" "
                                    class="form-input"
                                />
                                <label class="form-label">{"Ім'я"}</label>
                                <div class="form-line"></div>
                            </div>

                            <div class="form-group">
                                <input
                                    type="tel"
                                    placeholder=" "
                                    class="form-input"
                                />
                                <label class="form-label">{"Телефон"}</label>
                                <div class="form-line"></div>
                            </div>

                            <div class="form-group">
                                <textarea
                                    placeholder=" "
                                    class="form-textarea"
                                />
                                <label class="form-label">{"Опис проекту"}</label>
                                <div class="form-line"></div>
                            </div>

                            <button type="submit" class="submit-button">
                                <span class="submit-text">{"Надіслати заявку"}</span>
                                <span class="submit-progress"></span>
                            </button>
                        </form>
                    </div>
                </div>
            }
        </section>
        </div>
    }
}

#[derive(Default)]
struct FormData {
    name: String,
    phone: String,
    message: String,
}
