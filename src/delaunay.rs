#[path = "geometry.rs"] mod geometry;
use geometry::Point2d as Point2d;
use geometry::Line as Line;
use geometry::Triangle as Triangle;
use egui::Pos2;

fn bowyer_watson(pts: Vec<Point2d>) -> Vec<Triangle> {
    let mut triangulation: Vec<Triangle> = Vec::new();
    let super_triangle = Triangle {
        pts: [
            Point2d {
                x: 1000.0,
                y: 1000.0
            },
            Point2d {
                x: 1000.0,
                y: -1000.0
            },
            Point2d {
                x: -1000.0,
                y: 0.0
            }
        ]
    };
    
    let add_point = |mut triangulation: Vec<Triangle>, pt: &Point2d| -> Vec<Triangle> {

        let mut edges: Vec<Line> = Vec::new();
        
        triangulation = triangulation.iter()
            .filter(|t| {
                !t.within_circumcircle(*pt)
            })
            .map(|x| *x)
            .collect();
        triangulation.iter()
            .for_each(|t| edges.extend(t.get_edges()));

        triangulation.push(super_triangle);
        edges.extend(super_triangle.get_edges());

        let unique_edges: Vec<&Line> = edges.iter()
                .filter(|e| edges.iter().filter(|n| n == e).count() == 1)
                .collect();
        
        unique_edges.iter()
            .for_each(|e| {
                triangulation.push(Triangle {
                    pts: [e.pt1, e.pt2, *pt]
                });
            });

        triangulation
    };

    triangulation = pts.iter()
            .fold(triangulation, add_point).into_iter()
            .filter(|t| !t.pts.contains(&super_triangle.pts[0]))
            .filter(|t| !t.pts.contains(&super_triangle.pts[1]))
            .filter(|t| !t.pts.contains(&super_triangle.pts[2]))
            .collect::<Vec<_>>();

    triangulation
}

pub fn export_boyer_watson(pts: Vec<Pos2>) -> Vec<Vec<Pos2>> {
    let bowyer_watson_triangulation: Vec<Triangle> = bowyer_watson(pts.iter().map(|pt| Point2d::from_pos2(*pt)).collect());
    bowyer_watson_triangulation.iter()
        .map(|t| 
            t.get_edges().iter().map(|e| e.get_pts_as_vec()).flatten().collect()).collect()
}