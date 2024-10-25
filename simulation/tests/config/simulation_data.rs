use simulation::config::simulation_data::SimulationData;
use serde_json;

#[test]
fn test_simulation_data_deserialization() {
    let json_data = r#"
    {
        "common": "simulation_defaults.json",
        "geometry": [
            {
                "mesh": "cube1.msh",
                "transformation": {
                    "translation": [1.0, 2.0, 3.0]
                },
                "volume_selection": 1,
                "is_obstacle": true,
                "material": "Steel"
            },
            {
                "mesh": "sphere1.msh",
                "transformation": {
                    "translation": [4.0, 5.0, 6.0]
                },
                "volume_selection": 2,
                "is_obstacle": false,
                "material": "Aluminum"
            }
        ],
        "contact": {
            "friction_coefficient": 0.1,
            "enabled": true,
            "dhat": 0.001,
            "epsv": 0.01
        },
        "time": {
            "integrator": "ImplicitEuler",
            "tend": 5.0,
            "dt": 0.025
        },
        "materials": {
            "materials": [
                {
                    "name": "Steel",
                    "density": 8000.0,
                    "youngs_modulus": 200.0,
                    "poisson_ratio": 0.3
                },
                {
                    "name": "Aluminum",
                    "density": 2700.0,
                    "youngs_modulus": 69.0,
                    "poisson_ratio": 0.33
                }
            ]
        },
        "space": {
            "advanced": {
                "bc_method": "sample"
            }
        },
        "boundary_conditions": {
            "rhs": [0.0, 9.81, 0.0]
        },
        "solver": {
            "linear": {
                "solver": ["Eigen::PardisoLDLT"]
            },
            "nonlinear": {
                "x_delta": 1e-05
            },
            "advanced": {
                "lump_mass_matrix": true
            },
            "contact": {
                "friction_convergence_tol": 0.01,
                "friction_iterations": 1
            }
        },
        "output": {
            "paraview": {
                "file_name": "result.vtu",
                "options": {
                    "material": true,
                    "body_ids": true,
                    "tensor_values": true,
                    "nodes": true
                }
            },
            "vismesh_rel_area": 1e-5
        }
    }"#;

    let simulation_data: SimulationData = serde_json::from_str(json_data).unwrap();

    // Validate common field
    assert_eq!(simulation_data.common.unwrap(), "simulation_defaults.json");

    // Validate geometries
    assert_eq!(simulation_data.geometry.unwrap().len(), 2);

    let first_geometry = &simulation_data.geometry.unwrap()[0];
    assert_eq!(first_geometry.mesh, "cube1.msh");
    assert_eq!(first_geometry.material, "Steel");

    let second_geometry = &simulation_data.geometry.unwrap()[1];
    assert_eq!(second_geometry.mesh, "sphere1.msh");
    assert_eq!(second_geometry.material, "Aluminum");

    // Validate materials
    let materials = simulation_data.materials.unwrap().materials;
    assert_eq!(materials.len(), 2);

    let steel = materials.iter().find(|m| m.name == "Steel").unwrap();
    assert_eq!(steel.density, 8000.0);
    assert_eq!(steel.youngs_modulus, 200.0);
    assert_eq!(steel.poisson_ratio, 0.3);

    let aluminum = materials.iter().find(|m| m.name == "Aluminum").unwrap();
    assert_eq!(aluminum.density, 2700.0);
    assert_eq!(aluminum.youngs_modulus, 69.0);
    assert_eq!(aluminum.poisson_ratio, 0.33);

    // Additional validations can be added here
}
