progenitor::generate_api!(
    spec = "../../openapi/auth/auth-latest.json",
    interface = Positional,
    derives = [schemars::JsonSchema, PartialEq],
    inner_type = slog::Logger,
    pre_hook = (|log: &slog::Logger, request: &reqwest::Request| {
        slog::debug!(log, "client request";
            "method" => %request.method(),
            "uri" => %request.url(),
            "body" => ?&request.body(),
        );
    }),
    crates = {
        "lumen-uuid-kinds" = "*",
    },
);
