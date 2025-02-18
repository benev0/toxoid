use toxoid_api::*;
use toxoid_render::Renderer2D;
use toxoid_sokol::{bindings::*, SokolRenderer2D};
use crate::prefabs::create_render_target; 

#[no_mangle]
pub extern "C" fn fetch_callback(response: *const sfetch_response_t) {
    let response = unsafe { *response };
    // Get user data / entity 
    // println!("User data: {:?}", unsafe { *user_data });
    // println!("Data: {:?}", response.data.ptr);
    // println!("Data size: {:?}", response.data.size);
    // println!("Failed: {:?}", response.failed);
    let entity_id = unsafe { *(response.user_data as *mut u64) };
    let mut entity = Entity::from_id(entity_id);
    let fetch_request = entity.get::<FetchRequest>();
    let data = unsafe { std::slice::from_raw_parts(response.data.ptr as *const u8, response.data.size) };
    let data = data.to_vec();
    fetch_request.set_data(data);
    entity.remove::<Loading>();
    entity.add::<Loaded>();
}

fn sokol_fetch(path: &str, entity: &mut Entity) {
    // Create fetch description
    let mut sfetch_request: sfetch_request_t = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let path = std::ffi::CString::new(path).unwrap();
    sfetch_request.path = path.as_ptr();
    sfetch_request.channel = 0;
    // TODO: Figure out from server or filesystem what the size of the file is
    // 1 MB to be safe, or determine actual size
    let file_size = 1024 * 1024;
    // PNG buffer - 4.00 KB
    // let file_size = 4_096;
    let buffer = Box::into_raw(
        vec![0u8; file_size]
            .into_boxed_slice()
    );
    sfetch_request.buffer = sfetch_range_t {
        ptr: buffer as *const core::ffi::c_void,
        size: file_size
    };
    sfetch_request.callback = Some(fetch_callback);
    // Store entity in the user data / ctx of request so that
    // we can associate the entity with the fetch request / response
    let entity_id = Box::into_raw(Box::new(entity.get_id()));
    let ptr = entity_id as *mut core::ffi::c_void;
    let size = core::mem::size_of::<u64>();
    sfetch_request.user_data = sfetch_range_t {
        ptr,
        size
    };
    unsafe { sfetch_send(&sfetch_request) };
}

