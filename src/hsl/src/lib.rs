mod convert;

use proc_macro::TokenStream;
use quote::quote;

type Float = f64;

struct HSLInput {
	h: Float,
	s: Float,
	l: Float,
}

impl syn::parse::Parse for HSLInput {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		fn parse_n(
			input: syn::parse::ParseStream,
			name: &str,
			bound: u16,
			comma: bool,
		) -> syn::Result<Float> {
			let Ok(lit) = input.parse::<syn::LitInt>() else {
				return Err(syn::Error::new_spanned(
					input.to_string(),
					"expected integer literal",
				));
			};

			fn parse_bound(lit: &syn::LitInt, name: &str, bound: u16) -> syn::Result<u16> {
				if let Ok(val) = lit.base10_parse::<u16>() {
					if val <= bound {
						return Ok(val);
					}
				}
				return Err(syn::Error::new_spanned(
					lit,
					format!("expected {name} as uint literal with bound (0..={bound})"),
				));
			}
			let val = parse_bound(&lit, name, bound)?;

			if comma && input.parse::<syn::Token![,]>().is_err() {
				return Err(syn::Error::new_spanned(
					input.to_string(),
					"expected 3 comma-separated integers",
				));
			}

			Ok(val as Float / bound as Float)
		}

		Ok(HSLInput {
			h: parse_n(input, "hue", 360, true)?,
			s: parse_n(input, "saturation", 100, true)?,
			l: parse_n(input, "lightness", 100, false)?,
		})
	}
}

#[proc_macro]
pub fn hsl(input: TokenStream) -> TokenStream {
	let HSLInput { h, s, l } = syn::parse_macro_input!(input as HSLInput);

	let (r, g, b) = convert::hsl_to_rgb(h, s, l);

	quote!({ crate::__formatcp!("#{:#02x}{:#02x}{:#02x}", #r, #g, #b) }).into()
}
