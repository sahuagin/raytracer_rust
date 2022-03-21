use crate::aabb::BoundingBox;
use crate::prelude;
use prelude::{HitList, HitRecord, Hittable, Ray};
use std::sync::Arc;
use rand::Rng;

///! The BVH starts at the head of the hierarchy, it can't be empty
///! as there needs to be some boundary around the scene. From the
///! head you navigate to the leaves.
pub type Bvh = BvhNode;

//type BvhLink = Arc<*mut BvhNode>;
// The bvhlink will have bvhnodes for most of the tree, but the leaves
// will be objects. So the pointers are to hittables
type BvhLink = Arc<Box::<dyn Hittable>>;

/// These can potentially be empty, if so their bounding box isn't
/// meaningful
#[derive(Clone)]
pub struct BvhNode {
    p_left: BvhLink,
    p_right: BvhLink,
    pub bb: BoundingBox,
}

impl<'a> BvhNode {
    pub fn new() -> Self {
        BvhNode {
            p_left: Arc::new(Box::from(BoundingBox::default())),
            p_right: Arc::new(Box::from(BoundingBox::default())),
            bb: BoundingBox::default(),
        }
    }

    pub fn init_left(&mut self, node: &dyn Hittable) {
        std::mem::swap(
            &mut self.p_left,
            &mut Arc::new(node.box_clone())
        )
    }
    pub fn init_right(&mut self, node: &dyn Hittable) {
        std::mem::swap(
            &mut self.p_right,
            &mut Arc::new(node.box_clone()),
        );
    }

    pub fn left(&self) -> Option<&BvhLink> {
        Some(&self.p_left)
    }

    pub fn right(&self) -> Option<&BvhLink> {
        Some(&self.p_right)
    }

    pub fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    pub fn build(&self) -> Self {
        self.clone()
    }

    pub fn add_hitlist(& mut self, hl: & mut Arc<HitList>, t_min: f64, t_max: f64) -> &mut Self {
        // first we need to sort the list
        // choose a random axis to partition by
        let axis = Self::rand_axis();
        // make sure our bounding box contains everything we hold
        for item in &hl.list {
            self.bb = BoundingBox::expand_to_contain(
                self.bb.bounding_box(t_min, t_max),
                item.bounding_box(t_min, t_max)).unwrap_or_default();
        }
        
        match axis {
            0 => Arc::get_mut(hl).unwrap().list.sort_unstable_by(|a,b| BoundingBox::cmp_by_x(a, b).unwrap()),
            1 => Arc::get_mut(hl).unwrap().list.sort_unstable_by(|a,b| BoundingBox::cmp_by_y(a, b).unwrap()),
            2..=u8::MAX => Arc::get_mut(hl).unwrap().list.sort_unstable_by(|a,b| BoundingBox::cmp_by_z(a, b).unwrap()),
        }

        if hl.list.len() == 1 {
            // then we'll just put that item in both branches.
            self.p_left = Arc::new(hl.list[0].box_clone());
        } else if hl.list.len() == 2 {
            // then, one for each side
            self.p_left = Arc::new(hl.list[0].box_clone());
            self.p_right = Arc::new(hl.list[1].box_clone());
        }
        else {
            // we have a few things, we'll split and give 1/2 to each side
            let mut left_node = BvhNode::new();
            let mut right_node = BvhNode::new();
            let (left_list, right_list) = hl.list.split_at(hl.list.len()/2);
            let mut left_hl = HitList::new();
            left_hl.list=left_list.to_vec();
            let mut right_hl = HitList::new();
            right_hl.list=right_list.to_vec();
            let left_hl = left_hl;
            let right_hl = right_hl;
            
            left_node.add_hitlist(& mut Arc::new(left_hl), t_min, t_max);
            right_node.add_hitlist(& mut Arc::new(right_hl), t_min, t_max);
            self.p_left = Arc::new(Box::from(left_node));
            self.p_right = Arc::new(Box::from(right_node));
        }

        return self;
    }

    pub fn rand_axis() -> u8 {
        let mut rng = rand::thread_rng();
        (rng.gen::<f64>() * 3.) as u8
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // nodes should either have left and right populated, or not exist
        self.p_left.as_ref().hitter_fmt(f)?;
        self.p_right.as_ref().hitter_fmt(f)
    }
}

impl<'a> Default for BvhNode {
    fn default() -> Self {
        BvhNode::new()
    }
}
unsafe impl Send for BvhNode {}
unsafe impl Sync for BvhNode {}