// #[cfg(feature = "spine")]
pub fn bone_animation_loaded(entity: &mut Entity) {
    // Create spine atlas object from loaded atlas data.
    let mut atlas_desc: sspine_atlas_desc = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let atlas = entity.get::<Atlas>();
    let data = atlas.get_data();
    let size = data.len() as usize;
    
    // Convert Vec into Box to keep the data alive
    let data_box = data.into_boxed_slice();
    let data_ptr = Box::into_raw(data_box);
    
    atlas_desc.data = sspine_range {
        ptr: data_ptr as *const std::ffi::c_void,
        size
    };
    
    let spine_atlas = unsafe { sspine_make_atlas(&atlas_desc) };
    atlas.set_atlas(Box::into_raw(Box::new(spine_atlas)) as u64);

    // Next create a spine skeleton object, skeleton data files can be either
    // text (JSON) or binary (in our case, 'raptor-pro.skel' is a binary skeleton file).
    // In case of JSON data, make sure that the data is 0-terminated!
    let mut skeleton_desc: sspine_skeleton_desc = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let skeleton = entity.get::<Skeleton>();
    let data = skeleton.get_data();
    skeleton_desc.atlas = spine_atlas;
    
    // Convert Vec into Box to keep the data alive
    let data_box = data.into_boxed_slice();
    let data_ptr = Box::into_raw(data_box);
    
    skeleton_desc.json_data = data_ptr as *const i8;
    skeleton_desc.prescale = 1.0;
    skeleton_desc.anim_default_mix = 0.2;

    let spine_skeleton = unsafe { sspine_make_skeleton(&skeleton_desc) };
    skeleton.set_skeleton(Box::into_raw(Box::new(spine_skeleton)) as u64);

    let mut spine_instance_desc: sspine_instance_desc = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    spine_instance_desc.skeleton = spine_skeleton;

    // create a spine instance object, that's the thing that's actually rendered
    let instance = unsafe { sspine_make_instance(&spine_instance_desc) };
    entity.add::<SpineInstance>();
    let instance_component = entity.get::<SpineInstance>();
    instance_component.set_instance(Box::into_raw(Box::new(instance)) as u64);
    instance_component.set_instantiated(true);

    // configure a simple animation sequence

    unsafe { sspine_add_animation(instance, sspine_anim_by_name(spine_skeleton, c_string("idle_down")), 0, true, 0.) };
    unsafe { sspine_set_animation(instance, sspine_anim_by_name(spine_skeleton, c_string("idle_down")), 0, true) };

    let atlas_images_num = unsafe { sspine_num_images(spine_atlas) };

    // load all atlas images
    for img_index in 0..atlas_images_num {
        let img = unsafe { sspine_image_by_index(spine_atlas, img_index) };
        let img_info = unsafe { sspine_get_image_info(img) };
        let filename_c_str = unsafe { core::ffi::CStr::from_ptr(img_info.filename.cstr.as_ptr()) };
        // We'll store the sspine_image handle in the fetch request's user data
        // blob, because we need the image info again later in the fetch callback
        // in order to initialize the sokol-gfx image with the right parameters.
        //      
        // Also important to note: all image fetch requests load their data into the same
        // buffer. This is fine because sokol-fetch has been configured
        // with num_lanes=1, this will cause all requests on the same
        // channel to be serialized (not run in parallel). That way
        // the same buffer can be reused even if there are multiple atlas images.
        // The downside is that loading multiple images would take longer.
        let file_path = format!("assets/animations/{}", filename_c_str.to_str().unwrap());
        let file_path = file_path.as_str();
        let mut image_entity = load_bone_animation_image(file_path);
        image_entity.child_of_id(entity.get_id());
        let image = image_entity.get::<BoneAnimationImage>();
        image.set_info(Box::into_raw(Box::new(img_info)) as u64);
    }

    let game_config = World::get_singleton::<GameConfig>();
    let window_width = game_config.get_window_width();
    let game_width = game_config.get_game_width();
    
    // Set player entity in singleton
    let player_singleton = World::get_singleton::<Player>();
    let player_entity = Entity::from_id(player_singleton.get_entity());

    // Create render target with scaled size
    let scale_factor = window_width as f32 / game_width as f32;
    // TODO: Make this configurable for animations
    let rt_width = (150.0 * scale_factor) as u32;  // Keep original size since we're zooming with camera
    let rt_height = (150.0 * scale_factor) as u32;
    let mut rt_entity = create_render_target(rt_width, rt_height);
    rt_entity.add::<Position>();  // Make sure Position component is added
    rt_entity.add::<Size>();
    let rt_size = rt_entity.get::<Size>();
    rt_size.set_width(rt_width);
    rt_size.set_height(rt_height);
    rt_entity.child_of_id(player_entity.get_id());

    // Make spine instance child of render target
    entity.child_of_id(rt_entity.get_id());
    entity.add::<Blittable>();
}

