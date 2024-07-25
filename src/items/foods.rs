use crate::prelude::*;

pub fn carrots(quant: i32) -> impl Bundle {
    (
        ItemBundle {
            item: Item {
                name: "Carrots".into(),
                icon: "carrot".into(),
                description: "Can be traded or eaten".into(),
            },
            quantity: Quantity(quant),
            value: GoldValue(0.2),
            rarity: crate::items::Rarity::Common,
        },
        Eatable,
        Plantable,
    )
}

pub fn corn(quant: i32) -> impl Bundle {
    (
        ItemBundle {
            item: Item {
                name: "Corn".into(),
                icon: "corn".into(),
                description: "Can be traded or eaten".into(),
            },
            quantity: Quantity(quant),
            value: GoldValue(0.2),
            rarity: crate::items::Rarity::Common,
        },
        Eatable,
        Plantable,
    )
}

pub fn potatos(quant: i32) -> impl Bundle {
    (
        ItemBundle {
            item: Item {
                name: "Potatos".into(),
                icon: "potato".into(),
                description: "Can be traded or eaten".into(),
            },
            quantity: Quantity(quant),
            value: GoldValue(0.2),
            rarity: crate::items::Rarity::Common,
        },
        Eatable,
        Plantable,
    )
}
