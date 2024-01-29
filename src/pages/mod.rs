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
    tmpl.render(name, &ctx).unwrap_or_else(|_err| {
        ctx.insert("error_name", "not found");
        tmpl.render("error", &ctx).unwrap()
    })
}
