example.cpp has the code i use to write out evtc files.
check file header for environment: evtc bytes for filetype, arcdps build yyyymmdd for compatibility, target species id for boss/manual.
create agents and skills table from evtc, consider dynamic expansion - some nameless/combatless agents may not be written.
skill and agent names use utf8 encoding.
evtc_agent.name is a combo string on players - character name <null> account name <null> subgroup str literal <null>.
add u16 field to the agent table, agents[x].instance_id, initialized to 0.
add u64 fields agents[x].first_aware initialized to 0, and agents[x].last_aware initialized to u64max.
add u64 field agents[x].master_addr, initialized to 0.
if evtc_agent.is_elite == 0xffffffff && upper half of evtc_agent.prof == 0xffff, agent is a gadget with pseudo id as lower half of evtc_agent.prof (volatile id).
if evtc_agent.is_elite == 0xffffffff && upper half of evtc_agent.prof != 0xffff, agent is a character with species id as lower half of evtc_agent.prof (reliable id).
if evtc_agent.is_elite != 0xffffffff, agent is a player with profession as evtc_agent.prof and elite spec as evtc_agent.is_elite.
gadgets do not have true ids and are generated through a combination of gadget parameters - they may collide with characters and should be treated separately.
iterate through all events, assigning instance ids and first/last aware ticks.
set agents[x].instance_id = cbtevent.src_instid where agents[x].addr == cbtevent.src_agent && !cbtevent.is_statechange.
set agents[x].first_aware = cbtevent.time on first event, then all consecutive event times to agents[x].last_aware.
iterate through all events again, this time assigning master agent.
set agents[z].master_agent on encountering cbtevent.src_master_instid != 0.
agents[z].master_addr = agents[x].addr where agents[x].instance_id == cbtevent.src_master_instid && agent[x].first_aware < cbtevent.time < last_aware.
iterate through all events one last time, this time parsing for the data you want.
cbtevent.src_agent and cbtevent.dst_agent should be used to associate event data with local data.
parse event type order should check cbtevent.is_statechange > cbtevent.is_activation > cbtevent.is_buffremove.
cbtevent.is_statechange will do the heavy lifting of non-internal and parser-requested events - make sure to ignore unknown statechange types.
if you would like to see an event not obtainable through my cbtevent struct, let me know.
on cbtevent.is_activation == cancel_fire or cancel_cancel, value will be the ms duration of the time spent in animation.
on cbtevent.is_activation == normal or quickness, value will be the ms duration of the expected animation time.
on cbtevent.is_buffremove, value will be the total duration removed of the stack/s, buff_dmg will be the duration of the longest stack.
if they are all 0, it will be a buff application (!cbtevent.is_buffremove && cbtevent.is_buff) or physical hit (!cbtevent.is_buff).
on physical, cbtevent.value will be the damage done (negative = healing, if ever added).
on physical, cbtevent.result will be the result of the attack.
on buff && !cbtevent.buff_dmg && cbtevent.value, cbtevent.value will be the millisecond duration applied. cbtevent.overstack will be apx overstack in ms.
on buff && !cbtevent.buff_dmg && !cbtevent.value, it is negated condition damage via invuln or resistance.
on buff && cbtevent.buff_dmg && !cbtevent.value, cbtevent.buff_dmg will be the approximate damage done on tick (negative = healing, if ever added).

raid boss ids:
vale guardian  = 0x3C4E
gorseval       = 0x3C45
sabetha        = 0x3C0F
slothasor      = 0x3EFB
trio           = 0x3ED8, 0x3F09, 0x3EFD
matthias       = 0x3EF3
stk            = 
keep construct = 0x3F6B
xera           = 0x3F76, 0x3F9E
cairn          = 0x432A
overseer       = 0x4314
samarog        = 0x4324
deimos         = 0x4302
some fractal ids (thanks /u/hollywood_rag):
https://pastebin.com/3GpfDfGe

