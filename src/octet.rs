use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;
use std::ops::AddAssign;

// As defined in section 5.7.3
const OCT_EXP: [u8; 510] = [
   1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76,
   152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157,
   39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35,
   70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222,
   161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60,
   120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163,
   91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52,
   104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59,
   118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218,
   169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85,
   170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198,
   145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171,
   75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25,
   50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81,
   162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9,
   18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11,
   22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71,
   142, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38,
   76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192,
   157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159,
   35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111,
   222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30,
   60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223,
   163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26,
   52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147,
   59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218,
   169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85,
   170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198,
   145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171,
   75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25,
   50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81,
   162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9,
   18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11,
   22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71,
   142];

// As defined in section 5.7.4, but with a prepended zero to make this zero indexed
const OCT_LOG: [u8; 256] = [
   0, 0, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100,
   224, 14, 52, 141, 239, 129, 28, 193, 105, 248, 200, 8, 76, 113, 5,
   138, 101, 47, 225, 36, 15, 33, 53, 147, 142, 218, 240, 18, 130, 69,
   29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114,
   166, 6, 191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145,
   34, 136, 54, 208, 148, 206, 143, 150, 219, 189, 241, 210, 19, 92,
   131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126, 110, 107, 58, 40,
   84, 250, 133, 186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212,
   229, 172, 115, 243, 167, 87, 7, 112, 192, 247, 140, 128, 99, 13, 103,
   74, 222, 237, 49, 197, 254, 24, 227, 165, 153, 119, 38, 184, 180,
   124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91, 149, 188,
   207, 205, 144, 135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171,
   20, 42, 93, 158, 132, 60, 57, 83, 71, 109, 65, 162, 31, 45, 67, 216,
   183, 123, 164, 118, 196, 23, 73, 236, 127, 12, 111, 246, 108, 161,
   59, 82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90, 203,
   89, 95, 176, 156, 169, 160, 81, 11, 245, 22, 235, 122, 117, 44, 215,
   79, 174, 213, 233, 230, 231, 173, 232, 116, 214, 244, 234, 168, 80,
   88, 175];

pub const OCTET_MUL: [[u8; 256]; 256] = calculate_octet_mul_table();

// See "Screaming Fast Galois Field Arithmetic Using Intel SIMD Instructions" by Plank et al.
// Further adapted to AVX2
pub const OCTET_MUL_HI_BITS: [[u8; 32]; 256] = calculate_octet_mul_hi_table();
pub const OCTET_MUL_LOW_BITS: [[u8; 32]; 256] = calculate_octet_mul_low_table();

const fn const_mul(x: usize, y: usize) -> u8 {
    return OCT_EXP[OCT_LOG[x] as usize + OCT_LOG[y] as usize];
}

