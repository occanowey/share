use bitmask_enum::bitmask;
use bytemuck::{Pod, Zeroable};
use thiserror::Error;

use super::FromRawError;

#[derive(Debug, Pod, Clone, Copy, Zeroable)]
#[repr(C, packed(1))]
pub struct RawCardsStatus {
    collected_cards: [u8; 7],
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Level1Cards {
    HomeMadeFootball = 0,
    CrabJuice = 1,
    InsanityPepper = 2,
    Spinemelter2000 = 3,
    Parchment = 4,
    CarbonRod = 5,
    MrSparkleBox = 6,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Level2Cards {
    HeadOfJebediah,
    AmRadioToy,
    BonestormGame,
    BigButtSkinner,
    MrHoneybunny,
    DriversLicense,
    PregnancyTest,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Level3Cards {
    AngelSkeleton,
    BartsSoul,
    LisaLionheart,
    LisasValentine,
    LisasMachine,
    EvilBraces,
    SoyPop,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Level4Cards {
    MrPlowJacket,
    BurnsPortrait,
    LoveLetter,
    HomerBowlingBall,
    RedBlazer,
    BoudoirAlbum,
    PepperSpray,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Level5Cards {
    ApusTShirt,
    PinPalsShirt,
    Prop24Sign,
    BabyFeeder,
    GaneshCostume,
    ChutneySquishee,
    HotDog,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Level6Cards {
    RadioactiveManNum1,
    BortLicensePlate,
    BartTShirt,
    AustraliaBoot,
    ItchyandScratchyCel,
    GabboDoll,
    BartsFlyingHamsterScienceProject,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Level7Cards {
    SoulDonut,
    EvilKrustyDoll,
    HumanCookbook,
    TimeTravelToaster,
    HellToupee,
    MonkeysPaw,
    SmarchCalendar,
}

#[derive(Debug)]
pub struct CardsStatus {
    pub level1_cards: Level1Cards,
    pub level2_cards: Level2Cards,
    pub level3_cards: Level3Cards,
    pub level4_cards: Level4Cards,
    pub level5_cards: Level5Cards,
    pub level6_cards: Level6Cards,
    pub level7_cards: Level7Cards,
}

#[derive(Debug, Error)]
#[error("")]
pub struct FromRawCardsStatusError;

impl FromRawError for FromRawCardsStatusError {}

impl TryFrom<&RawCardsStatus> for CardsStatus {
    type Error = FromRawCardsStatusError;

    fn try_from(value: &RawCardsStatus) -> Result<Self, Self::Error> {
        let cards = value.collected_cards;

        // TODO: check if top bits is set?
        Ok(CardsStatus {
            level1_cards: cards[0].into(),
            level2_cards: cards[1].into(),
            level3_cards: cards[2].into(),
            level4_cards: cards[3].into(),
            level5_cards: cards[4].into(),
            level6_cards: cards[5].into(),
            level7_cards: cards[6].into(),
        })
    }
}
