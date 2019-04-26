const fibonacci = x => (x <= 2 ? 1 : fibonacci(x - 1) + fibonacci(x - 2));

module.exports = { fibonacci };
