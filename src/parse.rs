use std::path::Path;
use swc_common::comments::{Comments, SingleThreadedComments};
use swc_common::sync::Lrc;
use swc_common::Span;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_ast::{ExportDecl, ExportNamedSpecifier, ModuleExportName};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::*;

#[derive(Debug)]
pub struct HttpMethod {
    pub method_type: String,
    pub pos: Option<Span>,
}

#[derive(Default)]
struct ModuleExportTransform {
    http_methods: Vec<HttpMethod>,
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
            swc_ecma_ast::Decl::Fn(fnc) => self.http_methods.push(HttpMethod {
                method_type: String::from(fnc.ident.sym.to_string()),
                pos: Some(n.span),
            }),
            swc_ecma_ast::Decl::Var(var) => {
                let mut varexport: VarExportTransform = Default::default();
                var.decls.visit_with(&mut varexport);
                self.http_methods.push(HttpMethod {
                    method_type: varexport.http_method,
                    pos: Some(n.span),
                });
            }
            _ => (),
        }
    }

    // ? Comments are unimplemented for named exports
    fn visit_export_named_specifier(&mut self, n: &ExportNamedSpecifier) {
        // consider: export { api_route as GET }
        // Then "api_route" -> n.orig and "GET" -> n.exported
        // However n.exported may not exist in case of: export { GET }
        //  In this case we use n.orig
        if let Some(ModuleExportName::Ident(exported)) = &n.exported {
            self.http_methods.push(HttpMethod {
                method_type: exported.sym.to_string(),
                pos: None,
            });
        } else if let ModuleExportName::Ident(ident) = &n.orig {
            self.http_methods.push(HttpMethod {
                method_type: ident.sym.to_string(),
                pos: None,
            });
        }
    }
}

#[derive(Debug)]
pub struct ParseResult {
    pub path: String,
    pub method_metadata: Vec<MethodMetadata>,
}

pub fn parse(path: &String) -> ParseResult {
    // TODO: Cleanup this code
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm.load_file(Path::new(path)).expect("failed to load path");

    let comments: SingleThreadedComments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
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

    let mut method_metadata: Vec<MethodMetadata> = vec![];
    for method in module_export_transform.http_methods {
        let parsed_comments = if let Some(pos) = method.pos {
            if let Some(vec_comments) = comments.get_leading(pos.lo) {
                let mapped_comments: Vec<String> = vec_comments
                    .into_iter()
                    .map(|c| c.text.trim().to_string())
                    .collect();
                Some(mapped_comments)
            } else {
                None
            }
        } else {
            None
        };

        method_metadata.push(MethodMetadata {
            method_type: method.method_type,
            comment: parsed_comments,
        });
    }

    ParseResult {
        path: path.to_string(),
        method_metadata,
    }
}

#[derive(Debug)]
pub struct MethodMetadata {
    method_type: String,
    comment: Option<Vec<String>>,
}
