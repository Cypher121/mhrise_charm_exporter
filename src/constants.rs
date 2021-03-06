//Empty skill names are unallocated (or in case of the first one reserved for no skill
//Setting these to empty so it's easier to print
pub(crate) const SKILL_NAMES: [&str; 112] = [
    "",
    "Attack Boost",
    "Agitator",
    "Peak Performance",
    "Resentment",
    "Resuscitate",
    "Critical Eye",
    "Critical Boost",
    "Weakness Exploit",
    "Latent Power",
    "Maximum Might",
    "Critical Element",
    "",
    "Fire Attack",
    "Water Attack",
    "Ice Attack",
    "Thunder Attack",
    "Dragon Attack",
    "Poison Attack",
    "Paralysis Attack",
    "Sleep Attack",
    "Blast Attack",
    "Handicraft",
    "Razor Sharp",
    "Spare Shot",
    "Protective Polish",
    "Mind\'s Eye",
    "Aim Booster",
    "Bludgeoner",
    "Bow Charge Plus",
    "Focus",
    "Power Prolonger",
    "Marathon Runner",
    "Constitution",
    "Stamina Surge",
    "Guard",
    "Guard Up",
    "Offensive Guard",
    "Critical Draw",
    "Punishing Draw",
    "Quick Sheath",
    "Slugger",
    "Stamina Thief",
    "Affinity Sliding",
    "Horn Maestro",
    "Artilerry",
    "Load Shells",
    "Special Ammo Boost",
    "Normal/Rapid Up",
    "Pierce Up",
    "Spread Up",
    "Ammo Up",
    "Reload Speed",
    "Recoil Down",
    "Steadiness",
    "Rapid Fire Up",
    "Defense Boost",
    "Divine Blessing",
    "Recovery Up",
    "Recovery Speed",
    "Speed Eating",
    "Earplugs",
    "Windproof",
    "Tremor Resitance",
    "Bubbly Dance",
    "Evade Window",
    "Evade Extender",
    "Fire Resistance",
    "Water Resistance",
    "Ice Resistance",
    "Thunder Resistance",
    "Dragon Resistance",
    "Blight Resistance",
    "Poison Resistance",
    "Paralysis Resistance",
    "Sleep Resistance",
    "Stun Resistance",
    "Muck Resistance",
    "Blast Resistance",
    "Botanist",
    "Geologist",
    "Partbreaker",
    "Capture Master",
    "",
    "Good Luck",
    "Speed Sharpening",
    "Bombardier",
    "Mushroomancer",
    "Item Prolonger",
    "Wide-Range",
    "Free Meal",
    "Heroics",
    "Fortify",
    "Flinch Free",
    "Jump Master",
    "Carving Pro",
    "Hunger Resistance",
    "Leap of Faith",
    "Diversion",
    "Master Mounter",
    "",
    "",
    "",
    "",
    "Wirebug Whisperer",
    "Wall Runner",
    "Counterstrike",
    "Rapid Morph",
    "Hellfire Cloak",
    "Wind Alignment",
    "Thunder Alignment",
    "",
];

pub(crate) const CHARM_EQUIPMENT_TYPE: u32 = 3;

pub(crate) mod offsets {
    pub const DATA_MANAGER: usize = 0x14C0207E0;
    pub const EQUIPMENT_BOX: usize = 0x80;
    pub const EQUIPMENT_LIST: usize = 0x20;
    pub const EQUIPMENT_ITEMS: usize = 0x10;
    pub const EQUIPMENT_SIZE: usize = 0x18;

    pub const EQUIPMENT_TYPE: usize = 0x2C;

    pub const SLOT_POINTER: usize = 0x70;
    pub const SLOT_VALUES: usize = 0x24;

    pub const SKILL_ID_POINTER: usize = 0x78;
    pub const SKILL_ID_VALUES: usize = 0x20;

    pub const SKILL_LVL_POINTER: usize = 0x80;
    pub const SKILL_LVL_VALUES: usize = 0x20;
}
