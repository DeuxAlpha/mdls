use anyhow::Result;
use log::info;
use specs::prelude::*;
use std::error::Error;
// use lsp_types::notification::DidChangeTextDocument;
use lsp_types::request::*;
use lsp_types::*;

// use lsp_types::{
//     CodeActionOptions, CodeActionProviderCapability, CodeLensOptions, CompletionItem,
//     CompletionOptions, CompletionResponse, DocumentOnTypeFormattingOptions,
//     FoldingRangeProviderCapability, ImplementationProviderCapability, InitializeParams,
//     RenameOptions, RenameProviderCapability, SaveOptions, SelectionRangeProviderCapability,
//     ServerCapabilities, SignatureHelpOptions, TextDocumentSyncCapability, TextDocumentSyncKind,
//     TextDocumentSyncOptions, TypeDefinitionProviderCapability, WorkDoneProgressOptions,
// };

use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::RenameProviderCapability::Options;
use mdls_server::*;
// #[derive(Debug)]
// enum MdlsServerError {
//     Response(u32, u32),
//     Ignore(u32, u32),
//     Hover(u32, u32),
//     Complete(u32, u32),
//     Specs(u32, u32),
//     GoToDef(u32, u32),
// }
// impl MdlsServerError {
//     fn write_error(&self) {
//         match self {
//             MdlsServerError::Response(l, c) => {
//                 info!("Response failed here - line: {} col: {}", l, c)
//             }
//             MdlsServerError::Ignore(l, c) => {
//                 if VERBOSE {
//                     info!("Ignored failed line: {} col: {}", l, c)
//                 }
//             }
//             MdlsServerError::Hover(l, c) => info!("Hover failed here - line: {} col: {}", l, c),
//             MdlsServerError::Complete(l, c) => {
//                 info!("Complete failed here - line: {} col: {}", l, c)
//             }
//             MdlsServerError::Specs(l, c) => info!("Specs failed here - line: {} col: {}", l, c),
//             MdlsServerError::GoToDef(l, c) => info!("GoToDef failed here - line: {} col: {}", l, c),
//         }
//     }
// }

// const VERBOSE: bool = true;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Set up logging. Because `stdio_transport` gets a lock on stdout and stdin, we must have
    // our logging only write out to stderr.
    flexi_logger::Logger::with_str("info").start().unwrap();
    info!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_settings = ServerCapabilities {
        // text_document_sync: Some(TextDocumentSyncCapability::Options(
        //     TextDocumentSyncOptions {
        //         open_close: Some(true),
        //         change: Some(TextDocumentSyncKind::Full),
        //         will_save: None,
        //         will_save_wait_until: None,
        //         save: Some(SaveOptions::default()),
        //     },
        // )),
        hover_provider: Some(true),
        completion_provider: Some(CompletionOptions {
            resolve_provider: None,
            trigger_characters: Some(vec![":".to_string(), ".".to_string()]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
            retrigger_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        declaration_provider: None,
        definition_provider: Some(true),
        type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
        implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
        references_provider: Some(true),
        document_highlight_provider: Some(true),
        document_symbol_provider: Some(true),
        workspace_symbol_provider: Some(true),
        code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
            // Advertise support for all built-in CodeActionKinds
            code_action_kinds: Some(vec![
                lsp_types::code_action_kind::EMPTY.to_string(),
                lsp_types::code_action_kind::QUICKFIX.to_string(),
                lsp_types::code_action_kind::REFACTOR.to_string(),
                lsp_types::code_action_kind::REFACTOR_EXTRACT.to_string(),
                lsp_types::code_action_kind::REFACTOR_INLINE.to_string(),
                lsp_types::code_action_kind::REFACTOR_REWRITE.to_string(),
                lsp_types::code_action_kind::SOURCE.to_string(),
                lsp_types::code_action_kind::SOURCE_ORGANIZE_IMPORTS.to_string(),
            ]),
            work_done_progress_options: Default::default(),
        })),
        code_lens_provider: Some(CodeLensOptions {
            resolve_provider: Some(true),
        }),
        document_formatting_provider: Some(true),
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
            first_trigger_character: "=".to_string(),
            more_trigger_character: Some(vec![".".to_string(), ">".to_string()]),
        }),
        selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
        // semantic_highlighting: None,
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        rename_provider: Some(RenameProviderCapability::Options(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        document_link_provider: None,
        color_provider: None,
        execute_command_provider: None,
        workspace: None,
        // call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
        // semantic_tokens_provider: Some(
        //     SemanticTokensOptions {
        //         legend: SemanticTokensLegend {
        //             token_types: semantic_tokens::SUPPORTED_TYPES.to_vec(),
        //             token_modifiers: semantic_tokens::SUPPORTED_MODIFIERS.to_vec(),
        //         },

        //         document_provider: Some(SemanticTokensDocumentProvider::Bool(true)),
        //         range_provider: Some(true),
        //         work_done_progress_options: Default::default(),
        //     }
        //     .into(),
        // ),
        experimental: Default::default(),
        ..ServerCapabilities::default()
    };
    let server_capabilities = serde_json::to_value(&server_settings).unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(&connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    info!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: &Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut world = build_world(serde_json::from_value(params).unwrap());

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                let mut request = ReqMessage { req };
                if let Some(resp) = handle_hover(&mut world, &mut request) {
                    handle_request(connection, resp);
                }
                if let Some(resp) = handle_completion(&mut request) {
                    handle_request(connection, resp);
                }
                if let Some(resp) = handle_goto(&mut world, &mut request) {
                    handle_request(connection, resp);
                }
            }
            Message::Response(_resp) => {}
            Message::Notification(_not) => {}
        }
    }
    Ok(())
}

