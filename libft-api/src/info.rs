pub const GS_CAMPUS_ID: i32 = 69;
pub const FT_CURSUS_ID: i32 = 21;
pub const FT_PISCINE_CURSUS_ID: i32 = 9;
pub const TEST_USER_YONDOO06_ID: i32 = 185472;

pub use ft_cursus::*;
mod ft_cursus {
    pub use inner::*;
    pub const ALL_INNER_SUBJECTS_ID: [u16; 33] = [
        LIBFT,
        FT_PRINTF,
        GET_NEXT_LINE,
        BORN2BEROOT,
        MINITALK,
        PIPEX,
        PUSH_SWAP,
        FRACT_OL,
        FDF,
        EXAM_RANK_02,
        PHILOSOPHERS,
        MINISHELL,
        EXAM_RANK_03,
        NET_PRACTICE,
        MINI_RT,
        CUB3D,
        CPP_MODULE_00,
        CPP_MODULE_01,
        CPP_MODULE_02,
        CPP_MODULE_03,
        CPP_MODULE_04,
        EXAM_RANK_04,
        CPP_MODULE_05,
        CPP_MODULE_06,
        CPP_MODULE_07,
        CPP_MODULE_08,
        CPP_MODULE_09,
        INCEPTION,
        WEBSERV,
        FT_IRC,
        EXAM_RANK_05,
        FT_TRANSCENDENCE,
        EXAM_RANK_06,
    ];
    mod inner {
        // 0 circle
        pub const LIBFT: u16 = 1314;

        // 1 circle
        pub const FT_PRINTF: u16 = 1316;
        pub const GET_NEXT_LINE: u16 = 1327;
        pub const BORN2BEROOT: u16 = 1994;

        // 2 circle
        pub const MINITALK: u16 = 2005;
        pub const PIPEX: u16 = 2004;
        pub const PUSH_SWAP: u16 = 1471;
        pub const FRACT_OL: u16 = 1476;
        pub const FDF: u16 = 2008;

        pub const EXAM_RANK_02: u16 = 1320;

        // 3 circle
        pub const PHILOSOPHERS: u16 = 1334;
        pub const MINISHELL: u16 = 1331;

        pub const EXAM_RANK_03: u16 = 1321;

        // 4 circle
        pub const NET_PRACTICE: u16 = 2007;
        pub const MINI_RT: u16 = 1315;
        pub const CUB3D: u16 = 1326;
        pub const CPP_MODULE_00: u16 = 1338;
        pub const CPP_MODULE_01: u16 = 1339;
        pub const CPP_MODULE_02: u16 = 1340;
        pub const CPP_MODULE_03: u16 = 1341;
        pub const CPP_MODULE_04: u16 = 1342;

        pub const EXAM_RANK_04: u16 = 1322;

        // 5 circle
        pub const CPP_MODULE_05: u16 = 1343;
        pub const CPP_MODULE_06: u16 = 1344;
        pub const CPP_MODULE_07: u16 = 1345;
        pub const CPP_MODULE_08: u16 = 1346;
        pub const CPP_MODULE_09: u16 = 2309;
        pub const INCEPTION: u16 = 1983;
        pub const WEBSERV: u16 = 1332;
        pub const FT_IRC: u16 = 1336;

        pub const EXAM_RANK_05: u16 = 1323;

        // 6 circle
        pub const FT_TRANSCENDENCE: u16 = 1337;

        pub const EXAM_RANK_06: u16 = 1324;
    }

