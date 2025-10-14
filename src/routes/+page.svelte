<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import FieldEditor, { type JSONValue } from "$lib/components/FieldEditor.svelte";
  import {
    getPlayerCoins,
    getPlayerGems,
    setPlayerCoins,
    setPlayerGems,
    spawnArchers,
    spawnFarmers,
    spawnPikemen,
    spawnWorkers
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

  interface EntrySummary {
    key: string;
    type: string;
    detail: string | null;
  }

  interface CampaignSummary {
    label: string;
    islands: number;
  }

  function describeValue(value: JSONValue): string | null {
    if (Array.isArray(value)) {
      return `${value.length} entrées`;
    }

    if (typeof value === "object" && value !== null) {
      return `${Object.keys(value as Record<string, JSONValue>).length} clés`;
    }

    return null;
  }

  function makeSummaries(value: JSONValue | null): EntrySummary[] {
    if (!value || typeof value !== "object" || Array.isArray(value)) {
      return [];
    }

    return Object.entries(value as Record<string, JSONValue>).map(([key, entry]) => ({
      key,
      type: Array.isArray(entry)
        ? "Tableau"
        : entry === null
          ? "Null"
          : typeof entry === "object"
            ? "Objet"
            : typeof entry,
      detail: describeValue(entry)
    }));
  }

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
      showSuccess("global-v35 chargé");
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

  function applyUpdate(target: JSONValue, path: string[], value: JSONValue): JSONValue {
    if (path.length === 0) {
      return value;
    }

    const [segment, ...rest] = path;

    if (Array.isArray(target)) {
      const index = Number(segment);
      const updated = target.map((entry, idx) => (idx === index ? applyUpdate(entry, rest, value) : entry));
      return updated;
    }

    if (typeof target === "object" && target !== null) {
      const copy: Record<string, JSONValue> = { ...(target as Record<string, JSONValue>) };
      const current = copy[segment];
      copy[segment] = rest.length === 0 || current === undefined ? value : applyUpdate(current, rest, value);
      return copy;
    }

    return target;
  }

  function handleFieldChange(event: CustomEvent<{ path: string[]; value: JSONValue }>) {
    if (!data) {
      return;
    }

    data = applyUpdate(data, event.detail.path, event.detail.value);
  }

  async function save() {
    resetStatus();

    if (!filePath || !data) {
      showError("Aucun fichier chargé");
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
      showSuccess("Sauvegarde appliquée");
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

  let rootLabel = $derived(filePath?.split(/[\\/]/).pop() ?? "global-v35");
  let topLevelEntries = $derived(makeSummaries(data));
  let topLevelMap = $derived(
    !data || typeof data !== "object" || Array.isArray(data)
      ? null
      : (data as Record<string, JSONValue>)
  );
  let selectedValue = $derived(
    !data
      ? null
      : !selectedKey || !topLevelMap
        ? data
        : topLevelMap[selectedKey] ?? data
  );
  let selectedLabel = $derived(selectedKey ?? rootLabel);
  let selectedDetail = $derived(
    !selectedValue
      ? null
      : !selectedKey
        ? `${topLevelEntries.length} sections`
        : describeValue(selectedValue)
  );

  let campaigns = $derived(computeCampaignSummaries(data));
  let islands = $derived(computeIslandCount(data, selectedCampaign));

  $effect(() => {
    if (!data) {
      selectedKey = null;
      return;
    }

    if (selectedKey && !topLevelEntries.some((entry) => entry.key === selectedKey)) {
      selectedKey = topLevelEntries[0]?.key ?? null;
    }
  });

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

  function selectSection(key: string | null) {
    selectedKey = key;
  }

  function requireData(): JSONValue {
    if (!data) {
      throw new Error("Aucun fichier chargé");
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
      showSuccess("Pièces mises à jour");
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
      showSuccess("Gemmes mises à jour");
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
      showSuccess("Archers ajoutés");
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
      showSuccess("Ouvriers ajoutés");
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
      showSuccess("Fermiers ajoutés");
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
      showSuccess("Piquiers ajoutés");
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }
</script>

<main>
  <header>
    <h1>Éditeur Kingdom Two Crowns</h1>
    <p>Ouvrez le fichier <code>global-v35</code>, modifiez les valeurs et enregistrez.</p>
    <div class="actions">
      <button onclick={selectFile} disabled={isLoading}>
        {isLoading ? "Chargement..." : "Ouvrir global-v35"}
      </button>
      <button class="primary" onclick={save} disabled={!canSave()}>
        Sauvegarder
      </button>
    </div>
    {#if statusMessage}
      <p class={`status ${statusType ?? ""}`}>{statusMessage}</p>
    {/if}
    {#if backupPath}
      <p class="status info">Copie de sauvegarde: <code>{backupPath}</code></p>
    {/if}
  </header>

  {#if data}
    <section class="content">
      <aside class="sidebar">
        <h2>Sections</h2>
        <button
          type="button"
          class:selected={!selectedKey}
          onclick={() => selectSection(null)}
        >
          <span class="entry-name">Vue globale</span>
          <span class="entry-detail">{topLevelEntries.length} sections</span>
        </button>
        {#each topLevelEntries as entry}
          <button
            type="button"
            class:selected={selectedKey === entry.key}
            onclick={() => selectSection(entry.key)}
          >
            <span class="entry-name">{entry.key}</span>
            <span class="entry-type">{entry.type}</span>
            {#if entry.detail}
              <span class="entry-detail">{entry.detail}</span>
            {/if}
          </button>
        {/each}
      </aside>
      <div class="editor-area">
        <div class="actions-panel">
          <div class="selectors">
            <label>
              Campagne
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
              Île
              <select
                value={selectedIsland}
                onchange={(event) => (selectedIsland = Number((event.currentTarget as HTMLSelectElement).value))}
              >
                {#each Array.from({ length: Math.max(0, islands) }) as _, index}
                  <option value={index}>Île {index + 1}</option>
                {/each}
              </select>
            </label>
          </div>

          <div class="action-block">
            <h3>Or du joueur</h3>
            <p class="current-value">
              Actuel: {(() => {
                const value = getPlayerCoins(data, {
                  campaignIndex: selectedCampaign,
                  islandIndex: selectedIsland,
                  playerIndex: coinsPlayerIndex
                });
                return typeof value === "number" ? value : 0;
              })()}
            </p>
            <div class="action-row">
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

          <div class="action-block">
            <h3>Gemmes du joueur</h3>
            <p class="current-value">
              Actuel: {(() => {
                const value = getPlayerGems(data, {
                  campaignIndex: selectedCampaign,
                  islandIndex: selectedIsland,
                  playerIndex: gemsPlayerIndex
                });
                return typeof value === "number" ? value : 0;
              })()}
            </p>
            <div class="action-row">
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

          <div class="action-block">
            <h3>Ajouter des archers</h3>
            <p class="current-value">
              Actuel: {countUnits(data, selectedCampaign, selectedIsland, ARCHER_PREFABS) ?? 0}
            </p>
            <div class="action-row">
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

          <div class="action-block">
            <h3>Ajouter des ouvriers</h3>
            <p class="current-value">
              Actuel: {countUnits(data, selectedCampaign, selectedIsland, WORKER_PREFABS) ?? 0}
            </p>
            <div class="action-row">
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

          <div class="action-block">
            <h3>Ajouter des fermiers</h3>
            <p class="current-value">
              Actuel: {countUnits(data, selectedCampaign, selectedIsland, FARMER_PREFABS) ?? 0}
            </p>
            <div class="action-row">
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

          <div class="action-block">
            <h3>Ajouter des piquiers</h3>
            <p class="current-value">
              Actuel: {countUnits(data, selectedCampaign, selectedIsland, PIKEMAN_PREFABS) ?? 0}
            </p>
            <div class="action-row">
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
        </div>
        <div class="mobile-selector">
          <label for="section-select">Section</label>
          <select
            id="section-select"
            value={selectedKey ?? ""}
            onchange={(event) => {
              const next = (event.currentTarget as HTMLSelectElement).value;
              selectSection(next === "" ? null : next);
            }}
          >
            <option value="">Vue globale</option>
            {#each topLevelEntries as entry}
              <option value={entry.key}>{entry.key}</option>
            {/each}
          </select>
        </div>
        <div class="editor-header">
          <div>
            <h2>{selectedLabel}</h2>
            {#if selectedDetail}
              <p class="editor-detail">{selectedDetail}</p>
            {/if}
          </div>
          {#if filePath}
            <p class="editor-path">{filePath}</p>
          {/if}
        </div>
        <div class="editor-scroll">
          <FieldEditor label={selectedLabel} value={selectedValue ?? null} path={[]} on:change={handleFieldChange} />
        </div>
      </div>
    </section>
  {:else}
    <section class="placeholder">
      <p>Sélectionnez le fichier pour commencer.</p>
    </section>
  {/if}
</main>

<style>
  main {
    margin: 0 auto;
    padding: 2rem;
    max-width: 1200px;
    color: #1c1c1c;
    font-family: "Inter", system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }

  header {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 2rem;
  }

  h1 {
    margin: 0;
    font-size: 2rem;
  }

  p {
    margin: 0;
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  button {
    border: none;
    border-radius: 8px;
    padding: 0.7rem 1.2rem;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    background: #f1f1f1;
    color: #1c1c1c;
    transition: transform 0.15s ease, box-shadow 0.15s ease;
  }

  button.primary {
    background: #2a66f8;
    color: #ffffff;
  }

  button:disabled {
    opacity: 0.55;
    cursor: not-allowed;
    box-shadow: none;
  }

  button:not(:disabled):hover {
    transform: translateY(-1px);
    box-shadow: 0 6px 18px rgba(0, 0, 0, 0.12);
  }

  .status {
    font-weight: 500;
  }

  .status.error {
    color: #d32f2f;
  }

  .status.success {
    color: #2e7d32;
  }

  .status.info {
    color: #1565c0;
  }

  .content {
    display: grid;
    gap: 1.5rem;
    grid-template-columns: minmax(220px, 280px) 1fr;
    align-items: start;
  }

  .sidebar {
    position: sticky;
    top: 1rem;
    align-self: start;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1rem;
    border-radius: 12px;
    border: 1px solid rgba(0, 0, 0, 0.08);
    background: rgba(255, 255, 255, 0.7);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.05);
  }

  .sidebar h2 {
    font-size: 1rem;
    font-weight: 600;
    margin: 0 0 0.25rem;
  }

  .sidebar button {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.15rem;
    padding: 0.65rem 0.75rem;
    border-radius: 10px;
    background: transparent;
    color: inherit;
    font-weight: 500;
    border: none;
    box-shadow: none;
    transition: background 0.15s ease, transform 0.15s ease;
  }

  .sidebar button:hover {
    transform: translateY(-1px);
    background: rgba(42, 102, 248, 0.12);
  }

  .sidebar button.selected {
    background: rgba(42, 102, 248, 0.18);
    color: #1a3aa5;
  }

  .entry-name {
    font-weight: 600;
  }

  .entry-type {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(0, 0, 0, 0.55);
  }

  .entry-detail {
    font-size: 0.85rem;
    color: rgba(0, 0, 0, 0.6);
  }

  .editor-area {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .actions-panel {
    display: grid;
    gap: 1rem;
    padding: 1rem;
    border-radius: 16px;
    border: 1px solid rgba(0, 0, 0, 0.08);
    background: rgba(255, 255, 255, 0.8);
    box-shadow: 0 6px 16px rgba(15, 23, 42, 0.05);
  }

  .selectors {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 0.75rem;
  }

  .selectors label,
  .action-row label {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    font-weight: 600;
    font-size: 0.9rem;
  }

  .selectors select,
  .action-row select,
  .action-row input {
    border-radius: 8px;
    border: 1px solid rgba(0, 0, 0, 0.2);
    padding: 0.5rem;
    font-size: 0.95rem;
    background: rgba(255, 255, 255, 0.95);
  }

  .action-block {
    display: grid;
    gap: 0.75rem;
  }

  .action-block h3 {
    margin: 0;
    font-size: 1rem;
  }

  .current-value {
    margin: 0;
    font-size: 0.9rem;
    color: rgba(0, 0, 0, 0.65);
  }

  .action-row {
    display: grid;
    gap: 0.75rem;
    align-items: end;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  }

  .action-row button {
    align-self: stretch;
    background: #2a66f8;
    color: #ffffff;
  }

  .editor-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
  }

  .editor-header h2 {
    margin: 0;
    font-size: 1.4rem;
  }

  .editor-detail {
    margin: 0.2rem 0 0;
    color: rgba(0, 0, 0, 0.6);
  }

  .editor-path {
    margin: 0;
    font-size: 0.85rem;
    color: rgba(0, 0, 0, 0.55);
    word-break: break-all;
    text-align: right;
  }

  .editor-scroll {
    border-radius: 16px;
    border: 1px solid rgba(0, 0, 0, 0.08);
    padding: 1rem;
    background: rgba(255, 255, 255, 0.75);
    box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.04);
    max-height: 70vh;
    overflow: auto;
    scroll-behavior: smooth;
  }

  .mobile-selector {
    display: none;
    gap: 0.35rem;
  }

  .mobile-selector label {
    font-size: 0.85rem;
    font-weight: 600;
  }

  .mobile-selector select {
    border-radius: 8px;
    border: 1px solid rgba(0, 0, 0, 0.2);
    padding: 0.5rem;
    font-size: 0.95rem;
    background: rgba(255, 255, 255, 0.9);
  }

  .placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
    border: 2px dashed rgba(0, 0, 0, 0.1);
    border-radius: 12px;
    font-size: 1.1rem;
    color: rgba(0, 0, 0, 0.6);
  }

  code {
    background: rgba(0, 0, 0, 0.05);
    border-radius: 4px;
    padding: 0 0.35rem;
    font-size: 0.95rem;
  }

  @media (prefers-color-scheme: dark) {
    main {
      color: #f3f3f3;
      background: #1c1c1c;
    }

    button {
      background: #2f2f2f;
      color: #f3f3f3;
    }

    button.primary {
      background: #2a66f8;
    }

    .content {
      color: #f3f3f3;
    }

    .sidebar {
      background: rgba(34, 34, 34, 0.85);
      border-color: rgba(255, 255, 255, 0.08);
      box-shadow: 0 12px 28px rgba(0, 0, 0, 0.45);
    }

    .sidebar button.selected {
      background: rgba(42, 102, 248, 0.35);
      color: #dbe5ff;
    }

    .entry-type,
    .entry-detail,
    .editor-detail,
    .editor-path {
      color: rgba(235, 235, 235, 0.65);
    }

    .editor-scroll {
      background: rgba(32, 32, 32, 0.85);
      border-color: rgba(255, 255, 255, 0.08);
      box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.6);
    }

    .actions-panel {
      background: rgba(32, 32, 32, 0.85);
      border-color: rgba(255, 255, 255, 0.1);
      box-shadow: 0 12px 28px rgba(0, 0, 0, 0.5);
    }

    .selectors select,
    .action-row select,
    .action-row input {
      background: rgba(24, 24, 24, 0.9);
      border-color: rgba(255, 255, 255, 0.12);
      color: #f3f3f3;
    }

    .current-value {
      color: rgba(235, 235, 235, 0.7);
    }

    .placeholder {
      border-color: rgba(255, 255, 255, 0.2);
      color: rgba(255, 255, 255, 0.6);
    }

    code {
      background: rgba(255, 255, 255, 0.1);
    }
  }

  @media (max-width: 900px) {
    main {
      padding: 1.5rem;
    }

    .content {
      grid-template-columns: 1fr;
    }

    .sidebar {
      display: none;
    }

    .mobile-selector {
      display: grid;
    }

    .editor-scroll {
      max-height: none;
    }

    .actions-panel {
      padding: 0.75rem;
    }
  }
</style>
