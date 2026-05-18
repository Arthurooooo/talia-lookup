<script lang="ts">
  import { ChevronLeft, Mail, Phone, Gift, GraduationCap, FlagTriangleRight, Hash, FileText, CalendarDays, Eye, ListFilter, Euro, Zap, UserCheck, Users, Star, Phone as PhoneIcon, PhoneCall, Flag, Antenna, ArrowDownRight } from '@lucide/svelte';
  import Avatar from './Avatar.svelte';
  import PillBadge from './PillBadge.svelte';
  import Chip from './Chip.svelte';
  import StatTile from './StatTile.svelte';
  import ContractCard from './ContractCard.svelte';
  import SessionCard from './SessionCard.svelte';
  import MeetingRow from './MeetingRow.svelte';
  import type { BubbleUser, BubbleContract, BubbleSession, BubbleMeeting, BubbleClass } from '$lib/types';
  import { fullName, bestEmail, initials } from '$lib/types';
  import { longDate, ageFromBirth, currency, isContractActive, isSessionActive } from '$lib/format';

  let {
    user,
    contracts,
    sessions,
    meetings,
    accountManager,
    activeTeacher,
    activeClass,
    isLoading,
    onBack
  }: {
    user: BubbleUser;
    contracts: BubbleContract[];
    sessions: BubbleSession[];
    meetings: BubbleMeeting[];
    accountManager?: BubbleUser;
    activeTeacher?: BubbleUser;
    activeClass?: BubbleClass;
    isLoading: boolean;
    onBack: () => void;
  } = $props();

  let showAllContracts = $state(false);
  let showAllSessions = $state(false);

  const isLead = $derived(!user.studentStatus);
  const activeContracts = $derived(contracts.filter(isContractActive));
  const activeSessions = $derived(sessions.filter(isSessionActive));
  const shownContracts = $derived(showAllContracts ? contracts : activeContracts);
  const shownSessions = $derived(showAllSessions ? sessions : activeSessions);

  const revenueTotal = $derived(
    contracts.filter(c => !c.isCancelled).reduce((sum, c) => sum + (c.contractAmount ?? 0), 0)
  );
  const activeRevenue = $derived(
    activeContracts.reduce((sum, c) => sum + (c.contractAmount ?? 0), 0)
  );

  const shouldShowFunnel = $derived(
    meetings.length > 0 || !!user.etatLead || !!user.leadStatus || !!user.leadSource
  );

  // Group meetings by type, keep latest per type
  const groupedMeetings = $derived.by(() => {
    const groups: Record<string, BubbleMeeting[]> = {};
    for (const m of meetings) {
      const key = m.meetingType ?? 'Autre';
      (groups[key] ??= []).push(m);
    }
    return Object.entries(groups).map(([type, list]) => {
      const sorted = [...list].sort((a, b) =>
        new Date(b.startDate ?? 0).getTime() - new Date(a.startDate ?? 0).getTime()
      );
      return { type, latest: sorted[0], count: list.length };
    }).sort((a, b) =>
      new Date(b.latest.startDate ?? 0).getTime() - new Date(a.latest.startDate ?? 0).getTime()
    );
  });

  function starString(r: number): string {
    const n = Math.max(0, Math.min(5, Math.round(r)));
    return '★'.repeat(n) + '☆'.repeat(5 - n);
  }
</script>

