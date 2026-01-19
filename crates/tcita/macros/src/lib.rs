use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(darling::FromMeta)]
#[darling(derive_syn_parse)]
struct Params {
	lojban: String,
	glico: String,
}

impl Params {
	pub fn lojban_docs(&self) -> String {
		format!("```lojban\n{}\n```", self.lojban)
	}

	pub fn glico_docs(&self) -> String {
		format!("```glico\n{}\n```", self.glico)
	}
}

#[proc_macro_attribute]
pub fn veciksi(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item: TokenStream = item.into();
	let params: Params = match syn::parse(attr) {
		Ok(attrs) => attrs,
		Err(e) => return e.into_compile_error().into(),
	};
	if params.lojban.trim().is_empty() || params.glico.trim().is_empty() {
		panic!("You can't have empty lojban or glico entries!")
	}

	let lojban = params.lojban_docs();
	let glico = params.glico_docs();

	quote! {
		#[doc = #lojban]
		#[doc = #glico]
		#item
	}
	.into()
}

#[proc_macro_attribute]
pub fn tcita(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = syn::parse_macro_input!(item as DeriveInput);
	let ident = &item.ident;

	let params: syn::LitStr = syn::parse(attr).unwrap();

	quote! {
		impl ::ah_tcita::Ka_tcita for #ident {
			const TI_SELTCITA_BAU_LA_LOJBAN: &str = #params;
		}
		#item
	}
	.into()
}
