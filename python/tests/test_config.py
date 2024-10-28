import unittest
from config import SimulationConfig, BoundaryCondition

class TestSimulationConfig(unittest.TestCase):
    def test_boundary_conditions(self):
        json_data = '''
        {
            "common": "ipc-defaults.json",
            "geometry": [
                {
                    "mesh": "cube1.msh",
                    "transformation": {
                        "translation": [1.0, 2.0, 3.0]
                    },
                    "volume_selection": 1,
                    "is_obstacle": false,
                    "material": "Steel"
                },
                {
                    "mesh": "cube2.msh",
                    "transformation": {
                        "translation": [4.0, 5.0, 6.0]
                    },
                    "volume_selection": 2,
                    "is_obstacle": true,
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
        }
        '''
        config = SimulationConfig.from_json(json_data)
        self.assertEqual(config.common, "ipc-defaults.json")
        self.assertEqual(len(config.geometry), 2)
        self.assertEqual(config.geometry[0]["mesh"], "cube1.msh")
        self.assertEqual(config.geometry[1]["material"], "Aluminum")
        self.assertEqual(len(config.boundary_conditions), 3)
        self.assertEqual(config.boundary_conditions[0].type, "dirichlet")
        self.assertEqual(config.boundary_conditions[1].type, "neumann")
        self.assertEqual(config.boundary_conditions[2].type, "obstacle_displacement")

if __name__ == '__main__':
    unittest.main()
