import type { JSONValue } from "$lib/components/FieldEditor.svelte";

type MutableJSONObject = Record<string, any>;

interface SelectionOptions {
  campaignIndex: number;
  islandIndex: number;
  playerIndex: number;
}

export interface CoinsOptions extends SelectionOptions {
  coins: number;
}

export type CoinsQueryOptions = SelectionOptions;

export interface GemsOptions extends SelectionOptions {
  gems: number;
}

export type GemsQueryOptions = SelectionOptions;

export interface SpawnArchersOptions extends SelectionOptions {
  count: number;
}

interface CampaignIslandOptions {
  campaignIndex: number;
  islandIndex: number;
}

export interface GotoOptions extends CampaignIslandOptions {
  coins?: number;
  gems?: number;
  pikemen?: number;
  farmers?: number;
  boats?: number;
}

export interface TakeOverOptions extends CampaignIslandOptions {
  coins?: number;
  archerCount?: number;
  workerCount?: number;
  formationOffset?: number;
}

export interface DestroyPortalsOptions extends CampaignIslandOptions {}

export interface ExterminateOptions extends CampaignIslandOptions {}

export interface FormationOptions extends CampaignIslandOptions {
  playerIndex: number;
  position: number;
  archers?: number;
  pikemen?: number;
}

export interface PimpOptions extends CampaignIslandOptions {
  spawnCount?: number;
  coins?: number;
}

export interface TreesOptions extends CampaignIslandOptions {
  playerIndex?: number;
  coins?: number;
}

export interface SpawnUnitsOptions extends CampaignIslandOptions {
  playerIndex: number;
  archers?: number;
  workers?: number;
  pikemen?: number;
}

function toObject(value: unknown, message: string): MutableJSONObject {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(message);
  }

  return value as MutableJSONObject;
}

function toArray(value: unknown, message: string): MutableJSONObject[] {
  if (!Array.isArray(value)) {
    throw new Error(message);
  }

  return value as MutableJSONObject[];
}

function cloneJson<T extends JSONValue>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

function updateWallet(componentList: MutableJSONObject[], coins: number) {
  let updated = false;

  for (const component of componentList) {
    if (component.name === "Wallet" && typeof component.data === "string") {
      try {
        const payload = JSON.parse(component.data) as MutableJSONObject;
        payload.usesCurrencySystem = true;
        payload.coins = coins;

        if (typeof payload.currency === "object" && payload.currency !== null) {
          (payload.currency as MutableJSONObject).Coins = coins;
        }

        component.data = JSON.stringify(payload);
        updated = true;
      } catch (error) {
        throw new Error("Impossible de mettre à jour le portefeuille du joueur");
      }
    }
  }

  if (!updated) {
    throw new Error("Aucun composant Wallet trouvé pour le joueur ciblé");
  }
}

function extractWalletCoins(componentList: MutableJSONObject[]): number | null {
  for (const component of componentList) {
    if (component.name === "Wallet" && typeof component.data === "string") {
      try {
        const payload = JSON.parse(component.data) as MutableJSONObject;
        if (typeof payload.currency === "object" && payload.currency !== null) {
          const coins = (payload.currency as MutableJSONObject).Coins;
          return typeof coins === "number" ? coins : null;
        }
      } catch (error) {
        return null;
      }
    }
  }

  return null;
}

