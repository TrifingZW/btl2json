use std::mem::size_of;
use bincode::deserialize;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Master {
    #[serde(rename = "Btl版本")]
    pub btl_version: i32,

    #[serde(rename = "地图序号")]
    pub map_index: i32,

    #[serde(rename = "地图截取x")]
    pub map_capture_x: i32,

    #[serde(rename = "地图截取y")]
    pub map_capture_y: i32,

    #[serde(rename = "地图宽")]
    pub map_width: i32,

    #[serde(rename = "地图高")]
    pub map_height: i32,

    #[serde(rename = "军团总数")]
    pub total_legions: i32,

    #[serde(rename = "建筑总数")]
    pub total_buildings: i32,

    #[serde(rename = "军队总数")]
    pub total_armies: i32,

    #[serde(rename = "行为总数")]
    pub total_actions: i32,

    #[serde(rename = "事件总数")]
    pub total_events: i32,

    #[serde(rename = "天气总数")]
    pub total_weathers: i32,

    #[serde(rename = "胜利条件")]
    pub victory_condition: i32,

    #[serde(rename = "最小回合")]
    pub min_rounds: i32,

    #[serde(rename = "最大回合")]
    pub max_rounds: i32,

    #[serde(rename = "援军总数")]
    pub total_reinforcements: i32,

    #[serde(rename = "空袭总数")]
    pub total_airstrikes: i32,

    #[serde(rename = "放置单位A")]
    pub placement_unit_a: i32,

    #[serde(rename = "放置单位B")]
    pub placement_unit_b: i32,

    #[serde(rename = "国家首都")]
    pub total_capital: i32,

    #[serde(rename = "战役时代")]
    pub campaign_era: i32,

    #[serde(rename = "empty4")]
    pub empty4: i32,

    #[serde(rename = "地块总数")]
    pub total_tiles: i32,

    #[serde(rename = "积攒金钱")]
    pub accumulated_money: i32,

    #[serde(rename = "积攒齿轮")]
    pub accumulated_gears: i32,

    #[serde(rename = "积攒原子")]
    pub accumulated_atoms: i32,

    #[serde(rename = "陷阱总数")]
    pub total_traps: i32,

    #[serde(rename = "empty5")]
    pub empty5: i32,

    #[serde(rename = "战略总数")]
    pub total_strategies: i32,

    #[serde(rename = "empty6")]
    pub empty6: i32,

    #[serde(rename = "empty7")]
    pub empty7: i32,

    #[serde(rename = "空中支援")]
    pub total_air_support: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    #[serde(rename = "序号")]
    pub serial_number: i32, // 序号

    #[serde(rename = "国家")]
    pub country: i32,

    #[serde(rename = "金钱")]
    pub money: i32,

    #[serde(rename = "齿轮")]
    pub gears: i32,

    #[serde(rename = "原子")]
    pub atoms: i32,

    #[serde(rename = "控制")]
    pub control: i32,

    #[serde(rename = "阵营")]
    pub faction: i32,

    #[serde(rename = "战败条件")]
    pub defeat_condition: i32,

    #[serde(rename = "兵种加成")]
    pub unit_bonus: f32,

    #[serde(rename = "税率加成")]
    pub tax_bonus: f32,

    #[serde(rename = "R8")]
    pub r8: u8,

    #[serde(rename = "G8")]
    pub g8: u8,

    #[serde(rename = "B8")]
    pub b8: u8,

    #[serde(rename = "A8")]
    pub a8: u8,

    #[serde(rename = "原子弹数量")]
    pub nuclear_bombs: u32,

    #[serde(rename = "氢弹数量")]
    pub hydrogen_bombs: u32,

    #[serde(rename = "三相弹数量")]
    pub triad_bombs: i32,

    #[serde(rename = "反物质弹数量")]
    pub antimatter_bombs: i32,

    #[serde(rename = "机动等级")]
    pub mobility_level: i32,

    #[serde(rename = "步枪等级")]
    pub rifle_level: i32,

    #[serde(rename = "迷彩等级")]
    pub camouflage_level: i32,

    #[serde(rename = "工兵等级")]
    pub engineer_level: i32,

    #[serde(rename = "手雷等级")]
    pub grenade_level: i32,

    #[serde(rename = "迫击炮等级")]
    pub mortar_level: i32,

    #[serde(rename = "行军等级")]
    pub march_level: i32,

    #[serde(rename = "防弹衣等级")]
    pub armor_level: i32,

    #[serde(rename = "装甲等级")]
    pub tank_armor_level: i32,

    #[serde(rename = "主炮等级")]
    pub main_gun_level: i32,

    #[serde(rename = "车体等级")]
    pub chassis_level: i32,

    #[serde(rename = "引擎等级")]
    pub engine_level: i32,

    #[serde(rename = "机枪等级")]
    pub machine_gun_level: i32,

    #[serde(rename = "突袭等级")]
    pub raid_level: i32,

    #[serde(rename = "坦克防空等级")]
    pub tank_anti_air_level: i32,

    #[serde(rename = "强化车体等级")]
    pub enhanced_chassis_level: i32,

    #[serde(rename = "火炮炮击等级")]
    pub artillery_attack_level: i32,

    #[serde(rename = "火箭弹等级")]
    pub rocket_level: i32,

    #[serde(rename = "火炮牵引等级")]
    pub artillery_tow_level: i32,

    #[serde(rename = "火炮装甲等级")]
    pub artillery_armor_level: i32,

    #[serde(rename = "火炮火力等级")]
    pub artillery_firepower_level: i32,

    #[serde(rename = "火炮火箭等级")]
    pub artillery_rocket_level: i32,

    #[serde(rename = "伪装等级")]
    pub camouflage2_level: i32,

    #[serde(rename = "舰艇船体等级")]
    pub ship_hull_level: i32,

    #[serde(rename = "推进器等级")]
    pub propulsion_level: i32,

    #[serde(rename = "舰艇装甲等级")]
    pub ship_armor_level: i32,

    #[serde(rename = "武器等级")]
    pub weapon_level: i32,

    #[serde(rename = "舰艇舰炮等级")]
    pub ship_main_gun_level: i32,

    #[serde(rename = "鱼雷等级")]
    pub torpedo_level: i32,

    #[serde(rename = "舰艇扫雷")]
    pub ship_mine_sweeping_level: i32,

    #[serde(rename = "防空武器等级")]
    pub anti_air_weapon_level: i32,

    #[serde(rename = "现代舰艇等级")]
    pub modern_ship_level: i32,

    #[serde(rename = "航空燃油等级")]
    pub aviation_fuel_level: i32,

    #[serde(rename = "航空发动机等级")]
    pub aviation_engine_level: i32,

    #[serde(rename = "航空炸弹等级")]
    pub aviation_bomb_level: i32,

    #[serde(rename = "空袭等级")]
    pub airstrike_level: i32,

    #[serde(rename = "轰炸等级")]
    pub bombing_level: i32,

    #[serde(rename = "战略轰炸等级")]
    pub strategic_bombing_level: i32,

    #[serde(rename = "空降兵等级")]
    pub airborne_level: i32,

    #[serde(rename = "喷气发动机等级")]
    pub jet_engine_level: i32,

    #[serde(rename = "机枪堡等级")]
    pub machine_gun_bunker_level: i32,

    #[serde(rename = "要塞炮等级")]
    pub fortress_gun_level: i32,

    #[serde(rename = "海岸炮等级")]
    pub coastal_gun_level: i32,

    #[serde(rename = "火箭发射器等级")]
    pub rocket_launcher_level: i32,

    #[serde(rename = "工事等级")]
    pub fortification_level: i32,

    #[serde(rename = "高射机枪等级")]
    pub anti_aircraft_gun_level: i32,

    #[serde(rename = "防空炮等级")]
    pub anti_aircraft_cannon_level: i32,

    #[serde(rename = "对空导弹等级")]
    pub anti_air_missile_level: i32,

    #[serde(rename = "雷达等级")]
    pub radar_level: i32,

    #[serde(rename = "弹头")]
    pub warhead: i32,

    #[serde(rename = "固体火箭发动机等级")]
    pub solid_rocket_engine_level: i32,

    #[serde(rename = "破防等级")]
    pub armor_piercing_level: i32,

    #[serde(rename = "核聚变等级")]
    pub nuclear_fusion_level: i32,

    #[serde(rename = "empty1")]
    pub empty1: i32,

    #[serde(rename = "empty2")]
    pub empty2: i32,

    #[serde(rename = "empty3")]
    pub empty3: i32,

    #[serde(rename = "empty4")]
    pub empty4: i32,

    #[serde(rename = "科技等级")]
    pub tech_level: i32,

    #[serde(rename = "empty5")]
    pub empty5: i32,

    #[serde(rename = "empty6")]
    pub empty6: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topography {
    #[serde(rename = "地块类型")]
    pub terrain_type: u8,

    #[serde(rename = "地块ID")]
    pub terrain_id: u8,

    #[serde(rename = "地块X")]
    pub terrain_x: u8,

    #[serde(rename = "地块Y")]
    pub terrain_y: u8,

    #[serde(rename = "装饰类型A")]
    pub decoration_type_a: u8,

    #[serde(rename = "装饰AID")]
    pub decoration_a_id: u8,

    #[serde(rename = "装饰AX")]
    pub decoration_a_x: u8,

    #[serde(rename = "装饰AY")]
    pub decoration_a_y: u8,

    #[serde(rename = "装饰类型B")]
    pub decoration_type_b: u8,

    #[serde(rename = "装饰BID")]
    pub decoration_b_id: u8,

    #[serde(rename = "装饰BX")]
    pub decoration_b_x: u8,

    #[serde(rename = "装饰BY")]
    pub decoration_b_y: u8,

    #[serde(rename = "水边缘")]
    pub water_edge: u8,

    #[serde(rename = "路边缘")]
    pub road_edge: u8,

    #[serde(rename = "海岸")]
    pub coast: u8,

    #[serde(rename = "empty4")]
    pub empty4: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    #[serde(rename = "坐标")]
    pub coordinate: i16,

    #[serde(rename = "名称")]
    pub name: i16,

    #[serde(rename = "等级")]
    pub level: u8,

    #[serde(rename = "外观")]
    pub appearance: u8,

    #[serde(rename = "地标")]
    pub landmark: u8,

    #[serde(rename = "奇观")]
    pub wonder: u8,

    #[serde(rename = "奖励类型")]
    pub reward_type: i16,

    #[serde(rename = "奖励数量")]
    pub reward_quantity: i16,

    #[serde(rename = "仇恨值")]
    pub hatred: u8,

    #[serde(rename = "据点")]
    pub stronghold: u8,

    #[serde(rename = "触发事件")]
    pub trigger_event: u8,

    #[serde(rename = "empty1")]
    pub empty1: u8,

    #[serde(rename = "empty2")]
    pub empty2: u8,

    #[serde(rename = "empty3")]
    pub empty3: u8,

    #[serde(rename = "empty4")]
    pub empty4: u8,

    #[serde(rename = "运输船")]
    pub transport_ship: u8,

    #[serde(rename = "火焰类型")]
    pub flame_type: u8,

    #[serde(rename = "持续回合")]
    pub duration_rounds: u8,

    #[serde(rename = "防空武器")]
    pub anti_air_weapon: u8,

    #[serde(rename = "防空武器范围")]
    pub anti_air_weapon_range: u8,

    #[serde(rename = "工厂")]
    pub factory: u8,

    #[serde(rename = "科技")]
    pub technology: u8,

    #[serde(rename = "补给站")]
    pub supply_station: u8,

    #[serde(rename = "航空")]
    pub aviation: u8,

    #[serde(rename = "导弹")]
    pub missile: u8,

    #[serde(rename = "核弹")]
    pub nuclear_bomb: u8,

    #[serde(rename = "empty5")]
    pub empty5: u8,

    #[serde(rename = "empty6")]
    pub empty6: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Army {
    #[serde(rename = "坐标")]
    pub coordinate: i16,

    #[serde(rename = "兵种")]
    pub arms: u8,

    #[serde(rename = "等级")]
    pub level: u8,

    #[serde(rename = "编制")]
    pub organization: u8,

    #[serde(rename = "方向")]
    pub direction: u8,

    #[serde(rename = "移动力")]
    pub mobility: u8,

    #[serde(rename = "建造回合")]
    pub construction_rounds: u8,

    #[serde(rename = "兵种经验")]
    pub arms_experience: i16,

    #[serde(rename = "血量加成")]
    pub health_bonus: i16,

    #[serde(rename = "当前血量")]
    pub current_health: i16,

    #[serde(rename = "血量上限")]
    pub max_health: i16,

    #[serde(rename = "将领")]
    pub commander: i16,

    #[serde(rename = "军衔")]
    pub rank: u8,

    #[serde(rename = "爵位")]
    pub title: u8,

    #[serde(rename = "胸章一")]
    pub badge_one: u8,

    #[serde(rename = "胸章二")]
    pub badge_two: u8,

    #[serde(rename = "胸章三")]
    pub badge_three: u8,

    #[serde(rename = "技能等级1")]
    pub skill_level_1: u8,

    #[serde(rename = "技能等级2")]
    pub skill_level_2: u8,

    #[serde(rename = "技能等级3")]
    pub skill_level_3: u8,

    #[serde(rename = "技能等级4")]
    pub skill_level_4: u8,

    #[serde(rename = "技能等级5")]
    pub skill_level_5: u8,

    #[serde(rename = "据点")]
    pub stronghold: u8,

    #[serde(rename = "方针")]
    pub policy: u8,

    #[serde(rename = "运输船")]
    pub transport_ship: u8,

    #[serde(rename = "仇恨值")]
    pub hatred: u8,

    #[serde(rename = "移动目标")]
    pub move_target: i16,

    #[serde(rename = "empty1")]
    pub empty1: u8,

    #[serde(rename = "empty2")]
    pub empty2: u8,

    #[serde(rename = "行为方案")]
    pub behavior_plan: i16,

    #[serde(rename = "改变回合")]
    pub change_rounds: i16,

    #[serde(rename = "士气")]
    pub morale: u8,

    #[serde(rename = "士气持续回合")]
    pub morale_duration_rounds: u8,

    #[serde(rename = "触发事件")]
    pub trigger_event: u8,

    #[serde(rename = "盾牌标志")]
    pub shield_flag: u8,

    #[serde(rename = "固守最短距离")]
    pub minimum_defense_distance: i32,

    #[serde(rename = "empty3")]
    pub empty3: u8,

    #[serde(rename = "empty4")]
    pub empty4: u8,

    #[serde(rename = "empty5")]
    pub empty5: u8,

    #[serde(rename = "empty6")]
    pub empty6: u8,

    #[serde(rename = "empty7")]
    pub empty7: u8,

    #[serde(rename = "empty8")]
    pub empty8: u8,

    #[serde(rename = "ribbon1")]
    pub ribbon1: u16,

    #[serde(rename = "ribbon2")]
    pub ribbon2: u16,

    #[serde(rename = "ribbon3")]
    pub ribbon3: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pitfall {
    #[serde(rename = "坐标")]
    pub coordinate: i16,

    #[serde(rename = "所属军团")]
    pub affiliated_legion: i16,

    #[serde(rename = "陷阱编制")]
    pub trap_organization: i16,

    #[serde(rename = "陷阱血量")]
    pub trap_health: i32,

    #[serde(rename = "empty")]
    pub empty: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scheme {
    #[serde(rename = "方案编号")]
    pub scheme_id: i32,

    #[serde(rename = "终结事件")]
    pub end_event: i32,

    #[serde(rename = "触发回合")]
    pub trigger_round: i32,

    #[serde(rename = "目标地块")]
    pub target_tile: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    #[serde(rename = "天气类型")]
    pub weather_type: i32,

    #[serde(rename = "empty")]
    pub empty: i32,

    #[serde(rename = "触发回合")]
    pub trigger_round: i32,

    #[serde(rename = "持续回合")]
    pub duration_rounds: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Affair {
    #[serde(rename = "事件ID")]
    pub event_id: i32,

    #[serde(rename = "关联事件")]
    pub related_event: i32,

    #[serde(rename = "触发条件")]
    pub trigger_conditions: i32,

    #[serde(rename = "事件类型")]
    pub event_type: i32,

    #[serde(rename = "触发军团")]
    pub triggering_legion: i32,

    #[serde(rename = "影响军团")]
    pub affected_legion: i32,

    #[serde(rename = "目标值")]
    pub target_value: i32,

    #[serde(rename = "Zero")]
    pub zero: i32,

    #[serde(rename = "触发回合")]
    pub trigger_round: i32,

    #[serde(rename = "对话代码")]
    pub dialog_code: i32,

    #[serde(rename = "默认结束段")]
    pub default_end_segment: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reinforcement {
    #[serde(rename = "坐标")]
    pub coordinate: i32,

    #[serde(rename = "兵种")]
    pub unit_type: i32,

    #[serde(rename = "等级")]
    pub level: i32,

    #[serde(rename = "编制")]
    pub organization: i32,

    #[serde(rename = "运输船")]
    pub transport_ship: i32,

    #[serde(rename = "方针")]
    pub policy: i32,

    #[serde(rename = "未知")]
    pub unknown1: i32,

    #[serde(rename = "将领")]
    pub general: i32,

    #[serde(rename = "军衔")]
    pub rank: i32,

    #[serde(rename = "爵位")]
    pub title: i32,

    #[serde(rename = "技能等级1")]
    pub skill_level1: i32,

    #[serde(rename = "技能等级2")]
    pub skill_level2: i32,

    #[serde(rename = "技能等级3")]
    pub skill_level3: i32,

    #[serde(rename = "技能等级4")]
    pub skill_level4: i32,

    #[serde(rename = "技能等级5")]
    pub skill_level5: i32,

    #[serde(rename = "勋章1")]
    pub medal1: i32,

    #[serde(rename = "勋章2")]
    pub medal2: i32,

    #[serde(rename = "勋章3")]
    pub medal3: i32,

    #[serde(rename = "所属国家")]
    pub affiliated_country: i32,

    #[serde(rename = "爆兵回合")]
    pub explosion_round: i32,

    #[serde(rename = "胸章1")]
    pub badge1: i32,

    #[serde(rename = "胸章2")]
    pub badge2: i32,

    #[serde(rename = "胸章3")]
    pub badge3: i32,

    #[serde(rename = "勋带1")]
    pub ribbon1: i32,

    #[serde(rename = "勋带2")]
    pub ribbon2: i32,

    #[serde(rename = "勋带3")]
    pub ribbon3: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirRaid {
    #[serde(rename = "坐标")]
    pub coordinate: i32,

    #[serde(rename = "兵种")]
    pub unit_type: i16,

    #[serde(rename = "精英兵种")]
    pub elite_unit_type: i16,

    #[serde(rename = "弹药")]
    pub ammunition: i32,

    #[serde(rename = "军团")]
    pub legion: i32,

    #[serde(rename = "回合")]
    pub round: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmyPlacement {
    #[serde(rename = "坐标")]
    pub coordinate: i16,

    #[serde(rename = "empty")]
    pub empty: i16,

    #[serde(rename = "方向")]
    pub direction: u8,

    #[serde(rename = "Id")]
    pub id: u8,

    #[serde(rename = "运输船")]
    pub transport_ship: u8,

    #[serde(rename = "empty2")]
    pub empty2: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capital {
    #[serde(rename = "坐标")]
    pub coordinate: i16,

    #[serde(rename = "empty")]
    pub empty: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strategy {
    #[serde(rename = "军团序号")]
    pub legion_number: i32,

    #[serde(rename = "empty")]
    pub empty: i32,

    #[serde(rename = "回合")]
    pub round: i32,

    #[serde(rename = "建设代码")]
    pub construction_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirSupport {
    #[serde(rename = "兵种")]
    pub unit_type: i32,

    #[serde(rename = "弹药")]
    pub ammunition: i32,

    #[serde(rename = "军团")]
    pub legion: i32,

    #[serde(rename = "回合")]
    pub round: i32,
}

pub trait BtlDeserialize {
    fn deserialize_btl<T: DeserializeOwned>(buffer: &mut Vec<u8>, total: usize) -> bincode::Result<Vec<T>>
    {
        let struct_size = size_of::<T>();
        let size = struct_size * total;
        
        let mut value_vec: Vec<T> = vec![];

        // 处理切片并反序列化
        for i in 0..total {
            let chunk = &buffer[i * struct_size..(i + 1) * struct_size];
            let value = deserialize::<T>(chunk)?;
            value_vec.push(value);
        }

        // 删除处理过的部分
        buffer.drain(..size);
        Ok(value_vec)
    }
}

macro_rules! impl_btl_deserialize {
    ($($type:ty),*) => {
        $(
            impl BtlDeserialize for $type {}
        )*
    };
}

// 使用宏为多个类型实现BtlDeserialize trait
impl_btl_deserialize!(
    Country,
    City,
    Army,
    Pitfall,
    Scheme,
    Weather,
    Affair,
    Reinforcement,
    AirRaid,
    ArmyPlacement,
    Capital,
    Strategy,
    AirSupport
);



