#![allow(nonstandard_style)]
// Generated from ../grammar/GadgetLanguage.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::gadgetlanguageparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link GadgetLanguageParser}.
 */
pub trait GadgetLanguageVisitor<'input>: ParseTreeVisitor<'input,GadgetLanguageParserContextType>{
	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#colliderDef}.
	 * @param ctx the parse tree
	 */
	fn visit_colliderDef(&mut self, ctx: &ColliderDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#colliderBody}.
	 * @param ctx the parse tree
	 */
	fn visit_colliderBody(&mut self, ctx: &ColliderBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#shapeDef}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeDef(&mut self, ctx: &ShapeDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#shapeType}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeType(&mut self, ctx: &ShapeTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#shapeParams}.
	 * @param ctx the parse tree
	 */
	fn visit_shapeParams(&mut self, ctx: &ShapeParamsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#geometryRef}.
	 * @param ctx the parse tree
	 */
	fn visit_geometryRef(&mut self, ctx: &GeometryRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#listenerDef}.
	 * @param ctx the parse tree
	 */
	fn visit_listenerDef(&mut self, ctx: &ListenerDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#listenerBody}.
	 * @param ctx the parse tree
	 */
	fn visit_listenerBody(&mut self, ctx: &ListenerBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#eventType}.
	 * @param ctx the parse tree
	 */
	fn visit_eventType(&mut self, ctx: &EventTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#eventName}.
	 * @param ctx the parse tree
	 */
	fn visit_eventName(&mut self, ctx: &EventNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#targetId}.
	 * @param ctx the parse tree
	 */
	fn visit_targetId(&mut self, ctx: &TargetIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#actionRef}.
	 * @param ctx the parse tree
	 */
	fn visit_actionRef(&mut self, ctx: &ActionRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#actionDef}.
	 * @param ctx the parse tree
	 */
	fn visit_actionDef(&mut self, ctx: &ActionDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#actionBody}.
	 * @param ctx the parse tree
	 */
	fn visit_actionBody(&mut self, ctx: &ActionBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#actionType}.
	 * @param ctx the parse tree
	 */
	fn visit_actionType(&mut self, ctx: &ActionTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#actionParams}.
	 * @param ctx the parse tree
	 */
	fn visit_actionParams(&mut self, ctx: &ActionParamsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#paramPair}.
	 * @param ctx the parse tree
	 */
	fn visit_paramPair(&mut self, ctx: &ParamPairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link GadgetLanguageParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_value(&mut self, ctx: &ValueContext<'input>) { self.visit_children(ctx) }


}