    pub const ABSTRACT_DATA: u16 = 2371;
    pub const ABSTRACT_VM: u16 = 1461;
    pub const ACCESSIBLE_DIRECTORY: u16 = 2523;
    pub const ACTIVE_CONNECT: u16 = 2395;
    pub const ACTIVE_DISCOVERY: u16 = 2520;
    pub const ACTIVE_TECH_TALES: u16 = 2529;
    pub const ADMINISTRATIVE_DIRECTORY: u16 = 2522;
    pub const ALONE_IN_THE_DARK: u16 = 2310;
    pub const AL_CU: u16 = 2102;
    pub const AMAZON: u16 = 1959;
    pub const APPRENTISSAGE_1_AN: u16 = 1873;
    pub const APPRENTISSAGE_1_AN_1: u16 = 1877;
    pub const APPRENTISSAGE_1_AN_2: u16 = 1878;
    pub const APPRENTISSAGE_1_AN_3: u16 = 1879;
    pub const APPRENTISSAGE_1_AN_4: u16 = 1880;
    pub const APPRENTISSAGE_1_AN_BEGIN_EVALUATION: u16 = 2090;
    pub const APPRENTISSAGE_1_AN_FINAL_EVALUATION: u16 = 1876;
    pub const APPRENTISSAGE_2_ANS_1ÈRE_ANNÉE: u16 = 1857;
    pub const APPRENTISSAGE_2_ANS_1ÈRE_ANNÉE_1: u16 = 1861;
    pub const APPRENTISSAGE_2_ANS_1ÈRE_ANNÉE_2: u16 = 1862;
    pub const APPRENTISSAGE_2_ANS_1ÈRE_ANNÉE_3: u16 = 1863;
    pub const APPRENTISSAGE_2_ANS_1ÈRE_ANNÉE_4: u16 = 1864;
    pub const APPRENTISSAGE_2_ANS_1ÈRE_ANNÉE_ANNUAL_EVALUATION: u16 = 2093;
    pub const APPRENTISSAGE_2_ANS_1ÈRE_ANNÉE_BEGIN_EVALUATION: u16 = 2092;
    pub const APPRENTISSAGE_2_ANS_2ÈME_ANNÉE_1: u16 = 1869;
    pub const APPRENTISSAGE_2_ANS_2ÈME_ANNÉE_2: u16 = 1870;
    pub const APPRENTISSAGE_2_ANS_2ÈME_ANNÉE_3: u16 = 1871;
    pub const APPRENTISSAGE_2_ANS_2ÈME_ANNÉE_4: u16 = 1872;
    pub const APPRENTISSAGE_2_ANS_2ÈME_ANNÉE_FINAL_EVALUATION: u16 = 1868;
    pub const APPRENTISSAGE_2_ANS_2ÈME_ANNÉE_MID_EVALUATION: u16 = 2091;
    pub const AUTOMATIC_DIRECTORY: u16 = 2521;
    pub const AVAJ_LAUNCHER: u16 = 1435;
    pub const B: u16 = 2606;
    pub const BOOT2ROOT: u16 = 1446;
    pub const BOMBERMAN: u16 = 1389;
    pub const CAMAGRU: u16 = 1396;
    pub const CHALLENGE_42_QUÉBEC: u16 = 2525;
    pub const CHURN: u16 = 1934;
    pub const CITY_LIFE: u16 = 1957;
    pub const CLOUD_1: u16 = 1414;
    pub const CODAM_EXAM_TEST: u16 = 2212;
    pub const COMPUTORV1: u16 = 1382;
    pub const COMPUTORV2: u16 = 1433;
    pub const CONNECT4: u16 = 2174;
    pub const CONTRACT_UPLOAD_: u16 = 1874;
    pub const COREWAR: u16 = 1475;
    pub const COW_NECK_TID: u16 = 2068;
    pub const CURSUS_EU_ACEITO_RIO: u16 = 2217;
    pub const CYBERSECURITY: u16 = 2346;
    pub const CYBERSECURITY_ARACHNIDA_WEB: u16 = 2326;
    pub const CYBERSECURITY_FT_ONION_WEB: u16 = 2328;
    pub const CYBERSECURITY_FT_OTP_OTP: u16 = 2327;
    pub const CYBERSECURITY_INQUISITOR_NETWORK: u16 = 2343;
    pub const CYBERSECURITY_IRON_DOME_MALWARE: u16 = 2344;
    pub const CYBERSECURITY_REVERSE_ME_REV: u16 = 2336;
    pub const CYBERSECURITY_VACCINE_WEB: u16 = 2345;
    pub const DARKLY: u16 = 1405;
    pub const DARKLY_WEB: u16 = 1814;
    pub const DATA_SCIENCE_0: u16 = 2306;
    pub const DATA_SCIENCE_1: u16 = 2318;
    pub const DATA_SCIENCE_2: u16 = 2311;
    pub const DATA_SCIENCE_3: u16 = 2316;
    pub const DATA_SCIENCE_4: u16 = 2317;
    pub const DEATH: u16 = 1445;
    pub const DJANGO_0_INITIATION: u16 = 2190;
    pub const DJANGO_0_OOB: u16 = 2192;
    pub const DJANGO_0_STARTING: u16 = 2191;
    pub const DJANGO_1_BASE_DJANGO: u16 = 2194;
    pub const DJANGO_1_LIB: u16 = 2193;
    pub const DJANGO_2_SQL: u16 = 2195;
    pub const DJANGO_3_ADVANCED: u16 = 2197;
    pub const DJANGO_3_FINAL: u16 = 2198;
    pub const DJANGO_3_SESSIONS: u16 = 2196;
    pub const DOOM_NUKEM: u16 = 1853;
    pub const DRIVERS_AND_INTERRUPTS: u16 = 1422;
    pub const DR_QUINE: u16 = 1418;
    pub const DSLR: u16 = 1453;
    pub const ELECTRONICS_OLD: u16 = 1673;
    pub const ELECTRONIQUE: u16 = 1848;
    pub const EU_ACEITO_LISBOA: u16 = 2397;
    pub const EU_ACEITO_PORTO: u16 = 2396;
    pub const EXAM_TEST_42: u16 = 2238;
    pub const EXAM_TEST_43: u16 = 2374;
    pub const EXAM_TEST_44: u16 = 2556;
    pub const EXAM_TEST_45: u16 = 2557;
    pub const EXAM_TEST_46: u16 = 2558;
    pub const EXPERT_SYSTEM: u16 = 1384;
    pub const FAMINE: u16 = 1430;
    pub const FILESYSTEM: u16 = 1423;
    pub const FIX_ME: u16 = 1437;
    pub const FREDDIE_MERCURY: u16 = 1883;
    pub const FRIED_EGGS: u16 = 1960;
    pub const FR_APPRENTISSAGE_RNCP_6_1_AN: u16 = 2338;
    pub const FR_APPRENTISSAGE_RNCP_6_2_ANS: u16 = 2339;
    pub const FR_APPRENTISSAGE_RNCP_7_1_AN: u16 = 2340;
    pub const FR_APPRENTISSAGE_RNCP_7_2_ANS: u16 = 2341;
    pub const FTL_QUANTUM: u16 = 2225;
    pub const FT_ALITY: u16 = 1407;
    pub const FT_CONTAINERS: u16 = 1335;
    pub const FT_HANGOUTS: u16 = 1379;
    pub const FT_KALMAN: u16 = 2098;
    pub const FT_KLEENE: u16 = 2531;
    pub const FT_LEX: u16 = 2380;
    pub const FT_LINEAR_REGRESSION: u16 = 1391;
    pub const FT_LINUX: u16 = 1415;
    pub const FT_LS: u16 = 1479;
    pub const FT_MALCOLM: u16 = 1840;
    pub const FT_MICROSERVICES: u16 = 2394;
    pub const FT_MINECRAFT: u16 = 2126;
    pub const FT_NEWTON: u16 = 1962;
    pub const FT_NMAP: u16 = 1400;
    pub const FT_PING: u16 = 1397;
    pub const FT_RUN: u16 = 1387;
    pub const FT_SCRIPT: u16 = 1466;
    pub const FT_SELECT: u16 = 1469;
    pub const FT_SEOUL_TEST: u16 = 2006;
    pub const FT_SERVER: u16 = 1328;
    pub const FT_SERVICES: u16 = 1329;
    pub const FT_SH: u16 = 1854;
    pub const FT_SHIELD: u16 = 1447;
    pub const FT_SHMUP_: u16 = 2214;
    pub const FT_SQUADS: u16 = 1764;
    pub const FT_SSL_DES: u16 = 1452;
    pub const FT_SSL_MD5: u16 = 1451;
    pub const FT_SSL_RSA: u16 = 1450;
    pub const FT_TRACEROUTE: u16 = 1399;
    pub const FT_TURING: u16 = 1403;
    pub const FT_VOX: u16 = 1449;
    pub const FT_YACC: u16 = 2381;
    pub const FWA: u16 = 2347;
    pub const GBMU: u16 = 1411;
    pub const GOMOKU: u16 = 1383;
    pub const GUIMP: u16 = 1455;
    pub const H42N42: u16 = 1429;
    pub const HIVE_INTERNSHIP: u16 = 2239;
    pub const HIVE_INTERNSHIP_COMPANY_FINAL_EVALUATION: u16 = 2243;
    pub const HIVE_INTERNSHIP_COMPANY_MID_EVALUATION: u16 = 2242;
    pub const HIVE_INTERNSHIP_CONTRACT_UPLOAD: u16 = 2240;
    pub const HIVE_INTERNSHIP_DURATION: u16 = 2241;
    pub const HIVE_INTERNSHIP_PEER_VIDEO: u16 = 2244;
    pub const HIVE_STARTUP_INTERNSHIP: u16 = 2245;
    pub const HIVE_STARTUP_INTERNSHIP_CONTRACT_UPLOAD: u16 = 2246;
    pub const HIVE_STARTUP_INTERNSHIP_DURATION: u16 = 2247;
    pub const HIVE_STARTUP_INTERNSHIP_ENTREPRENEURSHIP_FINAL_EVALUATION: u16 = 2249;
    pub const HIVE_STARTUP_INTERNSHIP_ENTREPRENEURSHIP_MID_EVALUATION: u16 = 2248;
    pub const HIVE_STARTUP_INTERNSHIP_PEER_VIDEO: u16 = 2250;
    pub const HOTRACE: u16 = 2070;
    pub const HUMANGL: u16 = 1394;
    pub const HYPERTUBE: u16 = 1402;
    pub const INCEPTION_OF_THINGS: u16 = 2064;
    pub const INTERNSHIP_I: u16 = 1638;
    pub const INTERNSHIP_II: u16 = 1644;
    pub const INTERNSHIP_II_COMPANY_FINAL_EVALUATION: u16 = 1648;
    pub const INTERNSHIP_II_COMPANY_MID_EVALUATION: u16 = 1647;
    pub const INTERNSHIP_II_CONTRACT_UPLOAD: u16 = 1645;
    pub const INTERNSHIP_II_DURATION: u16 = 1646;
    pub const INTERNSHIP_II_PEER_VIDEO: u16 = 1649;
    pub const INTERNSHIP_I_COMPANY_FINAL_EVALUATION: u16 = 1642;
    pub const INTERNSHIP_I_COMPANY_MID_EVALUATION: u16 = 1641;
    pub const INTERNSHIP_I_CONTRACT_UPLOAD: u16 = 1640;
    pub const INTERNSHIP_I_DURATION: u16 = 1639;
    pub const INTERNSHIP_I_PEER_VIDEO: u16 = 1643;
    pub const IN_THE_SHADOWS: u16 = 1409;
    pub const JAVA_MODULE_00: u16 = 2320;
    pub const JAVA_MODULE_01: u16 = 2321;
    pub const JAVA_MODULE_02: u16 = 2322;
    pub const JAVA_MODULE_03: u16 = 2329;
    pub const JAVA_MODULE_04: u16 = 2330;
    pub const JAVA_MODULE_05: u16 = 2331;
    pub const JAVA_MODULE_06: u16 = 2332;
    pub const JAVA_MODULE_07: u16 = 2333;
    pub const JAVA_MODULE_08: u16 = 2334;
    pub const JAVA_MODULE_09: u16 = 2335;
    pub const KFS_1: u16 = 1425;
    pub const KFS_2: u16 = 1424;
    pub const KFS_3: u16 = 1426;
    pub const KFS_4: u16 = 1431;
    pub const KFS_5: u16 = 1432;
    pub const KFS_6: u16 = 1438;
    pub const KFS_7: u16 = 1439;
    pub const KFS_8: u16 = 1440;
    pub const KFS_9: u16 = 1441;
    pub const KFS_X: u16 = 1442;
    pub const KRPSIM: u16 = 1392;
    pub const LEAFFLICTION: u16 = 2372;
    pub const LEARN2_SLITHER: u16 = 2551;
    pub const LEM_IN: u16 = 1470;
    pub const LEM_IPC: u16 = 1464;
    pub const LIBASM: u16 = 1330;
    pub const LIBFTPP: u16 = 2550;
    pub const LIBUNIT: u16 = 2066;
    pub const LITTLE_PENGUIN_1: u16 = 1416;
    pub const MALLOC: u16 = 1468;
    pub const MANUFACTURING_LAB_HARDWARE_PART: u16 = 2263;
    pub const MATCHA: u16 = 1401;
    pub const MATRIX: u16 = 2077;
    pub const MATT_DAEMON: u16 = 1420;
    pub const MESSAGEQUEUE: u16 = 2353;
    pub const MESSAGE_QUEUE: u16 = 1958;
    pub const MICROSHOP: u16 = 2373;
    pub const MICRO_FORENS_X: u16 = 2527;
    pub const MICRO_SERVICES: u16 = 1947;
    pub const MINICOMP: u16 = 2607;
    pub const MOBILE: u16 = 2355;
    pub const MOBILE_0_BASIC_OF_THE_MOBILE_APPLICATION: u16 = 2354;
    pub const MOBILE_1_STRUCTURE_AND_LOGIC: u16 = 2356;
    pub const MOBILE_2_API_AND_DATA: u16 = 2357;
    pub const MOBILE_3_DESIGN: u16 = 2358;
    pub const MOBILE_4_AUTH_AND_DATA_BASE: u16 = 2359;
    pub const MOBILE_5_MANAGE_DATA_AND_DISPLAY: u16 = 2360;
    pub const MOD1: u16 = 1462;
    pub const MULTILAYER_PERCEPTRON: u16 = 1457;
    pub const MUSIC_ROOM: u16 = 1427;
    pub const MY_SPOTIFY: u16 = 1943;
    pub const NIBBLER: u16 = 1386;
    pub const NM: u16 = 1467;
    pub const N_PUZZLE: u16 = 1385;
    pub const OCAML_BASIC_SYNTAX_AND_SEMANTICS_0: u16 = 2230;
    pub const OCAML_FUNCTOR_1: u16 = 2279;
    pub const OCAML_IMPERATIVE_FEATURES_1: u16 = 2252;
    pub const OCAML_MONOIDS_AND_MONADS_3: u16 = 2282;
    pub const OCAML_OBJECT_ORIENTED_PROGRAMMING_1: u16 = 2280;
    pub const OCAML_OBJECT_ORIENTED_PROGRAMMING_2: u16 = 2281;
    pub const OCAML_O_CAMLS_MODULES_LANGUAGE_1: u16 = 2251;
    pub const OCAML_PATTERN_MATCHING_AND_DATA_TYPES_0: u16 = 2237;
    pub const OCAML_RECURSION_AND_HIGHER_ORDER_FUNCTIONS_0: u16 = 2234;
    pub const OLD_IRC: u16 = 1684;
    pub const OLD_PHILOSOPHERS: u16 = 1683;
    pub const OPEN_PROJECT: u16 = 1635;
    pub const OPEN_PROJECT_COMPLETE_THE_PROJECT: u16 = 1637;
    pub const OPEN_PROJECT_DEFINE_YOUR_SUBJECT: u16 = 1636;
    pub const OVERRIDE: u16 = 1448;
    pub const PARTICLE_SYSTEM: u16 = 1410;
    pub const PART_TIME_I: u16 = 1650;
    pub const PART_TIME_II: u16 = 1656;
    pub const PART_TIME_II_COMPANY_FINAL_EVALUATION: u16 = 1660;
    pub const PART_TIME_II_COMPANY_MID_EVALUATION: u16 = 1659;
    pub const PART_TIME_II_CONTRACT_UPLOAD: u16 = 1657;
    pub const PART_TIME_II_DURATION: u16 = 1658;
    pub const PART_TIME_II_PEER_VIDEO: u16 = 1661;
    pub const PART_TIME_I_COMPANY_FINAL_EVALUATION: u16 = 1653;
    pub const PART_TIME_I_COMPANY_MID_EVALUATION: u16 = 1654;
    pub const PART_TIME_I_CONTRACT_UPLOAD: u16 = 1651;
    pub const PART_TIME_I_DURATION: u16 = 1652;
    pub const PART_TIME_I_PEER_VIDEO: u16 = 1655;
    pub const PEACE_BREAK: u16 = 2552;
    pub const PESTILENCE: u16 = 1443;
    pub const PISCINE_DATA_SCIENCE: u16 = 2295;
    pub const PISCINE_DJANGO: u16 = 2189;
    pub const PISCINE_JAVA: u16 = 1799;
    pub const PISCINE_OBJECT: u16 = 2364;
    pub const PISCINE_OBJECT_MODULE_00_ENCAPSULATION: u16 = 2365;
    pub const PISCINE_OBJECT_MODULE_01_RELATIONSHIP: u16 = 2366;
    pub const PISCINE_OBJECT_MODULE_02_UML: u16 = 2367;
    pub const PISCINE_OBJECT_MODULE_03_SOLID: u16 = 2368;
    pub const PISCINE_OBJECT_MODULE_04_DESIGN_PATTERN: u16 = 2369;
    pub const PISCINE_OBJECT_MODULE_05_PRACTICAL_WORK: u16 = 2370;
    pub const PISCINE_OCAML: u16 = 2228;
    pub const PISCINE_PHP_SYMFONY: u16 = 1481;
    pub const PISCINE_RO_R: u16 = 2179;
    pub const PISCINE_RUBY_ON_RAILS: u16 = 1482;
    pub const PISCINE_SYMFONY: u16 = 2199;
    pub const PROCESS_AND_MEMORY: u16 = 1421;
    pub const PYTHON_0_STARTING: u16 = 2268;
    pub const PYTHON_1_ARRAY: u16 = 2269;
    pub const PYTHON_2_DATA_TABLE: u16 = 2270;
    pub const PYTHON_3_OOP: u16 = 2271;
    pub const PYTHON_4_DOD: u16 = 2272;
    pub const PYTHON_FOR_DATA_SCIENCE: u16 = 2267;
    pub const RAINFALL: u16 = 1417;
    pub const READY_SET_BOOLE: u16 = 2076;
    pub const RED_TETRIS: u16 = 1428;
    pub const RESTFUL: u16 = 2351;
    pub const RETRO_MFA: u16 = 2210;
    pub const RO_R_0_INITIATION: u16 = 2180;
    pub const RO_R_0_OOB: u16 = 2182;
    pub const RO_R_0_STARTING: u16 = 2181;
    pub const RO_R_1_BASE_RAILS: u16 = 2184;
    pub const RO_R_1_GEMS: u16 = 2183;
    pub const RO_R_2_SQL: u16 = 2185;
    pub const RO_R_3_ADVANCED: u16 = 2187;
    pub const RO_R_3_FINAL: u16 = 2188;
    pub const RO_R_3_SESSIONS: u16 = 2186;
    pub const RT: u16 = 1855;
    pub const RUBIK: u16 = 1393;
    pub const RUSHES: u16 = 2065;
    pub const SCOP: u16 = 1390;
    pub const SEOUL_LABS_MEMBER: u16 = 1993;
    pub const SHADERPIXEL: u16 = 1454;
    pub const SNOW_CRASH: u16 = 1404;
    pub const SO_LONG: u16 = 2009;
    pub const SPRING_BOOT: u16 = 2350;
    pub const STARTUP_INTERNSHIP: u16 = 1662;
    pub const STARTUP_INTERNSHIP_CONTRACT_UPLOAD: u16 = 1663;
    pub const STARTUP_INTERNSHIP_DURATION: u16 = 1664;
    pub const STARTUP_INTERNSHIP_PEER_VIDEO: u16 = 1667;
    pub const STARTUP_INTERNSHIP_TUTOR_FINAL_EVALUATION: u16 = 1666;
    pub const STARTUP_INTERNSHIP_TUTOR_MID_EVALUATION: u16 = 1665;
    pub const STRACE: u16 = 1388;
    pub const SWIFTY_COMPANION: u16 = 1395;
    pub const SWIFTY_PROTEINS: u16 = 1406;
    pub const SWINGY: u16 = 1436;
    pub const SYMFONY_0_INITIATION: u16 = 2200;
    pub const SYMFONY_0_OOB: u16 = 2202;
    pub const SYMFONY_0_STARTING: u16 = 2201;
    pub const SYMFONY_1_BASE_SYMFONY_: u16 = 2204;
    pub const SYMFONY_1_COMPOSER: u16 = 2203;
    pub const SYMFONY_2_SQL: u16 = 2205;
    pub const SYMFONY_3_ADVANCED: u16 = 2207;
    pub const SYMFONY_3_FINAL: u16 = 2208;
    pub const SYMFONY_3_SESSIONS: u16 = 2206;
    pub const TASKMASTER: u16 = 1381;
    pub const TINKY_WINKEY: u16 = 2097;
    pub const TOKENIZER: u16 = 2485;
    pub const TOKENIZE_ART: u16 = 2600;
    pub const TOTAL_PERSPECTIVE_VORTEX: u16 = 1460;
    pub const TWEET: u16 = 2319;
    pub const UBER: u16 = 1955;
    pub const UNITY: u16 = 2288;
    pub const UNITY_0_THE_BASICS_UNITY_TOOLS: u16 = 2286;
    pub const UNITY_1_3_D_PHYSICS_TAGS_LAYERS_AND_SCENE: u16 = 2284;
    pub const UNITY_2_2_D_ENVIRONMENT_TILES_AND_SPRITES: u16 = 2289;
    pub const UNITY_3_ADVANCED_INPUTS_AND_2_D_GUI: u16 = 2290;
    pub const UNITY_4_ANIMATIONS_AND_SOUND: u16 = 2292;
    pub const UNITY_5_SINGLETON_PLAYER_PREFS_AND_COROUTINES: u16 = 2287;
    pub const UNITY_6_NAVMESH_LIGHT_SOUND_AND_CAMERA: u16 = 2285;
    pub const UNLEASH_THE_BOX: u16 = 2393;
    pub const USERSPACE_DIGRESSIONS: u16 = 1456;
    pub const VERY_REAL_ENGINE: u16 = 2553;
    pub const WAR: u16 = 1444;
    pub const WOLFSBURG_I_ACCEPT: u16 = 2078;
    pub const WONG_KAR_WAI: u16 = 2122;
    pub const WOODY_WOODPACKER: u16 = 1419;
    pub const WORDLE: u16 = 2173;
    pub const XV: u16 = 1408;
    pub const YASL: u16 = 2136;
    pub const ZAPPY: u16 = 1463;
    pub const _APPRENTISSAGE_2_ANS_2ÈME_ANNÉE: u16 = 1865;
    pub const _BGP_AT_DOORS_OF_AUTONOMOUS_SYSTEMS_IS_SIMPLE: u16 = 2071;
    pub const _CYBERSECURITY_STOCKHOLM_MALWARE: u16 = 2337;
    pub const _UNDERSTANDING_CUSTOMER: u16 = 1956;
}
