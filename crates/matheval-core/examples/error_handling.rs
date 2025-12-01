/// Example demonstrating the improved error handling system
/// 
/// This shows how errors now include:
/// - Detailed error messages
/// - Position information (line and column)
/// - Source code context with visual pointer
/// - Helpful hints for common mistakes

use matheval_core::{Error, ErrorKind, Position};

fn main() {
    println!("=== Improved Error Handling Examples ===\n");
    
    // Example 1: Division by zero with helpful message
    println!("1. Division by zero error:");
    let err = Error::division_by_zero();
    println!("{}\n", err);
    
    // Example 2: Wrong argument count with position
    println!("2. Wrong function argument count:");
    let err = Error::wrong_arg_count("sin", 1, 2)
        .with_position(Position::new(1, 5, 4));
    println!("{}\n", err);
    
    // Example 3: Unexpected character with source context
    println!("3. Unexpected character with source context:");
    let source = "x + @ - y";
    let err = Error::unexpected_char('@', Position::new(1, 5, 4))
        .with_source(source.to_string());
    println!("{}\n", err);
    
    // Example 4: Missing parenthesis
    println!("4. Missing closing parenthesis:");
    let source = "sin(x + cos(y)";
    let err = Error::new(ErrorKind::MissingFunctionClosingParen("sin".to_string()))
        .with_position(Position::new(1, 15, 14))
        .with_source(source.to_string());
    println!("{}\n", err);
    
    // Example 5: Unknown function with suggestions
    println!("5. Unknown function:");
    let source = "foo(x) + bar(y)";
    let err = Error::new(ErrorKind::UnknownFunction("foo".to_string()))
        .with_position(Position::new(1, 1, 0))
        .with_source(source.to_string());
    println!("{}\n", err);
    
    // Example 6: Undefined variable
    println!("6. Undefined variable:");
    let err = Error::new(ErrorKind::UndefinedVariable("z".to_string()))
        .with_position(Position::new(1, 10, 9));
    println!("{}\n", err);
    
    println!("=== Benefits ===");
    println!("✓ Clear error messages");
    println!("✓ Exact position (line & column)");
    println!("✓ Visual source context");
    println!("✓ Helpful hints");
    println!("✓ Better debugging experience");
}
