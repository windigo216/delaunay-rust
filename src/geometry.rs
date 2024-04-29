

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

use egui::Pos2;
use egui::pos2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64
}

impl Sub for Point2d {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f64> for Point2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Point2d {x: self.x * rhs, y: self.y * rhs}
    }
}

impl Point2d {
    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn magnitude(self) -> f64 {
        self.dot(self).sqrt()
    }

    fn to_pos2(self) -> Pos2 {
        pos2(self.x as f32, self.y as f32)
    }

    pub fn from_pos2(other: Pos2) -> Point2d{
        Point2d {x: other.x as f64, y: other.y as f64}
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    m: f64,
    pub pt1: Point2d,
    pub pt2: Point2d,
}

impl Line {
    pub fn from_points(pt1: Point2d, pt2: Point2d) -> Line {
        let m = if pt1.x == pt2.x {
            std::f64::INFINITY
        } else {
            (pt1.y - pt2.y) / (pt1.x - pt2.x)
        };
        Line {
            m, pt1, pt2
        }
    }

    pub fn from_point_slope(pt: Point2d, m: f64) -> Line {
        Line {
            m, pt1: pt, pt2: pt
        }
    }

    pub fn from_x(self, x: f64) -> Point2d {
        Point2d {
            x,
            y: self.m * (x - self.pt1.x) + self.pt1.y
        }
    }

    pub fn _from_y(self, y: f64) -> Point2d {
        Point2d {
            x: (y - self.pt1.y) / self.m + self.pt1.x,
            y: y
        }
    }

    pub fn perp_bisector(self) -> Line {

        Line::from_point_slope((self.pt1 + self.pt2) * 0.5, if self.m == 0. {
            std::f64::INFINITY
        } else {
            -1./self.m
        })
            
    }

    pub fn intersect(self, other: Self) -> Point2d {
        if self.m == other.m {
            panic!("intersect: the two Lines have equivalent slope")            
        } else {
            if self.m == std::f64::INFINITY {
                other.from_x(self.pt1.x)
            } else if other.m == std::f64::INFINITY {
                self.from_x(other.pt1.x)
            } else {
                let x = (other.m * other.pt1.x - self.m * self.pt1.x + self.pt1.y - other.pt1.y) / (other.m - self.m);
                self.from_x(x)
            }
        }
    }

    pub fn get_pts_as_vec(self) -> Vec<Pos2> {
        vec![self.pt1.to_pos2(), self.pt2.to_pos2()]
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        if self.m == std::f64::INFINITY && other.m == std::f64::INFINITY {
            true
        } else {
            self.m == other.m && other.from_x(self.pt1.x).y == self.pt1.y
        }
        
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub pts: [Point2d; 3]
}

impl Triangle {
    pub fn circumcenter(self) -> Point2d {
        let p1 = self.pts[0];
        let p2 = self.pts[1];
        let p3 = self.pts[2];

        let l1 = Line::from_points(p1, p2);
        let l2 = Line::from_points(p2, p3);

        l1.perp_bisector().intersect(l2.perp_bisector())
    }

    pub fn circumradius(self) -> f64 {
        // TODO: make this set a property in struct
        (self.pts[0] - self.circumcenter()).magnitude()
    }

    // TODO: https://ianthehenry.com/posts/delaunay/ UPDATE BELOW WITH THIS
    pub fn within_circumcircle(self, pt: Point2d) -> bool {
        (pt - self.circumcenter()).magnitude() < self.circumradius()
    }

    pub fn get_edges(self) -> [Line; 3] {
        [Line::from_points(self.pts[0], self.pts[1]),
        Line::from_points(self.pts[1], self.pts[2]),
        Line::from_points(self.pts[2], self.pts[0])]
    }
}
