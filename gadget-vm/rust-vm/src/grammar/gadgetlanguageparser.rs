// Generated from ../grammar/GadgetLanguage.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::gadgetlanguagelistener::*;
use super::gadgetlanguagevisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const ID:isize=27; 
		pub const STRING:isize=28; 
		pub const NUMBER:isize=29; 
		pub const BOOLEAN:isize=30; 
		pub const WS:isize=31; 
		pub const COMMENT:isize=32; 
		pub const BLOCK_COMMENT:isize=33;
	pub const RULE_program:usize = 0; 
	pub const RULE_statement:usize = 1; 
	pub const RULE_colliderDef:usize = 2; 
	pub const RULE_colliderBody:usize = 3; 
	pub const RULE_shapeDef:usize = 4; 
	pub const RULE_shapeType:usize = 5; 
	pub const RULE_shapeParams:usize = 6; 
	pub const RULE_geometryRef:usize = 7; 
	pub const RULE_listenerDef:usize = 8; 
	pub const RULE_listenerBody:usize = 9; 
	pub const RULE_eventType:usize = 10; 
	pub const RULE_eventName:usize = 11; 
	pub const RULE_targetId:usize = 12; 
	pub const RULE_actionRef:usize = 13; 
	pub const RULE_actionDef:usize = 14; 
	pub const RULE_actionBody:usize = 15; 
	pub const RULE_actionType:usize = 16; 
	pub const RULE_actionParams:usize = 17; 
	pub const RULE_paramPair:usize = 18; 
	pub const RULE_value:usize = 19;
	pub const ruleNames: [&'static str; 20] =  [
		"program", "statement", "colliderDef", "colliderBody", "shapeDef", "shapeType", 
		"shapeParams", "geometryRef", "listenerDef", "listenerBody", "eventType", 
		"eventName", "targetId", "actionRef", "actionDef", "actionBody", "actionType", 
		"actionParams", "paramPair", "value"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;27] = [
		None, Some("'collider'"), Some("'{'"), Some("'}'"), Some("'shape'"), Some("'('"), 
		Some("')'"), Some("'box'"), Some("'sphere'"), Some("'cylinder'"), Some("'capsule'"), 
		Some("','"), Some("'geometry'"), Some("'listener'"), Some("'on'"), Some("'collision'"), 
		Some("'trigger'"), Some("'custom'"), Some("'target'"), Some("'action'"), 
		Some("'runAnim'"), Some("'stopAnim'"), Some("'playSound'"), Some("'stopSound'"), 
		Some("'httpCall'"), Some("'emitEvent'"), Some("'='")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;34]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, Some("ID"), Some("STRING"), Some("NUMBER"), Some("BOOLEAN"), 
		Some("WS"), Some("COMMENT"), Some("BLOCK_COMMENT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,GadgetLanguageParserExt, I, GadgetLanguageParserContextType , dyn GadgetLanguageListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type GadgetLanguageTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, GadgetLanguageParserContextType , dyn GadgetLanguageListener<'input> + 'a>;

/// Parser for GadgetLanguage grammar
pub struct GadgetLanguageParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				GadgetLanguageParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> GadgetLanguageParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> GadgetLanguageParser<'input, I, DefaultErrorStrategy<'input,GadgetLanguageParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for GadgetLanguageParser
pub trait GadgetLanguageParserContext<'input>:
	for<'x> Listenable<dyn GadgetLanguageListener<'input> + 'x > + 
	for<'x> Visitable<dyn GadgetLanguageVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=GadgetLanguageParserContextType>
{}

impl<'input, 'x, T> VisitableDyn<T> for dyn GadgetLanguageParserContext<'input> + 'input
where
    T: GadgetLanguageVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn GadgetLanguageVisitor<'input> + 'x))
    }
}

impl<'input> GadgetLanguageParserContext<'input> for TerminalNode<'input,GadgetLanguageParserContextType> {}
impl<'input> GadgetLanguageParserContext<'input> for ErrorNode<'input,GadgetLanguageParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn GadgetLanguageParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn GadgetLanguageListener<'input> + 'input{}

