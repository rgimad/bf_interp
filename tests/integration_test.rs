use libbrainsight::{parse, Op, Interpreter};

#[test]
fn test_parse_simple_operations() {
    let code = b"++-->>><<";
    let result = parse(code).unwrap();
    
    assert_eq!(result, vec![
        Op::Inc(2),
        Op::Dec(2),
        Op::Next(3),
        Op::Prev(2),
    ]);
}

#[test]
fn test_parse_combined_operations() {
    let code = b"+++++";
    let result = parse(code).unwrap();
    assert_eq!(result, vec![Op::Inc(5)]);
    
    let code = b"<<<<";
    let result = parse(code).unwrap();
    assert_eq!(result, vec![Op::Prev(4)]);
}

#[test]
fn test_parse_loop() {
    let code = b"[+++]";
    let result = parse(code).unwrap();
    
    assert_eq!(result, vec![
        Op::Loop(vec![Op::Inc(3)])
    ]);
}

#[test]
fn test_parse_nested_loops() {
    let code = b"[++[--]+]";
    let result = parse(code).unwrap();
    
    assert_eq!(result, vec![
        Op::Loop(vec![
            Op::Inc(2),
            Op::Loop(vec![Op::Dec(2)]),
            Op::Inc(1),
        ])
    ]);
}

#[test]
fn test_parse_unmatched_brackets() {
    let code = b"[[++]";
    assert!(parse(code).is_err());
    
    let code = b"++]]";
    assert!(parse(code).is_err());
}

#[test]
fn test_execute_increment() {
    let mut interpreter = Interpreter::new();
    let program = vec![Op::Inc(5)];
    
    interpreter.run(&program).unwrap();
    assert_eq!(interpreter.get_memory()[0], 5);
}

#[test]
fn test_execute_decrement() {
    let mut interpreter = Interpreter::new();
    interpreter.set_memory(0, 10);
    let program = vec![Op::Dec(3)];
    
    interpreter.run(&program).unwrap();
    assert_eq!(interpreter.get_memory()[0], 7);
}

#[test]
fn test_execute_pointer_movement() {
    let mut interpreter = Interpreter::new();
    interpreter.set_memory(0, 1);
    interpreter.set_memory(1, 2);
    interpreter.set_memory(2, 3);
    
    let program = vec![
        Op::Next(1),
        Op::Inc(5),
        Op::Next(1),
        Op::Dec(2),
    ];
    
    interpreter.run(&program).unwrap();
    assert_eq!(interpreter.get_memory()[0], 1);
    assert_eq!(interpreter.get_memory()[1], 7);
    assert_eq!(interpreter.get_memory()[2], 1);
}

#[test]
fn test_execute_simple_loop() {
    let mut interpreter = Interpreter::new();
    interpreter.set_memory(0, 3);
    
    // Program: [->+<] - moves value from cell 0 to cell 1
    let program = vec![
        Op::Loop(vec![
            Op::Dec(1),
            Op::Next(1),
            Op::Inc(1),
            Op::Prev(1),
        ])
    ];
    
    interpreter.run(&program).unwrap();
    assert_eq!(interpreter.get_memory()[0], 0);
    assert_eq!(interpreter.get_memory()[1], 3);
}

#[test]
fn test_hello_world_memory() {
    let mut interpreter = Interpreter::new();
    
    // Program to set cell 0 to 'H' (72)
    let program = vec![
        Op::Inc(72),
    ];
    
    interpreter.run(&program).unwrap();
    assert_eq!(interpreter.get_memory()[0], 72); // 'H' in ASCII
}

#[test]
fn test_pointer_overflow() {
    let mut interpreter = Interpreter::new();
    interpreter.set_data_ptr(29999);
    
    let program = vec![Op::Next(2)]; // This should overflow
    
    let result = interpreter.run(&program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("overflow"));
}

#[test]
fn test_pointer_underflow() {
    let mut interpreter = Interpreter::new();
    interpreter.set_data_ptr(0);
    
    let program = vec![Op::Prev(1)]; // This should underflow
    
    let result = interpreter.run(&program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("underflow"));
}

#[test]
fn test_wrapping_arithmetic() {
    let mut interpreter = Interpreter::new();
    interpreter.set_memory(0, 127); // Max i8 value
    
    let program = vec![Op::Inc(1)]; // This should wrap to -128
    
    interpreter.run(&program).unwrap();
    assert_eq!(interpreter.get_memory()[0], -128);
}

#[test]
fn test_print_output() {
    let mut interpreter = Interpreter::new();
    interpreter.set_memory(0, 65); // 'A' in ASCII
    
    let program = vec![Op::Print];
    
    // This will show output when running with --nocapture
    println!("Expected output: A");
    interpreter.run(&program).unwrap();
}
