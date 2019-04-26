const addon = require('./native');
const fibo = require('./lib/fibo');
const argv = require('yargs').argv;

const x = argv.value;
let before;
let after;
const fiboPromise = x =>
  new Promise((res, rej) => {
    before = new Date();
    addon.fibonacciAsync(x, (error, result) => {
      after = new Date();
      return error ? rej(error) : res(result);
    });
  });

const inter = setInterval(() => console.log('I am not blocked'), 1000);
fiboPromise(x)
  .then(fiboX => {
    clearInterval(inter);
    console.log(`NEON ASYNC Fibo(${x}) = ${after - before}ms -> ${fiboX}`);
  })
  .catch(console.error);

console.log('In the JS callstack');
setTimeout(() => console.log('Timeout at 3s'), 3000);
