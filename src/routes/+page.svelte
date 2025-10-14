<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { JSONValue } from "$lib/components/FieldEditor.svelte";
  import { detectLanguage, getTranslations, type Language } from "$lib/i18n";
  import {
    getPlayerCoins,
    getPlayerGems,
    setPlayerCoins,
    setPlayerGems,
    spawnArchers,
    spawnFarmers,
    spawnPikemen,
    spawnWorkers,
    destroyPortals,
    exterminateEnemies,
    gotoIsland,
    markTreesForRemoval,
    pimpIsland,
    spawnUnits,
    startFormationAssault,
    takeOverIsland
  } from "$lib/saveEditActions";

  interface LoadResponse {
    path: string;
    data: JSONValue;
  }

  let filePath = $state<string | null>(null);
  let data = $state<JSONValue | null>(null);
  let isLoading = $state(false);
  let statusType = $state<"error" | "success" | null>(null);
  let statusMessage = $state<string | null>(null);
  let backupPath = $state<string | null>(null);
  let selectedKey = $state<string | null>(null);
  let selectedCampaign = $state(0);
  let selectedIsland = $state(0);
  let coinsPlayerIndex = $state(0);
  let coinsAmount = $state(0);
  let gemsPlayerIndex = $state(0);
  let gemsAmount = $state(0);
  let archerCount = $state(1);
  let workerCount = $state(1);
  let farmerCount = $state(1);
  let pikemanCount = $state(1);
  let coinsSelectionSignature = $state<string | null>(null);
  let lastCoinsData = $state<JSONValue | null>(null);
  let gemsSelectionSignature = $state<string | null>(null);
  let lastGemsData = $state<JSONValue | null>(null);
  let archerPlayerIndex = $state(0);
  let gotoCoins = $state(69);
  let gotoGems = $state(3);
  let gotoPikemen = $state(4);
  let gotoFarmers = $state(4);
  let gotoBoats = $state(4);
  let takeOverCoins = $state(69);
  let takeOverArchers = $state(1);
  let takeOverWorkers = $state(1);
  let takeOverOffset = $state(3);
  let formationPlayerIndex = $state(0);
  let formationPosition = $state(0);
  let formationArchers = $state(5);
  let formationPikemen = $state(5);
  let pimpCoins = $state(69);
  let pimpSpawnCount = $state(10);
  let treesPlayerIndex = $state(0);
  let treesCoins = $state(69);
  let comboArchers = $state(0);
  let comboWorkers = $state(0);
  let comboPikemen = $state(0);
  let activeTab = $state<'resources' | 'navigation' | 'combat' | 'construction' | 'recruitment'>('resources');
  let currentLang = $state<Language>(detectLanguage());
  let t = $derived(getTranslations(currentLang));

  interface CampaignSummary {
    label: string;
    islands: number;
  }

  function getDefaultSavePath(): string {
    const platform = navigator.platform.toLowerCase();
    const userAgent = navigator.userAgent.toLowerCase();

    if (platform.includes('mac') || userAgent.includes('mac')) {
      const username = Intl.DateTimeFormat().resolvedOptions().timeZone.split('/')[0] || 'USERNAME';
      return `~/Library/Application Support/nl.noio.kingdom-two-crowns/Release/global-v35`;
    } else if (platform.includes('win') || userAgent.includes('windows')) {
      return `C:\\Users\\USERNAME\\AppData\\LocalLow\\noio\\KingdomTwoCrowns\\Release\\global-v35`;
    } else {
      return `~/.config/unity3d/noio/KingdomTwoCrowns/Release/global-v35`;
    }
  }

  function getOSName(): string {
    const platform = navigator.platform.toLowerCase();
    const userAgent = navigator.userAgent.toLowerCase();

    if (platform.includes('mac') || userAgent.includes('mac')) {
      return 'macOS';
    } else if (platform.includes('win') || userAgent.includes('windows')) {
      return 'Windows';
    } else if (platform.includes('linux') || userAgent.includes('linux')) {
      return 'Linux';
    } else {
      return 'Système détecté';
    }
  }

  let detectedOS = $state(getOSName());
  let defaultPath = $state(getDefaultSavePath());



  const ARCHER_PREFABS = [
    "Prefabs/Characters/Archer",
    "Prefabs/Characters/norselands/Archer_norselands"
  ];
  const WORKER_PREFABS = [
    "Prefabs/Characters/Worker",
    "Prefabs/Characters/norselands/Worker_norselands"
  ];
  const FARMER_PREFABS = [
    "Prefabs/Characters/Farmer",
    "Prefabs/Characters/norselands/Farmer_norselands"
  ];
  const PIKEMAN_PREFABS = [
    "Prefabs/Characters/Pikeman",
    "Prefabs/Characters/norselands/Knight_norselands",
    "Prefabs/Characters/norselands/Pikeman_norselands"
  ];

  function countUnits(
    value: JSONValue | null,
    campaignIndex: number,
    islandIndex: number,
    prefabs: string[]
  ): number | null {
    if (!value || typeof value !== "object" || Array.isArray(value)) {
      return null;
    }

    const campaigns = (value as Record<string, JSONValue>).campaigns;
    if (!Array.isArray(campaigns)) {
      return null;
    }

    const campaign = campaigns[campaignIndex];
    if (!campaign || typeof campaign !== "object" || Array.isArray(campaign)) {
      return null;
    }

    const islands = (campaign as Record<string, JSONValue>)._islands;
    if (!Array.isArray(islands)) {
      return null;
    }

    const island = islands[islandIndex];
    if (!island || typeof island !== "object" || Array.isArray(island)) {
      return null;
    }

    const objects = (island as Record<string, JSONValue>).objects;
    if (!Array.isArray(objects)) {
      return null;
    }

    return objects.reduce<number>((count, entry) => {
      if (entry && typeof entry === "object" && !Array.isArray(entry)) {
        const record = entry as Record<string, JSONValue>;
        const prefabPath = record.prefabPath;
        if (typeof prefabPath === "string" && prefabs.some((prefab) => prefabPath.includes(prefab))) {
          return count + 1;
        }
      }
      return count;
    }, 0);
  }

  function computeCampaignSummaries(value: JSONValue | null): CampaignSummary[] {
    if (!value || typeof value !== "object" || Array.isArray(value)) {
      return [];
    }

    const campaigns = (value as Record<string, JSONValue>).campaigns;
    if (!Array.isArray(campaigns)) {
      return [];
    }

    return campaigns.map((entry, index) => {
      const record = entry && typeof entry === "object" && !Array.isArray(entry)
        ? (entry as Record<string, JSONValue>)
        : {};
      const biome = typeof record.biomeIndex === "number" ? record.biomeIndex : null;
      const islandCount = Array.isArray(record._islands) ? record._islands.length : 0;
      return {
        label: biome === null ? `Campagne ${index + 1}` : `Campagne ${index + 1} (Biome ${biome})`,
        islands: islandCount,
      } satisfies CampaignSummary;
    });
  }

  function computeIslandCount(value: JSONValue | null, campaignIndex: number): number {
    if (!value || typeof value !== "object" || Array.isArray(value)) {
      return 0;
    }

    const campaigns = (value as Record<string, JSONValue>).campaigns;
    if (!Array.isArray(campaigns)) {
      return 0;
    }

    const campaign = campaigns[campaignIndex];
    if (!campaign || typeof campaign !== "object" || Array.isArray(campaign)) {
      return 0;
    }

    const islands = (campaign as Record<string, JSONValue>)._islands;
    return Array.isArray(islands) ? islands.length : 0;
  }

  function resetStatus() {
    statusType = null;
    statusMessage = null;
  }

  function showError(message: string) {
    statusType = "error";
    statusMessage = message;
  }

  function showSuccess(message: string) {
    statusType = "success";
    statusMessage = message;
  }

  async function selectFile() {
    resetStatus();
    backupPath = null;

    try {
      isLoading = true;
      console.info("Sélection d'un fichier global-v35 en cours");
      const response = await invoke<LoadResponse>("select_save_file");
      console.info("Fichier global-v35 chargé", {
        path: response.path,
        topLevelKeys: response.data && typeof response.data === "object" && !Array.isArray(response.data)
          ? Object.keys(response.data as Record<string, JSONValue>)
          : []
      });
      data = response.data;
      filePath = response.path;
      selectedCampaign = 0;
      selectedIsland = 0;
      selectedKey = (() => {
        if (!response.data || typeof response.data !== "object" || Array.isArray(response.data)) {
          return null;
        }

        const keys = Object.keys(response.data as Record<string, JSONValue>);
        return keys.length > 0 ? keys[0] : null;
      })();
      showSuccess("global-v35 loaded");
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error("Erreur lors du chargement de global-v35", error);
      if (message !== "Sélection annulée") {
        showError(message);
      }
    } finally {
      isLoading = false;
    }
  }

  async function reloadFile() {
    resetStatus();
    
    if (!filePath) {
      showError(t.status.noFileLoaded);
      return;
    }

    try {
      isLoading = true;
      console.info("Rechargement du fichier", { path: filePath });
      const response = await invoke<LoadResponse>("load_save_file", { path: filePath });
      data = response.data;
      console.info("Fichier rechargé avec succès");
      showSuccess("Save file reloaded");
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error("Erreur lors du rechargement", error);
      showError(message);
    } finally {
      isLoading = false;
    }
  }



  async function save() {
    resetStatus();

    if (!filePath || !data) {
      showError(t.status.noFileLoaded);
      return;
    }

    try {
      isLoading = true;
      console.info("Sauvegarde en cours", {
        path: filePath,
        hasData: Boolean(data)
      });
      const serialized = JSON.parse(JSON.stringify(data));
      const savedBackupPath = await invoke<string>("save_save_file", { path: filePath, data: serialized });
      console.info("Sauvegarde réussie", { backupPath: savedBackupPath });
      backupPath = savedBackupPath;
      try {
        console.info("Rechargement du fichier pour vérifier les modifications");
        const refreshed = await invoke<LoadResponse>("load_save_file", { path: filePath });
        data = refreshed.data;
        const workerTotal = countUnits(refreshed.data, selectedCampaign, selectedIsland, WORKER_PREFABS);
        const farmerTotal = countUnits(refreshed.data, selectedCampaign, selectedIsland, FARMER_PREFABS);
        const archerTotal = countUnits(refreshed.data, selectedCampaign, selectedIsland, ARCHER_PREFABS);
        const pikemanTotal = countUnits(refreshed.data, selectedCampaign, selectedIsland, PIKEMAN_PREFABS);
        console.info("Rechargement terminé", {
          workers: workerTotal,
          farmers: farmerTotal,
          archers: archerTotal,
          pikemen: pikemanTotal
        });
      } catch (error) {
        console.error("Impossible de recharger le fichier après sauvegarde", error);
      }
      showSuccess(t.status.saveApplied);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error("Erreur lors de la sauvegarde", error);
      showError(message);
    } finally {
      isLoading = false;
    }
  }

  function canSave() {
    return Boolean(filePath && data && !isLoading);
  }

  let campaigns = $derived(computeCampaignSummaries(data));
  let islands = $derived(computeIslandCount(data, selectedCampaign));

  $effect(() => {
    if (selectedCampaign >= campaigns.length) {
      selectedCampaign = 0;
    }

    if (selectedIsland >= islands) {
      selectedIsland = 0;
    }
  });

  $effect(() => {
    if (!data) {
      coinsSelectionSignature = null;
      lastCoinsData = null;
      coinsAmount = 0;
      gemsSelectionSignature = null;
      lastGemsData = null;
      gemsAmount = 0;
      return;
    }

    const signature = `${selectedCampaign}|${selectedIsland}|${coinsPlayerIndex}`;
    const selectionChanged = coinsSelectionSignature !== signature;
    const dataChanged = lastCoinsData !== data;
    if (!selectionChanged && !dataChanged) {
      return;
    }

    coinsSelectionSignature = signature;
    lastCoinsData = data;

    const coinsValue = getPlayerCoins(data, {
      campaignIndex: selectedCampaign,
      islandIndex: selectedIsland,
      playerIndex: coinsPlayerIndex,
    });

    if (typeof coinsValue === "number") {
      coinsAmount = coinsValue;
    } else {
      coinsAmount = 0;
    }
  });

  $effect(() => {
    if (!data) {
      return;
    }

    const signature = `${selectedCampaign}|${selectedIsland}|${gemsPlayerIndex}`;
    const selectionChanged = gemsSelectionSignature !== signature;
    const dataChanged = lastGemsData !== data;
    if (!selectionChanged && !dataChanged) {
      return;
    }

    gemsSelectionSignature = signature;
    lastGemsData = data;

    const gemsValue = getPlayerGems(data, {
      campaignIndex: selectedCampaign,
      islandIndex: selectedIsland,
      playerIndex: gemsPlayerIndex,
    });

    if (typeof gemsValue === "number") {
      gemsAmount = gemsValue;
    } else {
      gemsAmount = 0;
    }
  });

  function requireData(): JSONValue {
    if (!data) {
      throw new Error(t.status.noFileLoaded);
    }

    return data;
  }

  function handleCoinsUpdate() {
    resetStatus();

    try {
      const current = requireData();
      const beforeCoins = getPlayerCoins(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: coinsPlayerIndex,
      });
      console.info("Mise à jour des pièces demandée", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: coinsPlayerIndex,
        avant: beforeCoins,
        cible: coinsAmount,
      });
      const updated = setPlayerCoins(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: coinsPlayerIndex,
        coins: coinsAmount,
      });
      data = updated;
      backupPath = null;
      const afterCoins = getPlayerCoins(updated, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: coinsPlayerIndex,
      });
      if (typeof afterCoins === "number") {
        coinsAmount = afterCoins;
      }
      console.info("Pièces mises à jour", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: coinsPlayerIndex,
        apres: afterCoins,
      });
      showSuccess(t.status.coinsUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleGemsUpdate() {
    resetStatus();

    try {
      const current = requireData();
      const beforeGems = getPlayerGems(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: gemsPlayerIndex,
      });
      console.info("Mise à jour des gemmes demandée", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: gemsPlayerIndex,
        avant: beforeGems,
        cible: gemsAmount,
      });
      const updated = setPlayerGems(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: gemsPlayerIndex,
        gems: gemsAmount,
      });
      data = updated;
      backupPath = null;
      const afterGems = getPlayerGems(updated, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: gemsPlayerIndex,
      });
      if (typeof afterGems === "number") {
        gemsAmount = afterGems;
      }

      console.info("Gemmes mises à jour", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: gemsPlayerIndex,
        apres: afterGems,
      });
      showSuccess(t.status.gemsUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnArchers() {
    resetStatus();

    try {
      const current = requireData();
      const beforeArchers = countUnits(current, selectedCampaign, selectedIsland, ARCHER_PREFABS);
      console.info("Ajout d'archers demandé", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: archerPlayerIndex,
        avant: beforeArchers,
        ajout: archerCount,
      });
      const updated = spawnArchers(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: archerCount,
      });
      data = updated;
      backupPath = null;
      const afterArchers = countUnits(updated, selectedCampaign, selectedIsland, ARCHER_PREFABS);
      console.info("Archers ajoutés", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: archerPlayerIndex,
        apres: afterArchers,
      });
      showSuccess(t.status.archersAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnWorkers() {
    resetStatus();

    try {
      const current = requireData();
      const beforeWorkers = countUnits(current, selectedCampaign, selectedIsland, WORKER_PREFABS);
      console.info("Ajout d'ouvriers demandé", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: archerPlayerIndex,
        avant: beforeWorkers,
        ajout: workerCount,
      });
      const updated = spawnWorkers(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: workerCount,
      });
      data = updated;
      backupPath = null;
      const afterWorkers = countUnits(updated, selectedCampaign, selectedIsland, WORKER_PREFABS);
      console.info("Ouvriers ajoutés", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        apres: afterWorkers,
      });
      showSuccess(t.status.workersAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnFarmers() {
    resetStatus();

    try {
      const current = requireData();
      const beforeFarmers = countUnits(current, selectedCampaign, selectedIsland, FARMER_PREFABS);
      console.info("Ajout de fermiers demandé", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: archerPlayerIndex,
        avant: beforeFarmers,
        ajout: farmerCount,
      });
      const updated = spawnFarmers(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: farmerCount,
      });
      data = updated;
      backupPath = null;
      const afterFarmers = countUnits(updated, selectedCampaign, selectedIsland, FARMER_PREFABS);
      console.info("Fermiers ajoutés", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        apres: afterFarmers,
      });
      showSuccess(t.status.farmersAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnPikemen() {
    resetStatus();

    try {
      const current = requireData();
      const beforePikemen = countUnits(current, selectedCampaign, selectedIsland, PIKEMAN_PREFABS);
      console.info("Ajout de piquiers demandé", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: archerPlayerIndex,
        avant: beforePikemen,
        ajout: pikemanCount,
      });
      const updated = spawnPikemen(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: pikemanCount,
      });
      data = updated;
      backupPath = null;
      const afterPikemen = countUnits(updated, selectedCampaign, selectedIsland, PIKEMAN_PREFABS);
      console.info("Piquiers ajoutés", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        apres: afterPikemen,
      });
      showSuccess(t.status.pikemenAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleGoto() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Changement d'île demandé", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        coins: gotoCoins,
        gems: gotoGems,
        pikemen: gotoPikemen,
        farmers: gotoFarmers,
        boats: gotoBoats,
      });
      const updated = gotoIsland(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        coins: gotoCoins,
        gems: gotoGems,
        pikemen: gotoPikemen,
        farmers: gotoFarmers,
        boats: gotoBoats,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.destinationUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleTakeOver() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Prise de contrôle demandée", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        coins: takeOverCoins,
        archers: takeOverArchers,
        workers: takeOverWorkers,
        offset: takeOverOffset,
      });
      const updated = takeOverIsland(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        coins: takeOverCoins,
        archerCount: takeOverArchers,
        workerCount: takeOverWorkers,
        formationOffset: takeOverOffset,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.islandSecured);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleDestroyPortals() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Destruction des portails demandée", {
        campagne: selectedCampaign,
        ile: selectedIsland,
      });
      const updated = destroyPortals(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.portalsDestroyed);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleExterminate() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Extermination demandée", {
        campagne: selectedCampaign,
        ile: selectedIsland,
      });
      const updated = exterminateEnemies(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.enemiesRemoved);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleFormation() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Formation de combat demandée", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: formationPlayerIndex,
        position: formationPosition,
        archers: formationArchers,
        pikemen: formationPikemen,
      });
      const updated = startFormationAssault(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: formationPlayerIndex,
        position: formationPosition,
        archers: formationArchers,
        pikemen: formationPikemen,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.formationDeployed);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handlePimp() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Pimp my island", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        coins: pimpCoins,
        spawn: pimpSpawnCount,
      });
      const updated = pimpIsland(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        coins: pimpCoins,
        spawnCount: pimpSpawnCount,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.islandUpgraded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleMarkTrees() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Marquage des arbres", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: treesPlayerIndex,
        coins: treesCoins,
      });
      const updated = markTreesForRemoval(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: treesPlayerIndex,
        coins: treesCoins,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.treesMarked);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnCombo() {
    resetStatus();

    const archers = Math.max(0, comboArchers);
    const workers = Math.max(0, comboWorkers);
    const pikemen = Math.max(0, comboPikemen);

    if (archers === 0 && workers === 0 && pikemen === 0) {
      showError(t.status.addAtLeastOneUnit);
      return;
    }

    try {
      const current = requireData();
      console.info("Recrutement combiné", {
        campagne: selectedCampaign,
        ile: selectedIsland,
        joueur: archerPlayerIndex,
        archers,
        workers,
        pikemen,
      });
      const updated = spawnUnits(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        archers,
        workers,
        pikemen,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.unitsAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }
</script>

<main>
  {#if !data}
    <div class="welcome">
      <div class="welcome-content">
        <h1>{t.welcome.title}</h1>
        <p class="subtitle">{t.welcome.subtitle}</p>
        
        <div class="os-detection">
          <div class="os-badge">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M10 2L3 6V10C3 14.55 6.84 18.74 10 19.5C13.16 18.74 17 14.55 17 10V6L10 2Z" fill="currentColor" opacity="0.2"/>
              <path d="M10 2L3 6V10C3 14.55 6.84 18.74 10 19.5C13.16 18.74 17 14.55 17 10V6L10 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            {t.welcome.osDetected} <strong>{detectedOS}</strong>
          </div>
        </div>

        <div class="default-path">
          <div class="path-label">{t.welcome.defaultPath}</div>
          <div class="path-display">
            <code>{defaultPath}</code>
          </div>
          <p class="path-hint">{t.welcome.pathHint}</p>
        </div>

        <div class="welcome-actions">
          <button class="primary large" onclick={selectFile} disabled={isLoading}>
            {#if isLoading}
              <svg class="spinner" width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                <circle cx="10" cy="10" r="8" stroke="currentColor" stroke-width="3" fill="none" stroke-dasharray="50" stroke-dashoffset="25">
                  <animateTransform attributeName="transform" type="rotate" from="0 10 10" to="360 10 10" dur="1s" repeatCount="indefinite"/>
                </circle>
              </svg>
              {t.welcome.loading}
            {:else}
              <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M13 2H5C4.46957 2 3.96086 2.21071 3.58579 2.58579C3.21071 2.96086 3 3.46957 3 4V16C3 16.5304 3.21071 17.0391 3.58579 17.4142C3.96086 17.7893 4.46957 18 5 18H15C15.5304 18 16.0391 17.7893 16.4142 17.4142C16.7893 17.0391 17 16.5304 17 16V6L13 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M13 2V6H17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M10 10V14" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M8 12L10 14L12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              {t.welcome.openFile}
            {/if}
          </button>
        </div>

        <div class="features">
          <div class="feature">
            <div class="feature-icon">💰</div>
            <h3>{t.welcome.features.resources.title}</h3>
            <p>{t.welcome.features.resources.desc}</p>
          </div>
          <div class="feature">
            <div class="feature-icon">🗺️</div>
            <h3>{t.welcome.features.navigation.title}</h3>
            <p>{t.welcome.features.navigation.desc}</p>
          </div>
          <div class="feature">
            <div class="feature-icon">⚔️</div>
            <h3>{t.welcome.features.combat.title}</h3>
            <p>{t.welcome.features.combat.desc}</p>
          </div>
          <div class="feature">
            <div class="feature-icon">🏰</div>
            <h3>{t.welcome.features.construction.title}</h3>
            <p>{t.welcome.features.construction.desc}</p>
          </div>
          <div class="feature">
            <div class="feature-icon">🎖️</div>
            <h3>{t.welcome.features.recruitment.title}</h3>
            <p>{t.welcome.features.recruitment.desc}</p>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <header>
      <div class="header-top">
        <h1>{t.header.title}</h1>
        <div class="header-actions">
          <button onclick={reloadFile} disabled={isLoading} title="Reload current save file">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M14 8C14 11.3137 11.3137 14 8 14C4.68629 14 2 11.3137 2 8C2 4.68629 4.68629 2 8 2C9.86384 2 11.5289 2.87401 12.6 4.2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <path d="M12 1V4.5H8.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
          <button onclick={selectFile} disabled={isLoading}>
            {isLoading ? t.welcome.loading : t.header.changeFile}
          </button>
          <button class="primary" onclick={save} disabled={!canSave()}>
            {t.header.save}
          </button>
        </div>
      </div>
      {#if statusMessage}
        <p class={`status ${statusType ?? ""}`}>{statusMessage}</p>
      {/if}
      {#if backupPath}
        <p class="status info">{t.header.backupCreated} <code>{backupPath}</code></p>
      {/if}
      {#if filePath}
        <p class="file-path">{t.header.filePath} {filePath}</p>
      {/if}
    </header>

    <section class="content">
      <div class="editor-area">
        <section class="context-card card">
          <h3>{t.context.title}</h3>
          <p class="muted">{t.context.description}</p>
          <div class="form-row wrap">
            <label>
              {t.context.campaign}
              <select
                value={selectedCampaign}
                onchange={(event) => (selectedCampaign = Number((event.currentTarget as HTMLSelectElement).value))}
              >
                {#each campaigns as campaign, index}
                  <option value={index}>{campaign.label}</option>
                {/each}
              </select>
            </label>
            <label>
              {t.context.island}
              <select
                value={selectedIsland}
                onchange={(event) => (selectedIsland = Number((event.currentTarget as HTMLSelectElement).value))}
              >
                {#each Array.from({ length: Math.max(0, islands) }) as _, index}
                  <option value={index}>{t.context.island} {index + 1}</option>
                {/each}
              </select>
            </label>
          </div>
        </section>

        <div class="tabs">
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'resources'}
            onclick={() => activeTab = 'resources'}
          >
            {t.tabs.resources}
          </button>
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'navigation'}
            onclick={() => activeTab = 'navigation'}
          >
            {t.tabs.navigation}
          </button>
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'combat'}
            onclick={() => activeTab = 'combat'}
          >
            {t.tabs.combat}
          </button>
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'construction'}
            onclick={() => activeTab = 'construction'}
          >
            {t.tabs.construction}
          </button>
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'recruitment'}
            onclick={() => activeTab = 'recruitment'}
          >
            {t.tabs.recruitment}
          </button>
        </div>

        {#if activeTab === 'resources'}
        <div class="tab-content">
          <section class="card">
            <h3>Ressources</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>Or du joueur</h4>
                <span class="current-value">
                  Actuel: {(() => {
                    const value = getPlayerCoins(data, {
                      campaignIndex: selectedCampaign,
                      islandIndex: selectedIsland,
                      playerIndex: coinsPlayerIndex
                    });
                    return typeof value === "number" ? value : 0;
                  })()}
                </span>
              </div>
              <div class="form-row wrap">
                <label>
                  Joueur
                  <select
                    value={coinsPlayerIndex}
                    onchange={(event) => (coinsPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>Joueur 1</option>
                    <option value={1}>Joueur 2</option>
                  </select>
                </label>
                <label>
                  Pièces
                  <input
                    type="number"
                    min="0"
                    value={coinsAmount}
                    oninput={(event) => (coinsAmount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleCoinsUpdate}>Appliquer</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>Gemmes du joueur</h4>
                <span class="current-value">
                  Actuel: {(() => {
                    const value = getPlayerGems(data, {
                      campaignIndex: selectedCampaign,
                      islandIndex: selectedIsland,
                      playerIndex: gemsPlayerIndex
                    });
                    return typeof value === "number" ? value : 0;
                  })()}
                </span>
              </div>
              <div class="form-row wrap">
                <label>
                  Joueur
                  <select
                    value={gemsPlayerIndex}
                    onchange={(event) => (gemsPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>Joueur 1</option>
                    <option value={1}>Joueur 2</option>
                  </select>
                </label>
                <label>
                  Gemmes
                  <input
                    type="number"
                    min="0"
                    value={gemsAmount}
                    oninput={(event) => (gemsAmount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleGemsUpdate}>Appliquer</button>
              </div>
            </div>
          </section>
        </div>
        {/if}

        {#if activeTab === 'navigation'}
        <div class="tab-content">
          <section class="card">
            <h3>Navigation & conquête</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>Voyage rapide</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  Pièces
                  <input
                    type="number"
                    min="0"
                    value={gotoCoins}
                    oninput={(event) => (gotoCoins = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Gemmes
                  <input
                    type="number"
                    min="0"
                    value={gotoGems}
                    oninput={(event) => (gotoGems = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Piquiers
                  <input
                    type="number"
                    min="0"
                    value={gotoPikemen}
                    oninput={(event) => (gotoPikemen = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Fermiers
                  <input
                    type="number"
                    min="0"
                    value={gotoFarmers}
                    oninput={(event) => (gotoFarmers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Bateaux
                  <input
                    type="number"
                    min="0"
                    value={gotoBoats}
                    oninput={(event) => (gotoBoats = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleGoto}>Mettre à jour la destination</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>Prendre le contrôle</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  Pièces du joueur
                  <input
                    type="number"
                    min="0"
                    value={takeOverCoins}
                    oninput={(event) => (takeOverCoins = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Archers
                  <input
                    type="number"
                    min="0"
                    value={takeOverArchers}
                    oninput={(event) => (takeOverArchers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Ouvriers
                  <input
                    type="number"
                    min="0"
                    value={takeOverWorkers}
                    oninput={(event) => (takeOverWorkers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Décalage formation
                  <input
                    type="number"
                    value={takeOverOffset}
                    oninput={(event) => (takeOverOffset = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleTakeOver}>Nettoyer l'île</button>
              </div>
            </div>

            <div class="card-section">
              <div class="button-group">
                <button type="button" onclick={handleDestroyPortals}>Détruire les portails</button>
                <button type="button" onclick={handleExterminate}>Exterminer les ennemis</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>Élaguer la forêt</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  Joueur
                  <select
                    value={treesPlayerIndex}
                    onchange={(event) => (treesPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>Joueur 1</option>
                    <option value={1}>Joueur 2</option>
                  </select>
                </label>
                <label>
                  Pièces
                  <input
                    type="number"
                    min="0"
                    value={treesCoins}
                    oninput={(event) => (treesCoins = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleMarkTrees}>Marquer les arbres</button>
              </div>
            </div>
          </section>
        </div>
        {/if}

        {#if activeTab === 'combat'}
        <div class="tab-content">
          <section class="card">
            <h3>Combat</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>Formation de bataille</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  Joueur
                  <select
                    value={formationPlayerIndex}
                    onchange={(event) => (formationPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>Joueur 1</option>
                    <option value={1}>Joueur 2</option>
                  </select>
                </label>
                <label>
                  Position X
                  <input
                    type="number"
                    value={formationPosition}
                    oninput={(event) => (formationPosition = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Archers
                  <input
                    type="number"
                    min="0"
                    value={formationArchers}
                    oninput={(event) => (formationArchers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Piquiers
                  <input
                    type="number"
                    min="0"
                    value={formationPikemen}
                    oninput={(event) => (formationPikemen = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleFormation}>Déployer</button>
              </div>
            </div>
          </section>
        </div>
        {/if}

        {#if activeTab === 'construction'}
        <div class="tab-content">
          <section class="card">
            <h3>Construction</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>Pimper l'île</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  Pièces du joueur
                  <input
                    type="number"
                    min="0"
                    value={pimpCoins}
                    oninput={(event) => (pimpCoins = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Unités à recruter
                  <input
                    type="number"
                    min="0"
                    value={pimpSpawnCount}
                    oninput={(event) => (pimpSpawnCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handlePimp}>Mettre à niveau</button>
              </div>
            </div>
          </section>
        </div>
        {/if}

        {#if activeTab === 'recruitment'}
        <div class="tab-content">
          <section class="card">
            <h3>Recrutement</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>Point de rassemblement</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  Joueur
                  <select
                    value={archerPlayerIndex}
                    onchange={(event) => (archerPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>Joueur 1</option>
                    <option value={1}>Joueur 2</option>
                  </select>
                </label>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>Archers</h4>
                <span class="current-value">Actuel: {countUnits(data, selectedCampaign, selectedIsland, ARCHER_PREFABS) ?? 0}</span>
              </div>
              <div class="form-row wrap">
                <label>
                  Quantité
                  <input
                    type="number"
                    min="1"
                    value={archerCount}
                    oninput={(event) => (archerCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnArchers}>Ajouter</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>Ouvriers</h4>
                <span class="current-value">Actuel: {countUnits(data, selectedCampaign, selectedIsland, WORKER_PREFABS) ?? 0}</span>
              </div>
              <div class="form-row wrap">
                <label>
                  Quantité
                  <input
                    type="number"
                    min="1"
                    value={workerCount}
                    oninput={(event) => (workerCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnWorkers}>Ajouter</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>Fermiers</h4>
                <span class="current-value">Actuel: {countUnits(data, selectedCampaign, selectedIsland, FARMER_PREFABS) ?? 0}</span>
              </div>
              <div class="form-row wrap">
                <label>
                  Quantité
                  <input
                    type="number"
                    min="1"
                    value={farmerCount}
                    oninput={(event) => (farmerCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnFarmers}>Ajouter</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>Piquiers</h4>
                <span class="current-value">Actuel: {countUnits(data, selectedCampaign, selectedIsland, PIKEMAN_PREFABS) ?? 0}</span>
              </div>
              <div class="form-row wrap">
                <label>
                  Quantité
                  <input
                    type="number"
                    min="1"
                    value={pikemanCount}
                    oninput={(event) => (pikemanCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnPikemen}>Ajouter</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>Recrutement combiné</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  Archers
                  <input
                    type="number"
                    min="0"
                    value={comboArchers}
                    oninput={(event) => (comboArchers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Ouvriers
                  <input
                    type="number"
                    min="0"
                    value={comboWorkers}
                    oninput={(event) => (comboWorkers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  Piquiers
                  <input
                    type="number"
                    min="0"
                    value={comboPikemen}
                    oninput={(event) => (comboPikemen = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnCombo}>Ajouter les unités</button>
              </div>
            </div>
          </section>
        </div>
        {/if}
      </div>
    </section>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: #0a0a0a;
    color: #ffffff;
    font-family: "Inter", system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    line-height: 1.6;
  }

  main {
    margin: 0 auto;
    padding: 2rem;
    max-width: 1400px;
    min-height: 100vh;
  }

  .welcome {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  .welcome-content {
    max-width: 900px;
    width: 100%;
    text-align: center;
  }

  .welcome h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .subtitle {
    font-size: 1.25rem;
    color: #888888;
    margin-bottom: 3rem;
  }

  .os-detection {
    margin-bottom: 2.5rem;
  }

  .os-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    background: #141414;
    border: 1px solid #2a2a2a;
    border-radius: 12px;
    font-size: 1rem;
    color: #cccccc;
  }

  .os-badge svg {
    color: #3b82f6;
  }

  .os-badge strong {
    color: #ffffff;
  }

  .default-path {
    margin-bottom: 3rem;
    text-align: left;
    background: #141414;
    padding: 1.5rem;
    border-radius: 16px;
    border: 1px solid #2a2a2a;
  }

  .path-label {
    display: block;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #cccccc;
  }

  .path-display {
    margin-bottom: 0.75rem;
  }

  .path-display code {
    display: block;
    padding: 1rem;
    background: #0a0a0a;
    border-radius: 8px;
    font-size: 0.95rem;
    word-break: break-all;
  }

  .path-hint {
    font-size: 0.85rem;
    color: #666666;
    margin: 0;
  }

  .welcome-actions {
    margin-bottom: 4rem;
  }

  button.large {
    padding: 1.25rem 3rem;
    font-size: 1.1rem;
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
  }

  button.large svg {
    flex-shrink: 0;
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1.5rem;
    text-align: center;
  }

  .feature {
    padding: 1.5rem;
    background: var(--color-bg-card);
    border: 2px solid var(--color-border);
    border-radius: 8px;
    transition: all 0.3s ease;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.4);
    position: relative;
  }

  .feature::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: linear-gradient(90deg, transparent 0%, var(--color-gold) 50%, transparent 100%);
    opacity: 0.3;
  }

  .feature:hover {
    border-color: var(--color-gold);
    transform: translateY(-4px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.6), 0 0 30px rgba(212, 175, 55, 0.2);
  }

  .feature-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .feature h3 {
    margin: 0 0 0.5rem;
    font-size: 1.1rem;
    color: var(--color-gold-light);
    font-family: 'Cinzel', serif;
    font-weight: 700;
  }

  .feature p {
    margin: 0;
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    font-family: 'Cinzel', serif;
  }

  header {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 2.5rem;
    padding: 1.5rem;
    background: linear-gradient(135deg, var(--color-bg-card) 0%, var(--color-bg-secondary) 100%);
    border-radius: 8px;
    border: 2px solid var(--color-border);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.5);
    position: relative;
  }

  header::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 20%;
    right: 20%;
    height: 2px;
    background: linear-gradient(90deg, transparent 0%, var(--color-gold) 50%, transparent 100%);
    opacity: 0.4;
  }

  .header-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .header-top h1 {
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }

  .file-path {
    font-size: 0.9rem;
    color: var(--color-stone-light);
    margin: 0;
    font-family: 'Courier New', monospace;
  }

  p {
    margin: 0;
    color: var(--color-text-secondary);
    font-family: 'Cinzel', serif;
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  button.large {
    font-size: 1.1rem;
    padding: 1rem 2rem;
  }

  .status {
    font-weight: 600;
    padding: 1rem 1.5rem;
    border-radius: 6px;
    border: 2px solid;
    font-family: 'Cinzel', serif;
    letter-spacing: 0.02em;
  }

  .status.error {
    color: #ff6b6b;
    background: rgba(139, 37, 0, 0.2);
    border-color: var(--color-error);
  }

  .status.success {
    color: #98fb98;
    background: rgba(107, 142, 35, 0.2);
    border-color: var(--color-success);
  }

  .status.info {
    color: var(--color-gold-light);
    background: rgba(212, 175, 55, 0.1);
    border-color: var(--color-gold);
  }

  .content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .editor-area {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .context-card {
    background: var(--color-bg-card);
    border: 2px solid var(--color-border);
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.4);
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--color-bg-secondary);
    border-radius: 8px;
    border: 2px solid var(--color-border);
    overflow-x: auto;
    scrollbar-width: thin;
  }

  .tabs::-webkit-scrollbar {
    height: 6px;
  }

  .tabs::-webkit-scrollbar-track {
    background: var(--color-bg-primary);
    border-radius: 4px;
  }

  .tabs::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 4px;
  }

  .tab {
    padding: 0.75rem 1.5rem;
    background: transparent;
    border: 2px solid transparent;
    border-radius: 4px;
    color: var(--color-text-muted);
    font-family: 'Cinzel', serif;
    font-weight: 600;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .tab:hover {
    background: rgba(212, 175, 55, 0.1);
    color: var(--color-gold-light);
    border-color: var(--color-border-light);
  }

  .tab.active {
    background: linear-gradient(135deg, var(--color-gold) 0%, var(--color-gold-dark) 100%);
    color: var(--color-bg-primary);
    border-color: var(--color-gold-dark);
    box-shadow: 0 4px 12px rgba(212, 175, 55, 0.4);
    font-weight: 700;
  }

  .tab-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .card {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.5rem;
    border-radius: 8px;
    background: var(--color-bg-card);
    border: 2px solid var(--color-border);
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.4);
    transition: all 0.3s ease;
    position: relative;
  }

  .card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: linear-gradient(90deg, transparent 0%, var(--color-gold) 50%, transparent 100%);
    opacity: 0.3;
  }

  .card:hover {
    border-color: var(--color-border-light);
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.5);
  }

  .card h3 {
    margin: 0;
    font-size: 1.2rem;
    font-weight: 700;
    color: var(--color-gold-light);
    font-family: 'Cinzel', serif;
    letter-spacing: 0.03em;
  }

  .muted {
    margin: 0;
    font-size: 0.85rem;
    color: var(--color-text-muted);
    font-family: 'Cinzel', serif;
  }

  .card-section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1rem;
    border-radius: 6px;
    background: var(--color-bg-secondary);
    border: 2px solid var(--color-border);
  }

  .card-section-header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .card-section-header h4 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-text-primary);
    font-family: 'Cinzel', serif;
    letter-spacing: 0.02em;
  }

  .current-value {
    margin: 0;
    font-size: 0.85rem;
    color: var(--color-stone-light);
    font-weight: 600;
    font-family: 'Cinzel', serif;
  }

  .form-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: flex-end;
  }

  .form-row label {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex: 1 1 140px;
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    font-family: 'Cinzel', serif;
  }

  .form-row input,
  .form-row select {
    border-radius: 4px;
    border: 2px solid var(--color-border);
    padding: 0.65rem 0.85rem;
    font-size: 0.95rem;
    background: var(--color-bg-secondary);
    color: var(--color-text-primary);
    font-family: 'Cinzel', serif;
    transition: all 0.2s ease;
  }

  .form-row input:focus,
  .form-row select:focus {
    outline: none;
    border-color: var(--color-gold);
    box-shadow: 0 0 0 3px rgba(212, 175, 55, 0.2);
  }

  .form-row input:hover,
  .form-row select:hover {
    border-color: var(--color-border-light);
  }

  .form-row button {
    align-self: stretch;
    padding-inline: 1.5rem;
    background: linear-gradient(135deg, var(--color-gold) 0%, var(--color-gold-dark) 100%);
    color: var(--color-bg-primary);
    min-width: 140px;
    border-color: var(--color-gold-dark);
    font-weight: 700;
  }

  .button-group {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .button-group button {
    flex: 1 1 160px;
    background: linear-gradient(135deg, var(--color-gold) 0%, var(--color-gold-dark) 100%);
    color: var(--color-bg-primary);
    border-color: var(--color-gold-dark);
    font-weight: 700;
  }

  code {
    background: var(--color-bg-secondary);
    border-radius: 4px;
    padding: 0.3rem 0.6rem;
    font-size: 0.9rem;
    color: var(--color-gold-light);
    border: 1px solid var(--color-border);
    font-family: 'Courier New', monospace;
  }

  @media (max-width: 900px) {
    main {
      padding: 1rem;
    }

    .welcome h1 {
      font-size: 2rem;
    }

    .subtitle {
      font-size: 1rem;
    }

    .features {
      grid-template-columns: repeat(2, 1fr);
    }

    .header-top {
      flex-direction: column;
      align-items: stretch;
    }

    .header-actions {
      width: 100%;
      justify-content: stretch;
    }

    .header-actions button {
      flex: 1;
    }

    .tabs {
      overflow-x: auto;
    }

    button.large {
      width: 100%;
      justify-content: center;
    }
  }

  @media (max-width: 600px) {
    .features {
      grid-template-columns: 1fr;
    }

    .welcome h1 {
      font-size: 1.75rem;
    }
  }
</style>
