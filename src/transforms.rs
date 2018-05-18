use ::matrices::{ AffineMatrix, Primitives }
use ::vectors::{ KVector3, AffineVector }

pub struct Transform {
    transforms: Vec<AffineMatrix>,
    inversions: Vec<AffineMatrix>,    
}

impl Transform {

    pub fn append_translation(&self, v: KVector3) -> Transform {
        let matrix = AffineMatrix::new(Primitives::Translation(v.x(), v.y(), v.z()));
        self.transforms.push(matrix);
        self.inversions.push(matrix.inverse());
        self
    }


    pub fn merge_translation(&self, v: KVector3) -> Transform {
        self.transforms.push(self.transforms.pop() * v);
        self.inversions.push(self.inversions.pop() * v.inverse());        
    }
}

