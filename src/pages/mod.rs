use tera::Tera;

pub fn create_templates() -> (Tera, String) {
    let mut tera = Tera::default();

    tera.add_raw_templates(vec![
        ("error", include_str!("dist/error.html")),
        ("login", include_str!("dist/login.html")),
        ("provider", include_str!("dist/provider.html")),
    ])
    .unwrap();

    return tera;
}
