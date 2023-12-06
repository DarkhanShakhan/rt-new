use super::{
    material::Material, matrice::Matrice, point::Point, ray::Ray, shape::Shape, vector::Vector,
};

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct Object {
    pub material: Material,
    pub shape: Shape,
    transformation: Matrice,
    transformation_inverse: Matrice,
    transformation_inverse_transpose: Matrice,
}

impl Object {
    pub fn new(material: Material, shape: Shape, transformation: Matrice) -> Object {
        let mut output = Object {
            material,
            shape,
            transformation,
            ..Default::default()
        };
        output.transformation_inverse = output.transformation.inverse();
        output.transformation_inverse_transpose = output.transformation_inverse.transpose();
        output
    }
    pub fn builder() -> ObjectBuilder {
        ObjectBuilder::default()
    }
    pub fn sphere_builder() -> ObjectBuilder {
        ObjectBuilder::sphere()
    }
    pub fn plane_builder() -> ObjectBuilder {
        ObjectBuilder::plane()
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        let transformed_ray = ray.transform(&self.transformation_inverse);
        self.shape.intersect(&transformed_ray)
    }
    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = self.world_to_object(world_point);
        let local_normal = self.shape.normal_at(&object_point);
        self.normal_to_world(&local_normal)
    }
    fn world_to_object(&self, world_point: &Point) -> Point {
        &self.transformation_inverse * world_point
    }
    fn normal_to_world(&self, local_normal: &Vector) -> Vector {
        (&self.transformation_inverse_transpose * local_normal).normalize()
    }
    pub fn transformation_inverse(&self) -> &Matrice {
        &self.transformation_inverse
    }
    pub fn transformation(&self) -> &Matrice {
        &self.transformation
    }
    pub fn set_transformation(&mut self, transformation: Matrice) {
        self.transformation = transformation;
        self.transformation_inverse = self.transformation.inverse();
    }
}

#[derive(Default)]
pub struct ObjectBuilder {
    material: Option<Material>,
    shape: Option<Shape>,
    transformation: Option<Matrice>,
}

impl ObjectBuilder {
    pub fn material(mut self, material: Material) -> ObjectBuilder {
        self.material = Some(material);
        self
    }
    pub fn shape(mut self, shape: Shape) -> ObjectBuilder {
        self.shape = Some(shape);
        self
    }
    pub fn sphere() -> ObjectBuilder {
        ObjectBuilder {
            shape: Some(Shape::Sphere),
            ..Default::default()
        }
    }

    pub fn plane() -> ObjectBuilder {
        ObjectBuilder {
            shape: Some(Shape::Plane),
            ..Default::default()
        }
    }
    pub fn transformation(mut self, transformation: Matrice) -> ObjectBuilder {
        self.transformation = Some(transformation);
        self
    }
    pub fn build(self) -> Object {
        Object::new(
            self.material.unwrap_or_default(),
            self.shape.unwrap_or_default(),
            self.transformation.unwrap_or_default(),
        )
    }
}

#[cfg(test)]
mod object_builder_tests {
    use crate::features::transformations::translation;

    use super::*;
    #[test]
    fn build_shape_object() {
        let object = Object::sphere_builder()
            .material(Material::default())
            .transformation(translation(0.2, 0.2, 0.2))
            .build();
        assert_eq!(object.shape, Shape::Sphere)
    }
}
