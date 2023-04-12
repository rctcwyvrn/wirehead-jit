use std::collections::HashMap;

pub struct GroupId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjId(usize);

pub struct Wires {
    obj_to_point: Vec<(usize, usize)>,
    obj_to_groups: Vec<[GroupId; 4]>,
    // tog or trig
    // is standard lamp?
    obj_to_type: Vec<u8>,
    
    group_to_objs: Vec<Vec<usize>>,
    group_is_all_tog: Vec<bool>,

    // standard lamps
    // sl_to_point: Vec<(usize, usize)>,
    // sl_to_groups: Vec<Vec<GroupId>>,
}

impl Wires { 
    pub fn register_object(&mut self, x: usize, y: usize, groups: [GroupId; 4], tog_or_trig: bool, is_lamp: bool) -> ObjId {
        let id = self.obj_to_point.len();

        self.obj_to_point.push((x,y));
        self.obj_to_groups.push(groups);
        let ty = 0 | if tog_or_trig { 1 } else { 0 } | if is_lamp { 2 } else { 0 };
        self.obj_to_type.push(ty);
        ObjId(id)
    }

    pub fn init(num_groups: usize) -> Wires {
        Wires {
            obj_to_point: Vec::new(),
            obj_to_groups: Vec::new(),
            obj_to_type: Vec::new(),
            group_to_objs: Vec::with_capacity(num_groups),
            group_is_all_tog: Vec::with_capacity(num_groups),
        }
    }
}

// runtime state:
// - group_id -> bool
// - obj_id -> bool
// - stack = Stack<group_id>

// when dumping the world, we gotta save the initial obj state
// not sure how this will eventually work with the mod 
// one thing is for sure: if you add wires after "compilation" then wirehead will just completely ignore it and not know that it exists

// a group is triggered (Wirehead.HitWire is the entrypoint)
//  - the idea is it enters the compiled code and doesn't leave until its done
// enter the bytecode by jumping to the code for that group
// when done
// - parts of the state should get reflected back into the world, but which parts?
// - all of the stuff that changed that isnt a lamp i guess?

// toggleable group code
// - group_toggle(group_id)
// - jump stack.pop()

// non-tog group code
// - for each toggleable obj in group, one obj_toggle(obj_id)
// - for each triggerable obj in group,
//   - if is standard lamp faulty gate, one conditional_push(lamp's obj_id, gate's output group_id)
//   - other cases ??? pixel boxes????
// - jump stack.pop()

// bytecode
// - obj_toggle(obj_id) easy
// - group_toggle(group_id) easy
// - jump, reads group_id off the stack and jumps to the corresponding code
//    - if none then halt
// - conditional_push(obj_id, group_id)
//    - let on = state[obj_id]
//    - let groups = world.obj_to_group_ids[obj_id]
//    - let group_states = state[groups]
//    - on = on xor group_states[0] xor group_states[1] ... 
//    - if on, stack.push(group_id)

// optimization: code layout for caching, inlining

// compilation concerns: how to most efficiently store the runtime state
// group_id -> bool, (obj_id -> bool)
// during compilation, each object id should get a local_group_id, and then that second map would be local_group_id -> bool
// - would be a statically sized array instead of a map
// getting the state of an obj_id would be 
// - get all group_ids, compute/get all local_group_ids, index that into each group_ids vec