function randomId() {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return crypto.randomUUID();
  }

  return `uid-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

function findPlayerObject(objects: MutableJSONObject[], playerIndex: number) {
  const name = `Player ${playerIndex + 1}`;
  return objects.find((entry) => entry.name === name) ?? null;
}

function nextNumericValue(objects: MutableJSONObject[], key: string, fallback: number): number {
  let maxValue = fallback;

  for (const obj of objects) {
    const value = obj[key];
    if (typeof value === "number" && value > maxValue) {
      maxValue = value;
    }
  }

  return maxValue + 1;
}

export function setPlayerCoins(doc: JSONValue, options: CoinsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, coins } = options;

  const campaigns = toArray(toObject(root, "Sauvegarde invalide").campaigns, "La sauvegarde ne contient aucune campagne");
  const campaign = campaigns[campaignIndex];
  if (!campaign) {
    throw new Error("Campagne introuvable");
  }

  const campaignObject = toObject(campaign, "Campagne invalide");

  if (campaignObject.carryForward) {
    const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
    const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
    if (carry[key]) {
      const currency = toObject(carry[key], "Devise du joueur invalide");
      currency.Coins = coins;
      carry.usesCurrencySystem = true;
    }

    const legacyKey = playerIndex === 0 ? "coins1" : "coins2";
    if (legacyKey in carry) {
      carry[legacyKey] = coins;
    }
  }

  const islands = campaignObject._islands ? toArray(campaignObject._islands, "Aucune île trouvée pour la campagne") : [];
  const island = islands[islandIndex];
  if (island) {
    const islandObj = toObject(island, "Données d'île invalides");
    if (Array.isArray(islandObj.monarchy?.characters)) {
      const characters = islandObj.monarchy.characters as MutableJSONObject[];
      const target = characters[playerIndex];
      if (target) {
        toObject(target, "Personnage invalide").currency = coins;
      }
    }

    if (Array.isArray(islandObj.objects)) {
      const objects = islandObj.objects as MutableJSONObject[];
      const player = findPlayerObject(objects, playerIndex);
      if (player) {
        const components = Array.isArray(player.componentData2)
          ? (player.componentData2 as MutableJSONObject[])
          : [];
        updateWallet(components, coins);
      }
    }
  }

  return root;
}

export function setPlayerGems(doc: JSONValue, options: GemsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, gems } = options;

  const campaigns = toArray(toObject(root, "Sauvegarde invalide").campaigns, "La sauvegarde ne contient aucune campagne");
  const campaign = campaigns[campaignIndex];
  if (!campaign) {
    throw new Error("Campagne introuvable");
  }

  const campaignObject = toObject(campaign, "Campagne invalide");

  if (campaignObject.carryForward) {
    const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
    const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
    if (carry[key]) {
      const currency = toObject(carry[key], "Devise du joueur invalide");
      currency.Gems = gems;
      carry.usesCurrencySystem = true;
    }

    const legacyKey = playerIndex === 0 ? "gems1" : "gems2";
    if (legacyKey in carry) {
      carry[legacyKey] = gems;
    }
  }

  const islands = campaignObject._islands ? toArray(campaignObject._islands, "Aucune île trouvée pour la campagne") : [];
  const island = islands[islandIndex];
  if (island) {
    const islandObj = toObject(island, "Données d'île invalides");
    if (Array.isArray(islandObj.objects)) {
      const objects = islandObj.objects as MutableJSONObject[];
      const player = findPlayerObject(objects, playerIndex);
      if (player && Array.isArray(player.componentData2)) {
        for (const component of player.componentData2 as MutableJSONObject[]) {
          if (component.name === "Wallet" && typeof component.data === "string") {
            try {
              const payload = JSON.parse(component.data) as MutableJSONObject;
              payload.usesCurrencySystem = true;
              if (typeof payload.currency === "object" && payload.currency !== null) {
                (payload.currency as MutableJSONObject).Gems = gems;
              }
              component.data = JSON.stringify(payload);
            } catch (error) {
              console.warn("Impossible de mettre à jour les gemmes du portefeuille", error);
            }
          }
        }
      }
    }
  }

  return root;
}

export function spawnArchers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, {
    prefabs: [
      "Prefabs/Characters/Archer",
      "Prefabs/Characters/norselands/Archer_norselands"
    ],
    namePrefix: "Archer",
    positionError: "Position de l'archer invalide",
    duplicateName: "Archer"
  });
}

interface SpawnConfig {
  prefabs: readonly string[];
  namePrefix: string;
  positionError: string;
  duplicateName: string;
}

const DEFAULT_CONFIGS = {
  worker: {
    prefabs: ["Prefabs/Characters/Worker", "Prefabs/Characters/norselands/Worker_norselands"] as const,
    namePrefix: "Worker",
    positionError: "Position du travailleur invalide",
    duplicateName: "Worker",
  },
  farmer: {
    prefabs: ["Prefabs/Characters/Farmer", "Prefabs/Characters/norselands/Farmer_norselands"] as const,
    namePrefix: "Farmer",
    positionError: "Position du fermier invalide",
    duplicateName: "Farmer",
  },
  pikeman: {
    prefabs: ["Prefabs/Characters/Pikeman", "Prefabs/Characters/norselands/Knight_norselands", "Prefabs/Characters/norselands/Pikeman_norselands"] as const,
    namePrefix: "Pikeman",
    positionError: "Position du piquier invalide",
    duplicateName: "Pikeman",
  },
} satisfies Record<string, SpawnConfig>;

const archerTemplate: MutableJSONObject = {
  name: "Archer P1 [B]",
  parentObject: { linkedObjectID: "" },
  hierarchyPath: "Level/GameLayer/",
  prefabPath: "Prefabs/Characters/Archer",
  uniqueID: "Archer P1 [B]--6969",
  mode: 0,
  createOrder: 12345,
  linkOrder: 0,
  decayHint: 0,
  decayResistanceDays: 2,
  decayedVersionPrefabPath: "Prefabs/Characters/Peasant",
  netID: 1666,
  crpcType: 1,
  localPosition: { x: 120.98619079589844, y: 0.8799997568130493, z: 0.7556657791137695 },
  localScale: { x: -1.0, y: 1.0, z: 1.0 },
  componentData2: [
    {
      name: "Wallet",
      type: "WalletData",
      data: "{\"coins\":0,\"gems\":0,\"usesCurrencySystem\":true,\"currency\":{\"Coins\":1,\"Gems\":0,\"Crown\":0,\"Skulls\":0,\"Shades\":0,\"Merchandise\":0}}",
    },
    {
      name: "Character",
      type: "CharacterData",
      data: "{\"isGrabbed\":false,\"inert\":false}",
    },
    {
      name: "Archer",
      type: "ArcherData",
      data: "{\"tower\":{\"linkedObjectID\":\"\"},\"knight\":{\"linkedObjectID\":\"\"},\"guardSide\":1,\"guardDepth\":0,\"desiredAttackMode\":0,\"despawnOnLoad\":false}",
    },
    {
      name: "Damageable",
      type: "DamageableData",
      data: "{\"hitPoints\":0,\"invulnerable\":false}",
    },
    {
      name: "GenderAnimatorSelector",
      type: "GenderSelectorSaveData",
      data: "{\"IsFemale\":false}",
    },
    {
      name: "Petrifiable",
      type: "PetrifiableSaveData",
      data: "{\"RemainingHP\":0,\"IsPetrified\":false,\"RemainingDuration\":-106.38296508789063}",
    },
    {
      name: "Embarkee",
      type: "EmbarkeeSaveData",
      data: "{\"IsEmbarked\":false,\"SlotId\":-1,\"Embarkable\":{\"linkedObjectID\":\"\"}}",
    },
  ],
};

const workerTemplate: MutableJSONObject = {
  name: "Worker P1 [A]",
  parentObject: { linkedObjectID: "" },
  hierarchyPath: "Level/GameLayer/",
  prefabPath: "Prefabs/Characters/Worker",
  uniqueID: "Worker P1 [A]--6969",
  mode: 0,
  createOrder: 22222,
  linkOrder: 0,
  decayHint: 4,
  decayResistanceDays: 1,
  decayedVersionPrefabPath: "Prefabs/Characters/Peasant",
  netID: 1098,
  crpcType: 1,
  localPosition: { x: 127.18162536621094, y: 0.875, z: 0.5052622556686401 },
  localScale: { x: -1.0, y: 1.0, z: 1.0 },
  componentData2: [
    {
      name: "Wallet",
      type: "WalletData",
      data: "{\"coins\":0,\"gems\":0,\"usesCurrencySystem\":true,\"currency\":{\"Coins\":1,\"Gems\":0,\"Crown\":0,\"Skulls\":0,\"Shades\":0,\"Merchandise\":0}}",
    },
    {
      name: "Character",
      type: "CharacterData",
      data: "{\"isGrabbed\":false,\"inert\":false}",
    },
    {
      name: "Worker",
      type: "WorkerData",
      data: "{\"despawnOnLoad\":false}",
    },
    {
      name: "Damageable",
      type: "DamageableData",
      data: "{\"hitPoints\":0,\"invulnerable\":false}",
    },
    {
      name: "GenderAnimatorSelector",
      type: "GenderSelectorSaveData",
      data: "{\"IsFemale\":false}",
    },
    {
      name: "Petrifiable",
      type: "PetrifiableSaveData",
      data: "{\"RemainingHP\":0,\"IsPetrified\":false,\"RemainingDuration\":-106.38296508789063}",
    },
    {
      name: "Embarkee",
      type: "EmbarkeeSaveData",
      data: "{\"IsEmbarked\":false,\"SlotId\":-1,\"Embarkable\":{\"linkedObjectID\":\"\"}}",
    },
  ],
};

const pikemanTemplate: MutableJSONObject = {
  name: "Pikeman P1 [8]",
  parentObject: { linkedObjectID: "" },
  hierarchyPath: "Level/GameLayer/",
  prefabPath: "Prefabs/Characters/Pikeman",
  uniqueID: "Pikeman P1 [8]--59924",
  mode: 0,
  createOrder: 20668,
  linkOrder: 0,
  decayHint: 32,
  decayResistanceDays: 2,
  decayedVersionPrefabPath: "Prefabs/Characters/Peasant",
  netID: 1101,
  crpcType: 1,
  localPosition: { x: -7.411938667297363, y: 0.8804292678833008, z: 0.9069020748138428 },
  localScale: { x: -1.0, y: 1.0, z: 1.0 },
  componentData2: [
    {
      name: "Wallet",
      type: "WalletData",
      data: "{\"coins\":0,\"gems\":0,\"usesCurrencySystem\":true,\"currency\":{\"Coins\":3,\"Gems\":0,\"Crown\":0,\"Skulls\":0,\"Shades\":0,\"Merchandise\":0}}",
    },
    {
      name: "Character",
      type: "CharacterData",
      data: "{\"isGrabbed\":false,\"inert\":false}",
    },
    {
      name: "Pikeman",
      type: "PikemanData",
      data: "{\"fishingSide\":-1,\"assignedSide\":-1,\"assignedWall\":{\"linkedObjectID\":\"\"},\"assignedReadiness\":0,\"remainingPikeUses\":30,\"summonedToBoat\":false}",
    },
    {
      name: "Damageable",
      type: "DamageableData",
      data: "{\"hitPoints\":1,\"invulnerable\":false}",
    },
    {
      name: "GenderAnimatorSelector",
      type: "GenderSelectorSaveData",
      data: "{\"IsFemale\":true}",
    },
    {
      name: "Petrifiable",
      type: "PetrifiableSaveData",
      data: "{\"RemainingHP\":0,\"IsPetrified\":false,\"RemainingDuration\":-26.941898345947267}",
    },
    {
      name: "Embarkee",
      type: "EmbarkeeSaveData",
      data: "{\"IsEmbarked\":false,\"SlotId\":-1,\"Embarkable\":{\"linkedObjectID\":\"\"}}",
    },
  ],
};

const ENEMY_COMPONENTS = ["GreedArcher", "GreedKnight", "CrownStealer", "Troll", "Squid", "Boss"] as const;

const MAX_CASTLE_LEVEL = 6;
const MAX_WALL_LEVEL = 5;

function findTemplateByPrefab(objects: MutableJSONObject[], prefabs: readonly string[]): MutableJSONObject | null {
  return (
    objects.find((obj) => {
      const prefab = obj.prefabPath;
      if (typeof prefab !== "string") {
        return false;
      }

      return prefabs.some((candidate) => prefab.includes(candidate));
    }) ?? null
  );
}

function spawnUnitsByConfig(
  doc: JSONValue,
  options: SpawnArchersOptions,
  config: SpawnConfig
): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, count } = options;

  if (count <= 0) {
    throw new Error("Le nombre doit être positif");
  }

  const campaigns = toArray(toObject(root, "Sauvegarde invalide").campaigns, "La sauvegarde ne contient aucune campagne");
  const campaign = campaigns[campaignIndex];
  if (!campaign) {
    throw new Error("Campagne introuvable");
  }

  const campaignObject = toObject(campaign, "Campagne invalide");
  const islands = campaignObject._islands ? toArray(campaignObject._islands, "Aucune île trouvée pour la campagne") : [];
  const island = islands[islandIndex];
  if (!island) {
    throw new Error("Île introuvable");
  }

  const islandObj = toObject(island, "Données d'île invalides");
  const objects = toArray(islandObj.objects, "Aucun objet trouvé sur l'île");

  const template = findTemplateByPrefab(objects, config.prefabs);
  if (!template) {
    throw new Error(`Impossible de trouver un ${config.duplicateName.toLowerCase()} existant comme modèle`);
  }

  const playerObj = findPlayerObject(objects, playerIndex);
  const spawnSource = toObject(playerObj ?? template, "Objet d'inspiration invalide");
  const position = spawnSource.localPosition && typeof spawnSource.localPosition === "object"
    ? toObject(spawnSource.localPosition, config.positionError)
    : { x: 0, y: 0, z: 0 };

  let nextNetId = nextNumericValue(objects, "netID", 2000);
  let nextCreateOrder = nextNumericValue(objects, "createOrder", 20000);

  const baseWallet = Array.isArray(template.componentData2)
    ? extractWalletCoins(template.componentData2 as MutableJSONObject[])
    : null;

  for (let i = 0; i < count; i += 1) {
    const clone = cloneJson(template as JSONValue) as MutableJSONObject;
    clone.name = `${config.namePrefix} ${randomId()}`;
    clone.uniqueID = randomId();
    clone.netID = nextNetId++;
    clone.createOrder = nextCreateOrder++;

    if (clone.localPosition && typeof clone.localPosition === "object") {
      const loc = toObject(clone.localPosition, config.positionError);
      loc.x = typeof position.x === "number" ? position.x + i * 0.5 : i * 0.5;
      loc.y = typeof position.y === "number" ? position.y : 0;
      loc.z = typeof position.z === "number" ? position.z : 0;
    }

    const components = Array.isArray(clone.componentData2)
      ? (clone.componentData2 as MutableJSONObject[])
      : [];
    if (baseWallet !== null) {
      updateWallet(components, baseWallet);
    }
    objects.push(clone);
  }

  return root;
}

export function spawnWorkers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.worker);
}

export function spawnFarmers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.farmer);
}

export function spawnPikemen(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.pikeman);
}

function objHasComponent(obj: MutableJSONObject, componentName: string): boolean {
  if (!Array.isArray(obj.componentData2)) {
    return false;
  }

  return (obj.componentData2 as MutableJSONObject[]).some((entry) => entry?.name === componentName);
}

function ensureComponentList(obj: MutableJSONObject): MutableJSONObject[] {
  if (!Array.isArray(obj.componentData2)) {
    obj.componentData2 = [];
  }

  return obj.componentData2 as MutableJSONObject[];
}

function mutateComponentData(
  obj: MutableJSONObject,
  componentName: string,
  mutation: (payload: MutableJSONObject) => void
): boolean {
  if (!Array.isArray(obj.componentData2)) {
    return false;
  }

  let updated = false;

  for (const entry of obj.componentData2 as MutableJSONObject[]) {
    if (entry?.name === componentName && typeof entry.data === "string") {
      try {
        const payload = JSON.parse(entry.data) as MutableJSONObject;
        mutation(payload);
        entry.data = JSON.stringify(payload);
        updated = true;
      } catch (error) {
        console.warn(`Impossible de mettre à jour ${componentName}`, error);
      }
    }
  }

  return updated;
}

function getLocalPosition(obj: MutableJSONObject): MutableJSONObject {
  if (!obj.localPosition || typeof obj.localPosition !== "object" || Array.isArray(obj.localPosition)) {
    obj.localPosition = { x: 0, y: 0, z: 0 };
  }

  return obj.localPosition as MutableJSONObject;
}

function findObjectByName(objects: MutableJSONObject[], name: string): MutableJSONObject | null {
  return objects.find((entry) => entry?.name === name) ?? null;
}

function findObjectByNameContains(objects: MutableJSONObject[], needle: string): MutableJSONObject | null {
  return objects.find((entry) => typeof entry?.name === "string" && entry.name.includes(needle)) ?? null;
}

function markTree(obj: MutableJSONObject) {
  mutateComponentData(obj, "WorkableTree", (payload) => {
    payload.marked = true;
  });
}

function markTrees(objects: MutableJSONObject[]) {
  for (const obj of objects) {
    if (objHasComponent(obj, "WorkableTree")) {
      markTree(obj);
    }
  }
}

function setWalletCoinsOnObject(obj: MutableJSONObject, coins: number) {
  const components = ensureComponentList(obj);
  updateWallet(components, coins);
}

function setDamageableHP(obj: MutableJSONObject, hp: number) {
  mutateComponentData(obj, "Damageable", (payload) => {
    payload.hitPoints = hp;
  });
}

function setPlayerFormationActive(obj: MutableJSONObject, active: boolean) {
  mutateComponentData(obj, "Player", (payload) => {
    payload.isFormationActive = active;
  });
}

function createIdGenerators(objects: MutableJSONObject[]) {
  let nextNet = nextNumericValue(objects, "netID", 2999);
  let nextCreate = nextNumericValue(objects, "createOrder", 29999);
  return {
    nextNetId: () => nextNet++,
    nextCreateOrder: () => nextCreate++,
  };
}

type IdGenerators = ReturnType<typeof createIdGenerators>;

function createCharacterFromTemplate(
  template: MutableJSONObject,
  name: string,
  x: number,
  generators: IdGenerators
): MutableJSONObject {
  const clone = cloneJson(template as JSONValue) as MutableJSONObject;
  clone.name = name;
  clone.uniqueID = randomId();
  clone.netID = generators.nextNetId();
  clone.createOrder = generators.nextCreateOrder();
  const position = getLocalPosition(clone);
  position.x = x;
  if (typeof position.y !== "number") {
    position.y = 0;
  }
  if (typeof position.z !== "number") {
    position.z = 0;
  }
  return clone;
}

function makeArcher(name: string, x: number, generators: IdGenerators): MutableJSONObject {
  return createCharacterFromTemplate(archerTemplate, name, x, generators);
}

function makeWorker(name: string, x: number, generators: IdGenerators): MutableJSONObject {
  return createCharacterFromTemplate(workerTemplate, name, x, generators);
}

function makePikeman(name: string, x: number, generators: IdGenerators): MutableJSONObject {
  return createCharacterFromTemplate(pikemanTemplate, name, x, generators);
}

function initShop(kind: "bow" | "hammer" | "scythe", objects: MutableJSONObject[], generators: IdGenerators) {
  const identifier =
    kind === "bow"
      ? "ShopBow"
      : kind === "hammer"
        ? "ShopHammer"
        : kind === "scythe"
          ? "ShopScythe"
          : null;

  if (!identifier) {
    return;
  }

  const placeholder = findObjectByNameContains(objects, `${identifier} Placeholder`);
  if (!placeholder) {
    return;
  }

  const netId = generators.nextNetId();
  placeholder.name = `${identifier}_greece(Clone)`;
  placeholder.uniqueID = randomId();
  placeholder.prefabPath = `Prefabs/Buildings and Interactive/greece/${identifier}_greece`;
  placeholder.netID = netId;
}

function upgradeCastle(obj: MutableJSONObject, level: number) {
  const targetLevel = Math.min(level, MAX_CASTLE_LEVEL);
  obj.name = `Castle${targetLevel}_greece(Clone)`;
  obj.uniqueID = randomId();
  obj.prefabPath = `Prefabs/Buildings and Interactive/greece/Castle${targetLevel}_greece`;
  const components = ensureComponentList(obj);
  if (!objHasComponent(obj, "Castle")) {
    components.push({ name: "Castle", type: "CastleData", data: "{}" });
  }
  if (!objHasComponent(obj, "GhostHintTrigger")) {
    components.push({ name: "GhostHintTrigger", type: "GhostHintTriggerData", data: "{\"IsGhostHintEnabled\":true,\"hintDataPath\":\"UpgradeTheCastle1\"}" });
  }
}

function upgradeWalls(objects: MutableJSONObject[], level: number, generators: IdGenerators) {
  const targetLevel = Math.min(level, MAX_WALL_LEVEL);

  for (const obj of objects.slice()) {
    if (typeof obj.name === "string" && obj.name.includes("Wall0")) {
      obj.name = `Wall${targetLevel}_greece(Clone)`;
      obj.uniqueID = randomId();
      obj.prefabPath = `Prefabs/Buildings and Interactive/greece/Wall${targetLevel}_greece`;
      obj.decayedVersionPrefabPath = `Prefabs/Buildings and Interactive/greece/Wall${targetLevel} Wreck_greece`;
      const position = getLocalPosition(obj);
      const baseX = typeof position.x === "number" ? position.x : 0;
      for (let i = 0; i < 2; i += 1) {
        const worker = makeWorker("pimp my walls worker", baseX, generators);
        objects.push(worker);
      }
    }
  }
}

function getIslandContext(root: JSONValue, options: CampaignIslandOptions) {
  const rootObj = toObject(root, "Sauvegarde invalide");
  const campaigns = toArray(rootObj.campaigns, "La sauvegarde ne contient aucune campagne");
  const campaign = campaigns[options.campaignIndex];
  if (!campaign) {
    throw new Error("Campagne introuvable");
  }

  const campaignObject = toObject(campaign, "Campagne invalide");
  const islands = campaignObject._islands
    ? toArray(campaignObject._islands, "Aucune île trouvée pour la campagne")
    : [];
  const island = islands[options.islandIndex];
  if (!island) {
    throw new Error("Île introuvable");
  }

  const islandObject = toObject(island, "Données d'île invalides");
  const objects = islandObject.objects
    ? toArray(islandObject.objects, "Aucun objet trouvé sur l'île")
    : [];

  return { campaigns, campaignObject, islandObject, objects } as const;
}

function updateLandPortalCount(campaignObject: MutableJSONObject, islandIndex: number, portals: number) {
  try {
    if (!campaignObject.currentReign || typeof campaignObject.currentReign !== "object" || Array.isArray(campaignObject.currentReign)) {
      return;
    }

    const reign = toObject(campaignObject.currentReign, "Reign invalide");
    if (!Array.isArray(reign.landData)) {
      return;
    }

    const land = reign.landData[islandIndex];
    if (land && typeof land === "object" && !Array.isArray(land)) {
      (land as MutableJSONObject).portals = portals;
    }
  } catch (error) {
    console.warn("Impossible de mettre à jour le nombre de portails", error);
  }
}

function ensureCurrencyBag(target: MutableJSONObject, key: string): MutableJSONObject {
  if (!target[key] || typeof target[key] !== "object" || Array.isArray(target[key])) {
    target[key] = {};
  }

  return target[key] as MutableJSONObject;
}

export function getPlayerCoins(doc: JSONValue | null, options: CoinsQueryOptions): number | null {
  if (!doc) {
    return null;
  }

  try {
    const { campaignIndex, islandIndex, playerIndex } = options;
    const root = toObject(doc, "Sauvegarde invalide");
    const campaigns = Array.isArray(root.campaigns) ? (root.campaigns as MutableJSONObject[]) : null;
    if (!campaigns) {
      return null;
    }

    const campaign = campaigns[campaignIndex];
    if (!campaign) {
      return null;
    }

    const campaignObject = toObject(campaign, "Campagne invalide");

    if (campaignObject._islands) {
      const islands = toArray(campaignObject._islands, "Données d'îles invalides");
      const island = islands[islandIndex];
      if (island) {
        const islandObj = toObject(island, "Données d'île invalides");

        if (islandObj.monarchy && Array.isArray(islandObj.monarchy.characters)) {
          const characters = islandObj.monarchy.characters as MutableJSONObject[];
          const target = characters[playerIndex];
          if (target) {
            const charObj = toObject(target, "Personnage invalide");
            if (typeof charObj.currency === "number") {
              return charObj.currency;
            }
          }
        }

        if (Array.isArray(islandObj.objects)) {
          const objects = islandObj.objects as MutableJSONObject[];
          const player = findPlayerObject(objects, playerIndex);
          if (player) {
            const components = Array.isArray(player.componentData2)
              ? (player.componentData2 as MutableJSONObject[])
              : [];
            const coins = extractWalletCoins(components);
            if (typeof coins === "number") {
              return coins;
            }
          }
        }
      }
    }

    if (campaignObject.carryForward) {
      const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
      const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
      if (carry[key]) {
        const currency = toObject(carry[key], "Devise du joueur invalide");
        const coins = currency.Coins;
        if (typeof coins === "number") {
          return coins;
        }
      }
    }

    return null;
  } catch (error) {
    console.warn("Impossible de lire le nombre de pièces", error);
    return null;
  }
}

export function getPlayerGems(doc: JSONValue | null, options: GemsQueryOptions): number | null {
  if (!doc) {
    return null;
  }

  try {
    const { campaignIndex, islandIndex, playerIndex } = options;
    const root = toObject(doc, "Sauvegarde invalide");
    const campaigns = Array.isArray(root.campaigns) ? (root.campaigns as MutableJSONObject[]) : null;
    if (!campaigns) {
      return null;
    }

    const campaign = campaigns[campaignIndex];
    if (!campaign) {
      return null;
    }

    const campaignObject = toObject(campaign, "Campagne invalide");

    if (campaignObject._islands) {
      const islands = toArray(campaignObject._islands, "Données d'îles invalides");
      const island = islands[islandIndex];
      if (island) {
        const islandObj = toObject(island, "Données d'île invalides");

        if (Array.isArray(islandObj.objects)) {
          const objects = islandObj.objects as MutableJSONObject[];
          const player = findPlayerObject(objects, playerIndex);
          if (player && Array.isArray(player.componentData2)) {
            for (const component of player.componentData2 as MutableJSONObject[]) {
              if (component.name === "Wallet" && typeof component.data === "string") {
                try {
                  const payload = JSON.parse(component.data) as MutableJSONObject;
                  if (typeof payload.currency === "object" && payload.currency !== null) {
                    const gems = (payload.currency as MutableJSONObject).Gems;
                    if (typeof gems === "number") {
                      return gems;
                    }
                  }
                } catch (error) {
                  console.warn("Impossible de lire les gemmes du portefeuille", error);
                }
              }
            }
          }
        }
      }
    }

    if (campaignObject.carryForward) {
      const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
      const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
      if (carry[key]) {
        const currency = toObject(carry[key], "Devise du joueur invalide");
        const gems = currency.Gems;
        if (typeof gems === "number") {
          return gems;
        }
      }
    }

    return null;
  } catch (error) {
    console.warn("Impossible de lire le nombre de gemmes", error);
    return null;
  }
}

export function gotoIsland(doc: JSONValue, options: GotoOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject } = getIslandContext(root, options);

  const previousLand = campaignObject.currentLand;
  campaignObject.PreviousLand = previousLand ?? options.islandIndex;
  campaignObject.currentLand = options.islandIndex;

  if (campaignObject.currentReign && typeof campaignObject.currentReign === "object" && !Array.isArray(campaignObject.currentReign)) {
    const reign = toObject(campaignObject.currentReign, "Reign invalide");
    reign.currentLand = options.islandIndex;
  }

  if (campaignObject.carryForward) {
    try {
      const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
      const coins = options.coins ?? 69;
      const gems = options.gems ?? 3;
      const pikemen = options.pikemen ?? 4;
      const farmers = options.farmers ?? 4;
      const boats = options.boats ?? 4;

      carry.present = true;
      carry.usesCurrencySystem = true;

      const currency = ensureCurrencyBag(carry, "currencyP1");
      currency.Coins = coins;
      currency.Gems = gems;
      currency.Crown = 1;

      carry.numPikeman = pikemen;
      carry.numFarmers = farmers;
      carry.numFleetBoats = boats;

      const legacyCoins = "coins1";
      const legacyGems = "gems1";
      carry[legacyCoins] = coins;
      carry[legacyGems] = gems;
    } catch (error) {
      console.warn("Impossible de mettre à jour carryForward", error);
    }
  }

  return root;
}

export function takeOverIsland(doc: JSONValue, options: TakeOverOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);
  const generators = createIdGenerators(objects);

  let portalX = 0;
  const filtered: MutableJSONObject[] = [];

  for (const obj of objects) {
    if (objHasComponent(obj, "Portal")) {
      if (objHasComponent(obj, "QuestTransitionTrigger")) {
        setDamageableHP(obj, 1);
        const position = getLocalPosition(obj);
        portalX = typeof position.x === "number" ? position.x : 0;
        filtered.push(obj);
      }
    } else {
      filtered.push(obj);
    }
  }

  const targetX = portalX > 0 ? portalX - (options.formationOffset ?? 3) : portalX + (options.formationOffset ?? 3);
  const coins = options.coins ?? 69;

  const player = findObjectByName(filtered, "Player 1");
  if (player) {
    const position = getLocalPosition(player);
    position.x = targetX;
    setPlayerFormationActive(player, true);
    setWalletCoinsOnObject(player, coins);
  }

  const idol = findObjectByNameContains(filtered, "Idol");
  if (idol) {
    const position = getLocalPosition(idol);
    position.x = portalX;
  }

  const archerCount = Math.max(0, options.archerCount ?? 1);
  const workerCount = Math.max(0, options.workerCount ?? 1);

  for (let i = 0; i < archerCount; i += 1) {
    const name = archerCount === 1 ? "Steve" : `TakeOver Archer ${i + 1}`;
    filtered.push(makeArcher(name, targetX, generators));
  }

  for (let i = 0; i < workerCount; i += 1) {
    const name = workerCount === 1 ? "Joey" : `TakeOver Worker ${i + 1}`;
    filtered.push(makeWorker(name, targetX, generators));
  }

  islandObject.objects = filtered;
  updateLandPortalCount(campaignObject, options.islandIndex, 1);

  return root;
}

export function destroyPortals(doc: JSONValue, options: DestroyPortalsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);

  const remaining = objects.filter((obj) => !objHasComponent(obj, "Portal"));
  islandObject.objects = remaining;
  updateLandPortalCount(campaignObject, options.islandIndex, 0);

  return root;
}

export function exterminateEnemies(doc: JSONValue, options: ExterminateOptions): JSONValue {
  const root = cloneJson(doc);
  const { islandObject, objects } = getIslandContext(root, options);

  const survivors = objects.filter((obj) => !ENEMY_COMPONENTS.some((component) => objHasComponent(obj, component)));
  islandObject.objects = survivors;

  return root;
}

export function startFormationAssault(doc: JSONValue, options: FormationOptions): JSONValue {
  const root = cloneJson(doc);
  const { islandObject, objects } = getIslandContext(root, options);
  const generators = createIdGenerators(objects);

  const playerName = `Player ${options.playerIndex + 1}`;
  const player = findObjectByName(objects, playerName);
  if (!player) {
    throw new Error(`Impossible de trouver ${playerName}`);
  }

  const position = getLocalPosition(player);
  position.x = options.position;
  setPlayerFormationActive(player, true);

  const archerTotal = Math.max(0, options.archers ?? 5);
  const pikemanTotal = Math.max(0, options.pikemen ?? 5);

  for (let i = 0; i < archerTotal; i += 1) {
    objects.push(makeArcher(`Formation Archer ${i + 1}`, options.position, generators));
  }

  for (let i = 0; i < pikemanTotal; i += 1) {
    objects.push(makePikeman(`Formation Pikeman ${i + 1}`, options.position, generators));
  }

  islandObject.objects = objects;

  return root;
}

export function pimpIsland(doc: JSONValue, options: PimpOptions): JSONValue {
  const root = cloneJson(doc);
  const { islandObject, objects } = getIslandContext(root, options);
  const generators = createIdGenerators(objects);

  const player = findObjectByName(objects, "Player 1");
  const playerPosition = player ? getLocalPosition(player) : null;
  if (player && playerPosition) {
    playerPosition.x = 0;
    setWalletCoinsOnObject(player, options.coins ?? 69);
  }

  const castle = findObjectByNameContains(objects, "Castle");
  if (!castle) {
    throw new Error("Château introuvable sur l'île sélectionnée");
  }

  if (typeof castle.name === "string" && castle.name.includes("Castle0")) {
    initShop("bow", objects, generators);
    initShop("hammer", objects, generators);
    initShop("scythe", objects, generators);
  }

  upgradeCastle(castle, MAX_CASTLE_LEVEL);
  upgradeWalls(objects, MAX_WALL_LEVEL, generators);
  markTrees(objects);

  const spawnCount = Math.max(0, options.spawnCount ?? 10);
  const baseX = playerPosition && typeof playerPosition.x === "number" ? playerPosition.x : 0;

  for (let i = 0; i < spawnCount; i += 1) {
    objects.push(makeArcher(`Shooty Boy ${i + 1}`, baseX, generators));
    objects.push(makeWorker(`Good Ole Boy ${i + 1}`, baseX, generators));
    objects.push(makePikeman(`Pokey Boy ${i + 1}`, baseX, generators));
  }

  islandObject.objects = objects;

  return root;
}

export function markTreesForRemoval(doc: JSONValue, options: TreesOptions): JSONValue {
  const root = cloneJson(doc);
  const { islandObject, objects } = getIslandContext(root, options);

  markTrees(objects);

  const playerIndex = options.playerIndex ?? 0;
  const player = findObjectByName(objects, `Player ${playerIndex + 1}`);
  if (player) {
    setWalletCoinsOnObject(player, options.coins ?? 69);
  }

  islandObject.objects = objects;

  return root;
}

export function spawnUnits(doc: JSONValue, options: SpawnUnitsOptions): JSONValue {
  let current = doc;

  const archerCount = Math.max(0, options.archers ?? 0);
  if (archerCount > 0) {
    current = spawnArchers(current, { ...options, count: archerCount });
  }

  const workerCount = Math.max(0, options.workers ?? 0);
  if (workerCount > 0) {
    current = spawnWorkers(current, { ...options, count: workerCount });
  }

  const pikemanCount = Math.max(0, options.pikemen ?? 0);
  if (pikemanCount > 0) {
    current = spawnPikemen(current, { ...options, count: pikemanCount });
  }

  return current;
}
