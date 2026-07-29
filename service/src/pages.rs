use maud::{html, DOCTYPE};

pub(crate) fn home(version: &str, features: &[String]) -> String {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "polyfill service" }
                style { "body{font-family:system-ui,sans-serif;max-width:40rem;margin:3rem auto;padding:0 1rem;line-height:1.5}code{background:#8882;padding:.1em .3em;border-radius:.2em}" }
            }
            body {
                h1 { "polyfill service" }
                p {
                    "Serves a fixed, server-configured polyfill bundle tailored to the "
                    "requesting browser. The bundle is defined in this server's "
                    code { "polyfill.toml" }
                    " — URL parameters are ignored."
                }
                p {
                    "Bundle: " code { "/polyfill.min.js" }
                    " (minified) or " code { "/polyfill.js" }
                    " (readable, with per-feature comments)."
                }
                h2 { "Configuration" }
                p { "polyfill-library " (version) }
                ul {
                    @for feature in features {
                        li { code { (feature) } }
                    }
                }
            }
        }
    }
    .into_string()
}
