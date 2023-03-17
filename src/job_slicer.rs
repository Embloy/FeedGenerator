use std::fmt::{Debug, Formatter};
use rstar::{Point, RTree};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Job {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

impl Copy for Job {}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("id", &self.id)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl Point for Job {
    type Scalar = f32;

    const DIMENSIONS: usize = 2;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        Job {
            id: 0,
            x: generator(0),
            y: generator(1),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid index {}", index),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        todo!()
    }
}



pub struct JobSlicer {
    rtree: RTree<Job>,
    next_job_id: u32,
    slice_size: usize,
}

impl JobSlicer {
    fn initialize(&mut self, jobs: Vec<Job>, slice_size: usize) {
        self.rtree = RTree::bulk_load(jobs);
        self.slice_size = slice_size;
        self.next_job_id = 1;
    }

    fn fill(&mut self, jobs: Vec<Job>) {
        for job in jobs {
            self.insert(job);
        }
    }

    fn insert(&mut self, job: Job) {
        let job_with_id = Job {
            id: self.next_job_id,
            x: job.x,
            y: job.y,
        };
        self.next_job_id += 1;
        self.rtree.insert(job_with_id);
    }

    fn remove(&mut self, id: u32) -> Option<Job> {
        let job = self.rtree.remove(&Job { id, x: 0.0, y: 0.0 })?;
        Some(Job {
            id: job.id,
            x: job.x,
            y: job.y,
        })
    }

    fn slice(&self) -> Vec<Vec<Job>> {
        let mut slices = Vec::new();
        let mut i = 0;
        while i < self.rtree.size() {
            let slice_end = std::cmp::min(i + self.slice_size, self.rtree.size());
            let slice = self.rtree
                .iter()
                .skip(i)
                .take(slice_end - i)
                .cloned()
                .collect();
            slices.push(slice);
            i += self.slice_size;
        }
        slices
    }
    fn print_nodes(&self) {
        for node in self.rtree.iter() {
            println!("{:?}", node);
        }
    }

    fn get_points(&self) -> Vec<Job> {
        self.rtree.iter().cloned().collect()
    }

    fn print_jobs(job_slicer: &JobSlicer) {
        for job in job_slicer.get_points() {
            println!("Job {}: ({}, {})", job.id, job.x, job.y);
        }
    }
}

pub fn main() {
    println!("STARTED SLICER");
    let mut job_slicer = JobSlicer {
        rtree: RTree::new(),
        next_job_id: 1,
        slice_size: 5,
    };
    job_slicer.initialize(vec![
        Job { id: 1, x: 0.0, y: 0.0 },
        Job { id: 2, x: 1.0, y: 0.0 },
        Job { id: 3, x: 5.0, y: 2.0 }], 0);
    job_slicer.print_nodes();
    println!("FINISHED SLICER");
    return;
}