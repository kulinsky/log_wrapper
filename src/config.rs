pub fn get_env_params<'a>() -> [(&'a str, String); 4] {
    [
        (
            "container_name",
            std::env::var("CONTAINER_NAME").unwrap_or_default(),
        ),
        (
            "container_image",
            std::env::var("CONTAINER_IMAGE").unwrap_or_default(),
        ),
        ("env", std::env::var("ENV").unwrap_or_default()),
        (
            "k8s_namespace",
            std::env::var("K8S_NAMESPACE").unwrap_or_default(),
        ),
    ]
}
