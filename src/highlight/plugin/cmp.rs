use nvim_oxi::api::{self, opts::SetHighlightOpts};

const PRE: &str = "CmpItem";

pub fn load() -> Result<(), api::Error> {
	for (kind, link) in [
		("Text", "Normal"),
		("Function", "Function"),
		("Variable", "@variable"),
		("Value", "Number"),
		("Constant", "Constant"),
		("Struct", "Structure"),
		("Class", "@lsp.type.class"),
		("Interface", "@lsp.type.interface"),
		("Enum", "@lsp.type.enum"),
		("Field", "@property"),
		("Method", "@lsp.type.method"),
		("Property", "@property"),
		("EnumMember", "@lsp.type.enumMember"),
		("Constructor", "@constructor"),
		("Module", "@module"),
		("Unit", "RustHexNumber"),
		("Keyword", "Keyword"),
		("File", "@lsp.type.path"),
		("Snippet", "DiagnosticError"),
		("Color", "Added"),
		("Reference", "Changed"),
		("Folder", "Directory"),
		("Event", "@lsp.type.event"),
		("Operator", "Operator"),
		("TypeParameter", "Type"),
	] {
		api::set_hl(
			0,
			&format!("{PRE}Kind{kind}"),
			&SetHighlightOpts::builder().link(link).build(),
		)?;
	}

	Ok(())
}