const fn calculate_octet_mul_hi_table() -> [[u8; 32]; 256] {
    return [
        [0; 32],
        calculate_octet_mul_hi_table_inner(1),
        calculate_octet_mul_hi_table_inner(2),
        calculate_octet_mul_hi_table_inner(3),
        calculate_octet_mul_hi_table_inner(4),
        calculate_octet_mul_hi_table_inner(5),
        calculate_octet_mul_hi_table_inner(6),
        calculate_octet_mul_hi_table_inner(7),
        calculate_octet_mul_hi_table_inner(8),
        calculate_octet_mul_hi_table_inner(9),

        calculate_octet_mul_hi_table_inner(10),
        calculate_octet_mul_hi_table_inner(11),
        calculate_octet_mul_hi_table_inner(12),
        calculate_octet_mul_hi_table_inner(13),
        calculate_octet_mul_hi_table_inner(14),
        calculate_octet_mul_hi_table_inner(15),
        calculate_octet_mul_hi_table_inner(16),
        calculate_octet_mul_hi_table_inner(17),
        calculate_octet_mul_hi_table_inner(18),
        calculate_octet_mul_hi_table_inner(19),

        calculate_octet_mul_hi_table_inner(20),
        calculate_octet_mul_hi_table_inner(21),
        calculate_octet_mul_hi_table_inner(22),
        calculate_octet_mul_hi_table_inner(23),
        calculate_octet_mul_hi_table_inner(24),
        calculate_octet_mul_hi_table_inner(25),
        calculate_octet_mul_hi_table_inner(26),
        calculate_octet_mul_hi_table_inner(27),
        calculate_octet_mul_hi_table_inner(28),
        calculate_octet_mul_hi_table_inner(29),

        calculate_octet_mul_hi_table_inner(30),
        calculate_octet_mul_hi_table_inner(31),
        calculate_octet_mul_hi_table_inner(32),
        calculate_octet_mul_hi_table_inner(33),
        calculate_octet_mul_hi_table_inner(34),
        calculate_octet_mul_hi_table_inner(35),
        calculate_octet_mul_hi_table_inner(36),
        calculate_octet_mul_hi_table_inner(37),
        calculate_octet_mul_hi_table_inner(38),
        calculate_octet_mul_hi_table_inner(39),

        calculate_octet_mul_hi_table_inner(40),
        calculate_octet_mul_hi_table_inner(41),
        calculate_octet_mul_hi_table_inner(42),
        calculate_octet_mul_hi_table_inner(43),
        calculate_octet_mul_hi_table_inner(44),
        calculate_octet_mul_hi_table_inner(45),
        calculate_octet_mul_hi_table_inner(46),
        calculate_octet_mul_hi_table_inner(47),
        calculate_octet_mul_hi_table_inner(48),
        calculate_octet_mul_hi_table_inner(49),

        calculate_octet_mul_hi_table_inner(50),
        calculate_octet_mul_hi_table_inner(51),
        calculate_octet_mul_hi_table_inner(52),
        calculate_octet_mul_hi_table_inner(53),
        calculate_octet_mul_hi_table_inner(54),
        calculate_octet_mul_hi_table_inner(55),
        calculate_octet_mul_hi_table_inner(56),
        calculate_octet_mul_hi_table_inner(57),
        calculate_octet_mul_hi_table_inner(58),
        calculate_octet_mul_hi_table_inner(59),

        calculate_octet_mul_hi_table_inner(60),
        calculate_octet_mul_hi_table_inner(61),
        calculate_octet_mul_hi_table_inner(62),
        calculate_octet_mul_hi_table_inner(63),
        calculate_octet_mul_hi_table_inner(64),
        calculate_octet_mul_hi_table_inner(65),
        calculate_octet_mul_hi_table_inner(66),
        calculate_octet_mul_hi_table_inner(67),
        calculate_octet_mul_hi_table_inner(68),
        calculate_octet_mul_hi_table_inner(69),

        calculate_octet_mul_hi_table_inner(70),
        calculate_octet_mul_hi_table_inner(71),
        calculate_octet_mul_hi_table_inner(72),
        calculate_octet_mul_hi_table_inner(73),
        calculate_octet_mul_hi_table_inner(74),
        calculate_octet_mul_hi_table_inner(75),
        calculate_octet_mul_hi_table_inner(76),
        calculate_octet_mul_hi_table_inner(77),
        calculate_octet_mul_hi_table_inner(78),
        calculate_octet_mul_hi_table_inner(79),

        calculate_octet_mul_hi_table_inner(80),
        calculate_octet_mul_hi_table_inner(81),
        calculate_octet_mul_hi_table_inner(82),
        calculate_octet_mul_hi_table_inner(83),
        calculate_octet_mul_hi_table_inner(84),
        calculate_octet_mul_hi_table_inner(85),
        calculate_octet_mul_hi_table_inner(86),
        calculate_octet_mul_hi_table_inner(87),
        calculate_octet_mul_hi_table_inner(88),
        calculate_octet_mul_hi_table_inner(89),

        calculate_octet_mul_hi_table_inner(90),
        calculate_octet_mul_hi_table_inner(91),
        calculate_octet_mul_hi_table_inner(92),
        calculate_octet_mul_hi_table_inner(93),
        calculate_octet_mul_hi_table_inner(94),
        calculate_octet_mul_hi_table_inner(95),
        calculate_octet_mul_hi_table_inner(96),
        calculate_octet_mul_hi_table_inner(97),
        calculate_octet_mul_hi_table_inner(98),
        calculate_octet_mul_hi_table_inner(99),

        calculate_octet_mul_hi_table_inner(100),
        calculate_octet_mul_hi_table_inner(101),
        calculate_octet_mul_hi_table_inner(102),
        calculate_octet_mul_hi_table_inner(103),
        calculate_octet_mul_hi_table_inner(104),
        calculate_octet_mul_hi_table_inner(105),
        calculate_octet_mul_hi_table_inner(106),
        calculate_octet_mul_hi_table_inner(107),
        calculate_octet_mul_hi_table_inner(108),
        calculate_octet_mul_hi_table_inner(109),

        calculate_octet_mul_hi_table_inner(110),
        calculate_octet_mul_hi_table_inner(111),
        calculate_octet_mul_hi_table_inner(112),
        calculate_octet_mul_hi_table_inner(113),
        calculate_octet_mul_hi_table_inner(114),
        calculate_octet_mul_hi_table_inner(115),
        calculate_octet_mul_hi_table_inner(116),
        calculate_octet_mul_hi_table_inner(117),
        calculate_octet_mul_hi_table_inner(118),
        calculate_octet_mul_hi_table_inner(119),

        calculate_octet_mul_hi_table_inner(120),
        calculate_octet_mul_hi_table_inner(121),
        calculate_octet_mul_hi_table_inner(122),
        calculate_octet_mul_hi_table_inner(123),
        calculate_octet_mul_hi_table_inner(124),
        calculate_octet_mul_hi_table_inner(125),
        calculate_octet_mul_hi_table_inner(126),
        calculate_octet_mul_hi_table_inner(127),
        calculate_octet_mul_hi_table_inner(128),
        calculate_octet_mul_hi_table_inner(129),

        calculate_octet_mul_hi_table_inner(130),
        calculate_octet_mul_hi_table_inner(131),
        calculate_octet_mul_hi_table_inner(132),
        calculate_octet_mul_hi_table_inner(133),
        calculate_octet_mul_hi_table_inner(134),
        calculate_octet_mul_hi_table_inner(135),
        calculate_octet_mul_hi_table_inner(136),
        calculate_octet_mul_hi_table_inner(137),
        calculate_octet_mul_hi_table_inner(138),
        calculate_octet_mul_hi_table_inner(139),

        calculate_octet_mul_hi_table_inner(140),
        calculate_octet_mul_hi_table_inner(141),
        calculate_octet_mul_hi_table_inner(142),
        calculate_octet_mul_hi_table_inner(143),
        calculate_octet_mul_hi_table_inner(144),
        calculate_octet_mul_hi_table_inner(145),
        calculate_octet_mul_hi_table_inner(146),
        calculate_octet_mul_hi_table_inner(147),
        calculate_octet_mul_hi_table_inner(148),
        calculate_octet_mul_hi_table_inner(149),

        calculate_octet_mul_hi_table_inner(150),
        calculate_octet_mul_hi_table_inner(151),
        calculate_octet_mul_hi_table_inner(152),
        calculate_octet_mul_hi_table_inner(153),
        calculate_octet_mul_hi_table_inner(154),
        calculate_octet_mul_hi_table_inner(155),
        calculate_octet_mul_hi_table_inner(156),
        calculate_octet_mul_hi_table_inner(157),
        calculate_octet_mul_hi_table_inner(158),
        calculate_octet_mul_hi_table_inner(159),

        calculate_octet_mul_hi_table_inner(160),
        calculate_octet_mul_hi_table_inner(161),
        calculate_octet_mul_hi_table_inner(162),
        calculate_octet_mul_hi_table_inner(163),
        calculate_octet_mul_hi_table_inner(164),
        calculate_octet_mul_hi_table_inner(165),
        calculate_octet_mul_hi_table_inner(166),
        calculate_octet_mul_hi_table_inner(167),
        calculate_octet_mul_hi_table_inner(168),
        calculate_octet_mul_hi_table_inner(169),

        calculate_octet_mul_hi_table_inner(170),
        calculate_octet_mul_hi_table_inner(171),
        calculate_octet_mul_hi_table_inner(172),
        calculate_octet_mul_hi_table_inner(173),
        calculate_octet_mul_hi_table_inner(174),
        calculate_octet_mul_hi_table_inner(175),
        calculate_octet_mul_hi_table_inner(176),
        calculate_octet_mul_hi_table_inner(177),
        calculate_octet_mul_hi_table_inner(178),
        calculate_octet_mul_hi_table_inner(179),

        calculate_octet_mul_hi_table_inner(180),
        calculate_octet_mul_hi_table_inner(181),
        calculate_octet_mul_hi_table_inner(182),
        calculate_octet_mul_hi_table_inner(183),
        calculate_octet_mul_hi_table_inner(184),
        calculate_octet_mul_hi_table_inner(185),
        calculate_octet_mul_hi_table_inner(186),
        calculate_octet_mul_hi_table_inner(187),
        calculate_octet_mul_hi_table_inner(188),
        calculate_octet_mul_hi_table_inner(189),

        calculate_octet_mul_hi_table_inner(190),
        calculate_octet_mul_hi_table_inner(191),
        calculate_octet_mul_hi_table_inner(192),
        calculate_octet_mul_hi_table_inner(193),
        calculate_octet_mul_hi_table_inner(194),
        calculate_octet_mul_hi_table_inner(195),
        calculate_octet_mul_hi_table_inner(196),
        calculate_octet_mul_hi_table_inner(197),
        calculate_octet_mul_hi_table_inner(198),
        calculate_octet_mul_hi_table_inner(199),

        calculate_octet_mul_hi_table_inner(200),
        calculate_octet_mul_hi_table_inner(201),
        calculate_octet_mul_hi_table_inner(202),
        calculate_octet_mul_hi_table_inner(203),
        calculate_octet_mul_hi_table_inner(204),
        calculate_octet_mul_hi_table_inner(205),
        calculate_octet_mul_hi_table_inner(206),
        calculate_octet_mul_hi_table_inner(207),
        calculate_octet_mul_hi_table_inner(208),
        calculate_octet_mul_hi_table_inner(209),

        calculate_octet_mul_hi_table_inner(210),
        calculate_octet_mul_hi_table_inner(211),
        calculate_octet_mul_hi_table_inner(212),
        calculate_octet_mul_hi_table_inner(213),
        calculate_octet_mul_hi_table_inner(214),
        calculate_octet_mul_hi_table_inner(215),
        calculate_octet_mul_hi_table_inner(216),
        calculate_octet_mul_hi_table_inner(217),
        calculate_octet_mul_hi_table_inner(218),
        calculate_octet_mul_hi_table_inner(219),

        calculate_octet_mul_hi_table_inner(220),
        calculate_octet_mul_hi_table_inner(221),
        calculate_octet_mul_hi_table_inner(222),
        calculate_octet_mul_hi_table_inner(223),
        calculate_octet_mul_hi_table_inner(224),
        calculate_octet_mul_hi_table_inner(225),
        calculate_octet_mul_hi_table_inner(226),
        calculate_octet_mul_hi_table_inner(227),
        calculate_octet_mul_hi_table_inner(228),
        calculate_octet_mul_hi_table_inner(229),

        calculate_octet_mul_hi_table_inner(230),
        calculate_octet_mul_hi_table_inner(231),
        calculate_octet_mul_hi_table_inner(232),
        calculate_octet_mul_hi_table_inner(233),
        calculate_octet_mul_hi_table_inner(234),
        calculate_octet_mul_hi_table_inner(235),
        calculate_octet_mul_hi_table_inner(236),
        calculate_octet_mul_hi_table_inner(237),
        calculate_octet_mul_hi_table_inner(238),
        calculate_octet_mul_hi_table_inner(239),

        calculate_octet_mul_hi_table_inner(240),
        calculate_octet_mul_hi_table_inner(241),
        calculate_octet_mul_hi_table_inner(242),
        calculate_octet_mul_hi_table_inner(243),
        calculate_octet_mul_hi_table_inner(244),
        calculate_octet_mul_hi_table_inner(245),
        calculate_octet_mul_hi_table_inner(246),
        calculate_octet_mul_hi_table_inner(247),
        calculate_octet_mul_hi_table_inner(248),
        calculate_octet_mul_hi_table_inner(249),

        calculate_octet_mul_hi_table_inner(250),
        calculate_octet_mul_hi_table_inner(251),
        calculate_octet_mul_hi_table_inner(252),
        calculate_octet_mul_hi_table_inner(253),
        calculate_octet_mul_hi_table_inner(254),
        calculate_octet_mul_hi_table_inner(255),
    ];
}

