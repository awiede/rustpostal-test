use postal::{Context, ExpandAddressOptions, InitOptions};

fn expand(ctx: &Context, address: &str, languages: Vec<&str>) -> Vec<String> {
    let mut expand_options: ExpandAddressOptions = ExpandAddressOptions::new();

    expand_options.set_languages(languages.as_slice());

    match ctx.expand_address(address, &mut expand_options) {
        Err(e) => {
            println!("Error normalizing query: {:?} error: {:?}", address, e);
            vec![]
        }
        Ok(expansions) => expansions.map(|expansion| expansion.to_string()).collect(),
    }
}

fn main() {
    let mut libpostal_context = Context::new();
    libpostal_context
        .init(InitOptions {
            expand_address: true,
            parse_address: true,
        })
        .unwrap();

    let test_cases: Vec<(&str, Vec<&str>)> = vec![
        ("Foo Str", vec!["en"]),
        ("Foo Str", vec!["de"]),
        ("Foo Str", vec!["de", "en"]),
    ];

    for test_case in test_cases {
        println!(
            "Testing:\nAddress: {:?}\nLanguage(s): {:?}",
            test_case.0,
            test_case.1.clone(),
        );
        println!(
            "Results: {:?}",
            expand(&libpostal_context, test_case.0, test_case.1)
        );
    }
}