base npc stats (for levels 0 to 84):
def = 
{  123,  128,  134,  138,  143,  148,  153,  158,  162,  167,  175,  183,  185,  187,  190,  192,  202,  206,  210,  214,
   220,  224,  239,  245,  250,  256,  261,  267,  285,  291,  311,  320,  328,  337,  356,  365,  385,  394,  402,  411,
   432,  443,  465,  476,  486,  497,  517,  527,  550,  561,  575,  588,  610,  624,  649,  662,  676,  690,  711,  725,
   752,  769,  784,  799,  822,  837,  878,  893,  909,  924,  949,  968, 1011, 1030, 1049, 1067, 1090, 1109, 1155, 1174,
  1223, 1247, 1271, 1295, 1319}
  
pwr = 
{  162,  179,  197,  214,  231,  249,  267,  286,  303,  322,  344,  367,  389,  394,  402,  412,  439,  454,  469,  483,
   500,  517,  556,  575,  593,  612,  622,  632,  672,  684,  728,  744,  761,  778,  820,  839,  885,  905,  924,  943,
   991, 1016, 1067, 1093, 1119, 1145, 1193, 1220, 1275, 1304, 1337, 1372, 1427, 1461, 1525, 1562, 1599, 1637, 1692, 1731,
  1802, 1848, 1891, 1936, 1999, 2045, 2153, 2201, 2249, 2298, 2368, 2424, 2545, 2604, 2662, 2723, 2792, 2854, 2985, 3047,
  3191, 3269, 3348, 3427, 3508}
  
attr = 
{    5,   10,   17,   22,   27,   35,   45,   50,   55,   60,   68,   76,   84,   92,   94,   95,  103,  108,  112,  116,
   123,  129,  140,  147,  153,  160,  166,  171,  186,  192,  208,  219,  230,  238,  253,  259,  274,  279,  284,  290,
   304,  317,  339,  353,  366,  380,  401,  416,  440,  454,  471,  488,  514,  532,  561,  579,  598,  617,  643,  662,
   696,  718,  741,  765,  795,  818,  866,  891,  916,  941,  976, 1004, 1059, 1089, 1119, 1149, 1183, 1214, 1274, 1307,
  1374, 1413, 1453, 1493, 1534}

enums and structs: 
/* iff */
enum iff {
	IFF_FRIEND, // green vs green, red vs red
	IFF_FOE, // green vs red
	IFF_UNKNOWN // something very wrong happened
};

/* combat result (physical) */
enum cbtresult {
	CBTR_NORMAL, // good physical hit
	CBTR_CRIT, // physical hit was crit
	CBTR_GLANCE, // physical hit was glance
	CBTR_BLOCK, // physical hit was blocked eg. mesmer shield 4
	CBTR_EVADE, // physical hit was evaded, eg. dodge or mesmer sword 2
	CBTR_INTERRUPT, // physical hit interrupted something
	CBTR_ABSORB, // physical hit was "invlun" or absorbed eg. guardian elite
	CBTR_BLIND, // physical hit missed
	CBTR_KILLINGBLOW // physical hit was killing hit
};
	
/* combat activation */
enum cbtactivation {
	ACTV_NONE, // not used - not this kind of event
	ACTV_NORMAL, // activation without quickness
	ACTV_QUICKNESS, // activation with quickness
	ACTV_CANCEL_FIRE, // cancel with reaching channel time
	ACTV_CANCEL_CANCEL, // cancel without reaching channel time
	ACTV_RESET // animation completed fully
};

