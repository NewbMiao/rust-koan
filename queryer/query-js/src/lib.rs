use neon::prelude::*;

pub fn example_sql(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(queryer::example_sql()))
}

fn query(mut cx: FunctionContext) -> JsResult<JsString> {
    let sql = cx.argument::<JsString>(0)?.value(&mut cx);
    let output = match cx.argument::<JsString>(1) {
        Ok(v) => v.value(&mut cx),
        Err(_) => "csv".to_string(),
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = rt.block_on(async { queryer::query(sql).await.unwrap() });

    match output.as_str() {
        "csv" => Ok(cx.string(data.to_csv().unwrap())),
        v => cx.throw_type_error(format!("Output type {} not supported", v)),
    }
}

// https://github.com/tyrchen/geektime-rust/tree/master/06_queryer/queryer-js
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("example_sql", example_sql)?;
    cx.export_function("query", query)?;
    Ok(())
}
