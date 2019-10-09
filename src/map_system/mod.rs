use std::fs::File;
use std::io::BufReader;
use super::components;
use specs::Builder;

// the supplied position here describes the top left corner of the map
pub fn load_map(map_file : &File , position : &na::Vector2<f32>, world : &mut specs::World,tiles_in_row : i32,tile_width : i32,tile_height : i32) {
    let reader = BufReader::new(map_file);
    let deserialize : serde_json::Value  = serde_json::from_reader(reader).unwrap();
    let the_map_array = (deserialize["layers"][0]["data"].as_array().unwrap());
    for x in 0 .. the_map_array.len() {
        if the_map_array.get(x).unwrap() != 0  {
            let ypos = ( position.y - tile_height as f32 / 2.0 )-((x as f32  / tile_height as f32).floor() * tile_height as f32);
            let remainingxpos = (x as f32 / tiles_in_row as f32).floor() * tile_width as f32;
            let xpos = (position.x + tile_width as f32 / 2.0) + ( x as f32 - remainingxpos) * tile_width as f32;
            println!("your position : {:?},{:?}",xpos,ypos);
            let index = (the_map_array.get(x).unwrap().as_f64().unwrap() as f32 - 1.0f32);
            let x_coord = (index  as f32 % 16.0 as f32).floor()  * tile_width as f32;
            let y_coord = (index as f32 / 16.0 as f32).floor() * tile_height as f32;
            println!("the coords : {:?},{:?}",x_coord,y_coord);
            world.create_entity().with(components::Transform {
                position: (na::Vector3::<f32>::new(xpos, ypos , 0.0)),
                rotation: 0.0,
                scale: (na::Vector3::<f32>::new(tile_width as f32 / 2.0,tile_height as f32 / 2.0, 0.0))
            }).with(components::Sprite { tex_id: 2 ,  coords : na::Vector4::<f32>::new(x_coord,y_coord,32.0,32.0)}).with(components::Collider {
                body_handle: -1,
                position:  (na::Vector3::<f32>::new(xpos, ypos , 0.0)),
                rotation: 0.0,
                scale: (na::Vector3::<f32>::new(tile_width as f32 / 2.004,tile_height as f32 / 2.004, 0.0)),
                vel: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
                slope: false,
            }).build();


        }
    }
}