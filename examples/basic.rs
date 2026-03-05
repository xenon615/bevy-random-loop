
  use bevy::{
      camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
      prelude::*
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
          Transform::from_xyz(0., -30., 0.1).looking_at(Vec3::ZERO , Vec3::Y),
      ));



      let predefined = vec![(1., 0.), (-0.2, -1.), (-0.5, 1.), (0.5, 1.), (-1., 0.,), (-0.8, 0.5), (0.8, 0.5),  (0.8, -0.5), (-1., 0.8) ]
          .iter()
          .map(| x | vec3(x.0, 0., x.1) * 10.)
          .collect::<Vec<_>>();

      let point_m = meshes.add(Sphere::new(0.5));
      for p in &predefined {
          cmd.spawn((
              Transform::from_translation(*p),
              Mesh3d(point_m.clone()),
              MeshMaterial3d(materials.add(StandardMaterial{
                  emissive: LinearRgba::new(10., 0., 10., 0.1),
                  alpha_mode: AlphaMode::Blend,
                  ..default()
              }))
          ));
      }

      // let rpath = RandomPath::new(12, vec3(10., 0., 10.)).generate_convex_hull();
      let rpath = RandomPath::from_predefined(&predefined).generate_convex_hull();
      cmd.spawn((
          Mesh3d(meshes.add(Polyline3d::new(rpath.clone()))),
          MeshMaterial3d(materials.add(StandardMaterial{
              emissive: LinearRgba::rgb(0., 0., 10.),
              ..default()
          }))
      ));



      let rpath = RandomPath::vary_convex_hull(&rpath);
      cmd.spawn((
          Mesh3d(meshes.add(Polyline3d::new(rpath.clone()))),
          MeshMaterial3d(materials.add(StandardMaterial{
              emissive: LinearRgba::rgb(10., 0., 0.),
              ..default()
          }))
      ));


      let cr = CubicCardinalSpline::new(0.8, rpath).to_curve().unwrap();
      let spline = cr.iter_positions(120).collect::<Vec<_>>();
      cmd.spawn((
          Mesh3d(meshes.add(Polyline3d::new(spline))),
          MeshMaterial3d(materials.add(StandardMaterial{
              emissive: LinearRgba::rgb(0., 10., 0.),
              ..default()
          }))
      ));


  }
