/* write event chain */
uint32_t writeencounter(FILE* fd, AList* al_combat, AList* al_agents, uint32_t start_type) {
	/* file byte index */
	uint32_t fdindex = 0;

	/* write header (16 bytes) */
	char header[32];
	asnprintf(&header[0], 32, "EVTC%s", g->m_version);
	header[12] = 0;
	*(uint16_t*)(&header[13]) = g->m_game->m_area_cbt_cid;
	header[15] = 0;
	fseek(fd, 0, SEEK_SET);
	fwrite(&header[0], 16, 1, fd);
	fdindex += 16;

	/* define agent. stats range from 0-10 */
	typedef struct evtc_agent {
		uint64_t addr;
		uint32_t prof;
		uint32_t is_elite;
		int16_t toughness;
		int16_t concentration;
		int16_t healing;
		int16_t pad1;
		int16_t condition;
		int16_t pad2;
		char name[64];
	} evtc_agent;

	/* count agents */
	alisti itr;
	uint32_t ag_count = 0;
	int32_t max_toughness = 1;
	int32_t max_healing = 1;
	int32_t max_condition = 1;
	al_agents->IInitTail(&itr);
	evtc_agent* evag = (evtc_agent*)al_agents->INext(&itr);
	while (evag) {
		ag_count += 1;
		max_toughness = MAX(evag->toughness, max_toughness);
		max_healing = MAX(evag->healing, max_healing);
		max_condition = MAX(evag->condition, max_condition);
		evag = (evtc_agent*)al_agents->INext(&itr);
	}

	/* write agent count */
	fseek(fd, fdindex, SEEK_SET);
	fwrite(&ag_count, sizeof(uint32_t), 1, fd);
	fdindex += sizeof(uint32_t);

	/* write agent array */
	al_agents->IInitTail(&itr);
	evag = (evtc_agent*)al_agents->INext(&itr);
	while (evag) {
		evag->toughness = ((evag->toughness * 100) / max_toughness) / 10;
		evag->healing = ((evag->healing * 100) / max_healing) / 10;
		evag->condition = ((evag->condition * 100) / max_condition) / 10;
		fseek(fd, fdindex, SEEK_SET);
		fwrite(evag, sizeof(evtc_agent), 1, fd);
		fdindex += sizeof(evtc_agent);
		evag = (evtc_agent*)al_agents->INext(&itr);
	}

	/* count skills */
	uint8_t* sk_mask = (uint8_t*)acalloc(sizeof(uint8_t) * 65535);
	al_combat->IInitTail(&itr);
	cbtevent* cbtev = (cbtevent*)al_combat->INext(&itr);
	uint32_t skcount = 0;
	while (cbtev) {
		if (!sk_mask[cbtev->skillid]) {
			skcount += 1;
			sk_mask[cbtev->skillid] = 1;
		}
		cbtev = (cbtevent*)al_combat->INext(&itr);
	}

	/* write skill count */
	fseek(fd, fdindex, SEEK_SET);
	fwrite(&skcount, sizeof(uint32_t), 1, fd);
	fdindex += sizeof(uint32_t);

	/* define skill */
	typedef struct skill {
		int32_t id;
		char name[64];
	} skill;

	/* write skill array */
	skcount = 0;
	while (skcount < 65535) {
		if (sk_mask[skcount]) {
			skill temp;
			memset(&temp, 0, sizeof(skill));
			temp.id = g->m_game->m_ar_sks[skcount].skillid;
			asnprintf(&temp.name[0], RB_NAME_LEN, "%s", g->m_game->m_ar_sks[skcount].name);
			fseek(fd, fdindex, SEEK_SET);
			fwrite(&temp, sizeof(skill), 1, fd);
			fdindex += sizeof(skill);
		}
		skcount += 1;
	}
	acfree(sk_mask);

	/* write combat log */
	al_combat->IInitTail(&itr);
	cbtev = (cbtevent*)al_combat->INext(&itr);
	while (cbtev) {
		fseek(fd, fdindex, SEEK_SET);
		fwrite(cbtev, sizeof(cbtevent), 1, fd);
		fdindex += sizeof(cbtevent);
		cbtev = (cbtevent*)al_combat->INext(&itr);
	}

	/* cleanup */
	fclose(fd);
	return fdindex;
}
