const addon = require('./native');
const fibo = require('./lib/fibo');
const argv = require('yargs').argv;

const x = argv.value;

let before = new Date();
try {
  console.log(addon.fibonacci(x));
} catch (err) {
  console.error(err);
}
let after = new Date();
console.log(`NEON SYNC Fibo(${x}) = ${after - before}ms`);

before = new Date();
console.log(fibo.fibonacci(x));
after = new Date();
console.log(`JS Fibo(${x}) = ${after - before}ms`);
