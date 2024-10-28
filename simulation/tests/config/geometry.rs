use simulation::config::geometry::{Geometry, Transformation};
use serde_json;

#[test]
fn test_geometry_deserialization() {
    let json_data = r#"
    {
        "mesh": "cube.msh",
        "transformation": {
            "translation": [1.0, 2.0, 3.0]
        },
        "volume_selection": 1,
        "is_obstacle": true
    }"#;

    let geometry: Geometry = serde_json::from_str(json_data).unwrap();
    assert_eq!(geometry.mesh, "cube.msh");
    assert_eq!(geometry.transformation.unwrap().translation, Some([1.0, 2.0, 3.0]));
    assert_eq!(geometry.volume_selection, Some(1));
    assert_eq!(geometry.is_obstacle, Some(true));
}
