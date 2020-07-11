use crate::config::Float;
use crate::vec3;
use vec3::{Vec3, Color};
use crate::ray::{Ray,Hit};

fn reflect(v_in: &Vec3, normal: &Vec3) -> Vec3
{
    *v_in - 2.0 * vec3::dot(v_in, normal) * *normal
}

// If total reflection, return None.
fn refract(v_in: &Vec3, normal: &Vec3, ni_over_nt: Float) -> Option<Vec3>
{
    let uv = Vec3::unit(v_in);
    let dt = vec3::dot(&uv, normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0
    {
        Some(ni_over_nt * (uv - (*normal)*dt) - (*normal) * discriminant.sqrt())
    }
    else
    {
        None
    }
}

fn schlick(cos: Float, ref_idx: Float) -> Float
{
    let r0: Float = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r02 = r0 * r0;
    r02 + (1.0 - r02) * (1.0 - cos).powi(5)
}

pub trait Material
{
    // Return (ray, attenuation).
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian
{
    pub albedo: Vec3,
}

impl Material for Lambertian
{
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)>
    {
        let target = hit.p + hit.normal + Vec3::randInUnitSphere();
        Some((Ray { origin: hit.p, dir: target - hit.p }, self.albedo))
    }
}

pub struct Metal
{
    pub albedo: Vec3,
    pub roughness: Float,
}

impl Material for Metal
{
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)>
    {
        let reflected = reflect(&(Vec3::unit(&r_in.dir)), &hit.normal);
        let scattered = Ray
        {
            origin: hit.p,
            dir: self.roughness * Vec3::randInUnitSphere() + reflected,
        };

        if vec3::dot(&scattered.dir, &hit.normal) > 0.0
        {
            return Some((scattered, self.albedo));
        }
        else
        {
            return None;
        }
    }
}

pub struct Glass
{
    pub ref_index: Float,
}

impl Material for Glass
{
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)>
    {
        let mut ref_normal = Vec3::origin();
        let mut ni_over_nt: Float = 0.0;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut cos: Float;

        if vec3::dot(&r_in.dir, &hit.normal) > 0.0
        {
            ref_normal = -(hit.normal);
            ni_over_nt = self.ref_index;
            cos = self.ref_index * vec3::dot(&r_in.dir, &hit.normal) / r_in.dir.norm();
        }
        else
        {
            ref_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_index;
            cos = -vec3::dot(&r_in.dir, &hit.normal) / r_in.dir.norm();
        }

        let mut reflect_prob = schlick(cos, self.ref_index);
        if let Some(refracted) = refract(&r_in.dir, &ref_normal, ni_over_nt)
        {
            if rand::random::<Float>() < reflect_prob
            {
                let reflected = reflect(&r_in.dir, &hit.normal);
                return Some((Ray{origin: hit.p, dir: reflected}, attenuation));
            }
            else
            {
                return Some((Ray{origin: hit.p, dir: refracted}, attenuation));
            }
        }
        else
        {
            let reflected = reflect(&r_in.dir, &hit.normal);
            return Some((Ray{origin: hit.p, dir: reflected}, attenuation));
        }
    }
}

pub struct Null {}

impl Material for Null
{
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)>
    {
        None
    }
}

pub static NULL: Null = Null{};