pub struct GadgetLanguageParserContextType;
antlr_rust::type_id!{GadgetLanguageParserContextType}

impl<'input> ParserNodeType<'input> for GadgetLanguageParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn GadgetLanguageParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct GadgetLanguageParserExt{
}

impl GadgetLanguageParserExt{
}


impl<'input> TokenAware<'input> for GadgetLanguageParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for GadgetLanguageParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for GadgetLanguageParserExt{
	fn get_grammar_file_name(&self) -> & str{ "GadgetLanguage.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ProgramContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_program(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_program(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::type_id!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(43);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__12) | (1usize << T__18))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(40);
				recog.statement()?;

				}
				}
				recog.base.set_state(45);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(46);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for StatementContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_statement(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::type_id!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn colliderDef(&self) -> Option<Rc<ColliderDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listenerDef(&self) -> Option<Rc<ListenerDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionDef(&self) -> Option<Rc<ActionDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(51);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule colliderDef*/
					recog.base.set_state(48);
					recog.colliderDef()?;

					}
				}

			 T__12 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule listenerDef*/
					recog.base.set_state(49);
					recog.listenerDef()?;

					}
				}

			 T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule actionDef*/
					recog.base.set_state(50);
					recog.actionDef()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- colliderDef ----------------
pub type ColliderDefContextAll<'input> = ColliderDefContext<'input>;


pub type ColliderDefContext<'input> = BaseParserRuleContext<'input,ColliderDefContextExt<'input>>;

#[derive(Clone)]
pub struct ColliderDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ColliderDefContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ColliderDefContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_colliderDef(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_colliderDef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ColliderDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_colliderDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ColliderDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_colliderDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_colliderDef }
}
antlr_rust::type_id!{ColliderDefContextExt<'a>}

impl<'input> ColliderDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ColliderDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ColliderDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ColliderDefContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ColliderDefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}
fn colliderBody(&self) -> Option<Rc<ColliderBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ColliderDefContextAttrs<'input> for ColliderDefContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn colliderDef(&mut self,)
	-> Result<Rc<ColliderDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ColliderDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_colliderDef);
        let mut _localctx: Rc<ColliderDefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(53);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			recog.base.set_state(54);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			recog.base.set_state(55);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule colliderBody*/
			recog.base.set_state(56);
			recog.colliderBody()?;

			recog.base.set_state(57);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- colliderBody ----------------
pub type ColliderBodyContextAll<'input> = ColliderBodyContext<'input>;


pub type ColliderBodyContext<'input> = BaseParserRuleContext<'input,ColliderBodyContextExt<'input>>;