const fn calculate_octet_mul_hi_table_inner(x: usize) -> [u8; 32] {
    return [
        0,
        const_mul(x, 1 << 4),
        const_mul(x, 2 << 4),
        const_mul(x, 3 << 4),

        const_mul(x, 4 << 4),
        const_mul(x, 5 << 4),
        const_mul(x, 6 << 4),
        const_mul(x, 7 << 4),

        const_mul(x, 8 << 4),
        const_mul(x, 9 << 4),
        const_mul(x, 10 << 4),
        const_mul(x, 11 << 4),

        const_mul(x, 12 << 4),
        const_mul(x, 13 << 4),
        const_mul(x, 14 << 4),
        const_mul(x, 15 << 4),

        0,
        const_mul(x, 1 << 4),
        const_mul(x, 2 << 4),
        const_mul(x, 3 << 4),

        const_mul(x, 4 << 4),
        const_mul(x, 5 << 4),
        const_mul(x, 6 << 4),
        const_mul(x, 7 << 4),

        const_mul(x, 8 << 4),
        const_mul(x, 9 << 4),
        const_mul(x, 10 << 4),
        const_mul(x, 11 << 4),

        const_mul(x, 12 << 4),
        const_mul(x, 13 << 4),
        const_mul(x, 14 << 4),
        const_mul(x, 15 << 4),
    ]
}

