use {{ cookiecutter.repo_name }};

#[test]
fn test_{{ cookiecutter.repo_name }}() {
    {{ cookiecutter.repo_name }}::run();
    assert!(true)
}
