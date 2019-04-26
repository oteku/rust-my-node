// Unoptimzed because we want expensive computing
fn calculate_fibonacci(x: i32) -> i32 {
    if x <= 2 {
        1
    } else {
        calculate_fibonacci(x - 1) + calculate_fibonacci(x - 2)
    }
}
use neon::prelude::*;

pub fn fibonacci(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value() as i32;
    if x > 46 {
        cx.throw_range_error("Try with lesser x")
    } else {
        Ok(cx.number(calculate_fibonacci(x)))
    }
}

struct FibonacciTask {
    argument: i32,
}

// Neon Async Task trait
impl Task for FibonacciTask {
    type Output = i32;
    type Error = ();
    type JsEvent = JsNumber;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        Ok(calculate_fibonacci(self.argument))
    }

    fn complete<'a>(
        self,
        mut cx: TaskContext<'a>,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        Ok(cx.number(result.unwrap()))
    }
}

pub fn fibonacci_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value() as i32;
    let cb = cx.argument::<JsFunction>(1)?;

    let task = FibonacciTask { argument: x };

    task.schedule(cb);
    Ok(cx.undefined())
}