impl<'a> Hittable for Bvh {
    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        Some(self.bb.clone())
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // does it hit me?
        if self.bb.hit(r, t_min, t_max).is_some() {
            // does it hit the left branch?
            let lefthit = (*self.p_left).as_ref().hit(r, t_min, t_max);
            let righthit = (*self.p_right).as_ref().hit(r, t_min, t_max);
            if lefthit.is_some() && righthit.is_some() {
                if lefthit.clone().unwrap().t < righthit.clone().unwrap().t {
                    return lefthit;
                } else {
                    return righthit;
                }
            } else if lefthit.is_some() {
                return lefthit;
            } else if righthit.is_some() {
                return righthit;
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

mod test {
    #[allow(unused_imports)]
    use crate::util::{random_scene};
    #[allow(unused_imports)]
    use super::super::color_to_texture;
    #[allow(unused_imports)]
    use crate::prelude::BvhNode;
    #[allow(unused_imports)]
    use rand::Rng;
    #[allow(unused_imports)]
    use crate::prelude::{
        BoundingBox,
        Sphere,
        HitList,
        Point3,
        Vec3,
        MaterialType,
        Lambertian,
        Color,
        Ray,
        HitRecord,
        Hitters,
        Bvh,
    };
    #[allow(unused_imports)]
    use crate::prelude::Hittable;
    #[allow(unused_imports)]
    use super::super::vect;
    #[allow(unused_imports)]
    use crate::aabb::AabbF;

    #[test]
    fn test_build_bvh() {
        let mut rng = rand::thread_rng();
        let mut world = std::sync::Arc::new(random_scene(&mut rng, false));

        let mut bvh = BvhNode::default();
        bvh.add_hitlist(& mut world, 0., 0.);

        assert_ne!(bvh.bb.bounding_box(0., 0.), BoundingBox::default().bounding_box(0., 0.));
        // shows that it's empty, as it should be. Was just for visual verification
        // println!("The default Bounding box looks like: {}", BoundingBox::default());
        // Spent some time looking at this results were
        // "The BoundingBox after adding hitlist looks like: AabbF::BoundingBox: min(): -1000 -2000
        // -1000 max(): 1000 2 1000"
        // The extreme of the values, as well as the exactness of the led me to investigate.
        // As part of the random_scene, the surface they all rest on is a sphere at (0, -1000, 0)
        // with radius 1000. So it goes as high as y=0, and spans x from -1000 to 1000,
        // and z from -1000 to 1000. The "2" is from a specific sphere that is placed
        // in the scene. So, the external bounding appears to be correct
        //println!("The BoundingBox after adding hitlist looks like: {}", bvh.bounding_box(0., 0.).unwrap());
        let mut bb = AabbF::default();
        bb.minimum = vect!(-1000, -2000, -1000);
        bb.maximum = vect!(1000, 2, 1000);
        let bb = BoundingBox::AabbF(bb);
        assert_eq!(bvh.bounding_box(0., 0.), bb.bounding_box(0., 0.)); 
    }

    #[test]
    fn test_hit_bvh() {
       // taken from test_sphere_hit
        let pt1 = Point3::new(0.0, 0.0, 0.0);
        let pt2 = Point3::new(1.0, 1.0, 1.0);
        let l = MaterialType::Lambertian(Lambertian::new(&color_to_texture!(&Color::new(
            0.1, 0.8, 0.1
        ))));
        let r = Ray::new(&pt1, &pt2, None);
        let c0 = Point3::new(2.0, 2.0, 2.0);
        let c1 = Point3::new(3.0, 3.0, 3.0);
        let c2 = Point3::new(1.0, 1.0, 1.0);
        let c3 = Point3::new(-5.0, -32.0, 27.0);
        let radius = 3.0;

        let s0 = Sphere::new(&c0, radius, l.clone());
        let s1 = Sphere::new(&c1, radius, l.clone());
        let s2 = Sphere::new(&c2, radius, l.clone());
        let s3 = Sphere::new(&c3, radius, l.clone());

        let hitrec = HitRecord {
            t: 0.26794919243112264,
            p: Vec3 {
                x: 0.26794919243112264,
                y: 0.26794919243112264,
                z: 0.26794919243112264,
            },
            normal: Vec3 {
                x: -0.5773502691896258,
                y: -0.5773502691896258,
                z: -0.5773502691896258,
            },
            front_face: false,
            texture_coord: None,
            material: l.clone(),
        };
        let mut hl = HitList::new();
        hl.add(Hitters::Sphere(s0));
        hl.add(Hitters::Sphere(s1));
        hl.add(Hitters::Sphere(s2));
        hl.add(Hitters::Sphere(s3));

        let mut bvh = Bvh::new();
        bvh.add_hitlist(& mut std::sync::Arc::new(hl), 0., 0.);
        bvh.build();
        let hit_or_not = bvh.hit(&r, 0., 0.);
        // this should have 2 hits, but we'll return the closest one
        if hit_or_not.is_some() {
            println!("the result front_face: {}", hit_or_not.as_ref().unwrap().front_face);
            assert_eq!(hit_or_not.as_ref().unwrap().t, hitrec.t);
            println!("the result: {}", hit_or_not.as_ref().unwrap());
        }
        //println!("the result front_face: {}", result.material);

    }
}
