use bevy_rapier3d::prelude::Group;

pub const PLAYER_GROUP: Group = Group::GROUP_1;
pub const WALL_GROUP: Group = Group::GROUP_2;
pub const GROUND_GROUP: Group = Group::GROUP_3;

pub const PLAYER_FILTER: Group = WALL_GROUP.union(GROUND_GROUP);

pub const WALL_FILTER: Group = PLAYER_GROUP;

pub const GROUND_FILTER: Group = PLAYER_GROUP;
