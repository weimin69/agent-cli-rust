use anyhow::{Context, Result};
pub fn execute(path: &str) -> Result<()> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("failed to read config file"))?;

    //  || 表示“等真的出错时，再执行这段代码生成错误信息”
    //  它的好处是：如果读取成功，就不会执行 format!，不会额外创建字
    //  符串
    //
    println!("{}", content);
    Ok(())
}