const fn calculate_octet_mul_low_table() -> [[u8; 32]; 256] {
    return [
        [0; 32],
        calculate_octet_mul_low_table_inner(1),
        calculate_octet_mul_low_table_inner(2),
        calculate_octet_mul_low_table_inner(3),
        calculate_octet_mul_low_table_inner(4),
        calculate_octet_mul_low_table_inner(5),
        calculate_octet_mul_low_table_inner(6),
        calculate_octet_mul_low_table_inner(7),
        calculate_octet_mul_low_table_inner(8),
        calculate_octet_mul_low_table_inner(9),

        calculate_octet_mul_low_table_inner(10),
        calculate_octet_mul_low_table_inner(11),
        calculate_octet_mul_low_table_inner(12),
        calculate_octet_mul_low_table_inner(13),
        calculate_octet_mul_low_table_inner(14),
        calculate_octet_mul_low_table_inner(15),
        calculate_octet_mul_low_table_inner(16),
        calculate_octet_mul_low_table_inner(17),
        calculate_octet_mul_low_table_inner(18),
        calculate_octet_mul_low_table_inner(19),

        calculate_octet_mul_low_table_inner(20),
        calculate_octet_mul_low_table_inner(21),
        calculate_octet_mul_low_table_inner(22),
        calculate_octet_mul_low_table_inner(23),
        calculate_octet_mul_low_table_inner(24),
        calculate_octet_mul_low_table_inner(25),
        calculate_octet_mul_low_table_inner(26),
        calculate_octet_mul_low_table_inner(27),
        calculate_octet_mul_low_table_inner(28),
        calculate_octet_mul_low_table_inner(29),

        calculate_octet_mul_low_table_inner(30),
        calculate_octet_mul_low_table_inner(31),
        calculate_octet_mul_low_table_inner(32),
        calculate_octet_mul_low_table_inner(33),
        calculate_octet_mul_low_table_inner(34),
        calculate_octet_mul_low_table_inner(35),
        calculate_octet_mul_low_table_inner(36),
        calculate_octet_mul_low_table_inner(37),
        calculate_octet_mul_low_table_inner(38),
        calculate_octet_mul_low_table_inner(39),

        calculate_octet_mul_low_table_inner(40),
        calculate_octet_mul_low_table_inner(41),
        calculate_octet_mul_low_table_inner(42),
        calculate_octet_mul_low_table_inner(43),
        calculate_octet_mul_low_table_inner(44),
        calculate_octet_mul_low_table_inner(45),
        calculate_octet_mul_low_table_inner(46),
        calculate_octet_mul_low_table_inner(47),
        calculate_octet_mul_low_table_inner(48),
        calculate_octet_mul_low_table_inner(49),

        calculate_octet_mul_low_table_inner(50),
        calculate_octet_mul_low_table_inner(51),
        calculate_octet_mul_low_table_inner(52),
        calculate_octet_mul_low_table_inner(53),
        calculate_octet_mul_low_table_inner(54),
        calculate_octet_mul_low_table_inner(55),
        calculate_octet_mul_low_table_inner(56),
        calculate_octet_mul_low_table_inner(57),
        calculate_octet_mul_low_table_inner(58),
        calculate_octet_mul_low_table_inner(59),

        calculate_octet_mul_low_table_inner(60),
        calculate_octet_mul_low_table_inner(61),
        calculate_octet_mul_low_table_inner(62),
        calculate_octet_mul_low_table_inner(63),
        calculate_octet_mul_low_table_inner(64),
        calculate_octet_mul_low_table_inner(65),
        calculate_octet_mul_low_table_inner(66),
        calculate_octet_mul_low_table_inner(67),
        calculate_octet_mul_low_table_inner(68),
        calculate_octet_mul_low_table_inner(69),

        calculate_octet_mul_low_table_inner(70),
        calculate_octet_mul_low_table_inner(71),
        calculate_octet_mul_low_table_inner(72),
        calculate_octet_mul_low_table_inner(73),
        calculate_octet_mul_low_table_inner(74),
        calculate_octet_mul_low_table_inner(75),
        calculate_octet_mul_low_table_inner(76),
        calculate_octet_mul_low_table_inner(77),
        calculate_octet_mul_low_table_inner(78),
        calculate_octet_mul_low_table_inner(79),

        calculate_octet_mul_low_table_inner(80),
        calculate_octet_mul_low_table_inner(81),
        calculate_octet_mul_low_table_inner(82),
        calculate_octet_mul_low_table_inner(83),
        calculate_octet_mul_low_table_inner(84),
        calculate_octet_mul_low_table_inner(85),
        calculate_octet_mul_low_table_inner(86),
        calculate_octet_mul_low_table_inner(87),
        calculate_octet_mul_low_table_inner(88),
        calculate_octet_mul_low_table_inner(89),

        calculate_octet_mul_low_table_inner(90),
        calculate_octet_mul_low_table_inner(91),
        calculate_octet_mul_low_table_inner(92),
        calculate_octet_mul_low_table_inner(93),
        calculate_octet_mul_low_table_inner(94),
        calculate_octet_mul_low_table_inner(95),
        calculate_octet_mul_low_table_inner(96),
        calculate_octet_mul_low_table_inner(97),
        calculate_octet_mul_low_table_inner(98),
        calculate_octet_mul_low_table_inner(99),

        calculate_octet_mul_low_table_inner(100),
        calculate_octet_mul_low_table_inner(101),
        calculate_octet_mul_low_table_inner(102),
        calculate_octet_mul_low_table_inner(103),
        calculate_octet_mul_low_table_inner(104),
        calculate_octet_mul_low_table_inner(105),
        calculate_octet_mul_low_table_inner(106),
        calculate_octet_mul_low_table_inner(107),
        calculate_octet_mul_low_table_inner(108),
        calculate_octet_mul_low_table_inner(109),

        calculate_octet_mul_low_table_inner(110),
        calculate_octet_mul_low_table_inner(111),
        calculate_octet_mul_low_table_inner(112),
        calculate_octet_mul_low_table_inner(113),
        calculate_octet_mul_low_table_inner(114),
        calculate_octet_mul_low_table_inner(115),
        calculate_octet_mul_low_table_inner(116),
        calculate_octet_mul_low_table_inner(117),
        calculate_octet_mul_low_table_inner(118),
        calculate_octet_mul_low_table_inner(119),

        calculate_octet_mul_low_table_inner(120),
        calculate_octet_mul_low_table_inner(121),
        calculate_octet_mul_low_table_inner(122),
        calculate_octet_mul_low_table_inner(123),
        calculate_octet_mul_low_table_inner(124),
        calculate_octet_mul_low_table_inner(125),
        calculate_octet_mul_low_table_inner(126),
        calculate_octet_mul_low_table_inner(127),
        calculate_octet_mul_low_table_inner(128),
        calculate_octet_mul_low_table_inner(129),

        calculate_octet_mul_low_table_inner(130),
        calculate_octet_mul_low_table_inner(131),
        calculate_octet_mul_low_table_inner(132),
        calculate_octet_mul_low_table_inner(133),
        calculate_octet_mul_low_table_inner(134),
        calculate_octet_mul_low_table_inner(135),
        calculate_octet_mul_low_table_inner(136),
        calculate_octet_mul_low_table_inner(137),
        calculate_octet_mul_low_table_inner(138),
        calculate_octet_mul_low_table_inner(139),

        calculate_octet_mul_low_table_inner(140),
        calculate_octet_mul_low_table_inner(141),
        calculate_octet_mul_low_table_inner(142),
        calculate_octet_mul_low_table_inner(143),
        calculate_octet_mul_low_table_inner(144),
        calculate_octet_mul_low_table_inner(145),
        calculate_octet_mul_low_table_inner(146),
        calculate_octet_mul_low_table_inner(147),
        calculate_octet_mul_low_table_inner(148),
        calculate_octet_mul_low_table_inner(149),

        calculate_octet_mul_low_table_inner(150),
        calculate_octet_mul_low_table_inner(151),
        calculate_octet_mul_low_table_inner(152),
        calculate_octet_mul_low_table_inner(153),
        calculate_octet_mul_low_table_inner(154),
        calculate_octet_mul_low_table_inner(155),
        calculate_octet_mul_low_table_inner(156),
        calculate_octet_mul_low_table_inner(157),
        calculate_octet_mul_low_table_inner(158),
        calculate_octet_mul_low_table_inner(159),

        calculate_octet_mul_low_table_inner(160),
        calculate_octet_mul_low_table_inner(161),
        calculate_octet_mul_low_table_inner(162),
        calculate_octet_mul_low_table_inner(163),
        calculate_octet_mul_low_table_inner(164),
        calculate_octet_mul_low_table_inner(165),
        calculate_octet_mul_low_table_inner(166),
        calculate_octet_mul_low_table_inner(167),
        calculate_octet_mul_low_table_inner(168),
        calculate_octet_mul_low_table_inner(169),

        calculate_octet_mul_low_table_inner(170),
        calculate_octet_mul_low_table_inner(171),
        calculate_octet_mul_low_table_inner(172),
        calculate_octet_mul_low_table_inner(173),
        calculate_octet_mul_low_table_inner(174),
        calculate_octet_mul_low_table_inner(175),
        calculate_octet_mul_low_table_inner(176),
        calculate_octet_mul_low_table_inner(177),
        calculate_octet_mul_low_table_inner(178),
        calculate_octet_mul_low_table_inner(179),

        calculate_octet_mul_low_table_inner(180),
        calculate_octet_mul_low_table_inner(181),
        calculate_octet_mul_low_table_inner(182),
        calculate_octet_mul_low_table_inner(183),
        calculate_octet_mul_low_table_inner(184),
        calculate_octet_mul_low_table_inner(185),
        calculate_octet_mul_low_table_inner(186),
        calculate_octet_mul_low_table_inner(187),
        calculate_octet_mul_low_table_inner(188),
        calculate_octet_mul_low_table_inner(189),

        calculate_octet_mul_low_table_inner(190),
        calculate_octet_mul_low_table_inner(191),
        calculate_octet_mul_low_table_inner(192),
        calculate_octet_mul_low_table_inner(193),
        calculate_octet_mul_low_table_inner(194),
        calculate_octet_mul_low_table_inner(195),
        calculate_octet_mul_low_table_inner(196),
        calculate_octet_mul_low_table_inner(197),
        calculate_octet_mul_low_table_inner(198),
        calculate_octet_mul_low_table_inner(199),

        calculate_octet_mul_low_table_inner(200),
        calculate_octet_mul_low_table_inner(201),
        calculate_octet_mul_low_table_inner(202),
        calculate_octet_mul_low_table_inner(203),
        calculate_octet_mul_low_table_inner(204),
        calculate_octet_mul_low_table_inner(205),
        calculate_octet_mul_low_table_inner(206),
        calculate_octet_mul_low_table_inner(207),
        calculate_octet_mul_low_table_inner(208),
        calculate_octet_mul_low_table_inner(209),

        calculate_octet_mul_low_table_inner(210),
        calculate_octet_mul_low_table_inner(211),
        calculate_octet_mul_low_table_inner(212),
        calculate_octet_mul_low_table_inner(213),
        calculate_octet_mul_low_table_inner(214),
        calculate_octet_mul_low_table_inner(215),
        calculate_octet_mul_low_table_inner(216),
        calculate_octet_mul_low_table_inner(217),
        calculate_octet_mul_low_table_inner(218),
        calculate_octet_mul_low_table_inner(219),

        calculate_octet_mul_low_table_inner(220),
        calculate_octet_mul_low_table_inner(221),
        calculate_octet_mul_low_table_inner(222),
        calculate_octet_mul_low_table_inner(223),
        calculate_octet_mul_low_table_inner(224),
        calculate_octet_mul_low_table_inner(225),
        calculate_octet_mul_low_table_inner(226),
        calculate_octet_mul_low_table_inner(227),
        calculate_octet_mul_low_table_inner(228),
        calculate_octet_mul_low_table_inner(229),

        calculate_octet_mul_low_table_inner(230),
        calculate_octet_mul_low_table_inner(231),
        calculate_octet_mul_low_table_inner(232),
        calculate_octet_mul_low_table_inner(233),
        calculate_octet_mul_low_table_inner(234),
        calculate_octet_mul_low_table_inner(235),
        calculate_octet_mul_low_table_inner(236),
        calculate_octet_mul_low_table_inner(237),
        calculate_octet_mul_low_table_inner(238),
        calculate_octet_mul_low_table_inner(239),

        calculate_octet_mul_low_table_inner(240),
        calculate_octet_mul_low_table_inner(241),
        calculate_octet_mul_low_table_inner(242),
        calculate_octet_mul_low_table_inner(243),
        calculate_octet_mul_low_table_inner(244),
        calculate_octet_mul_low_table_inner(245),
        calculate_octet_mul_low_table_inner(246),
        calculate_octet_mul_low_table_inner(247),
        calculate_octet_mul_low_table_inner(248),
        calculate_octet_mul_low_table_inner(249),

        calculate_octet_mul_low_table_inner(250),
        calculate_octet_mul_low_table_inner(251),
        calculate_octet_mul_low_table_inner(252),
        calculate_octet_mul_low_table_inner(253),
        calculate_octet_mul_low_table_inner(254),
        calculate_octet_mul_low_table_inner(255),
    ];
}

