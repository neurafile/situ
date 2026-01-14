#![allow(nonstandard_style)]
// Generated from ../grammar/GadgetLanguage.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::gadgetlanguageparser::*;

pub trait GadgetLanguageListener<'input> : ParseTreeListener<'input,GadgetLanguageParserContextType>{

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#colliderDef}.
 * @param ctx the parse tree
 */
fn enter_colliderDef(&mut self, _ctx: &ColliderDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#colliderDef}.
 * @param ctx the parse tree
 */
fn exit_colliderDef(&mut self, _ctx: &ColliderDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#colliderBody}.
 * @param ctx the parse tree
 */
fn enter_colliderBody(&mut self, _ctx: &ColliderBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#colliderBody}.
 * @param ctx the parse tree
 */
fn exit_colliderBody(&mut self, _ctx: &ColliderBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#shapeDef}.
 * @param ctx the parse tree
 */
fn enter_shapeDef(&mut self, _ctx: &ShapeDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#shapeDef}.
 * @param ctx the parse tree
 */
fn exit_shapeDef(&mut self, _ctx: &ShapeDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#shapeType}.
 * @param ctx the parse tree
 */
fn enter_shapeType(&mut self, _ctx: &ShapeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#shapeType}.
 * @param ctx the parse tree
 */
fn exit_shapeType(&mut self, _ctx: &ShapeTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#shapeParams}.
 * @param ctx the parse tree
 */
fn enter_shapeParams(&mut self, _ctx: &ShapeParamsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#shapeParams}.
 * @param ctx the parse tree
 */
fn exit_shapeParams(&mut self, _ctx: &ShapeParamsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#geometryRef}.
 * @param ctx the parse tree
 */
fn enter_geometryRef(&mut self, _ctx: &GeometryRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#geometryRef}.
 * @param ctx the parse tree
 */
fn exit_geometryRef(&mut self, _ctx: &GeometryRefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#listenerDef}.
 * @param ctx the parse tree
 */
fn enter_listenerDef(&mut self, _ctx: &ListenerDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#listenerDef}.
 * @param ctx the parse tree
 */
fn exit_listenerDef(&mut self, _ctx: &ListenerDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#listenerBody}.
 * @param ctx the parse tree
 */
fn enter_listenerBody(&mut self, _ctx: &ListenerBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#listenerBody}.
 * @param ctx the parse tree
 */
fn exit_listenerBody(&mut self, _ctx: &ListenerBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#eventType}.
 * @param ctx the parse tree
 */
fn enter_eventType(&mut self, _ctx: &EventTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#eventType}.
 * @param ctx the parse tree
 */
fn exit_eventType(&mut self, _ctx: &EventTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#eventName}.
 * @param ctx the parse tree
 */
fn enter_eventName(&mut self, _ctx: &EventNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#eventName}.
 * @param ctx the parse tree
 */
fn exit_eventName(&mut self, _ctx: &EventNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#targetId}.
 * @param ctx the parse tree
 */
fn enter_targetId(&mut self, _ctx: &TargetIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#targetId}.
 * @param ctx the parse tree
 */
fn exit_targetId(&mut self, _ctx: &TargetIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#actionRef}.
 * @param ctx the parse tree
 */
fn enter_actionRef(&mut self, _ctx: &ActionRefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#actionRef}.
 * @param ctx the parse tree
 */
fn exit_actionRef(&mut self, _ctx: &ActionRefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#actionDef}.
 * @param ctx the parse tree
 */
fn enter_actionDef(&mut self, _ctx: &ActionDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#actionDef}.
 * @param ctx the parse tree
 */
fn exit_actionDef(&mut self, _ctx: &ActionDefContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#actionBody}.
 * @param ctx the parse tree
 */
fn enter_actionBody(&mut self, _ctx: &ActionBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#actionBody}.
 * @param ctx the parse tree
 */
fn exit_actionBody(&mut self, _ctx: &ActionBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#actionType}.
 * @param ctx the parse tree
 */
fn enter_actionType(&mut self, _ctx: &ActionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#actionType}.
 * @param ctx the parse tree
 */
fn exit_actionType(&mut self, _ctx: &ActionTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#actionParams}.
 * @param ctx the parse tree
 */
fn enter_actionParams(&mut self, _ctx: &ActionParamsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#actionParams}.
 * @param ctx the parse tree
 */
fn exit_actionParams(&mut self, _ctx: &ActionParamsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#paramPair}.
 * @param ctx the parse tree
 */
fn enter_paramPair(&mut self, _ctx: &ParamPairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#paramPair}.
 * @param ctx the parse tree
 */
fn exit_paramPair(&mut self, _ctx: &ParamPairContext<'input>) { }

/**
 * Enter a parse tree produced by {@link GadgetLanguageParser#value}.
 * @param ctx the parse tree
 */
fn enter_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link GadgetLanguageParser#value}.
 * @param ctx the parse tree
 */
fn exit_value(&mut self, _ctx: &ValueContext<'input>) { }

}
