use dioxus::prelude::*;

//static CSS: Asset = asset!("/assets/main.css");
static HERO_IMAGE: Asset = asset!("/assets/img.png", ImageAssetOptions::new().with_avif());
static AVATAR_1: Asset = asset!("/assets/dog.jpg", ImageAssetOptions::new().with_avif());
static AVATAR_2: Asset = asset!("/assets/dog2.jpg", ImageAssetOptions::new().with_avif());
static LOGO: Asset = asset!("/assets/favicon.ico");
static CSS: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        //document::Link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css" }
        //style { {include_str!("/assets/styles.css")} }
        div { class: "app",
            header {}
            hero_section {}
            features_section {}
            testimonials_section {}
            footer {}
        }
    }
}

#[component]
fn header() -> Element {
    rsx! {
        header { class: "header",
            nav { class: "nav",
                div { class: "nav-brand",
                    img { src: LOGO, alt: "Logo", class: "logo" }
                    h2 { class: "brand-name", "DesignLab" }
                }
                ul { class: "nav-links",
                    li { a { href: "#home", "Home" } }
                    li { a { href: "#features", "Features" } }
                    li { a { href: "#about", "About" } }
                    li { a { href: "#contact", "Contact" } }
                }
                div { class: "nav-actions",
                    button { class: "btn btn-outline", "Sign In" }
                    button { class: "btn btn-primary", "Get Started" }
                }
                button { class: "mobile-menu-toggle",
                    i { class: "fas fa-bars" }
                }
            }
        }
    }
}