const fn calculate_octet_mul_low_table_inner(x: usize) -> [u8; 32] {
    return [
        0,
        const_mul(x, 1),
        const_mul(x, 2),
        const_mul(x, 3),

        const_mul(x, 4),
        const_mul(x, 5),
        const_mul(x, 6),
        const_mul(x, 7),

        const_mul(x, 8),
        const_mul(x, 9),
        const_mul(x, 10),
        const_mul(x, 11),

        const_mul(x, 12),
        const_mul(x, 13),
        const_mul(x, 14),
        const_mul(x, 15),

        0,
        const_mul(x, 1),
        const_mul(x, 2),
        const_mul(x, 3),

        const_mul(x, 4),
        const_mul(x, 5),
        const_mul(x, 6),
        const_mul(x, 7),

        const_mul(x, 8),
        const_mul(x, 9),
        const_mul(x, 10),
        const_mul(x, 11),

        const_mul(x, 12),
        const_mul(x, 13),
        const_mul(x, 14),
        const_mul(x, 15),
    ]
}

const fn calculate_octet_mul_table() -> [[u8; 256]; 256] {
    return [
        [0; 256],
        calculate_octet_mul_table_inner(1),
        calculate_octet_mul_table_inner(2),
        calculate_octet_mul_table_inner(3),
        calculate_octet_mul_table_inner(4),
        calculate_octet_mul_table_inner(5),
        calculate_octet_mul_table_inner(6),
        calculate_octet_mul_table_inner(7),
        calculate_octet_mul_table_inner(8),
        calculate_octet_mul_table_inner(9),

        calculate_octet_mul_table_inner(10),
        calculate_octet_mul_table_inner(11),
        calculate_octet_mul_table_inner(12),
        calculate_octet_mul_table_inner(13),
        calculate_octet_mul_table_inner(14),
        calculate_octet_mul_table_inner(15),
        calculate_octet_mul_table_inner(16),
        calculate_octet_mul_table_inner(17),
        calculate_octet_mul_table_inner(18),
        calculate_octet_mul_table_inner(19),

        calculate_octet_mul_table_inner(20),
        calculate_octet_mul_table_inner(21),
        calculate_octet_mul_table_inner(22),
        calculate_octet_mul_table_inner(23),
        calculate_octet_mul_table_inner(24),
        calculate_octet_mul_table_inner(25),
        calculate_octet_mul_table_inner(26),
        calculate_octet_mul_table_inner(27),
        calculate_octet_mul_table_inner(28),
        calculate_octet_mul_table_inner(29),

        calculate_octet_mul_table_inner(30),
        calculate_octet_mul_table_inner(31),
        calculate_octet_mul_table_inner(32),
        calculate_octet_mul_table_inner(33),
        calculate_octet_mul_table_inner(34),
        calculate_octet_mul_table_inner(35),
        calculate_octet_mul_table_inner(36),
        calculate_octet_mul_table_inner(37),
        calculate_octet_mul_table_inner(38),
        calculate_octet_mul_table_inner(39),

        calculate_octet_mul_table_inner(40),
        calculate_octet_mul_table_inner(41),
        calculate_octet_mul_table_inner(42),
        calculate_octet_mul_table_inner(43),
        calculate_octet_mul_table_inner(44),
        calculate_octet_mul_table_inner(45),
        calculate_octet_mul_table_inner(46),
        calculate_octet_mul_table_inner(47),
        calculate_octet_mul_table_inner(48),
        calculate_octet_mul_table_inner(49),

        calculate_octet_mul_table_inner(50),
        calculate_octet_mul_table_inner(51),
        calculate_octet_mul_table_inner(52),
        calculate_octet_mul_table_inner(53),
        calculate_octet_mul_table_inner(54),
        calculate_octet_mul_table_inner(55),
        calculate_octet_mul_table_inner(56),
        calculate_octet_mul_table_inner(57),
        calculate_octet_mul_table_inner(58),
        calculate_octet_mul_table_inner(59),

        calculate_octet_mul_table_inner(60),
        calculate_octet_mul_table_inner(61),
        calculate_octet_mul_table_inner(62),
        calculate_octet_mul_table_inner(63),
        calculate_octet_mul_table_inner(64),
        calculate_octet_mul_table_inner(65),
        calculate_octet_mul_table_inner(66),
        calculate_octet_mul_table_inner(67),
        calculate_octet_mul_table_inner(68),
        calculate_octet_mul_table_inner(69),

        calculate_octet_mul_table_inner(70),
        calculate_octet_mul_table_inner(71),
        calculate_octet_mul_table_inner(72),
        calculate_octet_mul_table_inner(73),
        calculate_octet_mul_table_inner(74),
        calculate_octet_mul_table_inner(75),
        calculate_octet_mul_table_inner(76),
        calculate_octet_mul_table_inner(77),
        calculate_octet_mul_table_inner(78),
        calculate_octet_mul_table_inner(79),

        calculate_octet_mul_table_inner(80),
        calculate_octet_mul_table_inner(81),
        calculate_octet_mul_table_inner(82),
        calculate_octet_mul_table_inner(83),
        calculate_octet_mul_table_inner(84),
        calculate_octet_mul_table_inner(85),
        calculate_octet_mul_table_inner(86),
        calculate_octet_mul_table_inner(87),
        calculate_octet_mul_table_inner(88),
        calculate_octet_mul_table_inner(89),

        calculate_octet_mul_table_inner(90),
        calculate_octet_mul_table_inner(91),
        calculate_octet_mul_table_inner(92),
        calculate_octet_mul_table_inner(93),
        calculate_octet_mul_table_inner(94),
        calculate_octet_mul_table_inner(95),
        calculate_octet_mul_table_inner(96),
        calculate_octet_mul_table_inner(97),
        calculate_octet_mul_table_inner(98),
        calculate_octet_mul_table_inner(99),

        calculate_octet_mul_table_inner(100),
        calculate_octet_mul_table_inner(101),
        calculate_octet_mul_table_inner(102),
        calculate_octet_mul_table_inner(103),
        calculate_octet_mul_table_inner(104),
        calculate_octet_mul_table_inner(105),
        calculate_octet_mul_table_inner(106),
        calculate_octet_mul_table_inner(107),
        calculate_octet_mul_table_inner(108),
        calculate_octet_mul_table_inner(109),

        calculate_octet_mul_table_inner(110),
        calculate_octet_mul_table_inner(111),
        calculate_octet_mul_table_inner(112),
        calculate_octet_mul_table_inner(113),
        calculate_octet_mul_table_inner(114),
        calculate_octet_mul_table_inner(115),
        calculate_octet_mul_table_inner(116),
        calculate_octet_mul_table_inner(117),
        calculate_octet_mul_table_inner(118),
        calculate_octet_mul_table_inner(119),

        calculate_octet_mul_table_inner(120),
        calculate_octet_mul_table_inner(121),
        calculate_octet_mul_table_inner(122),
        calculate_octet_mul_table_inner(123),
        calculate_octet_mul_table_inner(124),
        calculate_octet_mul_table_inner(125),
        calculate_octet_mul_table_inner(126),
        calculate_octet_mul_table_inner(127),
        calculate_octet_mul_table_inner(128),
        calculate_octet_mul_table_inner(129),

        calculate_octet_mul_table_inner(130),
        calculate_octet_mul_table_inner(131),
        calculate_octet_mul_table_inner(132),
        calculate_octet_mul_table_inner(133),
        calculate_octet_mul_table_inner(134),
        calculate_octet_mul_table_inner(135),
        calculate_octet_mul_table_inner(136),
        calculate_octet_mul_table_inner(137),
        calculate_octet_mul_table_inner(138),
        calculate_octet_mul_table_inner(139),

        calculate_octet_mul_table_inner(140),
        calculate_octet_mul_table_inner(141),
        calculate_octet_mul_table_inner(142),
        calculate_octet_mul_table_inner(143),
        calculate_octet_mul_table_inner(144),
        calculate_octet_mul_table_inner(145),
        calculate_octet_mul_table_inner(146),
        calculate_octet_mul_table_inner(147),
        calculate_octet_mul_table_inner(148),
        calculate_octet_mul_table_inner(149),

        calculate_octet_mul_table_inner(150),
        calculate_octet_mul_table_inner(151),
        calculate_octet_mul_table_inner(152),
        calculate_octet_mul_table_inner(153),
        calculate_octet_mul_table_inner(154),
        calculate_octet_mul_table_inner(155),
        calculate_octet_mul_table_inner(156),
        calculate_octet_mul_table_inner(157),
        calculate_octet_mul_table_inner(158),
        calculate_octet_mul_table_inner(159),

        calculate_octet_mul_table_inner(160),
        calculate_octet_mul_table_inner(161),
        calculate_octet_mul_table_inner(162),
        calculate_octet_mul_table_inner(163),
        calculate_octet_mul_table_inner(164),
        calculate_octet_mul_table_inner(165),
        calculate_octet_mul_table_inner(166),
        calculate_octet_mul_table_inner(167),
        calculate_octet_mul_table_inner(168),
        calculate_octet_mul_table_inner(169),

        calculate_octet_mul_table_inner(170),
        calculate_octet_mul_table_inner(171),
        calculate_octet_mul_table_inner(172),
        calculate_octet_mul_table_inner(173),
        calculate_octet_mul_table_inner(174),
        calculate_octet_mul_table_inner(175),
        calculate_octet_mul_table_inner(176),
        calculate_octet_mul_table_inner(177),
        calculate_octet_mul_table_inner(178),
        calculate_octet_mul_table_inner(179),

        calculate_octet_mul_table_inner(180),
        calculate_octet_mul_table_inner(181),
        calculate_octet_mul_table_inner(182),
        calculate_octet_mul_table_inner(183),
        calculate_octet_mul_table_inner(184),
        calculate_octet_mul_table_inner(185),
        calculate_octet_mul_table_inner(186),
        calculate_octet_mul_table_inner(187),
        calculate_octet_mul_table_inner(188),
        calculate_octet_mul_table_inner(189),

        calculate_octet_mul_table_inner(190),
        calculate_octet_mul_table_inner(191),
        calculate_octet_mul_table_inner(192),
        calculate_octet_mul_table_inner(193),
        calculate_octet_mul_table_inner(194),
        calculate_octet_mul_table_inner(195),
        calculate_octet_mul_table_inner(196),
        calculate_octet_mul_table_inner(197),
        calculate_octet_mul_table_inner(198),
        calculate_octet_mul_table_inner(199),

        calculate_octet_mul_table_inner(200),
        calculate_octet_mul_table_inner(201),
        calculate_octet_mul_table_inner(202),
        calculate_octet_mul_table_inner(203),
        calculate_octet_mul_table_inner(204),
        calculate_octet_mul_table_inner(205),
        calculate_octet_mul_table_inner(206),
        calculate_octet_mul_table_inner(207),
        calculate_octet_mul_table_inner(208),
        calculate_octet_mul_table_inner(209),

        calculate_octet_mul_table_inner(210),
        calculate_octet_mul_table_inner(211),
        calculate_octet_mul_table_inner(212),
        calculate_octet_mul_table_inner(213),
        calculate_octet_mul_table_inner(214),
        calculate_octet_mul_table_inner(215),
        calculate_octet_mul_table_inner(216),
        calculate_octet_mul_table_inner(217),
        calculate_octet_mul_table_inner(218),
        calculate_octet_mul_table_inner(219),

        calculate_octet_mul_table_inner(220),
        calculate_octet_mul_table_inner(221),
        calculate_octet_mul_table_inner(222),
        calculate_octet_mul_table_inner(223),
        calculate_octet_mul_table_inner(224),
        calculate_octet_mul_table_inner(225),
        calculate_octet_mul_table_inner(226),
        calculate_octet_mul_table_inner(227),
        calculate_octet_mul_table_inner(228),
        calculate_octet_mul_table_inner(229),

        calculate_octet_mul_table_inner(230),
        calculate_octet_mul_table_inner(231),
        calculate_octet_mul_table_inner(232),
        calculate_octet_mul_table_inner(233),
        calculate_octet_mul_table_inner(234),
        calculate_octet_mul_table_inner(235),
        calculate_octet_mul_table_inner(236),
        calculate_octet_mul_table_inner(237),
        calculate_octet_mul_table_inner(238),
        calculate_octet_mul_table_inner(239),

        calculate_octet_mul_table_inner(240),
        calculate_octet_mul_table_inner(241),
        calculate_octet_mul_table_inner(242),
        calculate_octet_mul_table_inner(243),
        calculate_octet_mul_table_inner(244),
        calculate_octet_mul_table_inner(245),
        calculate_octet_mul_table_inner(246),
        calculate_octet_mul_table_inner(247),
        calculate_octet_mul_table_inner(248),
        calculate_octet_mul_table_inner(249),

        calculate_octet_mul_table_inner(250),
        calculate_octet_mul_table_inner(251),
        calculate_octet_mul_table_inner(252),
        calculate_octet_mul_table_inner(253),
        calculate_octet_mul_table_inner(254),
        calculate_octet_mul_table_inner(255),
    ];
}