<div style="display: flex; flex-direction: column; gap: 14px; height: 100%; overflow: hidden;">
  <!-- Header card -->
  <div class="card">
    <div class="flex items-center gap-12">
      <button type="button" class="icon-btn" onclick={onBack}><ChevronLeft size={16} /></button>
      <Avatar url={user.profilePicture} initials={initials(user)} size={44} />
      <div style="flex: 1;">
        <div class="fw-semibold" style="font-size: 17px;">{fullName(user)}</div>
        <div class="flex items-center gap-6" style="margin-top: 4px;">
          {#if user.studentStatus}<PillBadge text={user.studentStatus} tint="blue" />{/if}
          {#if !user.studentStatus && user.leadStatus}<PillBadge text={user.leadStatus} tint="gray" />{/if}
          {#if user.role && user.role.toLowerCase() !== 'lead'}<PillBadge text={user.role} tint="purple" />{/if}
        </div>
      </div>
      {#if isLoading}<div class="text-secondary text-2xs">…</div>{/if}
    </div>
    <div class="flex flex-wrap gap-6" style="margin-top: 10px;">
      {#if bestEmail(user)}<Chip text={bestEmail(user)!} Icon={Mail} tint="blue" copyable />{/if}
      {#if user.phoneNumber}<Chip text={user.phoneNumber} Icon={Phone} tint="green" copyable />{/if}
      {#if user.birthDate}
        {@const age = ageFromBirth(user.birthDate)}
        <Chip text={`${age} ans · ${longDate(user.birthDate)}`} Icon={Gift} tint="pink" />
      {/if}
      {#if !isLead && user.becameStudentDate}
        <Chip text={`Élève depuis ${longDate(user.becameStudentDate)}`} Icon={GraduationCap} tint="indigo" />
      {/if}
      {#if !isLead && user.currentTrainingEndDate}
        <Chip text={`Fin parcours ${longDate(user.currentTrainingEndDate)}`} Icon={FlagTriangleRight} tint="orange" />
      {/if}
      <Chip text={'ID …' + user.id.slice(-6)} Icon={Hash} tint="gray" clipboardText={user.id} copyable mono />
    </div>
  </div>

  <div class="scroll" style="flex: 1; display: flex; flex-direction: column; gap: 18px;">
    <!-- Stats -->
    <div class="flex flex-wrap gap-6">
      {#if revenueTotal > 0}
        <StatTile label={isLead ? 'CA potentiel' : 'CA total'} value={currency(revenueTotal)} Icon={Euro} tint="green" />
      {/if}
      {#if activeRevenue > 0 && activeRevenue !== revenueTotal}
        <StatTile label="CA actif" value={currency(activeRevenue)} Icon={Zap} tint="blue" />
      {/if}
      {#if accountManager}
        <StatTile label="AM" value={fullName(accountManager)} Icon={UserCheck} tint="indigo" />
      {/if}
      {#if activeClass?.name}
        <StatTile label="Classe" value={activeClass.name} Icon={Users} tint="orange" />
      {/if}
      {#if activeTeacher}
        <StatTile label="Prof principal" value={fullName(activeTeacher)} Icon={UserCheck} tint="pink" />
      {/if}
    </div>

    <!-- Funnel -->
    {#if shouldShowFunnel}
      <div>
        <div class="section-h" style="margin-bottom: 8px;">
          <ArrowDownRight class="icon" color="var(--teal)" />
          Funnel commercial
        </div>
        <div class="funnel">
          <div class="flex flex-wrap gap-6">
            {#if user.etatLead}<Chip text={`État: ${user.etatLead}`} Icon={Flag} tint="teal" />{/if}
            {#if user.leadStatus && isLead}<Chip text={`Statut: ${user.leadStatus}`} Icon={UserCheck} tint="indigo" />{/if}
            {#if user.leadSource}<Chip text={`Source: ${user.leadSource}`} Icon={Antenna} tint="gray" />{/if}
            {#if user.leadRating && user.leadRating > 0}<Chip text={`Note: ${starString(user.leadRating)}`} Icon={Star} tint="yellow" />{/if}
            {#if user.lastCallDate}<Chip text={`Dernier appel: ${longDate(user.lastCallDate)}`} Icon={PhoneIcon} tint="green" />{/if}
            {#if user.followUpCallDate && new Date(user.followUpCallDate) > new Date()}<Chip text={`À rappeler: ${longDate(user.followUpCallDate)}`} Icon={PhoneCall} tint="orange" />{/if}
          </div>
          {#if groupedMeetings.length}
            <div style="border-top: 1px dashed rgba(127,127,127,0.2); padding-top: 6px; display: flex; flex-direction: column; gap: 6px;">
              {#each groupedMeetings as g (g.type)}
                <MeetingRow group={g} />
              {/each}
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Contracts -->
    <div>
      <div class="section-h" style="margin-bottom: 8px;">
        <FileText class="icon" color="var(--blue)" />
        Contrats
        <span class="count">{shownContracts.length}/{contracts.length}</span>
        <div class="flex-1"></div>
        {#if contracts.length > activeContracts.length && contracts.length > 0}
          <button class="btn-link" onclick={() => (showAllContracts = !showAllContracts)}>
            {#if showAllContracts}<Eye size={11} /> Toutes
            {:else}<ListFilter size={11} /> Actives{/if}
          </button>
        {/if}
      </div>
      {#if shownContracts.length === 0 && !isLoading}
        <div class="text-secondary text-sm">{contracts.length === 0 ? 'Aucun contrat' : 'Aucun contrat actif'}</div>
      {:else}
        <div style="display: flex; flex-direction: column; gap: 6px;">
          {#each shownContracts as c (c.id)}
            <ContractCard contract={c} />
          {/each}
        </div>
      {/if}
    </div>

    <!-- Sessions -->
    <div>
      <div class="section-h" style="margin-bottom: 8px;">
        <CalendarDays class="icon" color="var(--purple)" />
        Sessions
        <span class="count">{shownSessions.length}/{sessions.length}</span>
        <div class="flex-1"></div>
        {#if sessions.length > activeSessions.length && sessions.length > 0}
          <button class="btn-link" onclick={() => (showAllSessions = !showAllSessions)}>
            {#if showAllSessions}<Eye size={11} /> Toutes
            {:else}<ListFilter size={11} /> Actives{/if}
          </button>
        {/if}
      </div>
      {#if shownSessions.length === 0 && !isLoading}
        <div class="text-secondary text-sm">{sessions.length === 0 ? 'Aucune session' : 'Aucune session active'}</div>
      {:else}
        <div style="display: flex; flex-direction: column; gap: 6px;">
          {#each shownSessions as s (s.id)}
            <SessionCard session={s} />
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>
