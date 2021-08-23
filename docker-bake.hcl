target "shellcheck" {
    dockerfile = "./dockerfiles/Dockerfile.shellcheck"
    target = "shellcheck"
    output = ["type=cacheonly"]
}