#[derive(Clone)]
pub struct ColliderBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ColliderBodyContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ColliderBodyContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_colliderBody(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_colliderBody(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ColliderBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_colliderBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for ColliderBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_colliderBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_colliderBody }
}
antlr_rust::type_id!{ColliderBodyContextExt<'a>}

impl<'input> ColliderBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ColliderBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ColliderBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ColliderBodyContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ColliderBodyContextExt<'input>>{

fn shapeDef(&self) -> Option<Rc<ShapeDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn geometryRef(&self) -> Option<Rc<GeometryRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ColliderBodyContextAttrs<'input> for ColliderBodyContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn colliderBody(&mut self,)
	-> Result<Rc<ColliderBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ColliderBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_colliderBody);
        let mut _localctx: Rc<ColliderBodyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(61);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule shapeDef*/
					recog.base.set_state(59);
					recog.shapeDef()?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule geometryRef*/
					recog.base.set_state(60);
					recog.geometryRef()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeDef ----------------
pub type ShapeDefContextAll<'input> = ShapeDefContext<'input>;


pub type ShapeDefContext<'input> = BaseParserRuleContext<'input,ShapeDefContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ShapeDefContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ShapeDefContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shapeDef(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_shapeDef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ShapeDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_shapeDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeDef }
}
antlr_rust::type_id!{ShapeDefContextExt<'a>}

impl<'input> ShapeDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeDefContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ShapeDefContextExt<'input>>{

fn shapeType(&self) -> Option<Rc<ShapeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn shapeParams(&self) -> Option<Rc<ShapeParamsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ShapeDefContextAttrs<'input> for ShapeDefContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeDef(&mut self,)
	-> Result<Rc<ShapeDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_shapeDef);
        let mut _localctx: Rc<ShapeDefContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(63);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule shapeType*/
			recog.base.set_state(64);
			recog.shapeType()?;

			recog.base.set_state(69);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__4 {
				{
				recog.base.set_state(65);
				recog.base.match_token(T__4,&mut recog.err_handler)?;

				/*InvokeRule shapeParams*/
				recog.base.set_state(66);
				recog.shapeParams()?;

				recog.base.set_state(67);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeType ----------------
pub type ShapeTypeContextAll<'input> = ShapeTypeContext<'input>;


pub type ShapeTypeContext<'input> = BaseParserRuleContext<'input,ShapeTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ShapeTypeContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ShapeTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shapeType(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_shapeType(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ShapeTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_shapeType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeType }
}
antlr_rust::type_id!{ShapeTypeContextExt<'a>}

impl<'input> ShapeTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeTypeContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ShapeTypeContextExt<'input>>{


}

impl<'input> ShapeTypeContextAttrs<'input> for ShapeTypeContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeType(&mut self,)
	-> Result<Rc<ShapeTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_shapeType);
        let mut _localctx: Rc<ShapeTypeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(71);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- shapeParams ----------------
pub type ShapeParamsContextAll<'input> = ShapeParamsContext<'input>;


pub type ShapeParamsContext<'input> = BaseParserRuleContext<'input,ShapeParamsContextExt<'input>>;

#[derive(Clone)]
pub struct ShapeParamsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ShapeParamsContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ShapeParamsContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_shapeParams(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_shapeParams(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ShapeParamsContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_shapeParams(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShapeParamsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shapeParams }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shapeParams }
}
antlr_rust::type_id!{ShapeParamsContextExt<'a>}

impl<'input> ShapeParamsContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShapeParamsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShapeParamsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShapeParamsContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ShapeParamsContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token NUMBER in current rule
fn NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NUMBER, starting from 0.
/// Returns `None` if number of children corresponding to token NUMBER is less or equal than `i`.
fn NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, i)
}

}

impl<'input> ShapeParamsContextAttrs<'input> for ShapeParamsContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn shapeParams(&mut self,)
	-> Result<Rc<ShapeParamsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShapeParamsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_shapeParams);
        let mut _localctx: Rc<ShapeParamsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(73);
			recog.base.match_token(NUMBER,&mut recog.err_handler)?;

			recog.base.set_state(78);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__10 {
				{
				{
				recog.base.set_state(74);
				recog.base.match_token(T__10,&mut recog.err_handler)?;

				recog.base.set_state(75);
				recog.base.match_token(NUMBER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(80);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- geometryRef ----------------
pub type GeometryRefContextAll<'input> = GeometryRefContext<'input>;


pub type GeometryRefContext<'input> = BaseParserRuleContext<'input,GeometryRefContextExt<'input>>;

#[derive(Clone)]
pub struct GeometryRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for GeometryRefContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for GeometryRefContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_geometryRef(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_geometryRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for GeometryRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_geometryRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for GeometryRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_geometryRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_geometryRef }
}
antlr_rust::type_id!{GeometryRefContextExt<'a>}

impl<'input> GeometryRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GeometryRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GeometryRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GeometryRefContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<GeometryRefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> GeometryRefContextAttrs<'input> for GeometryRefContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn geometryRef(&mut self,)
	-> Result<Rc<GeometryRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GeometryRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_geometryRef);
        let mut _localctx: Rc<GeometryRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(81);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			recog.base.set_state(82);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listenerDef ----------------
pub type ListenerDefContextAll<'input> = ListenerDefContext<'input>;


pub type ListenerDefContext<'input> = BaseParserRuleContext<'input,ListenerDefContextExt<'input>>;

#[derive(Clone)]
pub struct ListenerDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ListenerDefContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ListenerDefContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_listenerDef(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_listenerDef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ListenerDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_listenerDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ListenerDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listenerDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listenerDef }
}
antlr_rust::type_id!{ListenerDefContextExt<'a>}

impl<'input> ListenerDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListenerDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListenerDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ListenerDefContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ListenerDefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}
fn listenerBody(&self) -> Option<Rc<ListenerBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListenerDefContextAttrs<'input> for ListenerDefContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listenerDef(&mut self,)
	-> Result<Rc<ListenerDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListenerDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_listenerDef);
        let mut _localctx: Rc<ListenerDefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(84);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(85);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			recog.base.set_state(86);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule listenerBody*/
			recog.base.set_state(87);
			recog.listenerBody()?;

			recog.base.set_state(88);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listenerBody ----------------
pub type ListenerBodyContextAll<'input> = ListenerBodyContext<'input>;


pub type ListenerBodyContext<'input> = BaseParserRuleContext<'input,ListenerBodyContextExt<'input>>;

#[derive(Clone)]
pub struct ListenerBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ListenerBodyContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ListenerBodyContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_listenerBody(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_listenerBody(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ListenerBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_listenerBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for ListenerBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listenerBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listenerBody }
}
antlr_rust::type_id!{ListenerBodyContextExt<'a>}

impl<'input> ListenerBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListenerBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListenerBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ListenerBodyContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ListenerBodyContextExt<'input>>{

fn eventType(&self) -> Option<Rc<EventTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn targetId(&self) -> Option<Rc<TargetIdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionRef_all(&self) ->  Vec<Rc<ActionRefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn actionRef(&self, i: usize) -> Option<Rc<ActionRefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ListenerBodyContextAttrs<'input> for ListenerBodyContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listenerBody(&mut self,)
	-> Result<Rc<ListenerBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListenerBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_listenerBody);
        let mut _localctx: Rc<ListenerBodyContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(97);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule eventType*/
					recog.base.set_state(90);
					recog.eventType()?;

					}
				}

			 T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule targetId*/
					recog.base.set_state(91);
					recog.targetId()?;

					}
				}

			 T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(93); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						/*InvokeRule actionRef*/
						recog.base.set_state(92);
						recog.actionRef()?;

						}
						}
						recog.base.set_state(95); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==T__18) {break}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventType ----------------
