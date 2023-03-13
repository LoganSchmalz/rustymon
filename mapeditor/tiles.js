const floors = [
    {
        "name": "NONE",
        "id": 0,
        "x": 16,
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
        "name": "SGTL",
        "id": 16,
        "x": 96,
        "y": 48,
        "hasCollision": false
    },
    {
        "name": "SGTM",
        "id": 17,
        "x": 112,
        "y": 48,
        "hasCollision": false
    },
    {
        "name": "SGTR",
        "id": 18,
        "x": 128,
        "y": 48,
        "hasCollision": true
    },
    {
        "name": "SGML",
        "id": 19,
        "x": 96,
        "y": 64,
        "hasCollision": false
    },
    {
        "name": "SGC",
        "id": 20,
        "x": 112,
        "y": 64,
        "hasCollision": false
    },
    {
        "name": "SGMR",
        "id": 21,
        "x": 128,
        "y": 64,
        "hasCollision": false
    },
    {
        "name": "SGBL",
        "id": 22,
        "x": 96,
        "y": 80,
        "hasCollision": false
    },
    {
        "name": "SGBM",
        "id": 23,
        "x": 112,
        "y": 80,
        "hasCollision": false
    },
    {
        "name": "SGBR",
        "id": 24,
        "x": 128,
        "y": 80,
        "hasCollision": false
    },
    {
        "name": "FB1",
        "id": 25,
        "x": 112,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "GRASSPATH_V",
        "id": 26,
        "x": 192,
        "y": 80,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_AB",
        "id": 27,
        "x": 112,
        "y": 48,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_NB",
        "id": 28,
        "x": 144,
        "y": 48,
        "hasCollision": true
    },
    {
        "name": "GRASSPATH_LB",
        "id": 29,
        "x": 144,
        "y": 64,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_TB",
        "id": 30,
        "x": 160,
        "y": 48,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_BB",
        "id": 31,
        "x": 160,
        "y": 80,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_RB",
        "id": 32,
        "x": 176,
        "y": 64,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_LU",
        "id": 33,
        "x": 176,
        "y": 48,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_LD",
        "id": 34,
        "x": 192,
        "y": 48,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_UR",
        "id": 35,
        "x": 192,
        "y": 64,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_DL",
        "id": 36,
        "x": 144,
        "y": 80,
        "hasCollision": false
    },
    {
        "name": "GRASSPATH_H",
        "id": 37,
        "x": 192,
        "y": 80,
        "hasCollision": false
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
        "name": "TREE_BOTTOM",
        "id": 21,
        "x": 80,
        "y": 112,
        "hasCollision": true
    },
    {
        "name": "TREE_TOP",
        "id": 22,
        "x": 96,
        "y": 112,
        "hasCollision": false
    },
    {
        "name": "TREES",
        "id": 23,
        "x": 112,
        "y": 112,
        "hasCollision": true
    },
    {
        "name": "ROOF_1",
        "id": 24,
        "x": 304,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "ROOF_2",
        "id": 25,
        "x": 320,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "ROOF_3",
        "id": 26,
        "x": 336,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "ROOF_4",
        "id": 27,
        "x": 352,
        "y": 0,
        "hasCollision": true
    },
    {
        "name": "ROOF_5",
        "id": 28,
        "x": 288,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "ROOF_6",
        "id": 29,
        "x": 304,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "ROOF_7",
        "id": 30,
        "x": 320,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "ROOF_8",
        "id": 31,
        "x": 336,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "ROOF_9",
        "id": 32,
        "x": 352,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "ROOF_10",
        "id": 33,
        "x": 368,
        "y": 16,
        "hasCollision": true
    },
    {
        "name": "ROOF_11",
        "id": 34,
        "x": 288,
        "y": 32,
        "hasCollision": true
    },
    {
        "name": "ROOF_12",
        "id": 35,
        "x": 304,
        "y": 32,
        "hasCollision": true
    },
    {
        "name": "ROOF_13",
        "id": 36,
        "x": 320,
        "y": 32,
        "hasCollision": true
    },
    {
        "name": "ROOF_14",
        "id": 37,
        "x": 336,
        "y": 32,
        "hasCollision": true
    },
    {
        "name": "ROOF_15",
        "id": 38,
        "x": 352,
        "y": 32,
        "hasCollision": true
    },
    {
        "name": "ROOF_16",
        "id": 39,
        "x": 368,
        "y": 32,
        "hasCollision": true
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
    {
        "name": "DOOR",
        "id": 3,
        "x": 48,
        "y": 0,
        "hasCollision": true
    },
]
const empty = {
"id": 0
}