fn build_world(params: InitializeParams) -> World {
    info!("starting example main loop");
    let method_ano = type_annotations::parse_method_ron();
    let event_ano = type_annotations::parse_event_ron();
    let script_properties = scriptproperties::ScriptProperties::new(include_str!(
        "../../reference/scriptproperties.xml"
    ));
    // let mut world = generate_world(_params.root_uri.clone());
    let mut world = data_store::new_generate_world(params.root_uri.clone());
    world.insert(method_ano);
    world.insert(event_ano);
    world.insert(script_properties);
    world.maintain();
    let mut dispatcher = DispatcherBuilder::new()
        // .with(systems::PrintMe, "printme", &[])
        // .with(systems::PrintNames, "printme2", &[])
        // .with(systems::EventAdder, "addevents", &[])
        // .with(systems::MethodAdder, "addmethods", &[])
        // .with(systems::MdEventsPrint, "MdEventsPrint", &["addevents"])
        // .with(systems::MdMethodsPrint, "MdMethodsPrint", &["addmethods"])
        .with(systems::GraphTypingMethods, "GraphTypingMethods", &[])
        .with(systems::GraphTypingEvents, "GraphTypingEvents", &[])
        .with(systems::AddVarsToNodes, "AddVarsToNodes", &[])
        .with(systems::AddVarsToCues, "AddVarsToCues", &[])
        .with(systems::AddNodesToCues, "AddNodesToCues", &[])
        .with(systems::AddCuesToScript, "AddCuesToScript", &[])
        .with(
            systems::ParseExpressions,
            "ParseExpressions",
            &[
                "AddCuesToScript",
                "AddNodesToCues",
                "AddVarsToNodes",
                "AddVarsToCues",
            ],
        )
        .with(
            systems::PrintGraph,
            "PrintGraph",
            &[
                "AddCuesToScript",
                "AddNodesToCues",
                "AddVarsToNodes",
                "AddVarsToCues",
            ],
        )
        .build();

    dispatcher.dispatch(&mut world);
    world
}

fn handle_hover(world: &mut World, request: &mut ReqMessage) -> Option<Response> {
    if let Ok((id, param)) = request.cast::<HoverRequest>() {
        return Some(hover::new_hover_resp(id, param, world));
    }
    None
}

fn handle_completion(request: &mut ReqMessage) -> Option<Response> {
    if let Ok((id, params)) = request.cast::<Completion>() {
        if let Ok(json) = serde_json::to_value(&CompletionResponse::Array(
            completion_parser::simple_complete(params),
        )) {
            return Some(Response {
                id,
                result: Some(json),
                error: None,
            });
        }
    }
    None
}

fn handle_goto(world: &mut World, request: &mut ReqMessage) -> Option<Response> {
    if let Ok((id, params)) = request.cast::<GotoDefinition>() {
        if let Ok(json) = serde_json::to_value(lsp_types::GotoDefinitionResponse::Array(
            definition_parser::simple_definition(params, world),
        )) {
            return Some(Response {
                id,
                result: Some(json),
                error: None,
            });
        }
    }
    None
}

fn handle_request(connection: &Connection, response: Response) {
    if let Err(err) = connection.sender.send(Message::Response(response)) {
        // MdlsServerError::Response(line!(), column!());
    }
}

#[derive(Clone)]
struct ReqMessage {
    req: Request,
}

impl ReqMessage {
    fn cast<R>(&mut self) -> Result<(RequestId, R::Params), Request>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
    {
        self.clone().req.extract(R::METHOD)
    }
}