const fn calculate_octet_mul_table_inner(x: usize) -> [u8; 256] {
    return [
        0,
        const_mul(x, 1),
        const_mul(x, 2),
        const_mul(x, 3),
        const_mul(x, 4),
        const_mul(x, 5),
        const_mul(x, 6),
        const_mul(x, 7),
        const_mul(x, 8),
        const_mul(x, 9),

        const_mul(x, 10),
        const_mul(x, 11),
        const_mul(x, 12),
        const_mul(x, 13),
        const_mul(x, 14),
        const_mul(x, 15),
        const_mul(x, 16),
        const_mul(x, 17),
        const_mul(x, 18),
        const_mul(x, 19),

        const_mul(x, 20),
        const_mul(x, 21),
        const_mul(x, 22),
        const_mul(x, 23),
        const_mul(x, 24),
        const_mul(x, 25),
        const_mul(x, 26),
        const_mul(x, 27),
        const_mul(x, 28),
        const_mul(x, 29),

        const_mul(x, 30),
        const_mul(x, 31),
        const_mul(x, 32),
        const_mul(x, 33),
        const_mul(x, 34),
        const_mul(x, 35),
        const_mul(x, 36),
        const_mul(x, 37),
        const_mul(x, 38),
        const_mul(x, 39),

        const_mul(x, 40),
        const_mul(x, 41),
        const_mul(x, 42),
        const_mul(x, 43),
        const_mul(x, 44),
        const_mul(x, 45),
        const_mul(x, 46),
        const_mul(x, 47),
        const_mul(x, 48),
        const_mul(x, 49),

        const_mul(x, 50),
        const_mul(x, 51),
        const_mul(x, 52),
        const_mul(x, 53),
        const_mul(x, 54),
        const_mul(x, 55),
        const_mul(x, 56),
        const_mul(x, 57),
        const_mul(x, 58),
        const_mul(x, 59),

        const_mul(x, 60),
        const_mul(x, 61),
        const_mul(x, 62),
        const_mul(x, 63),
        const_mul(x, 64),
        const_mul(x, 65),
        const_mul(x, 66),
        const_mul(x, 67),
        const_mul(x, 68),
        const_mul(x, 69),

        const_mul(x, 70),
        const_mul(x, 71),
        const_mul(x, 72),
        const_mul(x, 73),
        const_mul(x, 74),
        const_mul(x, 75),
        const_mul(x, 76),
        const_mul(x, 77),
        const_mul(x, 78),
        const_mul(x, 79),

        const_mul(x, 80),
        const_mul(x, 81),
        const_mul(x, 82),
        const_mul(x, 83),
        const_mul(x, 84),
        const_mul(x, 85),
        const_mul(x, 86),
        const_mul(x, 87),
        const_mul(x, 88),
        const_mul(x, 89),

        const_mul(x, 90),
        const_mul(x, 91),
        const_mul(x, 92),
        const_mul(x, 93),
        const_mul(x, 94),
        const_mul(x, 95),
        const_mul(x, 96),
        const_mul(x, 97),
        const_mul(x, 98),
        const_mul(x, 99),

        const_mul(x, 100),
        const_mul(x, 101),
        const_mul(x, 102),
        const_mul(x, 103),
        const_mul(x, 104),
        const_mul(x, 105),
        const_mul(x, 106),
        const_mul(x, 107),
        const_mul(x, 108),
        const_mul(x, 109),

        const_mul(x, 110),
        const_mul(x, 111),
        const_mul(x, 112),
        const_mul(x, 113),
        const_mul(x, 114),
        const_mul(x, 115),
        const_mul(x, 116),
        const_mul(x, 117),
        const_mul(x, 118),
        const_mul(x, 119),

        const_mul(x, 120),
        const_mul(x, 121),
        const_mul(x, 122),
        const_mul(x, 123),
        const_mul(x, 124),
        const_mul(x, 125),
        const_mul(x, 126),
        const_mul(x, 127),
        const_mul(x, 128),
        const_mul(x, 129),

        const_mul(x, 130),
        const_mul(x, 131),
        const_mul(x, 132),
        const_mul(x, 133),
        const_mul(x, 134),
        const_mul(x, 135),
        const_mul(x, 136),
        const_mul(x, 137),
        const_mul(x, 138),
        const_mul(x, 139),

        const_mul(x, 140),
        const_mul(x, 141),
        const_mul(x, 142),
        const_mul(x, 143),
        const_mul(x, 144),
        const_mul(x, 145),
        const_mul(x, 146),
        const_mul(x, 147),
        const_mul(x, 148),
        const_mul(x, 149),

        const_mul(x, 150),
        const_mul(x, 151),
        const_mul(x, 152),
        const_mul(x, 153),
        const_mul(x, 154),
        const_mul(x, 155),
        const_mul(x, 156),
        const_mul(x, 157),
        const_mul(x, 158),
        const_mul(x, 159),

        const_mul(x, 160),
        const_mul(x, 161),
        const_mul(x, 162),
        const_mul(x, 163),
        const_mul(x, 164),
        const_mul(x, 165),
        const_mul(x, 166),
        const_mul(x, 167),
        const_mul(x, 168),
        const_mul(x, 169),

        const_mul(x, 170),
        const_mul(x, 171),
        const_mul(x, 172),
        const_mul(x, 173),
        const_mul(x, 174),
        const_mul(x, 175),
        const_mul(x, 176),
        const_mul(x, 177),
        const_mul(x, 178),
        const_mul(x, 179),

        const_mul(x, 180),
        const_mul(x, 181),
        const_mul(x, 182),
        const_mul(x, 183),
        const_mul(x, 184),
        const_mul(x, 185),
        const_mul(x, 186),
        const_mul(x, 187),
        const_mul(x, 188),
        const_mul(x, 189),

        const_mul(x, 190),
        const_mul(x, 191),
        const_mul(x, 192),
        const_mul(x, 193),
        const_mul(x, 194),
        const_mul(x, 195),
        const_mul(x, 196),
        const_mul(x, 197),
        const_mul(x, 198),
        const_mul(x, 199),

        const_mul(x, 200),
        const_mul(x, 201),
        const_mul(x, 202),
        const_mul(x, 203),
        const_mul(x, 204),
        const_mul(x, 205),
        const_mul(x, 206),
        const_mul(x, 207),
        const_mul(x, 208),
        const_mul(x, 209),

        const_mul(x, 210),
        const_mul(x, 211),
        const_mul(x, 212),
        const_mul(x, 213),
        const_mul(x, 214),
        const_mul(x, 215),
        const_mul(x, 216),
        const_mul(x, 217),
        const_mul(x, 218),
        const_mul(x, 219),

        const_mul(x, 220),
        const_mul(x, 221),
        const_mul(x, 222),
        const_mul(x, 223),
        const_mul(x, 224),
        const_mul(x, 225),
        const_mul(x, 226),
        const_mul(x, 227),
        const_mul(x, 228),
        const_mul(x, 229),

        const_mul(x, 230),
        const_mul(x, 231),
        const_mul(x, 232),
        const_mul(x, 233),
        const_mul(x, 234),
        const_mul(x, 235),
        const_mul(x, 236),
        const_mul(x, 237),
        const_mul(x, 238),
        const_mul(x, 239),

        const_mul(x, 240),
        const_mul(x, 241),
        const_mul(x, 242),
        const_mul(x, 243),
        const_mul(x, 244),
        const_mul(x, 245),
        const_mul(x, 246),
        const_mul(x, 247),
        const_mul(x, 248),
        const_mul(x, 249),

        const_mul(x, 250),
        const_mul(x, 251),
        const_mul(x, 252),
        const_mul(x, 253),
        const_mul(x, 254),
        const_mul(x, 255),
    ]
}

