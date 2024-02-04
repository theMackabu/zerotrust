use macros_rs::fmtstr;
use tera::{Context, Tera};

#[derive(Clone)]
pub struct TeraState(pub tera::Tera);

pub fn create_templates() -> TeraState {
    let mut tera = Tera::default();

    tera.add_raw_templates(vec![
        ("setup", include_str!("dist/setup.html")),
        ("error", include_str!("dist/error.html")),
        ("login", include_str!("dist/login.html")),
        ("logout", include_str!("dist/logout.html")),
        ("provider", include_str!("dist/provider.html")),
    ])
    .unwrap();

    return TeraState(tera);
}

pub fn render(name: &str, tmpl: &Tera, ctx: &mut Context) -> String {
    let config = crate::CONFIG.get().unwrap();

    ctx.insert("build_hash", env!("GIT_HASH"));
    ctx.insert("build_profile", env!("PROFILE"));
    ctx.insert("build_version", env!("CARGO_PKG_VERSION"));
    ctx.insert("app_name", &config.settings.app.name);
    ctx.insert("app_logo", &config.settings.app.logo);
    ctx.insert("app_accent", &config.settings.app.accent);
    ctx.insert("app_pages", &config.settings.app.pages);
    ctx.insert("prefix", &config.settings.server.prefix);

    match &config.settings.app.favicon {
        Some(icon) => ctx.insert("app_icon", &icon),
        None => ctx.insert("app_icon", &config.settings.app.logo),
    }

    tmpl.render(name, &ctx).unwrap_or_else(|err| {
        ctx.insert("error_code", &404);
        ctx.insert("error_name", "Template not found");
        ctx.insert("error_message", fmtstr!("The template {name} could not be found."));

        tracing::error!("{err:?}");
        tmpl.render("error", &ctx).unwrap()
    })
}
