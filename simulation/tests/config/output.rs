use simulation::config::output::Output;
use serde_yaml;

#[test]
fn test_output_deserialization() {
    let yaml_data = r#"
    paraview:
      file_name: "result.vtu"
      options:
        material: true
        body_ids: false
        tensor_values: false
        nodes: true
    vismesh_rel_area: 1e-5
    "#;

    let output: Output = serde_yaml::from_str(yaml_data).unwrap();
    assert_eq!(output.paraview.unwrap().file_name.unwrap(), "result.vtu");
    assert_eq!(output.paraview.unwrap().options.unwrap().material, Some(true));
    assert_eq!(output.paraview.unwrap().options.unwrap().body_ids, Some(false));
}
