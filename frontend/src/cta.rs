// cta.rs - оновлена версія з SVG іконками
use yew::prelude::*;

#[component]
pub fn CTASection() -> Html {
    let show_form = use_state(|| false);
    let form_data = use_state(|| FormData::default());

    //let on_show_form = {
    //    let show_form = show_form.clone();
    //    Callback::from(move |_| {
    //        show_form.set(true);
    //    })
    //};

    let on_close_form = {
        let show_form = show_form.clone();
        Callback::from(move |_| {
            show_form.set(false);
        })
    };

    let hover_states = use_state(|| vec![false; 7]);

    let on_hover = {
        let hover_states = hover_states.clone();
        Callback::from(move |(index, hover): (usize, bool)| {
            let mut states = (*hover_states).clone();
            states[index] = hover;
            hover_states.set(states);
        })
    };

    let contacts = vec![
        Contact {
            id: 0,
            icon: IconType::Phone,
            label: "Телефон",
            value: "+380 (63) 535-31-30",
            href: "tel:+380635353130",
            class: "phone-item",
            is_phone: true,
        },
        Contact {
            id: 1,
            icon: IconType::Instagram,
            label: "Instagram",
            value: "@company_gradprus",
            href: "https://instagram.com/company_gradprus",
            class: "instagram",
            is_phone: false,
        },
        Contact {
            id: 2,
            icon: IconType::Telegram,
            label: "Telegram",
            value: "@bkgradprus",
            href: "https://t.me/bkgradprus",
            class: "telegram",
            is_phone: false,
        },
        Contact {
            id: 3,
            icon: IconType::Viber,
            label: "Viber",
            value: "+380 (63) 535-31-30",
            href: "viber://chat?number=+380635353130",
            class: "viber",
            is_phone: false,
        },
        Contact {
            id: 4,
            icon: IconType::TikTok,
            label: "TikTok",
            value: "@gradprus",
            href: "https://tiktok.com/@gradprus",
            class: "tiktok",
            is_phone: false,
        },
        Contact {
            id: 5,
            icon: IconType::YouTube,
            label: "YouTube",
            value: "gradprus",
            href: "https://youtube.com/@gradprus?si=xB8tENRbCjyoBkxI",
            class: "youtube",
            is_phone: false,
        },
        Contact {
            id: 6,
            icon: IconType::Facebook,
            label: "Facebook",
            value: "gradprus",
            href: "https://www.facebook.com/share/1Ejs3cPsEP/?mibextid=wwXIfr",
            class: "facebook",
            is_phone: false,
        },
    ];

    html! {
        <div class="cta-main">
            <section id="cta" class={classes!("cta-section", if *show_form { "form-active" } else { "" })}>
                <div class="cta-container">
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
                            {"Наші експерти допоможуть реалізувати ваш проект від ідеї до ідеального результату. Професійний підхід, гарантія якості та прозора цінова політика."}
                        </p>
                    </div>

                    <div class="cta-actions">
                        //<button
                        //    class={classes!("cta-button", if *show_form { "button-expanded" } else { "" })}
                        //    onclick={on_show_form}
                        //>
                        //    <span class="button-text">{"Зв'язатися з нами"}</span>
                        //    <span class="button-arrow">
                        //        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                        //            <path d="M5 12H19M19 12L12 5M19 12L12 19"
                        //                  stroke="currentColor"
                        //                  stroke-width="2"
                        //                  stroke-linecap="round"
                        //                  stroke-linejoin="round"/>
                        //        </svg>
                        //    </span>
                        //    <span class="button-ring"></span>
                        //    <span class="button-sparkle"></span>
                        //</button>

                        <div class="contact-grid">
                            {for contacts.iter().map(|contact| {
                                let hover = (*hover_states)[contact.id];
                                let on_hover = on_hover.clone();
                                let id = contact.id;
                                let icon_svg = contact.icon.render();

                                html! {
                                    <a
                                        href={contact.href.clone()}
                                        target={if contact.is_phone { "_self" } else { "_blank" }}
                                        rel={if contact.is_phone { "" } else { "noopener noreferrer" }}
                                        class={classes!(
                                            "contact-item",
                                            contact.class.clone(),
                                            if hover { "hover" } else { "" }
                                        )}
                                        onmouseenter={Callback::from(
                    {
                                let on_hover = on_hover.clone();
                    move |_| {
                                            on_hover.emit((id, true));
                                        }})}
                                        onmouseleave={Callback::from(move |_| {
                                            on_hover.emit((id, false));
                                        })}
                                    >
                                        <div class="contact-icon">
                                            {if contact.is_phone {
                                                html! {
                                                    <>
                                                        <div class="phone-ring">
                                                            <div class="phone-ring-inner"></div>
                                                        </div>
                                                        {icon_svg}
                                                    </>
                                                }
                                            } else {
                                                icon_svg
                                            }}
                                        </div>
                                        <div class="contact-details">
                                            <div class="contact-label">{contact.label}</div>
                                            <div class="contact-value">{contact.value}</div>
                                        </div>
                                    </a>
                                }
                            })}
                        </div>
                    </div>
                </div>

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

struct Contact {
    id: usize,
    icon: IconType,
    label: &'static str,
    value: &'static str,
    href: &'static str,
    class: &'static str,
    is_phone: bool,
}

#[derive(Clone)]
enum IconType {
    Phone,
    Instagram,
    Telegram,
    Viber,
    TikTok,
    YouTube,
    Facebook,
}
impl IconType {
    fn render(&self) -> Html {
        match self {
            IconType::Phone => html! {
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07 19.5 19.5 0 01-6-6 19.79 19.79 0 01-3.07-8.67A2 2 0 014.11 2h3a2 2 0 012 1.72c.127.96.361 1.903.7 2.81a2 2 0 01-.45 2.11L8.09 9.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45c.907.339 1.85.573 2.81.7A2 2 0 0122 16.92z"/>
                </svg>
            },
            IconType::Instagram => html! {
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="2" y="2" width="20" height="20" rx="5" ry="5"/>
                    <path d="M16 11.37A4 4 0 1112.63 8 4 4 0 0116 11.37z"/>
                    <line x1="17.5" y1="6.5" x2="17.51" y2="6.5"/>
                </svg>
            },
            IconType::Telegram => html! {
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21.5 4.5L2.5 13.5L9 16.5L12.5 21.5L21.5 4.5Z"/>
                    <path d="M9 16.5L12.5 12.5"/>
                </svg>
            },
            IconType::Viber => html! {

            <svg width="20px" height="20px" viewBox="0 0 24 24" fill="none"  stroke="currentColor" >
            <path d="M13 4C12.4477 4 12 4.44772 12 5C12 5.55228 12.4477 6 13 6C14.2728 6 15.2557 6.41989 15.9179 7.08211C16.5801 7.74433 17 8.72725 17 10C17 10.5523 17.4477 11 18 11C18.5523 11 19 10.5523 19 10C19 8.27275 18.4199 6.75567 17.3321 5.66789C16.2443 4.58011 14.7272 4 13 4Z" fill="#0F0F0F"/>
            <path d="M5.014 8.00613C5.12827 7.1024 6.30277 5.87414 7.23488 6.01043L7.23339 6.00894C8.01251 6.15699 8.65217 7.32965 9.07373 8.10246C9.14298 8.22942 9.20635 8.34559 9.26349 8.44465C9.55041 8.95402 9.3641 9.4701 9.09655 9.68787C9.06561 9.7128 9.03317 9.73855 8.9998 9.76504C8.64376 10.0477 8.18114 10.4149 8.28943 10.7834C8.5 11.5 11 14 12.2296 14.7107C12.6061 14.9283 12.8988 14.5057 13.1495 14.1438C13.2087 14.0583 13.2656 13.9762 13.3207 13.9067C13.5301 13.6271 14.0466 13.46 14.5548 13.736C15.3138 14.178 16.0288 14.6917 16.69 15.27C17.0202 15.546 17.0977 15.9539 16.8689 16.385C16.4659 17.1443 15.3003 18.1456 14.4542 17.9421C12.9764 17.5868 7 15.27 5.08033 8.55801C4.97981 8.26236 4.99645 8.13792 5.01088 8.02991L5.014 8.00613Z" fill="#0F0F0F"/>
            <path d="M13 7C12.4477 7 12 7.44772 12 8C12 8.55228 12.4477 9 13 9C13.1748 9 13.4332 9.09745 13.6679 9.33211C13.9025 9.56676 14 9.82523 14 10C14 10.5523 14.4477 11 15 11C15.5523 11 16 10.5523 16 10C16 9.17477 15.5975 8.43324 15.0821 7.91789C14.5668 7.40255 13.8252 7 13 7Z" fill="#0F0F0F"/>
            <path fill-rule="evenodd" clip-rule="evenodd" d="M7.51742 23.8312C7.54587 23.8469 7.57508 23.8612 7.60492 23.874C8.14762 24.1074 8.81755 23.5863 10.1574 22.5442L11.5 21.5C14.1884 21.589 16.514 21.2362 18.312 20.6071C20.3227 19.9035 21.9036 18.3226 22.6072 16.3119C23.5768 13.541 23.5768 8.45883 22.6072 5.68794C21.9036 3.67722 20.3227 2.0963 18.312 1.39271C15.1103 0.272407 8.82999 0.293306 5.68806 1.39271C3.67733 2.0963 2.09642 3.67722 1.39283 5.68794C0.423255 8.45883 0.423255 13.541 1.39283 16.3119C2.09642 18.3226 3.67733 19.9035 5.68806 20.6071C6.08252 20.7451 6.52371 20.8965 7 21C7 22.6974 7 23.5461 7.51742 23.8312ZM9 20.9107V19.7909C9 19.5557 8.836 19.3524 8.60597 19.3032C7.84407 19.1403 7.08676 18.9776 6.34862 18.7193C4.91238 18.2168 3.78316 17.0875 3.2806 15.6513C2.89871 14.5599 2.66565 12.8453 2.66565 10.9999C2.66565 9.15453 2.89871 7.43987 3.2806 6.3485C3.78316 4.91227 4.91238 3.78304 6.34862 3.28048C7.61625 2.83692 9.71713 2.56282 11.9798 2.56032C14.2422 2.55782 16.3561 2.82723 17.6514 3.28048C19.0876 3.78304 20.2169 4.91227 20.7194 6.3485C21.1013 7.43987 21.3344 9.15453 21.3344 10.9999C21.3344 12.8453 21.1013 14.5599 20.7194 15.6513C20.2169 17.0875 19.0876 18.2168 17.6514 18.7193C15.5197 19.4652 13.259 19.549 11.0239 19.4828C10.9071 19.4794 10.7926 19.5165 10.7004 19.5882L9 20.9107Z" fill="#0F0F0F"/>
            </svg>
                        },
            IconType::TikTok => html! {
                <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor" stroke="none">
                    <path d="M18.6 4.8c-1.4-0.9-2.4-2.4-2.6-4.2h-3.6v14.4c0 2-1.6 3.6-3.6 3.6s-3.6-1.6-3.6-3.6 1.6-3.6 3.6-3.6c0.4 0 0.8 0 1.2 0.2V7.8c-0.4 0-0.8-0.2-1.2-0.2-4 0-7.2 3.2-7.2 7.2s3.2 7.2 7.2 7.2 7.2-3.2 7.2-7.2V10c1.4 1 3.2 1.6 5 1.6V8.2c-1.6 0-3-0.6-4.2-1.8z"/>
                </svg>
            },
            IconType::YouTube => html! {
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M22.54 6.42a2.78 2.78 0 00-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 00-1.94 2A29 29 0 001 11.75a29 29 0 00.46 5.33A2.78 2.78 0 003.4 19.1c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 001.94-2 29 29 0 00.46-5.25 29 29 0 00-.46-5.43z"/>
                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"/>
                </svg>
            },
            IconType::Facebook => html! {
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M18 2h-3a5 5 0 00-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 011-1h3z"/>
                </svg>
            },
        }
    }
}