#[derive(Clone, Debug, PartialEq)]
pub struct Octet {
    value: u8
}

impl Octet {
    pub fn new(value: u8) -> Octet {
        Octet {
            value
        }
    }

    pub fn zero() -> Octet {
        Octet {
            value: 0
        }
    }

    pub fn one() -> Octet {
        Octet {
            value: 1
        }
    }

    pub fn alpha(i: u8) -> Octet {
        Octet {
            value: OCT_EXP[i as usize]
        }
    }

    pub fn byte(&self) -> u8 {
        self.value
    }

    pub fn fma(&mut self, other1: &Octet, other2: &Octet) {
        if other1.value != 0 && other2.value != 0 {
            unsafe {
                // This is safe because value is a u8, and OCT_LOG is 256 elements long
                let log_u = *OCT_LOG.get_unchecked(other1.value as usize) as usize;
                let log_v = *OCT_LOG.get_unchecked(other2.value as usize) as usize;
                // This is safe because the sum of two values in OCT_LOG cannot exceed 509
                self.value ^= *OCT_EXP.get_unchecked(log_u + log_v)
            }
        }
    }
}

impl Add for Octet {
    type Output = Octet;

    fn add(self, other: Octet) -> Octet {
        Octet {
            // As defined in section 5.7.2, addition on octets is implemented as bitxor
            value: self.value ^ other.value
        }
    }
}

