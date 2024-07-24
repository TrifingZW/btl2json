use bincode::{deserialize, Error};
use serde_derive::{Deserialize, Serialize};
use crate::btl_structure::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BtlParser {
    #[serde(rename = "主数据")]
    pub master: Master,
    
    #[serde(rename = "国家列表")]
    pub country_vec: Vec<Country>,

    #[serde(rename = "省规划")]
    pub province_vec: Vec<i16>,
    
    #[serde(rename = "归属")]
    pub belong_vec: Vec<u8>,
    
    #[serde(rename = "城市列表")]
    pub city_vec: Vec<City>,
    
    #[serde(rename = "军队列表")]
    pub army_vec: Vec<Army>,
    
    #[serde(rename = "陷阱列表")]
    pub pitfall_vec: Vec<Pitfall>,
    
    #[serde(rename = "方案列表")]
    pub scheme_vec: Vec<Scheme>,
    
    #[serde(rename = "天气列表")]
    pub weather_vec: Vec<Weather>,
    
    #[serde(rename = "事件列表")]
    pub affair_vec: Vec<Affair>,
    
    #[serde(rename = "援军列表")]
    pub reinforcement_vec: Vec<Reinforcement>,
    
    #[serde(rename = "空袭列表")]
    pub air_raid_vec: Vec<AirRaid>,
    
    #[serde(rename = "兵种放置列表")]
    pub army_placement_vec: Vec<ArmyPlacement>,
    
    #[serde(rename = "首都列表")]
    pub capital_vec: Vec<Capital>,
    
    #[serde(rename = "策略列表")]
    pub strategy_vec: Vec<Strategy>,
    
    #[serde(rename = "空中支援列表")]
    pub air_support_vec: Vec<AirSupport>,
}

impl BtlParser {
    pub fn new() -> Self {
        BtlParser {
            master: Master {
                btl_version: 0,
                map_index: 0,
                map_capture_x: 0,
                map_capture_y: 0,
                map_width: 0,
                map_height: 0,
                total_legions: 0,
                total_buildings: 0,
                total_armies: 0,
                total_actions: 0,
                total_events: 0,
                total_weathers: 0,
                victory_condition: 0,
                min_rounds: 0,
                max_rounds: 0,
                total_reinforcements: 0,
                total_airstrikes: 0,
                placement_unit_a: 0,
                placement_unit_b: 0,
                total_capital: 0,
                campaign_era: 0,
                empty4: 0,
                total_tiles: 0,
                accumulated_money: 0,
                accumulated_gears: 0,
                accumulated_atoms: 0,
                total_traps: 0,
                empty5: 0,
                total_strategies: 0,
                empty6: 0,
                empty7: 0,
                total_air_support: 0,
            },
            country_vec: vec![],
            province_vec: vec![],
            belong_vec: vec![],
            city_vec: vec![],
            army_vec: vec![],
            pitfall_vec: vec![],
            scheme_vec: vec![],
            weather_vec: vec![],
            affair_vec: vec![],
            reinforcement_vec: vec![],
            air_raid_vec: vec![],
            army_placement_vec: vec![],
            capital_vec: vec![],
            strategy_vec: vec![],
            air_support_vec: vec![],
        }
    }

    pub fn parse(&mut self, mut buffer: Vec<u8>) -> Result<(), Error> {
        let size = 128;
        let chunk = &buffer[..size];
        // 处理切片并反序列化
        self.master = deserialize::<Master>(chunk)?;
        // 删除处理过的部分
        buffer.drain(..size);
        // 反序列化其他字段
        self.country_vec = Country::deserialize_btl(&mut buffer, self.master.total_legions as usize)?;
        self.province_vec = Country::deserialize_btl(&mut buffer, self.master.total_tiles as usize)?;
        self.belong_vec = Country::deserialize_btl(&mut buffer, self.master.total_tiles as usize)?;
        self.city_vec = City::deserialize_btl(&mut buffer, self.master.total_buildings as usize)?;
        self.army_vec = Army::deserialize_btl(&mut buffer, self.master.total_armies as usize)?;
        self.pitfall_vec = Pitfall::deserialize_btl(&mut buffer, self.master.total_traps as usize)?;
        self.scheme_vec = Scheme::deserialize_btl(&mut buffer, self.master.total_actions as usize)?;
        self.weather_vec = Weather::deserialize_btl(&mut buffer, self.master.total_weathers as usize)?;
        self.affair_vec = Affair::deserialize_btl(&mut buffer, self.master.total_events as usize)?;
        self.reinforcement_vec = Reinforcement::deserialize_btl(&mut buffer, self.master.total_reinforcements as usize)?;
        self.air_raid_vec = AirRaid::deserialize_btl(&mut buffer, self.master.total_airstrikes as usize)?;
        self.army_placement_vec = ArmyPlacement::deserialize_btl(&mut buffer, (self.master.placement_unit_a + self.master.placement_unit_b) as usize)?;
        self.capital_vec = Capital::deserialize_btl(&mut buffer, self.master.total_capital as usize)?;
        self.strategy_vec = Strategy::deserialize_btl(&mut buffer, self.master.total_strategies as usize)?;
        self.air_support_vec = AirSupport::deserialize_btl(&mut buffer, self.master.total_air_support as usize)?;

        Ok(())
    }

    pub fn to_json(&self) -> bincode::Result<String> {
        let serialized = serde_json::to_string_pretty(self).unwrap();
        return Ok(serialized);
    }
    
    pub fn to_bincode(&self) -> bincode::Result<Vec<u8>> {
        let serialized = bincode::serialize(self).unwrap();
        return Ok(serialized);
    }
}