pub type EventTypeContextAll<'input> = EventTypeContext<'input>;


pub type EventTypeContext<'input> = BaseParserRuleContext<'input,EventTypeContextExt<'input>>;

#[derive(Clone)]
pub struct EventTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for EventTypeContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for EventTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eventType(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_eventType(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for EventTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_eventType(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventType }
}
antlr_rust::type_id!{EventTypeContextExt<'a>}

impl<'input> EventTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventTypeContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<EventTypeContextExt<'input>>{

fn eventName(&self) -> Option<Rc<EventNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EventTypeContextAttrs<'input> for EventTypeContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventType(&mut self,)
	-> Result<Rc<EventTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_eventType);
        let mut _localctx: Rc<EventTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(99);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

			/*InvokeRule eventName*/
			recog.base.set_state(100);
			recog.eventName()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventName ----------------
pub type EventNameContextAll<'input> = EventNameContext<'input>;


pub type EventNameContext<'input> = BaseParserRuleContext<'input,EventNameContextExt<'input>>;

#[derive(Clone)]
pub struct EventNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for EventNameContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for EventNameContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eventName(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_eventName(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for EventNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_eventName(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventName }
}
antlr_rust::type_id!{EventNameContextExt<'a>}

impl<'input> EventNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventNameContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<EventNameContextExt<'input>>{


}

impl<'input> EventNameContextAttrs<'input> for EventNameContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventName(&mut self,)
	-> Result<Rc<EventNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_eventName);
        let mut _localctx: Rc<EventNameContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(102);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__14) | (1usize << T__15) | (1usize << T__16))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- targetId ----------------
pub type TargetIdContextAll<'input> = TargetIdContext<'input>;


pub type TargetIdContext<'input> = BaseParserRuleContext<'input,TargetIdContextExt<'input>>;

#[derive(Clone)]
pub struct TargetIdContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for TargetIdContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for TargetIdContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_targetId(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_targetId(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for TargetIdContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_targetId(self);
	}
}

impl<'input> CustomRuleContext<'input> for TargetIdContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_targetId }
	//fn type_rule_index() -> usize where Self: Sized { RULE_targetId }
}
antlr_rust::type_id!{TargetIdContextExt<'a>}

impl<'input> TargetIdContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TargetIdContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TargetIdContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TargetIdContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<TargetIdContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> TargetIdContextAttrs<'input> for TargetIdContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn targetId(&mut self,)
	-> Result<Rc<TargetIdContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TargetIdContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_targetId);
        let mut _localctx: Rc<TargetIdContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(104);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			recog.base.set_state(105);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- actionRef ----------------
