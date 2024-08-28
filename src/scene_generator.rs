use nalgebra::Vector3;

use crate::image::Image;

pub(crate) fn three_cylinders() -> Vec<Image> {
    let up = Vector3::new(0.0, 1.0, 0.0);
    let focus = Vector3::new(0.0, 0.0, 0.0);
    let height_angle = 30.0;
    let width = 1024;
    let height = 768;
    let image_0 = Image::new_from_file(
        "./data/input/cylinder/three_3,3,3.png".to_owned(),
        Vector3::new(3.0, 3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_1 = Image::new_from_file(
        "./data/input/cylinder/three_-3,-3,-3.png".to_owned(),
        Vector3::new(-3.0, -3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_2 = Image::new_from_file(
        "./data/input/cylinder/three_3,3,3.png".to_owned(),
        Vector3::new(3.0, 3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_3 = Image::new_from_file(
        "./data/input/cylinder/three_-3,-3,-3.png".to_owned(),
        Vector3::new(-3.0, -3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_4 = Image::new_from_file(
        "./data/input/cylinder/three_side.png".to_owned(),
        Vector3::new(5.0, 0.0, 0.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_5 = Image::new_from_file(
        "./data/input/cylinder/three_side.png".to_owned(),
        Vector3::new(-5.0, 0.0, 0.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_6 = Image::new_from_file(
        "./data/input/cylinder/three_side.png".to_owned(),
        Vector3::new(0.001, 5.0, 0.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_7 = Image::new_from_file(
        "./data/input/cylinder/three_side.png".to_owned(),
        Vector3::new(0.001, -5.0, 0.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_8 = Image::new_from_file(
        "./data/input/cylinder/three_side.png".to_owned(),
        Vector3::new(0.0, 0.0, 5.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_9 = Image::new_from_file(
        "./data/input/cylinder/three_side.png".to_owned(),
        Vector3::new(0.0, 0.0, -5.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    vec![image_0, image_1, image_2, image_3, image_4, image_5, image_6, image_7, image_8, image_9]
}

pub(crate) fn cone() -> Vec<Image> {
    let up = Vector3::new(0.0, 1.0, 0.0);
    let focus = Vector3::new(0.0, 0.0, 0.0);
    let height_angle = 30.0;
    let width = 1024;
    let height = 768;
    let image_0 = Image::new_from_file(
        "./data/input/cone/cone_3,3,3.png".to_owned(),
        Vector3::new(3.0, 3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_1 = Image::new_from_file(
        "./data/input/cone/cone_0,3,3.png".to_owned(),
        Vector3::new(0.0, 3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_2 = Image::new_from_file(
        "./data/input/cone/cone_-3,3,3.png".to_owned(),
        Vector3::new(-3.0, 3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_3 = Image::new_from_file(
        "./data/input/cone/cone_-3,3,-3.png".to_owned(),
        Vector3::new(-3.0, 3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_4 = Image::new_from_file(
        "./data/input/cone/cone_3,3,-3.png".to_owned(),
        Vector3::new(3.0, 3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_5 = Image::new_from_file(
        "./data/input/cone/cone_3,-05,0.png".to_owned(),
        Vector3::new(3.0, -0.5, 0.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_6 = Image::new_from_file(
        "./data/input/cone/cone_-3,-05,0.png".to_owned(),
        Vector3::new(-3.0, -0.5, 0.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_7 = Image::new_from_file(
        "./data/input/cone/cone_0,-05,3.png".to_owned(),
        Vector3::new(0.0, -0.5, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_8 = Image::new_from_file(
        "./data/input/cone/cone_0,-05,-3.png".to_owned(),
        Vector3::new(0.0, -0.5, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    vec![image_0, image_1, image_2, image_3, image_4]//, image_5, image_6, image_7, image_8]
}

pub(crate) fn two_cones() -> Vec<Image> {
    let up = Vector3::new(0.0, 1.0, 0.0);
    let focus = Vector3::new(0.0, 0.0, 0.0);
    let height_angle = 30.0;
    let width = 1024;
    let height = 768;
    let image_0 = Image::new_from_file(
        "./data/input/two_cones/cones_3,3,3.png".to_owned(),
        Vector3::new(3.0, 3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_1 = Image::new_from_file(
        "./data/input/two_cones/cones_0,3,3.png".to_owned(),
        Vector3::new(0.0, 3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_2 = Image::new_from_file(
        "./data/input/two_cones/cones_-3,3,3.png".to_owned(),
        Vector3::new(-3.0, 3.0, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_3 = Image::new_from_file(
        "./data/input/two_cones/cones_3,3,-3.png".to_owned(),
        Vector3::new(3.0, 3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_4 = Image::new_from_file(
        "./data/input/two_cones/cones_0,3,-3.png".to_owned(),
        Vector3::new(0.0, 3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_5 = Image::new_from_file(
        "./data/input/two_cones/cones_-3,3,-3.png".to_owned(),
        Vector3::new(-3.0, 3.0, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_6 = Image::new_from_file(
        "./data/input/two_cones/cones_0,-05,3.png".to_owned(),
        Vector3::new(0.0, -0.5, 3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_7 = Image::new_from_file(
        "./data/input/two_cones/cones_3,-05,0.png".to_owned(),
        Vector3::new(3.0, -0.5, 0.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_8 = Image::new_from_file(
        "./data/input/two_cones/cones_0,-05,-3.png".to_owned(),
        Vector3::new(0.0, -0.5, -3.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    let image_9 = Image::new_from_file(
        "./data/input/two_cones/cones_-3,-05,0.png".to_owned(),
        Vector3::new(-3.0, -0.5, 0.0),
        up,
        focus,
        height_angle,
        width,
        height,
    );
    vec![image_0, image_1, image_2, image_3, image_4, image_5, image_6, image_7, image_8, image_9]
}