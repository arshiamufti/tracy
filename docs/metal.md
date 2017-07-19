# Metal

At this point of the raytracer, I can render objects (spheres) of a certain
diffuse material. To add other materials like metals, dielectrics, etc, we can
take the object oriented approach and have a `material` interface.

**What does the material need to do?**

Given (1) a ray of light (2) a hit record, it needs to produce a scattered ray
(or say that it fully absorbed the incident ray). If the ray scattered, it
should express how much the ray should be attenuated (attenuation = reduction in
intensity). Call this function `scatter`.

Previously, the hit record contained the point of intersection `p`, the
normal at `p`, and the `t` value for which the ray hit the surface. Now, we
can add a `material` field (pointer) to this record, which indicates, for a
"world" (which could contain many different types of materials), the type
of material that the ray of light hits. Call this augmented hit record
`HitRecord1` (because I want to preserve the old `HitRecord`).

- Given a ray and a world of hitable surfaces, check if if the ray hits the
  world.
- If it does, a `HitRecord1` is created.
- Given that `HitRecord1`, check how that material scatters the ray (using
  `scatter`), getting the scattered ray and attentuation back, which
we use to render colour!
