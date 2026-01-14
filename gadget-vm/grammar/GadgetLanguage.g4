grammar GadgetLanguage;

// Entry point
program: statement* EOF;

statement
    : colliderDef
    | listenerDef
    | actionDef
    ;

// Collider definitions
colliderDef: 'collider' ID '{' colliderBody '}';
colliderBody
    : shapeDef
    | geometryRef
    ;
shapeDef: 'shape' shapeType ('(' shapeParams ')')?;
shapeType: 'box' | 'sphere' | 'cylinder' | 'capsule';
shapeParams: NUMBER (',' NUMBER)*;
geometryRef: 'geometry' STRING;

// Listener definitions
listenerDef: 'listener' ID '{' listenerBody '}';
listenerBody
    : eventType
    | targetId
    | actionRef+
    ;
eventType: 'on' eventName;
eventName: 'collision' | 'trigger' | 'custom';
targetId: 'target' STRING;
actionRef: 'action' ID;

// Action definitions
actionDef: 'action' ID '{' actionBody '}';
actionBody
    : actionType
    | actionParams?
    ;
actionType: 'runAnim' | 'stopAnim' | 'playSound' | 'stopSound' | 'httpCall' | 'emitEvent';
actionParams: paramPair (',' paramPair)*;
paramPair: ID '=' value;
value: STRING | NUMBER | BOOLEAN;

// Tokens
ID: [a-zA-Z_][a-zA-Z0-9_]*;
STRING: '"' (~["\\] | '\\' .)* '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
BOOLEAN: 'true' | 'false';

WS: [ \t\r\n]+ -> skip;
COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
