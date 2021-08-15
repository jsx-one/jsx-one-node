use jsx_one::code::code;
use neon::prelude::*;
fn parse(mut cx: FunctionContext) -> JsResult<JsString> {
    let js_code = "function main(){}";
    let code = code(js_code);
    Ok(cx.string(code))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse", parse)?;
    Ok(())
}
