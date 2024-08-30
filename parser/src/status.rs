pub fn format_progress_string(ok: usize, err: usize, total: usize) -> String {
    format!(
        "\x1b[32m[Ok: {ok}]\x1b[0m \x1b[31m[Err: {err}]\x1b[0m \x1b[33m[Left: {}]\x1b[0m",
        total - ok - err
    )
}