pub type ActionRefContextAll<'input> = ActionRefContext<'input>;


pub type ActionRefContext<'input> = BaseParserRuleContext<'input,ActionRefContextExt<'input>>;

#[derive(Clone)]
pub struct ActionRefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ActionRefContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ActionRefContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionRef(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_actionRef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ActionRefContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_actionRef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionRefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionRef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionRef }
}
antlr_rust::type_id!{ActionRefContextExt<'a>}

impl<'input> ActionRefContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionRefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionRefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionRefContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ActionRefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}

}

impl<'input> ActionRefContextAttrs<'input> for ActionRefContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionRef(&mut self,)
	-> Result<Rc<ActionRefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionRefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_actionRef);
        let mut _localctx: Rc<ActionRefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(107);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			recog.base.set_state(108);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- actionDef ----------------
pub type ActionDefContextAll<'input> = ActionDefContext<'input>;


pub type ActionDefContext<'input> = BaseParserRuleContext<'input,ActionDefContextExt<'input>>;

#[derive(Clone)]
pub struct ActionDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ActionDefContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ActionDefContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionDef(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_actionDef(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ActionDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_actionDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionDef }
}
antlr_rust::type_id!{ActionDefContextExt<'a>}

impl<'input> ActionDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionDefContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ActionDefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}
fn actionBody(&self) -> Option<Rc<ActionBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ActionDefContextAttrs<'input> for ActionDefContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionDef(&mut self,)
	-> Result<Rc<ActionDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_actionDef);
        let mut _localctx: Rc<ActionDefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(110);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			recog.base.set_state(111);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			recog.base.set_state(112);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule actionBody*/
			recog.base.set_state(113);
			recog.actionBody()?;

			recog.base.set_state(114);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- actionBody ----------------
pub type ActionBodyContextAll<'input> = ActionBodyContext<'input>;


pub type ActionBodyContext<'input> = BaseParserRuleContext<'input,ActionBodyContextExt<'input>>;