// Fetch Observers
pub fn init() {
    Observer::dsl("FetchRequest, Loading", vec![Event::OnAdd], |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let fetch_request = entity.get::<FetchRequest>();
            let path = fetch_request.get_path();
            sokol_fetch(&path, entity);
        });
    })
        .build();

    Observer::dsl("FetchRequest, Loaded", vec![Event::OnAdd], |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let fetch_request = entity.get::<FetchRequest>();
            let data = fetch_request.get_data();
            let data_type = fetch_request.get_data_type();
            let size = data.len() as usize;
            match data_type as u8 {
                d if d == DataType::BoneAnimationImage as u8 => {
                    let mut image_entity = Entity::from_id(fetch_request.get_user_data());
                    let image = image_entity.get::<BoneAnimationImage>();
                    let img_info = image.get_info();
                    let img_info = unsafe { &*(img_info as *const sspine_image_info) };
                    let data_box = data.into_boxed_slice();
                    let data_ptr = Box::into_raw(data_box);
                    // Initialize sokol-gfx image object
                    SokolRenderer2D::init_image(img_info.sgimage, data_ptr as *const u8, size);
                    // Initialize sokol-gfx sampler object
                    SokolRenderer2D::init_sampler(
                        img_info.sgsampler,
                        img_info.min_filter,
                        img_info.mag_filter,
                        img_info.mipmap_filter,
                        img_info.wrap_u,
                        img_info.wrap_v,
                        &img_info.filename.cstr as *const _ as *const i8
                    );
                },
                d if d == DataType::Image as u8 => {
                    // Create entity from entity ID passed to user data
                    let mut image_entity = Entity::from_id(fetch_request.get_user_data());
                    // Get data
                    let data_box = data.clone().into_boxed_slice();
                    let data_ptr = Box::into_raw(data_box);
                    // Create sokol image
                    let sokol_image = SokolRenderer2D::create_image(data_ptr as *const u8, size);
                    let image_width = sokol_image.width();
                    let image_height = sokol_image.height();
                    // Set size
                    let size = image_entity.get::<Size>();
                    size.set_width(image_width);
                    size.set_height(image_height);
                    // Set image
                    let image = image_entity.get::<toxoid_api::Image>();
                    image.set_image(Box::into_raw(sokol_image) as *mut () as u64);
                    image.set_path(fetch_request.get_path());
                    image.set_data(data);
                    image_entity.add::<Loaded>();
                }
                d if d == DataType::Sprite as u8 => {
                    // Create entity from entity ID passed to user data
                    let mut sprite_entity = Entity::from_id(fetch_request.get_user_data());
                    // Get data
                    let data_box = data.into_boxed_slice();
                    let data_ptr = Box::into_raw(data_box);
                    // Create sokol sprite
                    let sokol_sprite = SokolRenderer2D::create_sprite(data_ptr as *const u8, size);
                    let sprite_width = sokol_sprite.width();
                    let sprite_height = sokol_sprite.height();
                    // Set size
                    let size = sprite_entity.get::<Size>();
                    size.set_width(sprite_width);
                    size.set_height(sprite_height);
                    // Set sprite
                    let sprite = sprite_entity.get::<Sprite>();
                    sprite.set_sprite(Box::into_raw(sokol_sprite) as *mut () as u64);
                    sprite_entity.add::<Blittable>();
                    // Create render target entity
                    let mut rt_entity = create_render_target(sprite_width, sprite_height);
                    sprite_entity.child_of_id(rt_entity.get_id());
                    // Create renderable entity
                    if sprite_entity.has::<RenderableOnLoad>() {
                        rt_entity.add::<Renderable>();
                    }
                    sprite_entity.add::<Loaded>();
                },
                d if d == DataType::BoneAnimationAtlas as u8 => {
                    let mut animation_entity = Entity::from_id(fetch_request.get_user_data());
                    let atlas = animation_entity.get::<Atlas>();
                    atlas.set_data(data);
                    atlas.set_loaded(true);
                    if animation_entity.get::<Skeleton>().get_loaded() {
                        bone_animation_loaded(&mut animation_entity);
                    }
                },
                d if d == DataType::BoneAnimationSkeleton as u8 => {
                    let mut animation_entity = Entity::from_id(fetch_request.get_user_data());
                    let skeleton = animation_entity.get::<Skeleton>();
                    skeleton.set_data(data);
                    skeleton.set_loaded(true);
                    if animation_entity.get::<Atlas>().get_loaded() {
                        bone_animation_loaded(&mut animation_entity);
                    }
                },
                d if d == DataType::Worldmap as u8 => {
                    let mut world_entity = Entity::from_id(fetch_request.get_user_data());
                    let world = world_entity.get::<TiledWorld>();
                    let data_str = std::str::from_utf8(data.as_slice()).unwrap();
                    let tiled_world = toxoid_tiled::parse_world(data_str);
                    world.set_world(Box::into_raw(Box::new(tiled_world.clone())) as u64);
                    let world_entity_id = world_entity.get_id();
                    
                    tiled_world
                        .maps
                        .unwrap()
                        .iter()
                        .for_each(|cell| {
                            let mut cell_entity = toxoid_api::load_cell(format!("assets/{}", cell.file_name).as_str(), true);
                            cell_entity.child_of_id(world_entity_id);
                            
                            // Set cell position
                            cell_entity.add::<Position>();
                            let cell_pos = cell_entity.get::<Position>();
                            cell_pos.set_x(cell.x);
                            cell_pos.set_y(cell.y);
                            
                            // Set cell size
                            cell_entity.add::<Size>();
                            let cell_size = cell_entity.get::<Size>();
                            cell_size.set_width(cell.width);
                            cell_size.set_height(cell.height);

                            // Add blittable component
                            cell_entity.add::<Blittable>();

                            // Parent cell to player
                            let player_singleton = World::get_singleton::<Player>();
                            let mut player_entity = Entity::from_id(player_singleton.get_entity());
                            player_entity.child_of_id(cell_entity.get_id());
                        });
                },
                d if d == DataType::Cell as u8 => {
                    let mut cell_entity = Entity::from_id(fetch_request.get_user_data());
                    let cell = cell_entity.get::<TiledCell>();
                    let data_str = std::str::from_utf8(data.as_slice()).unwrap();
                    let tiled_cell = toxoid_tiled::parse_cell(data_str);
                    
                    // Add Size component with map dimensions
                    cell_entity.add::<Size>();
                    let size = cell_entity.get::<Size>();
                    let map_width = tiled_cell.width * tiled_cell.tilewidth;
                    let map_height = tiled_cell.height * tiled_cell.tileheight;
                    size.set_width(map_width);
                    size.set_height(map_height);
                    
                    cell.set_cell(Box::into_raw(Box::new(tiled_cell.clone())) as u64);
                    let tileset = tiled_cell.tilesets.get(0).unwrap();
                    let mut tileset_entity = toxoid_api::load_tileset(format!("assets/{}", tileset.image.as_str()).as_str(), true);
                    // cell_entity.add_relationship(Relationship::Custom(TilesetRelationship::get_id()), tileset_entity);
                    tileset_entity.child_of_id(cell_entity.get_id());
                    cell_entity.add::<Blittable>();
                    // let mut tileset = tileset_entity.get::<Tileset>();
                    // tileset.set_tileset(Box::into_raw(Box::new(tileset.clone())) as u64);
                },
                d if d == DataType::Tileset as u8 => {
                    let mut tileset_entity = Entity::from_id(fetch_request.get_user_data());
                    // Convert Vec into Box to keep the data alive while creating the sprite
                    let data_box = data.into_boxed_slice();
                    let data_ptr = Box::into_raw(data_box);
                    // Create sokol sprite using the boxed data
                    let sokol_sprite = SokolRenderer2D::create_sprite(data_ptr as *const u8, size);
                    // Set size
                    let size = tileset_entity.get::<Size>();
                    size.set_width(sokol_sprite.width());
                    size.set_height(sokol_sprite.height());
                    // Set sprite
                    let sprite = tileset_entity.get::<Sprite>();
                    sprite.set_sprite(Box::into_raw(sokol_sprite) as *mut () as u64);
                    tileset_entity.add::<Blittable>();
                },
                _ => {
                    println!("File with unknown data type fetched: {:?}", data_type);
                }
            }

        });
    })
        .build();
}

