use ::rustfmt::{
    config::{
        Config,
        Density,
        MultilineStyle,
    },
    format_input,
    Input,
    Summary,
};
use std::path::{
    Path,
    PathBuf,
};

#[fn_fixture::snapshot("snapshot-tests/code")]
fn expected<T: std::fmt::Debug>(t: T) -> T { t }

#[fn_fixture::snapshot("snapshot-tests/examples")]
fn parse_unsigned_number(value: &str) -> Result<usize, impl std::fmt::Debug> {
    value.parse()
}

#[fn_fixture::snapshot("snapshot-tests/examples")]
fn parse_signed_number(value: &str) -> Result<isize, impl std::fmt::Debug> {
    value.parse()
}

#[fn_fixture::snapshot(plaintext "snapshot-tests/source")]
fn transform_plaintext(
    params: (&str, &str),
) -> impl std::fmt::Display {
    let (_, result) = transform_common(params).unwrap();
    let (result, _, _) = result.unwrap();

    struct Wrap(Vec<String>);
    impl std::fmt::Display for Wrap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for string in self.0.iter() {
                writeln!(f, "{}", string)?;
            }
            Ok(())
        }
    }

    Wrap(result)
}

#[fn_fixture::snapshot("snapshot-tests/source")]
fn transform(
    params: (&str, &str),
) -> impl std::fmt::Debug {
    transform_common(params)
}

fn transform_common(
    params: (&str, &str),
) -> Result<
    (
        bool,
        Result<
            (Vec<String>, Summary, String),
            (std::io::Error, Summary),
        >,
    ),
    Box<dyn std::error::Error>
> {
    let (path, contents) = params;
    let result = fn_fixture_lib::make_snapshots(
        &path.parse().map_err(|err| format!("{:?}", err))?,
        &contents.parse().map_err(|err| format!("{:?}", err))?,
    );
    let err = result.is_err();
    let raw = format!("{}", match &result {
        Ok(value) => value,
        Err(value) => value,
    });

    let search_path = format!("{:?}", Path::new(".")
        .canonicalize()
        .ok()
        .as_ref()
        .map(PathBuf::as_path)
        .and_then(Path::to_str)
        .expect("We have two parents")
    );
    let search_path_str = &search_path[0..(search_path.len() - 1)];
    let double_path = format!("{:?}", &search_path_str[1..]);
    let double_path_str = &double_path[0..(double_path.len() - 1)];

    let fmt_result = format_input(
        Input::Text(raw),
        &make_fmt_config()?,
        None as Option<&mut Vec<u8>>
    )
        .map(|(summary, filemap, report)| (
            match filemap.as_slice() {
                &[(_, ref contents)] => {
                    format!("{}", contents)
                        .lines()
                        .map(str::to_string)
                        .map(|line| {
                            // All of this replacing is to make the tests less system-dependent.
                            let mut new = line
                                .replace(search_path_str, "\".")
                                .replace(double_path_str, "\".")
                                ;
                            if line.len() != new.len() && std::path::MAIN_SEPARATOR == '\\' {
                                new = new.replace("\\\\\\\\", "/").replace("\\\\", "/");
                            }
                            new
                        })
                        .collect()
                },
                _ => unreachable!(),
            },
            summary,
            format!("{}", report),
        ));

    Ok((err, fmt_result))
}

fn make_fmt_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config = Config::default();
    let mut setter = config.set();
    setter.error_on_line_overflow(false);
    setter.combine_control_expr(false);
    setter.struct_lit_multiline_style(MultilineStyle::ForceMulti);
    setter.fn_args_density(Density::Vertical);

    Ok(config)
}