impl<'a, 'b> Add<&'b Octet> for &'a Octet {
    type Output = Octet;

    fn add(self, other: &'b Octet) -> Octet {
        Octet {
            // As defined in section 5.7.2, addition on octets is implemented as bitxor
            value: self.value ^ other.value
        }
    }
}

impl AddAssign for Octet {
    fn add_assign(&mut self, other: Octet) {
        self.value ^= other.value;
    }
}

impl<'a> AddAssign<&'a Octet> for Octet {
    fn add_assign(&mut self, other: &'a Octet) {
        self.value ^= other.value;
    }
}

impl Sub for Octet {
    type Output = Octet;

    fn sub(self, rhs: Octet) -> Octet {
        Octet {
            // As defined in section 5.7.2, subtraction on octets is implemented as bitxor
            value: self.value ^ rhs.value
        }
    }
}

impl Mul for Octet {
    type Output = Octet;

    fn mul(self, other: Octet) -> Octet {
        &self * &other
    }
}

impl<'a, 'b> Mul<&'b Octet> for &'a Octet {
    type Output = Octet;

    fn mul(self, other: &'b Octet) -> Octet {
        // As defined in section 5.7.2, multiplication is implemented via the tables above
        if self.value == 0 || other.value == 0 {
            Octet {
                value: 0
            }
        }
        else {
            unsafe {
                // This is safe because value is a u8, and OCT_LOG is 256 elements long
                let log_u = *OCT_LOG.get_unchecked(self.value as usize) as usize;
                let log_v = *OCT_LOG.get_unchecked(other.value as usize) as usize;
                // This is safe because the sum of two values in OCT_LOG cannot exceed 509
                Octet {
                    value: *OCT_EXP.get_unchecked(log_u + log_v)
                }
            }
        }
    }
}

impl Div for Octet {
    type Output = Octet;

    fn div(self, rhs: Octet) -> Octet {
        &self / &rhs
    }
}

impl<'a, 'b> Div<&'b Octet> for &'a Octet {
    type Output = Octet;

    fn div(self, rhs: &'b Octet) -> Octet {
        assert_ne!(0, rhs.value);
        // As defined in section 5.7.2, division is implemented via the tables above
        if self.value == 0 {
            Octet {
                value: 0
            }
        }
        else {
            let log_u = OCT_LOG[self.value as usize] as usize;
            let log_v = OCT_LOG[rhs.value as usize] as usize;
            Octet {
                value: OCT_EXP[255 + log_u - log_v]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::octet::Octet;
    use crate::octet::OCT_LOG;
    use crate::octet::OCT_EXP;
    use crate::octet::OCTET_MUL_LOW_BITS;
    use crate::octet::OCTET_MUL_HI_BITS;

    #[test]
    fn multiplication_tables() {
        for i in 0..=255 {
            for j in 0..=255 {
                let expected = Octet::new(i) * Octet::new(j);
                let low = OCTET_MUL_LOW_BITS[i as usize][(j & 0x0F) as usize];
                let hi = OCTET_MUL_HI_BITS[i as usize][((j & 0xF0) >> 4) as usize];
                assert_eq!(low ^ hi, expected.byte());
            }
        }
    }

    #[test]
    fn addition() {
        let octet = Octet {
            value: rand::thread_rng().gen()
        };
        // See section 5.7.2. u is its own additive inverse
        assert_eq!(Octet::zero(), &octet + &octet);
    }

    #[test]
    fn multiplication_identity() {
        let octet = Octet {
            value: rand::thread_rng().gen()
        };
        assert_eq!(octet, &octet * &Octet::one());
    }

    #[test]
    fn multiplicative_inverse() {
        let octet = Octet {
            value: rand::thread_rng().gen_range(1, 255)
        };
        let one = Octet::one();
        assert_eq!(one, &octet * &(&one / &octet));
    }

    #[test]
    fn division() {
        let octet = Octet {
            value: rand::thread_rng().gen_range(1, 255)
        };
        assert_eq!(Octet::one(), &octet / &octet);
    }

    #[test]
    fn unsafe_mul_gaurantees() {
        let max_value = *OCT_LOG.iter().max().unwrap() as usize;
        assert!(2*max_value < OCT_EXP.len());
    }

    #[test]
    fn fma() {
        let mut result = Octet::zero();
        let mut fma_result = Octet::zero();
        for i in 0..255 {
            for j in 0..255 {
                result += Octet::new(i) * Octet::new(j);
                fma_result.fma(&Octet::new(i), &Octet::new(j));
                assert_eq!(result, fma_result);
            }
        }
    }
}