/* combat state change */
enum cbtstatechange {
	CBTS_NONE, // not used - not this kind of event
	CBTS_ENTERCOMBAT, // src_agent entered combat, dst_agent is subgroup
	CBTS_EXITCOMBAT, // src_agent left combat
	CBTS_CHANGEUP, // src_agent is now alive
	CBTS_CHANGEDEAD, // src_agent is now dead
	CBTS_CHANGEDOWN, // src_agent is now downed
	CBTS_SPAWN, // src_agent is now in game tracking range
	CBTS_DESPAWN, // src_agent is no longer being tracked
	CBTS_HEALTHUPDATE, // src_agent has reached a health marker. dst_agent = percent * 10000 (eg. 99.5% will be 9950)
	CBTS_LOGSTART, // log start. value = server unix timestamp **uint32**. buff_dmg = local unix timestamp. src_agent = 0x637261 (arcdps id)
	CBTS_LOGEND, // log end. value = server unix timestamp **uint32**. buff_dmg = local unix timestamp. src_agent = 0x637261 (arcdps id)
	CBTS_WEAPSWAP, // src_agent swapped weapon set. dst_agent = current set id (0/1 water, 4/5 land)
	CBTS_MAXHEALTHUPDATE, // src_agent has had it's maximum health changed. dst_agent = new max health
	CBTS_POINTOFVIEW, // src_agent will be agent of "recording" player
	CBTS_LANGUAGE, // src_agent will be text language
	CBTS_GWBUILD, // src_agent will be game build
	CBTS_SHARDID, // src_agent will be sever shard id
	CBTS_REWARD // src_agent is self, dst_agent is reward id, value is reward type. these are the wiggly boxes that you get
};

/* combat buff remove type */
enum cbtbuffremove {
	CBTB_NONE, // not used - not this kind of event
	CBTB_ALL, // all stacks removed
	CBTB_SINGLE, // single stack removed. disabled on server trigger, will happen for each stack on cleanse
	CBTB_MANUAL, // autoremoved by ooc or allstack (ignore for strip/cleanse calc, use for in/out volume)
};

/* custom skill ids */
enum cbtcustomskill {
	CSK_RESURRECT = 1066, // not custom but important and unnamed
	CSK_BANDAGE = 1175, // personal healing only
	CSK_DODGE = 65001 // will occur in is_activation==normal event
};

/* language */
enum gwlanguage {
	GWL_ENG = 0,
	GWL_FRE = 2,
	GWL_GEM = 3,
	GWL_SPA = 4,
};

/* combat event */
typedef struct cbtevent {
	uint64_t time; /* timegettime() at time of event */
	uint64_t src_agent; /* unique identifier */
	uint64_t dst_agent; /* unique identifier */
	int32_t value; /* event-specific */
	int32_t buff_dmg; /* estimated buff damage. zero on application event */
	uint16_t overstack_value; /* estimated overwritten stack duration for buff application */
	uint16_t skillid; /* skill id */
	uint16_t src_instid; /* agent map instance id */
	uint16_t dst_instid; /* agent map instance id */
	uint16_t src_master_instid; /* master source agent map instance id if source is a minion/pet */
	uint8_t iss_offset; /* internal tracking. garbage */
	uint8_t iss_offset_target; /* internal tracking. garbage */
	uint8_t iss_bd_offset; /* internal tracking. garbage */
	uint8_t iss_bd_offset_target; /* internal tracking. garbage */
	uint8_t iss_alt_offset; /* internal tracking. garbage */
	uint8_t iss_alt_offset_target; /* internal tracking. garbage */
	uint8_t skar; /* internal tracking. garbage */
	uint8_t skar_alt; /* internal tracking. garbage */
	uint8_t skar_use_alt; /* internal tracking. garbage */
	uint8_t iff; /* from iff enum */
	uint8_t buff; /* buff application, removal, or damage event */
	uint8_t result; /* from cbtresult enum */
	uint8_t is_activation; /* from cbtactivation enum */
	uint8_t is_buffremove; /* buff removed. src=relevant, dst=caused it (for strips/cleanses). from cbtr enum */
	uint8_t is_ninety; /* source agent health was over 90% */
	uint8_t is_fifty; /* target agent health was under 50% */
	uint8_t is_moving; /* source agent was moving */
	uint8_t is_statechange; /* from cbtstatechange enum */
	uint8_t is_flanking; /* target agent was not facing source */
	uint8_t is_shields; /* all or part damage was vs barrier/shield */
	uint8_t result_local; /* internal tracking. garbage */
	uint8_t ident_local; /* internal tracking. garbage */
} cbtevent;
