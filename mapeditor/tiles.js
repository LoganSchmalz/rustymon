const floors = [
    {
        "name": "NONE",
        "id": 0,
        "x": 0,
        "y": 0,
        "hasCollision": false
    },
    {
        "name": "GRASS1",
        "id": 1,
        "x": 32,
        "y": 0,
        "hasCollision": false
    },
    {
        "name": "GRASS2",
        "id": 2,
        "x": 48,
        "y": 0,
        "hasCollision": false
    },
    {
        "name": "WATER1",
        "id": 3,
        "x": 16,
        "y": 64,
        "hasCollision": true
    },
    {
        "name": "WGTL",
        "id": 4,
        "x": 0,
        "y": 48,
        "hasCollision": true
    },
    {
        "name": "WGT",
        "id": 5,
        "x": 16,
        "y": 48,
        "hasCollision": true
    },
    {
        "name": "WGTR",
        "id": 6,
        "x": 32,
        "y": 48,
        "hasCollision": true
    },
    {
        "name": "WGL",
        "id": 7,
        "x": 0,
        "y": 64,
        "hasCollision": true
    },
    {
        "name": "WGR",
        "id": 8,
        "x": 32,
        "y": 64,
        "hasCollision": true
    },
    {
        "name": "WGBL",
        "id": 9,
        "x": 0,
        "y": 80,
        "hasCollision": true
    },
    {
        "name": "WGB",
        "id": 10,
        "x": 16,
        "y": 80,
        "hasCollision": true
    },
    {
        "name": "WGBR",
        "id": 11,
        "x": 32,
        "y": 80,
        "hasCollision": true
    },
    {
        "name": "GWTL",
        "id": 12,
        "x": 48,
        "y": 48,
        "hasCollision": true
    },
    {
        "name": "GWTR",
        "id": 13,
        "x": 80,
        "y": 48,
        "hasCollision": true
    },
    {
        "name": "GWBL",
        "id": 14,
        "x": 48,
        "y": 80,
        "hasCollision": true
    },
    {
        "name": "GWBR",
        "id": 15,
        "x": 80,
        "y": 80,
        "hasCollision": true
    },
    {
        "name": "FB1",
        "id": 16,
        "x": 112,
        "y": 0,
        "hasCollision": true
    },
]
const walls = [
    {
        "name": "NONE",
        "id": 0,
        "x": 0,
        "y": 0,
        "hasCollision": false
    },
    {
        "name": "WOODL",
        "id": 1,
        "x": 128,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "WOODR",
        "id": 2,
        "x": 160,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "WOOD",
        "id": 3,
        "x": 134,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "WINDOW",
        "id": 4,
        "x": 176,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "FENCE_L",
        "id": 5,
        "x": 96,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_M",
        "id": 6,
        "x": 112,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_R",
        "id": 7,
        "x": 128,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_S",
        "id": 8,
        "x": 144,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_HL",
        "id": 9,
        "x": 160,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_HR",
        "id": 10,
        "x": 176,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_TR",
        "id": 11,
        "x": 192,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_TL",
        "id": 12,
        "x": 208,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_DL",
        "id": 13,
        "x": 224,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_BL",
        "id": 14,
        "x": 240,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_DR",
        "id": 15,
        "x": 256,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "FENCE_BR",
        "id": 16,
        "x": 272,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "TGRASS_1",
        "id": 17,
        "x": 0,
        "y": 112,
        "hasCollision": false
    },
    {
        "name": "TGRASS_2",
        "id": 18,
        "x": 16,
        "y": 112,
        "hasCollision": false
    },
    {
        "name": "TGRASS_3",
        "id": 19,
        "x": 32,
        "y": 112,
        "hasCollision": false
    },
    {
        "name": "TGRASS_4",
        "id": 20,
        "x": 48,
        "y": 112,
        "hasCollision": false
    },
    {
        "name": "TGRASS_5",
        "id": 21,
        "x": 80,
        "y": 112,
        "hasCollision": false
    },
]
const objects = [
    {
        "name": "NONE",
        "id": 0,
        "x": 0,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "BERRY1",
        "id": 1,
        "x": 16,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "BERRY2",
        "id": 2,
        "x": 32,
        "y": 0,
        "hasCollision": true
    },
]
const empty = {
"id": 0
}