#[component]
fn hero_section() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-content",
                div { class: "hero-text",
                    h1 { class: "hero-title",
                        "Build Amazing "
                        span { class: "gradient-text", "Digital Experiences" }
                    }
                    p { class: "hero-description",
                        "Create stunning web applications with our cutting-edge design tools and frameworks.
                        Transform your ideas into reality with pixel-perfect precision."
                    }
                    div { class: "hero-actions",
                        button { class: "btn btn-primary btn-large",
                            i { class: "fas fa-rocket" }
                            "Start Building"
                        }
                        button { class: "btn btn-ghost btn-large",
                            i { class: "fas fa-play" }
                            "Watch Demo"
                        }
                    }
                    div { class: "hero-stats",
                        div { class: "stat",
                            h3 { "10K+" }
                            p { "Active Users" }
                        }
                        div { class: "stat",
                            h3 { "99.9%" }
                            p { "Uptime" }
                        }
                        div { class: "stat",
                            h3 { "24/7" }
                            p { "Support" }
                        }
                    }
                }
                div { class: "hero-visual",
                    div { class: "hero-image-container",
                        img { src: HERO_IMAGE, alt: "Hero", class: "hero-image" }
                        div { class: "floating-card card-1",
                            i { class: "fas fa-chart-line" }
                            "Analytics"
                        }
                        div { class: "floating-card card-2",
                            i { class: "fas fa-shield-alt" }
                            "Security"
                        }
                        div { class: "floating-card card-3",
                            i { class: "fas fa-bolt" }
                            "Performance"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn features_section() -> Element {
    rsx! {
        section { class: "features",
            div { class: "container",
                div { class: "section-header",
                    h2 { class: "section-title", "Powerful Features" }
                    p { class: "section-description",
                        "Everything you need to build modern applications"
                    }
                }
                div { class: "features-grid",
                    feature_card {
                        icon: "fas fa-palette",
                        title: "Beautiful Design",
                        description: "Stunning UI components with modern aesthetics"
                    }
                    feature_card {
                        icon: "fas fa-code",
                        title: "Developer Friendly",
                        description: "Clean APIs and excellent documentation"
                    }
                    feature_card {
                        icon: "fas fa-mobile-alt",
                        title: "Responsive",
                        description: "Works perfectly on all devices and screen sizes"
                    }
                    feature_card {
                        icon: "fas fa-lightning-bolt",
                        title: "Lightning Fast",
                        description: "Optimized performance with minimal overhead"
                    }
                    feature_card {
                        icon: "fas fa-puzzle-piece",
                        title: "Modular",
                        description: "Extensible architecture with plugin support"
                    }
                    feature_card {
                        icon: "fas fa-globe",
                        title: "Global CDN",
                        description: "Worldwide content delivery for best performance"
                    }
                }
            }
        }
    }
}

#[component]
fn feature_card(icon: String, title: String, description: String) -> Element {
    rsx! {
        div { class: "feature-card",
            div { class: "feature-icon",
                i { class: "{icon}" }
            }
            h3 { class: "feature-title", "{title}" }
            p { class: "feature-description", "{description}" }
        }
    }
}

#[component]
fn testimonials_section() -> Element {
    rsx! {
        section { class: "testimonials",
            div { class: "container",
                div { class: "section-header",
                    h2 { class: "section-title", "What People Say" }
                    p { class: "section-description",
                        "Join thousands of satisfied developers"
                    }
                }
                div { class: "testimonials-grid",
                    testimonial_card {
                        avatar: AVATAR_1,
                        name: "Sarah Johnson",
                        role: "Frontend Developer",
                        content: "This platform has completely transformed how I build web applications. The developer experience is phenomenal!"
                    }
                    testimonial_card {
                        avatar: AVATAR_2,
                        name: "Michael Chen",
                        role: "Tech Lead",
                        content: "The performance gains we've seen are incredible. Our users love the new interface and it's so much easier to maintain."
                    }
                }
            }
        }
    }
}

#[component]
fn testimonial_card(avatar: Asset, name: String, role: String, content: String) -> Element {
    rsx! {
        div { class: "testimonial-card",
            div { class: "testimonial-content",
                p { "\"{content}\"" }
            }
            div { class: "testimonial-author",
                img { src: avatar, alt: "{name}", class: "author-avatar" }
                div { class: "author-info",
                    h4 { class: "author-name", "{name}" }
                    p { class: "author-role", "{role}" }
                }
            }
        }
    }
}

#[component]
fn footer() -> Element {
    rsx! {
        footer { class: "footer",
            div { class: "container",
                div { class: "footer-content",
                    div { class: "footer-section",
                        h3 { "DesignLab" }
                        p { "Building the future of web development, one component at a time." }
                        div { class: "social-links",
                            a { href: "#", class: "social-link",
                                i { class: "fab fa-twitter" }
                            }
                            a { href: "#", class: "social-link",
                                i { class: "fab fa-github" }
                            }
                            a { href: "#", class: "social-link",
                                i { class: "fab fa-linkedin" }
                            }
                        }
                    }
                    div { class: "footer-section",
                        h4 { "Product" }
                        ul { class: "footer-links",
                            li { a { href: "#", "Features" } }
                            li { a { href: "#", "Pricing" } }
                            li { a { href: "#", "Documentation" } }
                            li { a { href: "#", "API Reference" } }
                        }
                    }
                    div { class: "footer-section",
                        h4 { "Company" }
                        ul { class: "footer-links",
                            li { a { href: "#", "About" } }
                            li { a { href: "#", "Blog" } }
                            li { a { href: "#", "Careers" } }
                            li { a { href: "#", "Contact" } }
                        }
                    }
                    div { class: "footer-section",
                        h4 { "Newsletter" }
                        p { "Stay updated with our latest news and updates." }
                        div { class: "newsletter-form",
                            input {
                                type: "email",
                                placeholder: "Enter your email",
                                class: "newsletter-input"
                            }
                            button { class: "btn btn-primary", "Subscribe" }
                        }
                    }
                }
                div { class: "footer-bottom",
                    p { "© 2024 DesignLab. All rights reserved." }
                    div { class: "footer-legal",
                        a { href: "#", "Privacy Policy" }
                        a { href: "#", "Terms of Service" }
                    }
                }
            }
        }
    }
}
