import json
from typing import List, Optional, Union

class BoundaryCondition:
    def __init__(self, bc_type: str, id: int, value: Union[List[float], List[str]]):
        self.type = bc_type
        self.id = id
        self.value = value

    def to_dict(self):
        return {
            "type": self.type,
            "id": self.id,
            "value": self.value
        }

class SimulationConfig:
    def __init__(self, common: Optional[str], geometry: List[dict], contact: dict,
                 time: dict, space: dict, boundary_conditions: List[BoundaryCondition],
                 materials: List[dict], solver: dict, output: dict):
        self.common = common
        self.geometry = geometry
        self.contact = contact
        self.time = time
        self.space = space
        self.boundary_conditions = boundary_conditions
        self.materials = materials
        self.solver = solver
        self.output = output

    def to_json(self):
        bc = [bc.to_dict() for bc in self.boundary_conditions]
        data = {
            "common": self.common,
            "geometry": self.geometry,
            "contact": self.contact,
            "time": self.time,
            "space": self.space,
            "boundary_conditions": {
                "conditions": bc
            },
            "materials": {
                "materials": self.materials
            },
            "solver": self.solver,
            "output": self.output
        }
        return json.dumps(data, indent=4)

    @staticmethod
    def from_json(json_str: str):
        data = json.loads(json_str)
        bcs = data.get("boundary_conditions", {}).get("conditions", [])
        boundary_conditions = [BoundaryCondition(**bc) for bc in bcs]
        return SimulationConfig(
            common=data.get("common"),
            geometry=data.get("geometry", []),
            contact=data.get("contact", {}),
            time=data.get("time", {}),
            space=data.get("space", {}),
            boundary_conditions=boundary_conditions,
            materials=data.get("materials", {}).get("materials", []),
            solver=data.get("solver", {}),
            output=data.get("output", {})
        )
