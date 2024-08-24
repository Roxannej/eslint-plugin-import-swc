use swc_ecma_ast::*;
use swc_ecma_visit::{ VisitMut, VisitMutWith};
use swc_ecma_ast::Ident;

pub struct RemoveUnusedImports;

impl VisitMut for RemoveUnusedImports {
    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        let mut used_imports = Vec::new();

        let mut visitor =CollectUsedIdentifiers {
            used_imports: &mut used_imports,
        };

        items.visit_mut_with(&mut visitor);

        items.retain(|item| {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
                if import.type_only {
                    return true;
                }

                import.specifiers.iter().any(|specifier| {
                    match specifier {
                        ImportSpecifier::Named(named) => {
                            used_imports.contains(&named.local.sym.to_string())
                        }
                        ImportSpecifier::Default(default) => {
                            used_imports.contains(&default.local.sym.to_string())
                        }
                        ImportSpecifier::Namespace(namespace) => {
                            used_imports.contains(&namespace.local.sym.to_string())
                        }
                    }
                })
            } else {
                true
            }
        })
    }
}

struct CollectUsedIdentifiers<'a> {
    used_imports: &'a mut Vec<String>,
}

impl<'a> VisitMut for CollectUsedIdentifiers<'a> {
    fn visit_mut_ident(&mut self, ident: &mut Ident) {
        self.used_imports.push(ident.sym.to_string());
    }
}
