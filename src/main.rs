use std::{ops::Shr, collections::HashMap};

mod world;

fn unconcat(xy: &usize) -> (usize, usize) {
    (xy.shr(16) & 0xffff as usize, xy & 0xffff)  
}

fn read_from_flat2<T: Clone>(v: Vec<T>, x: usize, y: usize, y_max: usize) -> T {
    v[x*y_max + y].clone()
}

fn main() -> Result<(), serde_json::Error>{
    let metadata: HashMap<String, usize> = serde_json::from_str(include_str!("/mnt/h/t/metadata.json")).unwrap();
    let num_groups: usize = serde_json::from_str(include_str!("/mnt/h/t/numGroups.json")).unwrap();
    let is_group_toggle_only: Vec<bool> = serde_json::from_str(include_str!("/mnt/h/t/groupToggleable.json")).unwrap();
    let toggleable: Vec<Vec<usize>> = serde_json::from_str(include_str!("/mnt/h/t/toggleable.json")).unwrap();
    let triggerable: Vec<Vec<usize>> = serde_json::from_str(include_str!("/mnt/h/t/triggerable.json")).unwrap();

    for group in 0..num_groups {
        let toggles: Vec<(usize, usize)> = toggleable[group].clone().iter().map(unconcat).collect();
        let triggers: Vec<(usize, usize)> = triggerable[group].clone().iter().map(unconcat).collect();
        if is_group_toggle_only[group] && !triggers.is_empty() {
            println!("waterfuck? {}", group)
        }
    }
    Ok(())
}
