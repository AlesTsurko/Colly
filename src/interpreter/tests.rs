use super::*;
use crate::parser::{CollyParser, Rule};

#[ignore]
#[test]
fn interpret_pattern_complex() {
    let mut context = Context::default();
    // let result: ast::Pattern = tests::parse_source_for_rule("|01 2|", Rule::Pattern).unwrap();

    let result: ast::Pattern = CollyParser::parse_source_for_rule(
        "| 01*:23 01[0 1 23]* (012 34)* 01(23 4)5*1: 1: |",
        Rule::Pattern,
    )
    .unwrap();
    // let pattern: types::Pattern = result.interpret(&mut context).unwrap();
    // let expected = ...;
    // assert_eq!(expected, result);
}

#[test]
fn interpret_event_group_methods() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("a*._:", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        depth: 0,
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: 0.0,
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree::from(10)),
                duration: 3.0,
                octave: None,
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Tie,
                duration: 0.5,
                octave: None,
                beat_position: 3.0,
                beat: 0,
            }
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_event_group_alterations() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("++a-+--b", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        depth: 0,
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: 0.0,
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree {
                    value: 10,
                    alteration: 2,
                }),
                duration: 1.0,
                octave: None,
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree {
                    value: 11,
                    alteration: -2,
                }),
                duration: 1.0,
                octave: None,
                beat_position: 1.0,
                beat: 0,
            }
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_event_group_octaves() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("OOaooob", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        depth: 0,
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: 0.0,
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree::from(10)),
                duration: 1.0,
                octave: Some(Octave::with_octave(7)),
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(11)),
                duration: 1.0,
                octave: Some(Octave::with_octave(4)),
                beat_position: 1.0,
                beat: 0,
            }
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_parenthesised_group_simple() {
    assert!(false);
}

#[test]
fn interpret_parenthesised_group() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("(ab 0)", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        depth: 0,
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: 0.0,
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree::from(10)),
                duration: 0.25,
                octave: None,
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(11)),
                duration: 0.25,
                octave: None,
                beat_position: 0.25,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.5,
                octave: None,
                beat_position: 0.5,
                beat: 0,
            },
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_mixed_event() { 
    assert!(false);
}

#[test]
fn interpret_pattern_inner_simple() {
    use types::*;

    let mut context = Context::default();
    let pattern: ast::Pattern =
        CollyParser::parse_source_for_rule("| 01 2 |", Rule::Pattern).unwrap();
    let inner_interpreter = PatternInnerInterpreter::new(pattern.0);
    let result = inner_interpreter.interpret(&mut context).unwrap();
    let expected = vec![
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            octave: None,
            duration: 0.5,
            beat_position: 0.0,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(1)),
            octave: None,
            duration: 0.5,
            beat_position: 0.5,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(2)),
            octave: None,
            duration: 1.0,
            beat_position: 0.0,
            beat: 1,
        },
    ];

    assert_eq!(expected, result);
}

#[test]
fn interpret_pattern_inner_methods() {
    use types::*;

    let mut context = Context::default();
    let pattern: ast::Pattern =
        CollyParser::parse_source_for_rule("| 0:1. 2* |", Rule::Pattern)
            .unwrap();
    let inner_interpreter = PatternInnerInterpreter::new(pattern.0);
    let result = inner_interpreter.interpret(&mut context).unwrap();
    let expected = vec![
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            octave: None,
            duration: 0.25,
            beat_position: 0.0,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(1)),
            octave: None,
            duration: 0.75,
            beat_position: 0.25,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(2)),
            octave: None,
            duration: 1.0,
            beat_position: 0.0,
            beat: 1,
        },
    ];

    assert_eq!(expected, result);;
}

#[test]
fn interpret_pattern_inner_parenthesised() {
    assert!(false);
}

#[test]
fn interpret_pattern_inner_ties() {
    assert!(false);
}
