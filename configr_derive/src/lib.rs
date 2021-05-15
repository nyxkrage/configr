use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Configr)]
pub fn configr_no_default(input: TokenStream) -> TokenStream {
	let DeriveInput { ident, data, .. } = parse_macro_input!(input);
	if let syn::Data::Struct(s) = data {
		if let syn::Fields::Named(f) = s.fields {
			let fields: Vec<String> = f
				.named
				.into_iter()
				.map(|f| f.ident.map(|i| i.to_string()).unwrap_or("".to_string()))
				.collect();
			return format!(
				r#"impl Config<Self> for {} {{
                fn populate_template(fd: std::fs::File) -> std::io::Result<()> {{
                    use std::io::Write;
                    let mut writer = std::io::BufWriter::new(fd);
                    for f in &{:?} {{
                        writer.write_fmt(format_args!("{{}}=\n", f))?;
                    }}
                    writer.flush()?;
                    Ok(())
                }}
            }}"#,
				ident, fields
			)
			.parse()
			.unwrap();
		}
	}
	return "".parse().unwrap();
}

#[proc_macro_derive(ConfigrDefault)]
pub fn configr(input: TokenStream) -> TokenStream {
	let DeriveInput { ident, .. } = parse_macro_input!(input);
	format!(
		r#"impl Config<Self> for {} {{
		fn populate_template(fd: std::fs::File) -> std::io::Result<()> {{
			use std::io::Write;
			let mut writer = std::io::BufWriter::new(fd);
			writer.write(toml::to_string::<Self>(&Default::default()).unwrap().as_bytes())?;
			writer.flush()?;
			Ok(())
		}}
	}}"#,
		ident
	)
	.parse()
	.unwrap()
}