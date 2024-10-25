use simulation::config::boundary_conditions::{BoundaryConditions, BoundaryConditionType};
use serde_json;

#[test]
fn test_boundary_conditions_deserialization() {
    let json_data = r#"
    {
        "conditions": [
            {
                "type": "dirichlet",
                "id": 1,
                "value": [0.0, 0.0, 0.0]
            },
            {
                "type": "neumann",
                "id": 2,
                "force": [0.0, -9.81, 0.0]
            },
            {
                "type": "obstacle_displacement",
                "id": 2,
                "value": [
                    "0.1 * t",
                    "0.0",
                    "0.0"
                ]
            }
        ]
    }
    "#;

    let boundary_conditions: BoundaryConditions = serde_json::from_str(json_data).unwrap();
    assert_eq!(boundary_conditions.conditions.len(), 3);

    match &boundary_conditions.conditions[0] {
        BoundaryConditionType::Dirichlet { id, value } => {
            assert_eq!(*id, 1);
            assert_eq!(*value, [0.0, 0.0, 0.0]);
        },
        _ => panic!("Expected Dirichlet boundary condition"),
    }

    match &boundary_conditions.conditions[1] {
        BoundaryConditionType::Neumann { id, force } => {
            assert_eq!(*id, 2);
            assert_eq!(*force, [0.0, -9.81, 0.0]);
        },
        _ => panic!("Expected Neumann boundary condition"),
    }

    match &boundary_conditions.conditions[2] {
        BoundaryConditionType::ObstacleDisplacement { id, value } => {
            assert_eq!(*id, 2);
            assert_eq!(value, &["0.1 * t".to_string(), "0.0".to_string(), "0.0".to_string()]);
        },
        _ => panic!("Expected ObstacleDisplacement boundary condition"),
    }
}
