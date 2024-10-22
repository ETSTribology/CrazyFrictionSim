use assert_cmd::Command;

#[test]
fn test_simulation_output() {
    let mut cmd = Command::cargo_bin("fem_simulation").unwrap();
    cmd.arg("--input").arg("test_mesh.msh")
        .assert()
        .success();
}
