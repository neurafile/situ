grammar GadgetLanguage;

// Entry point
program: statement* EOF;

statement
    : colliderDef
    | listenerDef
    | actionDef
    ;

// ----------------------
// Collider definitions
// ----------------------
colliderDef: 'collider' ID '{' colliderItem* '}' ;

colliderItem
    : shapeDef semi?
    | geometryRef semi?
    ;

shapeDef: 'shape' shapeType ('(' shapeParams? ')')? ;
shapeType: 'box' | 'sphere' | 'cylinder' | 'capsule' ;
shapeParams: NUMBER (',' NUMBER)* ;
geometryRef: 'geometry' STRING ;

// ----------------------
// Listener definitions
// ----------------------
listenerDef: 'listener' ID '{' listenerItem* '}' ;

listenerItem
    : eventType semi?
    | targetSpec semi?
    | actionRef semi?
    ;

// keep your event names for now
eventType: 'on' eventName ;
eventName: 'collision' | 'trigger' | 'custom' ;

// target can be an ID (recommended) or a quoted string (legacy-friendly)
targetSpec: 'target' (ID | STRING) ;

// allow multiple action lines; each adds to the pipeline
actionRef: 'action' ID ;

// ----------------------
// Action definitions
// ----------------------
actionDef: 'action' ID '{' actionItem* '}' ;

// allow declaring the action type + params in any order
actionItem
    : actionType semi?
    | paramPair semi?
    ;

actionType
    : 'runAnim'
    | 'stopAnim'
    | 'playSound'
    | 'stopSound'
    | 'httpCall'
    | 'emitEvent'
    ;

// Params as repeated pairs is friendlier than comma-lists in practice
paramPair: ID '=' value ;

value: STRING | NUMBER | BOOLEAN ;

// Optional semicolon
semi: ';' ;

// ----------------------
// Tokens
// ----------------------
ID: [a-zA-Z_][a-zA-Z0-9_]* ;
STRING: '"' (~["\\] | '\\' .)* '"' ;
NUMBER: '-'? [0-9]+ ('.' [0-9]+)? ;
BOOLEAN: 'true' | 'false' ;

WS: [ \t\r\n]+ -> skip ;
COMMENT: '//' ~[\r\n]* -> skip ;
BLOCK_COMMENT: '/*' .*? '*/' -> skip ;
