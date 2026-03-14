
use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin}, prelude::*
};
use bevy_random_path::RandomPath;

fn main () {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Camera3d::default(),
        Camera::default(),
        FreeCamera::default(),
        Transform::from_xyz(0., -300., 0.1).looking_at(Vec3::ZERO , Vec3::Y),
    ));

    // let predefined = vec![(1., 0.), (-0.2, -1.), (-0.5, 1.), (0.5, 1.), (-1., 0.,), (-0.8, 0.5), (0.8, 0.5),  (0.8, -0.5), (-1., 0.8) ]
    //     .iter()
    //     .map(| x | vec3(x.0, 0., x.1) * 10.)
    //     .collect::<Vec<_>>();

    // let mut rpath = RandomPath::from_predefined(&predefined).generate();

    let mut rpath = RandomPath::new(12, vec3(100., 0., 100.)).generate();
    // for rpe in &rpath {
    //     print!("{:?}, ", rpe);
    // }
    // println!("");
    // let mut rpath = vec![
    //     vec3(12.34076, 0.0, -99.92538), vec3(59.4512, 0.0, -35.786034), vec3(2.3444176, -0.0, 93.42969), vec3(-10.13267, -0.0, 96.50256),
    //     vec3(-90.65795, -0.0, 53.471207), vec3(8.522677, -0.0, -98.371124), vec3(11.38624, 0.0, -99.53681),
    // ];



    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(rpath.clone()))),
        MeshMaterial3d(materials.add(StandardMaterial{
            emissive: LinearRgba::rgb(0., 0., 10.),
            ..default()
        }))
    ));

    RandomPath::vary(&mut rpath, 50.);
    // cmd.spawn((
    //     Mesh3d(meshes.add(Polyline3d::new(rpath.clone()))),
    //     MeshMaterial3d(materials.add(StandardMaterial{
    //         emissive: LinearRgba::rgb(10., 10., 0.),
    //         ..default()
    //     }))
    // ));


    RandomPath::smooth_out(&mut rpath, 120f32.to_radians(), 20.);

    // ------------------------------------

    // let last =  rpath.len() - 1;
    // let mut vec1 = Vec3::ZERO;
    // let mut vec2 = Vec3::ZERO;
    // let mut bisec = Vec3::ZERO;

    // for _k in 0 .. 100 {
    //     vec1 = (rpath[last - 1 ] - rpath[last]).normalize();
    //     vec2 = (rpath[1] - rpath[0]).normalize();
    //     let angle = vec1.dot(vec2).acos().to_degrees();
    //     if angle > 120. {
    //         break;
    //     }
    //     println!("{}", angle);


    //     bisec = (vec1 + vec2).normalize();
    //     rpath[0] += bisec * 0.5;
    //     rpath[last] += bisec * 0.5;
    // }

    // let a1 = vec![rpath[last] , rpath[last] + vec1 * 10.];
    // let a2 = vec![rpath[0] , rpath[0] + vec2 * 10.];
    // let a3 = vec![rpath[0] , rpath[0] + bisec * 20.];
    // let a4 = vec![rpath[last] , rpath[last] + bisec * 20.];

    // let test_mat = materials.add(StandardMaterial {
    //     emissive: LinearRgba::rgb(10., 10., 10.),
    //     ..default()
    // });
    // cmd.spawn((
    //     Mesh3d(meshes.add(Polyline3d::new(a1))),
    //     MeshMaterial3d(test_mat.clone())
    // ));

    // cmd.spawn((
    //     Mesh3d(meshes.add(Polyline3d::new(a2))),
    //     MeshMaterial3d(test_mat.clone())
    // ));

    // cmd.spawn((
    //     Mesh3d(meshes.add(Polyline3d::new(a3))),
    //     MeshMaterial3d(test_mat.clone())
    // ));

    // cmd.spawn((
    //     Mesh3d(meshes.add(Polyline3d::new(a4))),
    //     MeshMaterial3d(test_mat.clone())
    // ));


// ----------------------------------
    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(rpath.clone()))),
        MeshMaterial3d(materials.add(StandardMaterial{
            emissive: LinearRgba::rgb(10., 0., 0.),
            ..default()
        }))
    ));


    // return;
    let cr = CubicBSpline::new(rpath).to_curve_cyclic().unwrap();
    let spline = cr.iter_positions(120).collect::<Vec<_>>();


    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(spline))),
        MeshMaterial3d(materials.add(StandardMaterial{
            emissive: LinearRgba::rgb(0., 10., 0.),
            ..default()
        }))
    ));
}