#[derive(Clone)]
pub struct ActionBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ActionBodyContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ActionBodyContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionBody(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_actionBody(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ActionBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_actionBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionBody }
}
antlr_rust::type_id!{ActionBodyContextExt<'a>}

impl<'input> ActionBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionBodyContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ActionBodyContextExt<'input>>{

fn actionType(&self) -> Option<Rc<ActionTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn actionParams(&self) -> Option<Rc<ActionParamsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ActionBodyContextAttrs<'input> for ActionBodyContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionBody(&mut self,)
	-> Result<Rc<ActionBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_actionBody);
        let mut _localctx: Rc<ActionBodyContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(120);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__19 | T__20 | T__21 | T__22 | T__23 | T__24 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule actionType*/
					recog.base.set_state(116);
					recog.actionType()?;

					}
				}

			 T__2 | ID 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(118);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ID {
						{
						/*InvokeRule actionParams*/
						recog.base.set_state(117);
						recog.actionParams()?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- actionType ----------------
pub type ActionTypeContextAll<'input> = ActionTypeContext<'input>;


pub type ActionTypeContext<'input> = BaseParserRuleContext<'input,ActionTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ActionTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ActionTypeContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ActionTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionType(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_actionType(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ActionTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_actionType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionType }
}
antlr_rust::type_id!{ActionTypeContextExt<'a>}

impl<'input> ActionTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionTypeContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ActionTypeContextExt<'input>>{


}

impl<'input> ActionTypeContextAttrs<'input> for ActionTypeContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionType(&mut self,)
	-> Result<Rc<ActionTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_actionType);
        let mut _localctx: Rc<ActionTypeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(122);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__19) | (1usize << T__20) | (1usize << T__21) | (1usize << T__22) | (1usize << T__23) | (1usize << T__24))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- actionParams ----------------
pub type ActionParamsContextAll<'input> = ActionParamsContext<'input>;


pub type ActionParamsContext<'input> = BaseParserRuleContext<'input,ActionParamsContextExt<'input>>;

