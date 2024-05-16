pub enum RenameType {
    None,
    CamelCase,
}

pub fn find_type_in_attr<I>(attrs: Vec<syn::Attribute>, ident: &I) -> RenameType
where
    I: ?Sized + AsRef<str>,
{
    if attrs.is_empty() {
        return RenameType::None;
    }

    let mut rename_all = RenameType::None;

    for atrr in attrs {
        if atrr.path().is_ident("serde") {
            // from atrr try get rename_all
            let expr: syn::Expr = atrr.parse_args().unwrap();

            let assign = match expr {
                syn::Expr::Assign(assign) => Some(assign),
                _ => None,
            };

            if assign.is_none() {
                continue;
            }

            let assign = assign.unwrap();

            let left = assign.left.clone();
            let left = *left;

            let is_rename_all = match left {
                syn::Expr::Path(left) => left.path.is_ident(ident),
                _ => false,
            };

            let right = assign.right.clone();
            let right = *right;

            match (is_rename_all, right) {
                (true, syn::Expr::Lit(expr_lit)) => match expr_lit.lit {
                    syn::Lit::Str(lit_str) => {
                        let value = lit_str.value();

                        match value.as_str() {
                            "camelCase" => {
                                rename_all = RenameType::CamelCase;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
                _ => {}
            };
        }
    }

    rename_all
}
