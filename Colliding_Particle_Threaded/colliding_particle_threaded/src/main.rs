use rand;

#[derive(Debug, Copy, Clone)]
struct Particle{
    x: f32,
    y: f32
}

impl Particle{
    fn new(x: f32, y: f32) -> Self
    {
        Particle{x,y}
    }

    fn collide(other_Particle: Particle) -> Self
    {
        // Get position
        
        // Compare distance to other particle
        // Distance < 0.1 collided
    }
}

struct ParticleSystem{
    particles: Vec<Particle>,
    NUM_PARTICLES: usize,
    MAX_X: f32,
    MAX_Y: f32,
    MIN_X: f32,
    MIN_Y: f32
}

impl ParticleSystem{
    fn new() -> Self{
        ParticleSystem{
            particles: Vec::new(),
            NUM_PARTICLES: 100,
            MAX_X: 10.0,
            MAX_Y: 10.0,
            MIN_X: 0.0,
            MIN_Y: 0.0
        }
    }

    fn create_particles(&mut self)
    {
        for i in 0..self.NUM_PARTICLES
        {
            self.particles.push(Particle::new(0.0,0.0));
        }
    }

    fn move_particles(&mut self)
    {
        const NUM_OF_THREADS: usize = 4;
        const NUM_OF_CHUNKS: usize = 25;
        let mut pool = scoped_threadpool::Pool::new(NUM_OF_THREADS as u32);
        pool.scoped(|scope| {
            for slice in self.particles.chunks_mut(NUM_OF_CHUNKS)
            {
                for _ in 0..375000
                {
                    scope.execute(move || {
                        thread_main(slice, 10.0);
                    });
                }
            }
        });
    }

}



fn main() {

    let mut ps = ParticleSystem::new();
    ps.create_particles();

    for p in &ps.particles
    {
        println!("Particle: x: {}, y: {}", p.x, p.y);
    }

    ps.move_particles();

    for p in &ps.particles
    {
        println!("Particle: x: {}, y: {}", p.x, p.y);
    }
}

fn thread_main(list: &mut [Particle],enclosure_size:f32)
{
    for p in list
    {
        let xMove = rand::random::<f32>();
        let yMove = rand::random::<f32>();
        p.x += xMove;
        p.y += yMove;
        if p.x > enclosure_size
        {
            p.x = 0.0;
        }
        if p.x < 0.0
        {
            p.x = enclosure_size;
        }
        if p.y > enclosure_size
        {
            p.y = 0.0;
        }
        if p.y < 0.0
        {
            p.y = enclosure_size;
        }
    }


}