#[derive(Clone)]
pub struct ActionParamsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ActionParamsContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ActionParamsContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionParams(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_actionParams(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ActionParamsContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_actionParams(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionParamsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionParams }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionParams }
}
antlr_rust::type_id!{ActionParamsContextExt<'a>}

impl<'input> ActionParamsContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionParamsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionParamsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionParamsContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ActionParamsContextExt<'input>>{

fn paramPair_all(&self) ->  Vec<Rc<ParamPairContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn paramPair(&self, i: usize) -> Option<Rc<ParamPairContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ActionParamsContextAttrs<'input> for ActionParamsContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionParams(&mut self,)
	-> Result<Rc<ActionParamsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionParamsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_actionParams);
        let mut _localctx: Rc<ActionParamsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule paramPair*/
			recog.base.set_state(124);
			recog.paramPair()?;

			recog.base.set_state(129);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__10 {
				{
				{
				recog.base.set_state(125);
				recog.base.match_token(T__10,&mut recog.err_handler)?;

				/*InvokeRule paramPair*/
				recog.base.set_state(126);
				recog.paramPair()?;

				}
				}
				recog.base.set_state(131);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- paramPair ----------------
pub type ParamPairContextAll<'input> = ParamPairContext<'input>;


pub type ParamPairContext<'input> = BaseParserRuleContext<'input,ParamPairContextExt<'input>>;

#[derive(Clone)]
pub struct ParamPairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ParamPairContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ParamPairContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_paramPair(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_paramPair(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ParamPairContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_paramPair(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamPairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramPair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramPair }
}
antlr_rust::type_id!{ParamPairContextExt<'a>}

impl<'input> ParamPairContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamPairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamPairContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamPairContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ParamPairContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}
fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParamPairContextAttrs<'input> for ParamPairContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramPair(&mut self,)
	-> Result<Rc<ParamPairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamPairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_paramPair);
        let mut _localctx: Rc<ParamPairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(132);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			recog.base.set_state(133);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			/*InvokeRule value*/
			recog.base.set_state(134);
			recog.value()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;


pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> GadgetLanguageParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn GadgetLanguageListener<'input> + 'a> for ValueContext<'input>{
	fn enter(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_value(self);
	}
	fn exit(&self,listener: &mut (dyn GadgetLanguageListener<'input> + 'a)) {
		listener.exit_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn GadgetLanguageVisitor<'input> + 'a> for ValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn GadgetLanguageVisitor<'input> + 'a)) {
		visitor.visit_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = GadgetLanguageParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::type_id!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn GadgetLanguageParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueContextAttrs<'input>: GadgetLanguageParserContext<'input> + BorrowMut<ValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,GadgetLanguageParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}

}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

impl<'input, I, H> GadgetLanguageParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(136);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << STRING) | (1usize << NUMBER) | (1usize << BOOLEAN))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x23\u{8d}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x03\x02\x07\x02\x2c\x0a\x02\x0c\
	\x02\x0e\x02\x2f\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x05\x03\
	\x36\x0a\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\
	\x05\x05\x05\x40\x0a\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x05\x06\x48\x0a\x06\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x07\x08\x4f\
	\x0a\x08\x0c\x08\x0e\x08\x52\x0b\x08\x03\x09\x03\x09\x03\x09\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x06\x0b\x60\
	\x0a\x0b\x0d\x0b\x0e\x0b\x61\x05\x0b\x64\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x11\x03\x11\x05\x11\x79\x0a\
	\x11\x05\x11\x7b\x0a\x11\x03\x12\x03\x12\x03\x13\x03\x13\x03\x13\x07\x13\
	\u{82}\x0a\x13\x0c\x13\x0e\x13\u{85}\x0b\x13\x03\x14\x03\x14\x03\x14\x03\
	\x14\x03\x15\x03\x15\x03\x15\x02\x02\x16\x02\x04\x06\x08\x0a\x0c\x0e\x10\
	\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x02\x06\x03\x02\x09\x0c\
	\x03\x02\x11\x13\x03\x02\x16\x1b\x03\x02\x1e\x20\x02\u{84}\x02\x2d\x03\x02\
	\x02\x02\x04\x35\x03\x02\x02\x02\x06\x37\x03\x02\x02\x02\x08\x3f\x03\x02\
	\x02\x02\x0a\x41\x03\x02\x02\x02\x0c\x49\x03\x02\x02\x02\x0e\x4b\x03\x02\
	\x02\x02\x10\x53\x03\x02\x02\x02\x12\x56\x03\x02\x02\x02\x14\x63\x03\x02\
	\x02\x02\x16\x65\x03\x02\x02\x02\x18\x68\x03\x02\x02\x02\x1a\x6a\x03\x02\
	\x02\x02\x1c\x6d\x03\x02\x02\x02\x1e\x70\x03\x02\x02\x02\x20\x7a\x03\x02\
	\x02\x02\x22\x7c\x03\x02\x02\x02\x24\x7e\x03\x02\x02\x02\x26\u{86}\x03\x02\
	\x02\x02\x28\u{8a}\x03\x02\x02\x02\x2a\x2c\x05\x04\x03\x02\x2b\x2a\x03\x02\
	\x02\x02\x2c\x2f\x03\x02\x02\x02\x2d\x2b\x03\x02\x02\x02\x2d\x2e\x03\x02\
	\x02\x02\x2e\x30\x03\x02\x02\x02\x2f\x2d\x03\x02\x02\x02\x30\x31\x07\x02\
	\x02\x03\x31\x03\x03\x02\x02\x02\x32\x36\x05\x06\x04\x02\x33\x36\x05\x12\
	\x0a\x02\x34\x36\x05\x1e\x10\x02\x35\x32\x03\x02\x02\x02\x35\x33\x03\x02\
	\x02\x02\x35\x34\x03\x02\x02\x02\x36\x05\x03\x02\x02\x02\x37\x38\x07\x03\
	\x02\x02\x38\x39\x07\x1d\x02\x02\x39\x3a\x07\x04\x02\x02\x3a\x3b\x05\x08\
	\x05\x02\x3b\x3c\x07\x05\x02\x02\x3c\x07\x03\x02\x02\x02\x3d\x40\x05\x0a\
	\x06\x02\x3e\x40\x05\x10\x09\x02\x3f\x3d\x03\x02\x02\x02\x3f\x3e\x03\x02\
	\x02\x02\x40\x09\x03\x02\x02\x02\x41\x42\x07\x06\x02\x02\x42\x47\x05\x0c\
	\x07\x02\x43\x44\x07\x07\x02\x02\x44\x45\x05\x0e\x08\x02\x45\x46\x07\x08\
	\x02\x02\x46\x48\x03\x02\x02\x02\x47\x43\x03\x02\x02\x02\x47\x48\x03\x02\
	\x02\x02\x48\x0b\x03\x02\x02\x02\x49\x4a\x09\x02\x02\x02\x4a\x0d\x03\x02\
	\x02\x02\x4b\x50\x07\x1f\x02\x02\x4c\x4d\x07\x0d\x02\x02\x4d\x4f\x07\x1f\
	\x02\x02\x4e\x4c\x03\x02\x02\x02\x4f\x52\x03\x02\x02\x02\x50\x4e\x03\x02\
	\x02\x02\x50\x51\x03\x02\x02\x02\x51\x0f\x03\x02\x02\x02\x52\x50\x03\x02\
	\x02\x02\x53\x54\x07\x0e\x02\x02\x54\x55\x07\x1e\x02\x02\x55\x11\x03\x02\
	\x02\x02\x56\x57\x07\x0f\x02\x02\x57\x58\x07\x1d\x02\x02\x58\x59\x07\x04\
	\x02\x02\x59\x5a\x05\x14\x0b\x02\x5a\x5b\x07\x05\x02\x02\x5b\x13\x03\x02\
	\x02\x02\x5c\x64\x05\x16\x0c\x02\x5d\x64\x05\x1a\x0e\x02\x5e\x60\x05\x1c\
	\x0f\x02\x5f\x5e\x03\x02\x02\x02\x60\x61\x03\x02\x02\x02\x61\x5f\x03\x02\
	\x02\x02\x61\x62\x03\x02\x02\x02\x62\x64\x03\x02\x02\x02\x63\x5c\x03\x02\
	\x02\x02\x63\x5d\x03\x02\x02\x02\x63\x5f\x03\x02\x02\x02\x64\x15\x03\x02\
	\x02\x02\x65\x66\x07\x10\x02\x02\x66\x67\x05\x18\x0d\x02\x67\x17\x03\x02\
	\x02\x02\x68\x69\x09\x03\x02\x02\x69\x19\x03\x02\x02\x02\x6a\x6b\x07\x14\
	\x02\x02\x6b\x6c\x07\x1e\x02\x02\x6c\x1b\x03\x02\x02\x02\x6d\x6e\x07\x15\
	\x02\x02\x6e\x6f\x07\x1d\x02\x02\x6f\x1d\x03\x02\x02\x02\x70\x71\x07\x15\
	\x02\x02\x71\x72\x07\x1d\x02\x02\x72\x73\x07\x04\x02\x02\x73\x74\x05\x20\
	\x11\x02\x74\x75\x07\x05\x02\x02\x75\x1f\x03\x02\x02\x02\x76\x7b\x05\x22\
	\x12\x02\x77\x79\x05\x24\x13\x02\x78\x77\x03\x02\x02\x02\x78\x79\x03\x02\
	\x02\x02\x79\x7b\x03\x02\x02\x02\x7a\x76\x03\x02\x02\x02\x7a\x78\x03\x02\
	\x02\x02\x7b\x21\x03\x02\x02\x02\x7c\x7d\x09\x04\x02\x02\x7d\x23\x03\x02\
	\x02\x02\x7e\u{83}\x05\x26\x14\x02\x7f\u{80}\x07\x0d\x02\x02\u{80}\u{82}\
	\x05\x26\x14\x02\u{81}\x7f\x03\x02\x02\x02\u{82}\u{85}\x03\x02\x02\x02\u{83}\
	\u{81}\x03\x02\x02\x02\u{83}\u{84}\x03\x02\x02\x02\u{84}\x25\x03\x02\x02\
	\x02\u{85}\u{83}\x03\x02\x02\x02\u{86}\u{87}\x07\x1d\x02\x02\u{87}\u{88}\
	\x07\x1c\x02\x02\u{88}\u{89}\x05\x28\x15\x02\u{89}\x27\x03\x02\x02\x02\u{8a}\
	\u{8b}\x09\x05\x02\x02\u{8b}\x29\x03\x02\x02\x02\x0c\x2d\x35\x3f\x47\x50\
	\x61\x63\x78\x7a\u{83}";

