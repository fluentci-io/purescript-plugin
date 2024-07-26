use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let path = dag().get_env("PATH")?;
    let home = dag().get_env("HOME")?;
    dag().set_envs(vec![("PATH".into(), format!("{}/.bun/bin:{}", home, path))])?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["npm", "install", "-g", "spago"])?
        .with_exec(vec!["type npm || pkgx install npmjs.com"])?
        .with_exec(vec![
            "npm",
            "install",
            "-g",
            &format!("purescript@{}", version),
        ])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "build", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "test", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn bundle_app(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "bundle-app", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn bundle_module(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "bundle-module", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn docs(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "docs", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn verify(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "verify", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn verify_set(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "verify-set", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn bump_version(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "type purs > /dev/null 2>&1 || pkgx npm install -g purescript",
        ])?
        .with_exec(vec!["bunx", "spago", "bump-version", &args])?
        .stdout()?;
    Ok(stdout)
}
