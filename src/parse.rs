use std::path::Path;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_ast::{ExportDecl, ModuleExportName};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::*;

#[derive(Default)]
struct ModuleExportTransform {
    http_methods: Vec<String>,
}

#[derive(Default)]
struct VarExportTransform {
    http_method: String,
}

impl Visit for VarExportTransform {
    fn visit_var_declarator(&mut self, n: &swc_ecma_ast::VarDeclarator) {
        // ! TODO: Test if this is unsafe.
        self.http_method = n.name.as_ident().unwrap().sym.to_string();
    }
}

impl Visit for ModuleExportTransform {
    fn visit_export_decl(&mut self, n: &ExportDecl) {
        match &n.decl {
            swc_ecma_ast::Decl::Fn(fnc) => self
                .http_methods
                .push(String::from(fnc.ident.sym.to_string())),
            swc_ecma_ast::Decl::Var(var) => {
                let mut varexport: VarExportTransform = Default::default();
                var.decls.visit_with(&mut varexport);
                self.http_methods.push(varexport.http_method);
            }
            _ => (),
        }
    }
    fn visit_module_export_name(&mut self, n: &ModuleExportName) {
        if let ModuleExportName::Ident(ident) = n {
            self.http_methods.push(ident.sym.to_string());
        }
    }
}

pub fn parse(path: &String) {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm.load_file(Path::new(path)).expect("failed to load path");

    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let _module = parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parser module");

    let mut module_export_transform: ModuleExportTransform = Default::default();
    _module.visit_with(&mut module_export_transform);

    println!("{path}: {:?}", module_export_transform.http_methods)
}
