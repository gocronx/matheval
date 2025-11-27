import matheval
import pytest


def test_simple_arithmetic():
    """Test basic arithmetic operations"""
    compiler = matheval.Compiler()
    program = compiler.compile("1 + 2 * 3")
    context = matheval.Context()
    
    result = program.eval(context)
    assert result == 7.0


def test_variables():
    """Test variable substitution"""
    compiler = matheval.Compiler()
    program = compiler.compile("x + y")
    
    context = matheval.Context()
    context.set("x", 10.0)
    context.set("y", 20.0)
    
    result = program.eval(context)
    assert result == 30.0


def test_functions():
    """Test built-in functions"""
    compiler = matheval.Compiler()
    program = compiler.compile("max(1, 2, 3) + min(4, 5)")
    context = matheval.Context()
    
    result = program.eval(context)
    assert result == 7.0  # max(1,2,3) = 3, min(4,5) = 4, 3+4 = 7


def test_precedence():
    """Test operator precedence"""
    compiler = matheval.Compiler()
    
    # 2 * 3 + 4 = 10
    p1 = compiler.compile("2 * 3 + 4")
    # 2 + 3 * 4 = 14
    p2 = compiler.compile("2 + 3 * 4")
    
    ctx = matheval.Context()
    assert p1.eval(ctx) == 10.0
    assert p2.eval(ctx) == 14.0


def test_right_associativity():
    """Test right-associative power operator"""
    compiler = matheval.Compiler()
    # 2 ^ 3 ^ 2 = 2 ^ (3 ^ 2) = 2 ^ 9 = 512
    program = compiler.compile("2 ^ 3 ^ 2")
    context = matheval.Context()
    
    result = program.eval(context)
    assert result == 512.0


def test_compilation_error():
    """Test that invalid expressions raise errors"""
    compiler = matheval.Compiler()
    
    with pytest.raises(ValueError):
        compiler.compile("1 + + 2")  # Invalid syntax


def test_runtime_error():
    """Test that undefined variables raise errors"""
    compiler = matheval.Compiler()
    program = compiler.compile("x + y")
    context = matheval.Context()
    # Only set x, not y
    context.set("x", 10.0)
    
    with pytest.raises(RuntimeError):
        program.eval(context)


def test_complex_expression():
    """Test a more complex real-world expression"""
    compiler = matheval.Compiler()
    # Quadratic formula: (-b + sqrt(b^2 - 4*a*c)) / (2*a)
    program = compiler.compile("(-b + sqrt(b ^ 2 - 4 * a * c)) / (2 * a)")
    
    context = matheval.Context()
    context.set("a", 1.0)
    context.set("b", -5.0)
    context.set("c", 6.0)
    
    result = program.eval(context)
    # For x^2 - 5x + 6 = 0, one solution is x = 3
    assert abs(result - 3.0) < 0.0001


def test_eval_batch():
    """Test batch evaluation"""
    compiler = matheval.Compiler()
    program = compiler.compile("x * 2 + y")
    
    # Check variable order (alphabetical or first appearance depending on implementation)
    # In Rust implementation it's first appearance: x, y
    var_names = program.var_names
    assert "x" in var_names
    assert "y" in var_names
    
    # Prepare variable sets
    # x=1, y=2 -> 4
    # x=3, y=4 -> 10
    # x=5, y=6 -> 16
    var_sets = [
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0]
    ]
    
    results = program.eval_batch(var_sets)
    assert len(results) == 3
    assert results[0] == 4.0
    assert results[1] == 10.0
    assert results[2] == 16.0

    # Test with empty list
    assert program.eval_batch([]) == []

    # Test error handling (wrong number of variables)
    with pytest.raises(RuntimeError):
        program.eval_batch([[1.0]])  # Missing y


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
