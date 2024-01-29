use macros_rs::fmtstr;
use tera::{Context, Tera};

#[derive(Clone)]
pub struct TeraState(pub tera::Tera);

pub fn create_templates() -> TeraState {
    let mut tera = Tera::default();

    tera.add_raw_templates(vec![
        ("error", include_str!("dist/error.html")),
        ("login", include_str!("dist/login.html")),
        ("provider", include_str!("dist/provider.html")),
    ])
    .unwrap();

    return TeraState(tera);
}

pub fn render(name: &str, tmpl: &Tera, ctx: &mut Context) -> String {
    let config = crate::CONFIG.get().unwrap();

    ctx.insert("app_name", &config.settings.app.name);
    ctx.insert("app_logo", &config.settings.app.logo);
    ctx.insert("app_accent", &config.settings.app.accent);
    ctx.insert("app_pages", &config.settings.app.pages);

    tmpl.render(name, &ctx).unwrap_or_else(|_err| {
        ctx.insert("error_code", &404);
        ctx.insert("error_name", "Template not found");
        ctx.insert("error_message", fmtstr!("The template {name} could not be found."));
        tmpl.render("error", &ctx).unwrap()
    })
}
